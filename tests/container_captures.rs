//! T04 opt-in execution through the public boundary, without reference execution.
use rml32::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
include!("boundary_support.rs.inc");
const SCHEMA: GlobalBankSchema = GlobalBankSchema { slots: 0 };
fn object_code(flags: i8, target: i16) -> ProgramImage {
    image(
        &[
            fun(0, 1, 0, 9, &[(0, 0, 0x37), (target, flags, 0x17)]),
            fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
            fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
            fun(3, 0, 0, 9, &[(4, 0, 0x17)]),
            fun(4, 0, 0, 2, &[(42, 0, 0x3b)]),
        ],
        0,
        &[],
    )
}
fn finish(vm: &mut Vm) -> HostValue {
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    vm.take_returned().unwrap()
}
fn register(store: &mut ObjectStore, new: bool, image: ProgramImage) -> ObjectId {
    let prepared = if new {
        prepare_program_with_container_captures(image, SCHEMA)
    } else {
        prepare_program_with_callable_values(image, SCHEMA)
    }
    .unwrap();
    let p = store.register_program(prepared).unwrap();
    store.create(p, &[]).unwrap()
}
fn container(store: &ObjectStore, mapping: bool) -> HostValue {
    if mapping {
        store
            .mapping(&[(ValueInput::Int(0), ValueInput::Int(7))])
            .unwrap()
    } else {
        store.array(&[ValueInput::Int(7)]).unwrap()
    }
}
#[test]
fn t04_public_object_global_and_host_capture_matrix_keeps_header_identity() {
    for mapping in [false, true] {
        // Object -> Object/Global/Host; Global -> Global/Host.
        for (global_creator, host, global_target) in [
            (false, false, false),
            (false, false, true),
            (false, true, false),
            (true, false, true),
            (true, true, false),
        ] {
            let mut store = ObjectStore::new().unwrap();
            store.register_globals(&[]).unwrap();
            let calls = Rc::new(Cell::new(0));
            let count = calls.clone();
            store
                .register_host_function(
                    0,
                    HostFunction {
                        name: "t04".into(),
                        signature: HostSignature {
                            required: vec![HostType::Any],
                            optional: vec![],
                            rest: None,
                            returns: HostReturn::Mixed,
                        },
                        callback: Rc::new(move |_, _, _| {
                            count.set(count.get() + 1);
                            Ok(HostResult::Borrowed(0))
                        }),
                    },
                )
                .unwrap();
            let g = global_image(
                &[
                    fun(
                        559,
                        1,
                        0,
                        9,
                        &[
                            (0, 0, 0x37),
                            (
                                if host { 0 } else { 560 },
                                if host { -63 } else { -127 },
                                0x17,
                            ),
                        ],
                    ),
                    fun(560, 1, 0, 255, &[(0, 0, 0x37)]),
                ],
                0,
                &[],
            );
            let g = store
                .register_program(
                    prepare_global_program_with_container_captures(g, SCHEMA).unwrap(),
                )
                .unwrap();
            store.bind_global_program(g).unwrap();
            let code = if global_creator {
                image(
                    &[
                        fun(0, 1, 0, 9, &[(0, 0, 0x37), (559, -127, 7)]),
                        fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
                    ],
                    0,
                    &[],
                )
            } else {
                object_code(
                    if host {
                        -63
                    } else if global_target {
                        -127
                    } else {
                        1
                    },
                    if host {
                        0
                    } else if global_target {
                        560
                    } else {
                        1
                    },
                )
            };
            let owner = register(&mut store, true, code);
            let old = register(&mut store, false, object_code(1, 1));
            let value = container(&store, mapping);
            let expected = value.snapshot().unwrap();
            let mut vm = Vm::new(store);
            vm.start(owner, 0, &[ValueInput::Handle(&value)]).unwrap();
            let f = finish(&mut vm);
            let ValueSnapshot::Function(meta) = f.snapshot().unwrap() else {
                panic!()
            };
            assert_eq!(meta.bound_count, 1);
            assert_eq!(meta.owner, if global_creator { None } else { Some(owner) });
            assert!(matches!(meta.target, FunctionTarget::Host { .. }) == host);
            vm.start(old, 2, &[ValueInput::Handle(&f)]).unwrap();
            assert_eq!(
                vm.run(100).unwrap_err().kind,
                VmErrorKind::UnsupportedFeature
            );
            assert_eq!(calls.get(), 0);
            vm.start(owner, 2, &[ValueInput::Handle(&f)]).unwrap();
            assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
            assert_eq!(calls.get(), 0);
            let result = finish(&mut vm);
            assert_eq!(result.snapshot().unwrap(), expected);
            assert_eq!(calls.get(), usize::from(host));
            assert_eq!(value.snapshot().unwrap(), expected);
        }
    }
}
#[test]
fn t04_old_caller_rejects_container_profile_but_accepts_new_scalar_and_zero_capture() {
    let mut store = ObjectStore::new().unwrap();
    let new = register(&mut store, true, object_code(1, 1));
    let old = register(&mut store, false, object_code(1, 1));
    let array = container(&store, false);
    let mut vm = Vm::new(store);
    vm.start(new, 0, &[ValueInput::Handle(&array)]).unwrap();
    let f = finish(&mut vm);
    vm.start(old, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let error = vm.run(1).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(error.pc, Some(3));
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(100).unwrap_err(), error);
    for (scalar, expected) in [
        (ValueInput::Int(9), ValueSnapshot::Int(9)),
        (
            ValueInput::Bytes(b"scalar"),
            ValueSnapshot::Bytes(b"scalar".to_vec()),
        ),
        (ValueInput::Undefined, ValueSnapshot::Undefined),
        (ValueInput::Void, ValueSnapshot::Void),
        (
            ValueInput::Real(RealBits::from_bits(0x80000000)),
            ValueSnapshot::Real(RealBits::from_bits(0x80000000)),
        ),
    ] {
        vm.start(new, 0, &[scalar]).unwrap();
        let f = finish(&mut vm);
        vm.start(old, 2, &[ValueInput::Handle(&f)]).unwrap();
        let result = finish(&mut vm);
        assert_eq!(result.snapshot().unwrap(), expected);
    }
    vm.start(new, 3, &[]).unwrap();
    let f = finish(&mut vm);
    vm.start(old, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(42));
    vm.start(old, 0, &[ValueInput::Handle(&array)]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::UnsupportedType);
}
#[test]
fn t04_new_entry_still_rejects_direct_object_function_and_foreign_roots() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(&mut store, true, object_code(1, 1));
    let mut vm = Vm::new(store);
    vm.start(o, 3, &[]).unwrap();
    let f = finish(&mut vm);
    for input in [ValueInput::Object(o), ValueInput::Handle(&f)] {
        vm.start(o, 0, &[input]).unwrap();
        vm.run(3).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(vm.stack().unwrap(), before);
    }
    let other = ObjectStore::new().unwrap();
    let foreign = other.array(&[]).unwrap();
    assert_eq!(
        vm.start(o, 0, &[ValueInput::Handle(&foreign)])
            .unwrap_err()
            .kind,
        VmErrorKind::InvalidState
    );
}
#[test]
fn t04_nested_members_and_shared_dag_remain_shallow() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(&mut store, true, object_code(1, 1));
    let mut vm = Vm::new(store);
    vm.start(o, 3, &[]).unwrap();
    let inner = finish(&mut vm);
    let child = vm
        .objects()
        .array(&[ValueInput::Handle(&inner), ValueInput::Object(o)])
        .unwrap();
    let dag = vm
        .objects()
        .array(&[ValueInput::Handle(&child), ValueInput::Handle(&child)])
        .unwrap();
    let expected = dag.snapshot().unwrap();
    vm.start(o, 0, &[ValueInput::Handle(&dag)]).unwrap();
    let f = finish(&mut vm);
    for _ in 0..2 {
        vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
        assert_eq!(finish(&mut vm).snapshot().unwrap(), expected);
    }
}
#[test]
fn t04_host_signature_failure_occurs_before_callback() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(&mut store, true, object_code(-63, 0));
    let calls = Rc::new(Cell::new(0));
    let count = calls.clone();
    store
        .register_host_function(
            0,
            HostFunction {
                name: "typed".into(),
                signature: HostSignature {
                    required: vec![HostType::Int],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Mixed,
                },
                callback: Rc::new(move |_, _, _| {
                    count.set(count.get() + 1);
                    Ok(HostResult::Void)
                }),
            },
        )
        .unwrap();
    let array = container(&store, false);
    let mut vm = Vm::new(store);
    vm.start(o, 0, &[ValueInput::Handle(&array)]).unwrap();
    let f = finish(&mut vm);
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
    assert_eq!(calls.get(), 0);
    assert_eq!(vm.stack().unwrap(), before);
}
#[test]
fn t04_host_post_callback_failure_keeps_saved_root_and_never_replays() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(&mut store, true, object_code(-63, 0));
    let calls = Rc::new(Cell::new(0));
    let count = calls.clone();
    let saved = Rc::new(RefCell::new(None));
    let keep = saved.clone();
    store
        .register_host_function(
            0,
            HostFunction {
                name: "once".into(),
                signature: HostSignature {
                    required: vec![HostType::Any],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Exact(HostType::Int),
                },
                callback: Rc::new(move |_, scope, args| {
                    count.set(count.get() + 1);
                    *keep.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
                    Ok(HostResult::Borrowed(0))
                }),
            },
        )
        .unwrap();
    let array = container(&store, false);
    let expected = array.snapshot().unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, &[ValueInput::Handle(&array)]).unwrap();
    let f = finish(&mut vm);
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let error = vm.run(1).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(calls.get(), 1);
    assert_eq!(
        saved.borrow().as_ref().unwrap().snapshot().unwrap(),
        expected
    );
    assert_eq!(vm.run(100).unwrap_err(), error);
    assert_eq!(calls.get(), 1);
}

#[test]
fn t04_member_write_cannot_close_function_container_cycle() {
    for mapping in [false, true] {
        // Use the same prepared maker plus a writer with a temporary container reference.
        let code = image(
            &[
                fun(0, 1, 0, 9, &[(0, 0, 0x37), (1, 1, 0x17)]),
                fun(1, 1, 0, 255, &[(0, 0, 0x37)]),
                fun(
                    5,
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
                ),
            ],
            0,
            &[],
        );
        let mut store = ObjectStore::new().unwrap();
        let o = register(&mut store, true, code);
        let value = container(&store, mapping);
        let expected = value.snapshot().unwrap();
        let mut vm = Vm::new(store);
        vm.start(o, 0, &[ValueInput::Handle(&value)]).unwrap();
        let f = finish(&mut vm);
        vm.start(o, 5, &[ValueInput::Handle(&value), ValueInput::Handle(&f)])
            .unwrap();
        vm.run(7).unwrap();
        let before = vm.stack().unwrap();
        let error = vm.run(1).unwrap_err();
        assert_eq!(error.kind, VmErrorKind::UnsupportedType);
        assert_eq!(error.pc, Some(7));
        assert_eq!(value.snapshot().unwrap(), expected);
        assert_eq!(vm.stack().unwrap(), before);
    }
}
#[test]
fn t04_host_container_result_graph_limit_is_checked_after_single_callback() {
    let mut store = ObjectStore::with_limits(ValueLimits {
        max_graph_nodes: 6,
        ..ValueLimits::default()
    })
    .unwrap();
    let o = register(&mut store, true, object_code(-63, 0));
    let array = container(&store, false);
    let calls = Rc::new(Cell::new(0));
    let count = calls.clone();
    let saved = Rc::new(RefCell::new(None));
    let keep = saved.clone();
    store
        .register_host_function(
            0,
            HostFunction {
                name: "graph".into(),
                signature: HostSignature {
                    required: vec![HostType::Any],
                    optional: vec![],
                    rest: None,
                    returns: HostReturn::Mixed,
                },
                callback: Rc::new(move |_, scope, args| {
                    count.set(count.get() + 1);
                    *keep.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
                    Ok(HostResult::Owned(
                        scope.array(&[ValueInput::Int(1); 5]).unwrap(),
                    ))
                }),
            },
        )
        .unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, &[ValueInput::Handle(&array)]).unwrap();
    let f = finish(&mut vm);
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let error = vm.run(1).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::Limit);
    assert_eq!(error.pc, Some(3));
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(calls.get(), 1);
    assert_eq!(
        saved.borrow().as_ref().unwrap().snapshot().unwrap(),
        array.snapshot().unwrap()
    );
    assert_eq!(vm.run(100).unwrap_err(), error);
    assert_eq!(calls.get(), 1);
}

#[test]
fn t04_missing_mapping_query_does_not_insert_or_change_capture() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(
        &mut store,
        true,
        image(
            &[
                fun(0, 1, 0, 9, &[(0, 0, 0x37), (1, 1, 0x17)]),
                fun(1, 1, 0, 255, &[(0, 0, 0x37), (999, 0, 0x3b), (0, 0, 0x43)]),
                fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
            ],
            0,
            &[],
        ),
    );
    let map = container(&store, true);
    let expected = map.snapshot().unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, &[ValueInput::Handle(&map)]).unwrap();
    let f = finish(&mut vm);
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(
        finish(&mut vm).snapshot().unwrap(),
        ValueSnapshot::Undefined
    );
    assert_eq!(map.snapshot().unwrap(), expected);
}

#[test]
fn t04_container_bound_arguments_precede_new_arguments_and_keep_scalar_bits() {
    let mut store = ObjectStore::new().unwrap();
    let o = register(
        &mut store,
        true,
        image(
            &[
                fun(0, 2, 0, 9, &[(1, 0, 0x37), (0, 0, 0x37), (1, 2, 0x17)]),
                fun(1, 3, 0, 6, &[(0, 0, 0x15)]),
                fun(2, 2, 0, 6, &[(1, 0, 0x37), (0, 0, 0x37), (0, 2, 5)]),
            ],
            0,
            &[],
        ),
    );
    let map = container(&store, true);
    let expected = map.snapshot().unwrap();
    let mut vm = Vm::new(store);
    let bits = RealBits::from_bits(0x80000000);
    vm.start(o, 0, &[ValueInput::Handle(&map), ValueInput::Real(bits)])
        .unwrap();
    let f = finish(&mut vm);
    vm.start(o, 2, &[ValueInput::Handle(&f), ValueInput::Bytes(b"new")])
        .unwrap();
    let ValueSnapshot::Array { elements, .. } = finish(&mut vm).snapshot().unwrap() else {
        panic!()
    };
    assert_eq!(
        elements,
        vec![
            expected,
            ValueSnapshot::Real(bits),
            ValueSnapshot::Bytes(b"new".to_vec())
        ]
    );
}

#[test]
fn t04_callee_uses_its_own_preparation_capability() {
    let mut store = ObjectStore::new().unwrap();
    let maker = register(&mut store, true, object_code(1, 1));
    let old = register(
        &mut store,
        false,
        image(
            &[
                fun(0, 0, 0, 9, &[(1, 0, 0x17)]),
                fun(1, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
            ],
            0,
            &[],
        ),
    );
    let caller = register(
        &mut store,
        true,
        image(
            &[fun(0, 2, 0, 255, &[(1, 0, 0x37), (0, 0, 0x37), (0, 2, 5)])],
            0,
            &[],
        ),
    );
    let array = container(&store, false);
    let mut vm = Vm::new(store);
    vm.start(maker, 0, &[ValueInput::Handle(&array)]).unwrap();
    let container_fn = finish(&mut vm);
    vm.start(old, 0, &[]).unwrap();
    let scalar_wrapper = finish(&mut vm);
    vm.start(
        caller,
        0,
        &[
            ValueInput::Handle(&scalar_wrapper),
            ValueInput::Handle(&container_fn),
        ],
    )
    .unwrap();
    let error = vm.run(100).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::UnsupportedFeature);
    assert_eq!(error.function, Some(1));
    assert_eq!(error.pc, Some(3));
}
