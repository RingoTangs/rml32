//! Bounded production mode: managed inheritance, callable values and loop/catch.
use rml32::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
include!("boundary_support.rs.inc");
const ALL: LifecycleBudget = LifecycleBudget {
    host_roots: 32,
    objects: 8,
    metadata_entries: 4096,
};
const ZERO: LifecycleBudget = LifecycleBudget {
    host_roots: 0,
    objects: 0,
    metadata_entries: 0,
};
fn store() -> ObjectStore {
    ObjectStore::with_bounded_lifecycle(
        ValueLimits::default(),
        LifecycleLimits {
            max_objects: 2,
            max_function_heads: 2,
            max_ordinary_refs_per_object: 6,
            max_exec_refs_per_object: 2,
            max_owners_per_header: 8,
        },
    )
    .unwrap()
}
fn finish(vm: &mut Vm) -> HostValue {
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    vm.take_returned().unwrap()
}
fn layer(
    create: i16,
    destruct: i16,
    marker: i16,
    vars: usize,
    fail: bool,
    rollback_fail: bool,
) -> ProgramImage {
    let mut p = image(
        &[
            fun(create, 0, 0, 1, if fail { &[(0, 0, 0x33)] } else { &[] }),
            fun(
                destruct,
                0,
                0,
                1,
                if rollback_fail { &[(0, 0, 0x33)] } else { &[] },
            ),
        ],
        vars,
        &[],
    );
    p.functions[0].name = Some(ByteString::new(b"create\0"));
    p.functions[1].name = Some(ByteString::new(b"destruct\0"));
    p.init.nodes = vec![(marker, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)];
    p.init.nodes_offset = 9000;
    p
}
fn family(fail: bool) -> ProgramCatalog {
    let a = layer(0, 1, 10, 1, false, fail);
    let b = layer(0, 1, 20, 1, false, fail);
    let mut root = layer(4, 5, 30, 3, fail, fail);
    root.constitution_count = Some(3);
    for (name, p, bases) in [
        (b"a".as_slice(), &a, [1, 0, 1]),
        (b"b".as_slice(), &b, [2, 2, 2]),
    ] {
        root.inherits.push(program::Inherit {
            name: Some(ByteString::new(name)),
            offsets: [
                bases[0],
                bases[1],
                bases[2],
                p.constitution_count.unwrap(),
                p.max_functions,
                p.max_vars,
            ],
        });
    }
    let mut c = ProgramCatalog::new();
    for (key, p, create, destruct) in [("a", a, 0, 1), ("b", b, 0, 1), ("root", root, 4, 5)] {
        c.insert(key, p).unwrap();
        c.bind_lifecycle(
            key,
            LifecycleBinding {
                create: Some(create),
                destruct: Some(destruct),
            },
        )
        .unwrap();
    }
    c.bind_inherit("root", b"a", "a");
    c.bind_inherit("root", b"b", "b");
    c
}
#[test]
fn bounded_multilayer_construction_and_rollback_preserve_primary_failure() {
    for fail in [false, true] {
        let mut s = store();
        let p = s
            .register_program(
                prepare_constructible_program(&family(fail), "root", LinkLimits::default())
                    .unwrap(),
            )
            .unwrap();
        let mut vm = Vm::new(s);
        let o = vm.begin_construct(p, None).unwrap();
        if fail {
            let error = vm.run(1000).unwrap_err();
            assert_eq!(error.kind, VmErrorKind::StackUnderflow);
            assert_eq!(
                error.lifecycle.as_ref().unwrap().phase,
                LifecyclePhase::Create
            );
            assert_eq!(vm.rollback_errors().len(), 3);
            assert_eq!(vm.run(100).unwrap_err(), error);
            assert_eq!(vm.maintain_lifecycle(ZERO).unwrap().queued_objects, 1);
            let clean = vm.maintain_lifecycle(ALL).unwrap();
            assert_eq!(clean.drained_objects, 1);
            assert_eq!(clean.reaped_objects, 1);
        } else {
            assert_eq!(
                vm.run(1000).unwrap(),
                RunOutcome::Finished(CompletionMeta::Constructed(o))
            );
            assert_eq!(
                vm.objects().get(o).unwrap().variables().unwrap(),
                vec![
                    ValueSnapshot::Int(30),
                    ValueSnapshot::Int(10),
                    ValueSnapshot::Int(20)
                ]
            );
            assert_eq!(vm.frame_depth(), 0);
        }
    }
}
#[test]
fn bounded_global_host_function_return_and_post_callback_failure_do_not_replay() {
    let mut s = store();
    s.register_globals(&[]).unwrap();
    let count = Rc::new(Cell::new(0));
    let calls = count.clone();
    let saved = Rc::new(RefCell::new(None));
    let keep = saved.clone();
    s.register_host_function(
        0,
        HostFunction {
            name: "borrowed".into(),
            signature: HostSignature {
                required: vec![HostType::Any],
                optional: vec![],
                rest: None,
                returns: HostReturn::Mixed,
            },
            callback: Rc::new(move |_, scope, args| {
                calls.set(calls.get() + 1);
                *keep.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
                Ok(HostResult::Borrowed(0))
            }),
        },
    )
    .unwrap();
    let g = s
        .register_program(
            prepare_global_program_with_container_captures(
                global_image(
                    &[fun(559, 1, 0, 9, &[(0, 0, 0x37), (0, -63, 0x17)])],
                    0,
                    &[],
                ),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )
        .unwrap();
    s.bind_global_program(g).unwrap();
    let p = s
        .register_program(
            prepare_program_with_container_captures(
                image(
                    &[
                        fun(0, 1, 0, 9, &[(0, 0, 0x37), (559, -127, 7)]),
                        fun(1, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
                    ],
                    0,
                    &[],
                ),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    let array = s.array(&[ValueInput::Int(7)]).unwrap();
    let expected = array.snapshot().unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Handle(&array)]).unwrap();
    let f = finish(&mut vm);
    let ValueSnapshot::Function(info) = f.snapshot().unwrap() else {
        panic!()
    };
    assert_eq!(info.owner, None);
    assert!(matches!(info.target, FunctionTarget::Host { slot: 0 }));
    vm.start(o, 1, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), expected);
    assert_eq!(count.get(), 1);
    vm.objects_mut().remove_host_function(0).unwrap();
    let calls = count.clone();
    vm.objects_mut()
        .register_host_function(
            0,
            HostFunction {
                name: "bad-return".into(),
                signature: HostSignature {
                    required: vec![HostType::Any],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Exact(HostType::Int),
                },
                callback: Rc::new(move |_, _, _| {
                    calls.set(calls.get() + 1);
                    Ok(HostResult::Borrowed(0))
                }),
            },
        )
        .unwrap();
    vm.start(o, 1, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let error = vm.run(1).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(count.get(), 2);
    assert_eq!(vm.run(100).unwrap_err(), error);
    assert_eq!(count.get(), 2);
    assert_eq!(
        saved.borrow().as_ref().unwrap().snapshot().unwrap(),
        expected
    );
}
fn mapping_loop(body: &[Node]) -> Fun {
    let mut nodes = vec![
        (-1, 0, 0x39),
        (0, 0, 0x37),
        (997, 0, 0x3b),
        (-2, 0, 0x23),
        ((body.len() + 1) as i16, -2, 0x12),
    ];
    nodes.extend(body);
    nodes.extend([(-((body.len() + 1) as i16), -2, 0x50), (-1, 0, 0x37)]);
    fun(0, 1, 2, 255, &nodes)
}
fn catch(index: i16, body: &[Node]) -> Fun {
    let mut nodes = vec![((body.len() + 2) as i16, 0, 0x2c)];
    nodes.extend(body);
    nodes.extend([(0, 0, 0x1a), (1, 0, 0x1f), (0, 0, 0x0e)]);
    fun(index, 1, 0, 255, &nodes)
}
#[test]
fn bounded_mapping_loop_allocates_output_and_catch_unwinds_real_leases() {
    for caught in [false, true] {
        let mut s = store();
        let functions = if caught {
            let mut leaf = mapping_loop(&[(9, 0, 0x3b), (0, 0, 0x3b), (0, 0, 0x19)]);
            leaf.index = 1;
            vec![catch(0, &[(0, 0, 0x37), (1, 1, 7)]), leaf]
        } else {
            vec![mapping_loop(&[])]
        };
        let p = s
            .register_program(prepare_program(image(&functions, 0, &[])).unwrap())
            .unwrap();
        let o = s.create(p, &[]).unwrap();
        let map = s
            .mapping(&[
                (ValueInput::Int(0), ValueInput::Int(0)),
                (ValueInput::Int(1), ValueInput::Int(1)),
                (ValueInput::Int(2), ValueInput::Int(2)),
            ])
            .unwrap();
        let mut vm = Vm::new(s);
        vm.start(o, 0, &[ValueInput::Handle(&map)]).unwrap();
        let result = finish(&mut vm);
        if caught {
            assert_eq!(result.snapshot().unwrap(), ValueSnapshot::Int(1));
        } else {
            let ValueSnapshot::Array { elements, .. } = result.snapshot().unwrap() else {
                panic!()
            };
            assert_eq!(
                elements,
                vec![
                    ValueSnapshot::Int(0),
                    ValueSnapshot::Int(0),
                    ValueSnapshot::Int(2)
                ]
            );
        }
        assert_eq!(vm.frame_depth(), 0);
        vm.objects_mut().remove(o).unwrap();
        assert!(vm.objects().get(o).is_err());
        assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
    }
}

#[test]
fn bounded_mapping_loop_reuses_output_header_and_preserves_zero_step() {
    let mut s = store();
    let mut f = mapping_loop(&[]);
    f.args = 2;
    f.nodes
        .splice(2..2, [(1, 0, 0x37), (-1, 0, 0x4d), (0, 0, 0x33)]);
    let p = s
        .register_program(prepare_program(image(&[f], 0, &[])).unwrap())
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    let map = s
        .mapping(&[
            (ValueInput::Int(0), ValueInput::Int(0)),
            (ValueInput::Int(1), ValueInput::Int(1)),
        ])
        .unwrap();
    let alias = s.array(&[ValueInput::Int(9); 3]).unwrap();
    let ValueSnapshot::Array { id, .. } = alias.snapshot().unwrap() else {
        panic!()
    };
    let mut vm = Vm::new(s);
    vm.start(
        o,
        0,
        &[ValueInput::Handle(&map), ValueInput::Handle(&alias)],
    )
    .unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    let ValueSnapshot::Array { elements, .. } = alias.snapshot().unwrap() else {
        panic!()
    };
    assert_eq!(elements[2], ValueSnapshot::Int(9));
    let result = finish(&mut vm);
    let ValueSnapshot::Array {
        id: after,
        elements,
    } = result.snapshot().unwrap()
    else {
        panic!()
    };
    assert_eq!(id, after);
    assert_eq!(
        elements,
        vec![
            ValueSnapshot::Int(0),
            ValueSnapshot::Int(0),
            ValueSnapshot::Int(1)
        ]
    );
    assert_eq!(alias.snapshot().unwrap(), result.snapshot().unwrap());
}

#[test]
fn bounded_mapping_assignment_peak_rejection_preserves_prior_member_creation() {
    let mut s = ObjectStore::with_bounded_lifecycle(
        ValueLimits::default(),
        LifecycleLimits {
            max_objects: 2,
            max_function_heads: 0,
            max_ordinary_refs_per_object: 4,
            max_exec_refs_per_object: 1,
            max_owners_per_header: 8,
        },
    )
    .unwrap();
    let p = s
        .register_program(
            prepare_program(image(
                &[fun(
                    0,
                    2,
                    0,
                    255,
                    &[
                        (1, 0, 0x37),
                        (0, 0, 0x37),
                        (0, 0, 0x13),
                        (0, 0, 0x3b),
                        (0, 0, 0x44),
                        (0, 0, 3),
                    ],
                )],
                0,
                &[],
            ))
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    let victim = s.create(p, &[]).unwrap();
    let map = s.mapping(&[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(
        o,
        0,
        &[ValueInput::Handle(&map), ValueInput::Object(victim)],
    )
    .unwrap();
    vm.run(7).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(
        map.get(ValueInput::Int(0)).unwrap(),
        Some(ValueSnapshot::Undefined)
    );
    let error = vm.run(1).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::Limit);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(
        map.get(ValueInput::Int(0)).unwrap(),
        Some(ValueSnapshot::Undefined)
    );
    assert_eq!(vm.run(100).unwrap_err(), error);
}
#[test]
fn bounded_mapping_inserted_object_member_keeps_and_releases_its_owner() {
    let mut s = store();
    let p = s
        .register_program(
            prepare_program(image(
                &[fun(
                    0,
                    2,
                    0,
                    255,
                    &[
                        (1, 0, 0x37),
                        (0, 0, 0x37),
                        (0, 0, 0x13),
                        (0, 0, 0x3b),
                        (0, 0, 0x44),
                        (0, 0, 3),
                    ],
                )],
                0,
                &[],
            ))
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    let victim = s.create(p, &[]).unwrap();
    let map = s.mapping(&[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(
        o,
        0,
        &[ValueInput::Handle(&map), ValueInput::Object(victim)],
    )
    .unwrap();
    let returned = finish(&mut vm);
    drop(returned);
    vm.objects_mut().remove(victim).unwrap();
    assert_eq!(
        map.get(ValueInput::Int(0)).unwrap(),
        Some(ValueSnapshot::Object(victim))
    );
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reaped_objects, 0);
    let mut s = vm.into_objects();
    drop(map);
    assert_eq!(s.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
}
