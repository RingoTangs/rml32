//! Real prepared Host Function values through a private checked lifecycle mode.
use super::*;
use crate::{ProgramImage, ProgramMode, parse_program};
use serde_json::{Value as Json, json};
include!("../tests/boundary_support.rs.inc");
const SCHEMA: crate::GlobalBankSchema = crate::GlobalBankSchema { slots: 0 };
fn store() -> ObjectStore {
    ObjectStore::with_test_host_function_lifecycle(ValueLimits::default()).unwrap()
}
fn callable_image() -> ProgramImage {
    image(
        &[
            fun(0, 0, 0, 9, &[(0, -64, 0x17)]),
            fun(1, 1, 0, 9, &[(0, 0, 0x37), (0, -63, 0x17)]),
            fun(2, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
            fun(3, 1, 0, 255, &[(0, 0, 0x37), (0, -63, 0x17), (0, 1, 5)]),
            fun(4, 2, 0, 255, &[(1, 0, 0x37), (0, 0, 0x37), (0, 2, 5)]),
            fun(5, 0, 0, 255, &[(0, -64, 0x17), (0, 1, 5)]),
            fun(6, 1, 0, 255, &[(0, 0, 0x37)]),
            fun(7, 0, 0, 9, &[(559, -128, 7)]),
            fun(8, 1, 0, 255, &[(0, 0, 0x37), (560, -127, 7)]),
        ],
        0,
        &[],
    )
}
fn binding(callback: HostCallback) -> HostFunction {
    HostFunction {
        name: "2bg-synthetic".into(),
        signature: host::HostSignature {
            required: vec![],
            optional: vec![],
            rest: Some(host::HostType::Any),
            returns: host::HostReturn::Mixed,
        },
        callback,
    }
}
fn echo() -> HostCallback {
    Rc::new(|_, scope, args| {
        if args.is_empty() {
            Ok(HostResult::Owned(
                scope.value(ValueInput::Int(9)).map_err(host_error)?,
            ))
        } else {
            Ok(HostResult::Borrowed(0))
        }
    })
}
fn machine(callback: HostCallback, container: bool) -> (Vm, ObjectId) {
    let mut s = store();
    let p = s
        .register_program(
            if container {
                crate::prepare_program_with_container_captures(callable_image(), SCHEMA)
            } else {
                crate::prepare_program_with_callable_values(callable_image(), SCHEMA)
            }
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    s.register_host_function(0, binding(callback)).unwrap();
    (Vm::new(s), o)
}
fn finish(vm: &mut Vm) -> HostValue {
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    vm.take_returned().unwrap()
}
fn created(vm: &mut Vm, o: ObjectId, input: Option<ValueInput<'_>>) -> HostValue {
    if let Some(value) = input {
        vm.start(o, 1, &[value]).unwrap()
    } else {
        vm.start(o, 0, &[]).unwrap()
    }
    finish(vm)
}
fn view(root: &HostValue) -> crate::value::FunctionValue {
    let value::Value::Function(f) = root.registry.raw(root).unwrap() else {
        panic!("function root required")
    };
    f
}
fn info(root: &HostValue) -> FunctionSnapshot {
    let ValueSnapshot::Function(f) = root.snapshot().unwrap() else {
        panic!()
    };
    f
}
fn audit(vm: &Vm) -> Json {
    let entries = vm.registry.entries.borrow();
    let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    let inner = vm.inner.as_ref().unwrap();
    inner.lifecycle_audit_host_for_test(&roots);
    json!({"life":inner.objects().lifecycle_snapshot_for_test(),"frames":inner.managed_snapshot_for_test()["frames"],"roots":entries.len()})
}
fn refs(s: &Json, o: ObjectId) -> u64 {
    s["life"]["objects"][o.get().to_string()]["refs"]
        .as_u64()
        .unwrap()
}
fn state(s: &Json, o: ObjectId) -> &str {
    s["life"]["objects"][o.get().to_string()]["state"]
        .as_str()
        .unwrap()
}
fn container(vm: &Vm, mapping: bool, o: Option<ObjectId>) -> HostValue {
    let value = o.map_or(ValueInput::Int(7), ValueInput::Object);
    if mapping {
        vm.objects()
            .mapping(&[(ValueInput::Int(0), value)])
            .unwrap()
    } else {
        vm.objects().array(&[value]).unwrap()
    }
}
#[test]
fn object_head_copies_have_one_creator_and_observations_have_no_owners() {
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, None);
    assert_eq!(refs(&audit(&vm), o), 1);
    let frozen = f.snapshot().unwrap();
    let _ = format!("{f:?}");
    let copied = f.retain().unwrap();
    assert_eq!(info(&f).owners, 2);
    assert_eq!(refs(&audit(&vm), o), 1);
    drop(copied);
    assert_eq!(f.snapshot().unwrap(), frozen);
    drop(f);
    assert_eq!(refs(&audit(&vm), o), 0);
    assert_eq!(state(&audit(&vm), o), "alive");
}
#[test]
fn unique_scalar_call_retires_target_capture_and_creator_once() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Borrowed(0))
        }),
        false,
    );
    vm.start(o, 3, &[ValueInput::Int(7)]).unwrap();
    vm.run(4).unwrap();
    let alias = vm.inner.as_ref().unwrap().host_target_alias_for_test();
    let value::Value::Function(f) = alias.read().unwrap() else {
        panic!()
    };
    assert_eq!(f.0.counter.owners(), 1);
    let before = audit(&vm)["frames"].clone();
    vm.run(1).unwrap();
    assert_eq!(calls.get(), 1);
    assert_eq!(alias.read().unwrap(), value::Value::Undefined);
    assert_eq!(f.0.bound[0].read().unwrap(), value::Value::Undefined);
    assert_eq!(refs(&audit(&vm), o), 0);
    assert_eq!(audit(&vm)["frames"], before);
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(7));
}
#[test]
fn shared_scalar_call_keeps_capture_and_one_creator() {
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, Some(ValueInput::Int(7)));
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(7));
    assert_eq!(view(&f).0.bound[0].read().unwrap(), value::Value::Int(7));
    assert_eq!(info(&f).owners, 1);
    assert_eq!(refs(&audit(&vm), o), 1);
}
#[test]
fn unique_container_return_and_callback_retain_preserve_members() {
    for mapping in [false, true] {
        let saved = Rc::new(RefCell::new(None));
        let out = saved.clone();
        let (mut vm, o) = machine(
            Rc::new(move |_, scope, args| {
                *out.borrow_mut() = Some(scope.retain(&args[0]).map_err(host_error)?);
                Ok(HostResult::Borrowed(0))
            }),
            true,
        );
        let a = container(&vm, mapping, Some(o));
        let expected = a.snapshot().unwrap();
        vm.start(o, 3, &[ValueInput::Handle(&a)]).unwrap();
        vm.run(4).unwrap();
        let alias = vm.inner.as_ref().unwrap().host_target_alias_for_test();
        let value::Value::Function(f) = alias.read().unwrap() else {
            panic!()
        };
        assert_eq!(f.0.counter.owners(), 1);
        vm.run(1).unwrap();
        assert_eq!(f.0.bound[0].read().unwrap(), value::Value::Undefined);
        let result = finish(&mut vm);
        assert_eq!(result.snapshot().unwrap(), expected);
        assert_eq!(
            saved.borrow().as_ref().unwrap().snapshot().unwrap(),
            expected
        );
        assert_eq!(refs(&audit(&vm), o), 1);
        drop(a);
        drop(result);
        drop(saved.borrow_mut().take());
        assert_eq!(refs(&audit(&vm), o), 0);
    }
}
#[test]
fn shared_array_mapping_calls_preserve_identity_and_member_slot_counts() {
    for mapping in [false, true] {
        let (mut vm, o) = machine(echo(), true);
        let a = container(&vm, mapping, Some(o));
        let f = created(&mut vm, o, Some(ValueInput::Handle(&a)));
        assert_eq!(refs(&audit(&vm), o), 2);
        let expected = a.snapshot().unwrap();
        for _ in 0..2 {
            vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
            assert_eq!(finish(&mut vm).snapshot().unwrap(), expected);
            audit(&vm);
        }
        drop(a);
        assert_eq!(refs(&audit(&vm), o), 2);
        drop(f);
        assert_eq!(refs(&audit(&vm), o), 0);
    }
}
#[test]
fn captures_precede_new_arguments_in_order() {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let out = seen.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, args| {
            *out.borrow_mut() = args
                .iter()
                .map(BorrowedValue::snapshot)
                .collect::<Result<_, _>>()
                .map_err(host_error)?;
            Ok(HostResult::Borrowed(1))
        }),
        false,
    );
    let f = created(&mut vm, o, Some(ValueInput::Int(7)));
    vm.start(o, 4, &[ValueInput::Handle(&f), ValueInput::Int(9)])
        .unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(9));
    assert_eq!(
        *seen.borrow(),
        vec![ValueSnapshot::Int(7), ValueSnapshot::Int(9)]
    );
    audit(&vm);
}
#[test]
fn binding_removal_and_replacement_are_observed_at_call_time() {
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, None);
    vm.objects_mut().remove_host_function(0).unwrap();
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert!(vm.run(1).is_err());
    assert_eq!(vm.stack().unwrap(), before);
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(|_, scope, _| {
                Ok(HostResult::Owned(
                    scope.value(ValueInput::Int(20)).map_err(host_error)?,
                ))
            })),
        )
        .unwrap();
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(20));
    audit(&vm);
}
#[test]
fn head_does_not_retain_removed_callback_or_signature() {
    let cb = echo();
    let weak = Rc::downgrade(&cb);
    let (mut vm, o) = machine(cb, false);
    let f = created(&mut vm, o, None);
    vm.objects_mut().remove_host_function(0).unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(info(&f).target, FunctionTarget::Host { slot: 0 });
    let copy = f.retain().unwrap();
    assert_eq!(info(&copy).owners, 2);
    audit(&vm);
}
#[test]
fn changed_signature_fails_before_callback() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, None);
    vm.objects_mut().remove_host_function(0).unwrap();
    let mut b = binding(Rc::new(move |_, _, _| {
        c.set(c.get() + 1);
        Ok(HostResult::Void)
    }));
    b.signature.required = vec![host::HostType::Int];
    b.signature.rest = None;
    vm.objects_mut().register_host_function(0, b).unwrap();
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::ArgumentCount);
    assert_eq!(calls.get(), 0);
    audit(&vm);
}
#[test]
fn creation_requires_binding_and_failure_keeps_source_slots() {
    let (mut vm, o) = machine(echo(), false);
    vm.objects_mut().remove_host_function(0).unwrap();
    vm.start(o, 1, &[ValueInput::Int(7)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert!(vm.run(1).is_err());
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(refs(&audit(&vm), o), 0);
}
#[test]
fn deleted_creator_rejects_call_without_callback_but_raw_retain_is_allowed() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Void)
        }),
        false,
    );
    let f = created(&mut vm, o, None);
    let p = vm.objects().get(o).unwrap().program();
    let caller = vm.objects_mut().create(p, &[]).unwrap();
    vm.start(caller, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let copy = f.retain().unwrap();
    assert_eq!(state(&audit(&vm), o), "shell");
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
    assert_eq!(calls.get(), 0);
    drop(copy);
    audit(&vm);
}
#[test]
fn pending_creator_can_call_until_its_real_frame_exits() {
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, Some(ValueInput::Int(7)));
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(state(&audit(&vm), o), "pending");
    vm.run(1).unwrap();
    assert_eq!(state(&audit(&vm), o), "pending");
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(7));
    assert_eq!(state(&audit(&vm), o), "queued");
    vm.inner
        .as_mut()
        .unwrap()
        .objects_mut()
        .drain_one_for_test()
        .unwrap();
    assert_eq!(state(&audit(&vm), o), "shell");
    drop(f);
    assert_eq!(state(&audit(&vm), o), "freed");
}
#[test]
fn callback_context_and_execution_reference_belong_to_invoker() {
    let seen = Rc::new(RefCell::new(None));
    let out = seen.clone();
    let (mut vm, creator) = machine(
        Rc::new(move |context, _, _| {
            *out.borrow_mut() = Some(context.clone());
            Ok(HostResult::Void)
        }),
        false,
    );
    let f = created(&mut vm, creator, None);
    let p = vm.objects().get(creator).unwrap().program();
    let caller = vm.objects_mut().create(p, &[]).unwrap();
    vm.start(caller, 2, &[ValueInput::Handle(&f)]).unwrap();
    vm.run(3).unwrap();
    let frames = audit(&vm)["frames"].clone();
    vm.run(1).unwrap();
    assert_eq!(seen.borrow().as_ref().unwrap().object, Some(caller));
    assert_eq!(audit(&vm)["frames"], frames);
    assert_eq!(
        audit(&vm)["life"]["objects"][creator.get().to_string()]["execs"],
        0
    );
    assert_eq!(
        audit(&vm)["life"]["objects"][caller.get().to_string()]["execs"],
        1
    );
}
fn globals(vm: &mut Vm) {
    vm.objects_mut().register_globals(&[]).unwrap();
    let p = vm
        .objects_mut()
        .register_program(
            crate::prepare_global_program_with_container_captures(
                global_image(
                    &[
                        fun(559, 0, 0, 9, &[(0, -64, 0x17)]),
                        fun(560, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
                    ],
                    0,
                    &[],
                ),
                SCHEMA,
            )
            .unwrap(),
        )
        .unwrap();
    vm.objects_mut().bind_global_program(p).unwrap();
}
#[test]
fn global_creation_has_no_creator_and_global_invocation_no_object_context() {
    let seen = Rc::new(RefCell::new(None));
    let out = seen.clone();
    let (mut vm, o) = machine(
        Rc::new(move |c, scope, _| {
            *out.borrow_mut() = Some(c.object);
            Ok(HostResult::Owned(
                scope.value(ValueInput::Int(8)).map_err(host_error)?,
            ))
        }),
        true,
    );
    globals(&mut vm);
    vm.start(o, 7, &[]).unwrap();
    let f = finish(&mut vm);
    assert_eq!(info(&f).owner, None);
    assert_eq!(refs(&audit(&vm), o), 0);
    vm.start(o, 8, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(8));
    assert_eq!(*seen.borrow(), Some(None));
    audit(&vm);
}
#[test]
fn callback_failure_preserves_sources_and_saved_root_without_replay() {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, scope, args| {
            c.set(c.get() + 1);
            *out.borrow_mut() = Some(scope.retain(&args[0]).map_err(host_error)?);
            Err(host::HostError {
                kind: VmErrorKind::InvalidState,
                message: "once".into(),
            })
        }),
        true,
    );
    let a = container(&vm, false, Some(o));
    vm.start(o, 3, &[ValueInput::Handle(&a)]).unwrap();
    vm.run(4).unwrap();
    let before = vm.stack().unwrap();
    let f = vm
        .inner
        .as_ref()
        .unwrap()
        .host_target_alias_for_test()
        .read()
        .unwrap();
    let first = vm.run(1).unwrap_err();
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(100).unwrap_err(), first);
    assert_eq!(calls.get(), 1);
    let value::Value::Function(f) = f else {
        panic!()
    };
    assert!(matches!(
        f.0.bound[0].read().unwrap(),
        value::Value::Array(_)
    ));
    assert!(saved.borrow().as_ref().unwrap().snapshot().is_ok());
    audit(&vm);
}
#[test]
fn wrong_return_type_keeps_captures_target_and_pc() {
    let (mut vm, o) = machine(echo(), true);
    vm.objects_mut().remove_host_function(0).unwrap();
    let mut b = binding(echo());
    b.signature.returns = host::HostReturn::Exact(host::HostType::Int);
    vm.objects_mut().register_host_function(0, b).unwrap();
    let a = container(&vm, true, Some(o));
    vm.start(o, 3, &[ValueInput::Handle(&a)]).unwrap();
    vm.run(4).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
    assert_eq!(vm.stack().unwrap(), before);
    audit(&vm);
}
#[test]
fn post_callback_workspace_failure_rolls_back_without_replay() {
    for after in 0..64 {
        let calls = Rc::new(Cell::new(0));
        let c = calls.clone();
        let (mut vm, o) = machine(
            Rc::new(move |_, _, _| {
                c.set(c.get() + 1);
                Ok(HostResult::Borrowed(0))
            }),
            false,
        );
        vm.start(o, 3, &[ValueInput::Int(7)]).unwrap();
        vm.run(4).unwrap();
        let before = vm.stack().unwrap();
        let before_life = audit(&vm);
        let result = crate::copy_plan::with_workspace_failure(after, || vm.run(1));
        if result.is_err() && calls.get() == 1 {
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(audit(&vm), before_life);
            assert!(vm.run(1).is_err());
            assert_eq!(calls.get(), 1);
            return;
        }
    }
    panic!("no post-callback fault reached");
}
#[test]
fn borrowed_capture_rejects_commit_after_one_callback() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Borrowed(0))
        }),
        false,
    );
    vm.start(o, 3, &[ValueInput::Int(7)]).unwrap();
    vm.run(4).unwrap();
    let value::Value::Function(f) = vm
        .inner
        .as_ref()
        .unwrap()
        .host_target_alias_for_test()
        .read()
        .unwrap()
    else {
        panic!()
    };
    let before = vm.stack().unwrap();
    f.0.bound[0].with_read_borrow_for_test(|| assert!(vm.run(1).is_err()));
    assert_eq!(calls.get(), 1);
    assert_eq!(vm.stack().unwrap(), before);
    audit(&vm);
}
#[test]
fn pre_callback_allocation_failure_has_zero_effects() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Void)
        }),
        false,
    );
    vm.start(o, 5, &[]).unwrap();
    vm.run(3).unwrap();
    let before = audit(&vm);
    crate::cell::with_allocation_failure(0, || assert!(vm.run(1).is_err()));
    assert_eq!(calls.get(), 0);
    assert_eq!(audit(&vm), before);
}
#[test]
fn actual_head_capture_creator_and_exec_audits_detect_corruption() {
    let (mut vm, o) = machine(echo(), true);
    let a = container(&vm, false, Some(o));
    let f = created(&mut vm, o, Some(ValueInput::Handle(&a)));
    let actual = view(&f);
    let registry = vm.registry.clone();
    let entries = registry.entries.borrow();
    let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    for kind in [
        "missing_creator",
        "duplicate_creator",
        "missing_capture",
        "extra_exec",
    ] {
        registry
            .test_life
            .as_ref()
            .unwrap()
            .assert_host_corruption_detected(&actual, &roots, kind);
    }
    drop(entries);
    audit(&vm);
}
#[test]
fn scalar_capture_categories_preserve_bits() {
    for input in [
        ValueInput::Undefined,
        ValueInput::Void,
        ValueInput::Int(-8),
        ValueInput::Real(RealBits::from_bits(0x7fc01234)),
        ValueInput::Bytes(&[255, 0, 128]),
    ] {
        let (mut vm, o) = machine(echo(), false);
        let expected = vm.objects().value(input).unwrap().snapshot().unwrap();
        let f = created(&mut vm, o, Some(input));
        vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
        assert_eq!(finish(&mut vm).snapshot().unwrap(), expected);
        audit(&vm);
    }
}
#[test]
fn maximum_capture_and_argument_boundaries_preserve_atomicity() {
    for total in [127usize, 128] {
        let calls = Rc::new(Cell::new(0));
        let c = calls.clone();
        let mut s = store();
        s.register_host_function(
            0,
            binding(Rc::new(move |_, scope, args| {
                c.set(c.get() + 1);
                Ok(HostResult::Owned(
                    scope
                        .value(ValueInput::Int(args.len() as i32))
                        .map_err(host_error)?,
                ))
            })),
        )
        .unwrap();
        let mut nodes = vec![(1, 0, 0x3b); total - 63];
        nodes.extend(vec![(2, 0, 0x3b); 63]);
        nodes.extend([(0, -1, 0x17), (0, (total - 63 + 1) as i8, 5)]);
        let p = s
            .register_program(
                crate::prepare_program_with_callable_values(
                    image(&[fun(0, 0, 0, 2, &nodes)], 0, &[]),
                    SCHEMA,
                )
                .unwrap(),
            )
            .unwrap();
        let o = s.create(p, &[]).unwrap();
        let mut vm = Vm::new(s);
        vm.start(o, 0, &[]).unwrap();
        if total == 127 {
            assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(127));
            assert_eq!(calls.get(), 1);
        } else {
            vm.run(nodes.len() + 1).unwrap();
            let before = vm.stack().unwrap();
            assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
            assert_eq!(vm.stack().unwrap(), before);
            assert_eq!(calls.get(), 0);
        }
        audit(&vm);
    }
}
#[test]
fn preparation_flags_slots_and_direct_capture_gates_remain_closed() {
    for slot in [-1, 559, 32767] {
        assert!(
            crate::prepare_program_with_callable_values(
                image(&[fun(0, 0, 0, 9, &[(slot, -64, 0x17)])], 0, &[]),
                SCHEMA
            )
            .is_err()
        );
    }
    assert!(
        crate::prepare_program_with_callable_values(
            image(&[fun(0, 0, 0, 9, &[(0, 64, 0x17)])], 0, &[]),
            SCHEMA
        )
        .is_err()
    );
    let (mut vm, o) = machine(echo(), true);
    let f = created(&mut vm, o, None);
    for arg in [ValueInput::Object(o), ValueInput::Handle(&f)] {
        vm.start(o, 1, &[arg]).unwrap();
        vm.run(3).unwrap();
        let before = vm.stack().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(vm.stack().unwrap(), before);
        audit(&vm);
    }
}
#[test]
fn legacy_lifecycle_modes_reject_host_function_programs() {
    for mut s in [
        object::ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap(),
        object::ObjectStore::with_test_host_lifecycle(ValueLimits::default()).unwrap(),
        object::ObjectStore::with_test_managed_lifecycle(ValueLimits::default()).unwrap(),
    ] {
        let p = crate::prepare_program_with_callable_values(callable_image(), SCHEMA).unwrap();
        assert_eq!(
            s.register_program(p).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
        assert_eq!(s.lifecycle_snapshot_for_test()["resident"], json!([]));
    }
}
#[test]
fn foreign_function_and_container_profile_rejection_are_atomic() {
    let (mut first, o) = machine(echo(), true);
    let a = container(&first, false, None);
    let f = created(&mut first, o, Some(ValueInput::Handle(&a)));
    let (mut second, o2) = machine(echo(), true);
    let before = audit(&second);
    assert!(second.start(o2, 2, &[ValueInput::Handle(&f)]).is_err());
    assert_eq!(audit(&second), before);
    let p = first
        .objects_mut()
        .register_program(
            crate::prepare_program_with_callable_values(callable_image(), SCHEMA).unwrap(),
        )
        .unwrap();
    let caller = first.objects_mut().create(p, &[]).unwrap();
    first.start(caller, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(
        first.run(100).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    audit(&first);
}
#[test]
fn nested_function_members_share_one_creator_per_head() {
    let (mut vm, o) = machine(echo(), true);
    let inner = created(&mut vm, o, None);
    let a = vm
        .objects()
        .array(&[ValueInput::Handle(&inner), ValueInput::Handle(&inner)])
        .unwrap();
    let outer = created(&mut vm, o, Some(ValueInput::Handle(&a)));
    assert_eq!(refs(&audit(&vm), o), 2);
    assert_eq!(info(&inner).owners, 3);
    vm.start(o, 2, &[ValueInput::Handle(&outer)]).unwrap();
    let result = finish(&mut vm);
    drop(inner);
    drop(a);
    drop(result);
    assert_eq!(refs(&audit(&vm), o), 2);
    drop(outer);
    assert_eq!(refs(&audit(&vm), o), 0);
}
#[test]
fn container_member_cycle_rejection_preserves_source_graph() {
    for mapping in [false, true] {
        let (mut vm, o) = machine(echo(), true);
        let a = container(&vm, mapping, None);
        let f = created(&mut vm, o, Some(ValueInput::Handle(&a)));
        let p = vm
            .objects_mut()
            .register_program(
                crate::prepare_program_with_container_captures(
                    image(
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
                    ),
                    SCHEMA,
                )
                .unwrap(),
            )
            .unwrap();
        let caller = vm.objects_mut().create(p, &[]).unwrap();
        vm.start(caller, 0, &[ValueInput::Handle(&a), ValueInput::Handle(&f)])
            .unwrap();
        vm.run(7).unwrap();
        let before = vm.stack().unwrap();
        let expected = a.snapshot().unwrap();
        assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(a.snapshot().unwrap(), expected);
        assert_eq!(vm.stack().unwrap(), before);
        audit(&vm);
    }
}
#[test]
fn shared_container_copy_does_not_recursively_normalize_dead_members() {
    let (mut vm, o) = machine(echo(), true);
    let p = vm.objects().get(o).unwrap().program();
    let other = vm.objects_mut().create(p, &[]).unwrap();
    let a = container(&vm, false, Some(other));
    let f = created(&mut vm, o, Some(ValueInput::Handle(&a)));
    vm.objects_mut().remove(other).unwrap();
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    let r = finish(&mut vm);
    assert_eq!(r.elements().unwrap(), vec![ValueSnapshot::Object(other)]);
    assert_eq!(state(&audit(&vm), other), "shell");
}
#[test]
fn frozen_unique_branch_survives_callback_head_retain() {
    let observed = Rc::new(RefCell::new(None::<crate::value::FunctionValue>));
    let input = observed.clone();
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, scope, _| {
            let f = input.borrow().as_ref().unwrap().clone();
            *out.borrow_mut() = Some(
                scope
                    .registry
                    .publish(value::Value::Function(f))
                    .map_err(host_error)?,
            );
            Ok(HostResult::Borrowed(0))
        }),
        false,
    );
    vm.start(o, 3, &[ValueInput::Int(7)]).unwrap();
    vm.run(4).unwrap();
    let value::Value::Function(f) = vm
        .inner
        .as_ref()
        .unwrap()
        .host_target_alias_for_test()
        .read()
        .unwrap()
    else {
        panic!()
    };
    *observed.borrow_mut() = Some(f.clone());
    assert_eq!(f.0.counter.owners(), 1);
    vm.run(1).unwrap();
    assert_eq!(f.0.bound[0].read().unwrap(), value::Value::Undefined);
    assert_eq!(f.0.counter.owners(), 1);
    assert_eq!(refs(&audit(&vm), o), 1);
    drop(saved.borrow_mut().take());
    assert_eq!(refs(&audit(&vm), o), 0);
}
#[test]
fn retired_function_physical_view_cannot_resurrect_a_head() {
    let (mut vm, o) = machine(echo(), false);
    vm.start(o, 3, &[ValueInput::Int(7)]).unwrap();
    vm.run(4).unwrap();
    let value::Value::Function(f) = vm
        .inner
        .as_ref()
        .unwrap()
        .host_target_alias_for_test()
        .read()
        .unwrap()
    else {
        panic!()
    };
    vm.run(1).unwrap();
    assert_eq!(f.0.counter.owners(), 0);
    assert!(vm.registry.publish(value::Value::Function(f)).is_err());
    audit(&vm);
}
#[test]
fn extraction_rebuild_preserves_function_origin_mode_and_binding() {
    let (mut vm, o) = machine(echo(), true);
    let a = container(&vm, true, None);
    let f = created(&mut vm, o, Some(ValueInput::Handle(&a)));
    let identity = vm.registry.identity.id;
    let s = vm.into_objects();
    assert_eq!(s.registry.identity.id, identity);
    let mut vm = Vm::new(s);
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), a.snapshot().unwrap());
    audit(&vm);
}
#[test]
fn closing_store_detaches_binding_cycles_and_keeps_host_function_storage() {
    let (mut vm, o) = machine(echo(), false);
    let f = created(&mut vm, o, None);
    let s = vm.into_objects();
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let registry = Rc::downgrade(&s.registry);
    let mut s = s;
    let retained = f.retain().unwrap();
    s.remove_host_function(0).unwrap();
    s.register_host_function(
        0,
        binding(Rc::new(move |_, _, _| {
            let _ = &retained;
            Ok(HostResult::Void)
        })),
    )
    .unwrap();
    drop(s);
    assert!(probe.upgrade().is_some());
    assert!(f.retain().is_err());
    drop(f);
    assert!(probe.upgrade().is_none());
    assert!(registry.upgrade().is_none());
}
#[test]
fn host_function_inside_managed_parameter_is_owned_without_task_invocation() {
    let (mut vm, o) = machine(echo(), true);
    let f = created(&mut vm, o, None);
    let map = vm
        .objects()
        .mapping(&[(ValueInput::Int(0), ValueInput::Handle(&f))])
        .unwrap();
    let mut p = image(&[], 0, &[]);
    p.init.nodes = vec![(0, 0, 0x48)];
    let mut c = crate::ProgramCatalog::new();
    c.insert("root", p).unwrap();
    c.bind_lifecycle("root", Default::default()).unwrap();
    let p = vm
        .objects_mut()
        .register_program(
            crate::prepare_constructible_program(&c, "root", Default::default()).unwrap(),
        )
        .unwrap();
    let new = vm
        .begin_construct(p, Some(ValueInput::Handle(&map)))
        .unwrap();
    audit(&vm);
    assert_eq!(
        vm.run(1).unwrap(),
        RunOutcome::Finished(CompletionMeta::Constructed(new))
    );
    assert_eq!(refs(&audit(&vm), o), 1);
    drop(map);
    assert_eq!(info(&f).owners, 1);
}
#[test]
fn foreign_owned_return_fails_after_one_callback_and_keeps_target() {
    let foreign_store = store();
    let foreign = Rc::new(foreign_store.value(ValueInput::Int(4)).unwrap());
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = machine(
        Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Owned(foreign.retain().unwrap()))
        }),
        false,
    );
    vm.start(o, 5, &[]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert!(vm.run(1).is_err());
    assert!(vm.run(1).is_err());
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(calls.get(), 1);
    audit(&vm);
}
#[test]
#[ignore = "explicit callable timeline worker; requires RML32_CALLABLE_TRACES"]
fn replay_host_function_traces() {
    use std::io::BufRead;
    let file = std::fs::File::open(
        std::env::var_os("RML32_CALLABLE_TRACES").expect("explicit trace input required"),
    )
    .unwrap();
    let mut traces = 0;
    let mut steps = 0;
    for line in std::io::BufReader::new(file).lines() {
        let case: Json = serde_json::from_str(&line.unwrap()).unwrap();
        let mode = case["callback"].as_str().unwrap().to_owned();
        let saved = Rc::new(RefCell::new(None));
        let out = saved.clone();
        let calls = Rc::new(Cell::new(0));
        let count = calls.clone();
        let callback = Rc::new(
            move |_: &HostCallContext, scope: &HostScope<'_>, args: &[BorrowedValue<'_>]| {
                count.set(count.get() + 1);
                if matches!(mode.as_str(), "save" | "error") && !args.is_empty() {
                    *out.borrow_mut() = Some(scope.retain(&args[0]).map_err(host_error)?);
                }
                if mode == "error" {
                    return Err(host::HostError {
                        kind: VmErrorKind::InvalidState,
                        message: "once".into(),
                    });
                }
                if args.is_empty() {
                    Ok(HostResult::Owned(
                        scope.value(ValueInput::Int(9)).map_err(host_error)?,
                    ))
                } else {
                    Ok(HostResult::Borrowed(0))
                }
            },
        );
        let (mut vm, creator) = machine(callback, true);
        let p = vm.objects().get(creator).unwrap().program();
        let other = vm.objects_mut().create(p, &[]).unwrap();
        assert_eq!(other.get(), 1);
        let registry = vm.registry.clone();
        let weak_registry = Rc::downgrade(&registry);
        let weak_life = Rc::downgrade(registry.test_life.as_ref().unwrap());
        let probes =
            [creator, other].map(|o| vm.inner.as_ref().unwrap().objects().probe_for_test(o.slot));
        let kind = case["capture"].as_str().unwrap();
        let mut array = match kind {
            "array" => Some(container(&vm, false, Some(other))),
            "mapping" => Some(container(&vm, true, Some(other))),
            _ => None,
        };
        let graph = array.as_ref().map(|a| {
            Rc::downgrade(
                &registry
                    .entries
                    .borrow()
                    .iter()
                    .find(|e| e.id == a.id)
                    .unwrap()
                    .payload
                    .value
                    .counter()
                    .unwrap(),
            )
        });
        let mut machine = Some(vm);
        let mut storage: Option<ObjectStore> = None;
        let mut function: Option<HostValue> = None;
        let mut retained = None;
        let mut result = None;
        let mut observed: Option<crate::value::FunctionValue> = None;
        let mut head_probes = Vec::new();
        for row in case["steps"].as_array().unwrap() {
            let command = &row["command"];
            let op = command[0].as_str().unwrap();
            match op {
                "start-make" | "start-unique" => {
                    let slot = if kind == "none" {
                        if op == "start-make" { 0 } else { 5 }
                    } else if op == "start-make" {
                        1
                    } else {
                        3
                    };
                    let input = array
                        .as_ref()
                        .map(ValueInput::Handle)
                        .unwrap_or(ValueInput::Int(7));
                    machine
                        .as_mut()
                        .unwrap()
                        .start(
                            creator,
                            slot,
                            if kind == "none" {
                                &[]
                            } else {
                                std::slice::from_ref(&input)
                            },
                        )
                        .unwrap();
                }
                "start-call" => {
                    let caller = if command[1] == 0 { creator } else { other };
                    machine
                        .as_mut()
                        .unwrap()
                        .start(caller, 2, &[ValueInput::Handle(function.as_ref().unwrap())])
                        .unwrap();
                }
                "start-global" => {
                    globals(machine.as_mut().unwrap());
                    machine.as_mut().unwrap().start(creator, 7, &[]).unwrap();
                }
                "clear-global" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .clear_global_program()
                        .unwrap();
                }
                "run" => {
                    let actual = match machine
                        .as_mut()
                        .unwrap()
                        .run(command[1].as_u64().unwrap() as usize)
                    {
                        Ok(RunOutcome::BudgetExhausted) => "budget",
                        Ok(RunOutcome::Finished(_)) => "finished",
                        Err(_) => "error",
                    };
                    assert_eq!(actual, command[2].as_str().unwrap(), "{}", case["name"]);
                }
                "take-function" => {
                    let f = machine.as_mut().unwrap().take_returned().unwrap();
                    observed = Some(view(&f));
                    function = Some(f);
                }
                "take-result" => {
                    result = Some(machine.as_mut().unwrap().take_returned().unwrap());
                }
                "retain-function" => {
                    retained = Some(function.as_ref().unwrap().retain().unwrap());
                }
                "drop-function" => {
                    drop(function.take());
                }
                "drop-retained" => {
                    drop(retained.take());
                }
                "drop-result" => {
                    drop(result.take());
                }
                "drop-container" => {
                    drop(array.take());
                }
                "drop-saved" => {
                    drop(saved.borrow_mut().take());
                }
                "remove-binding" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .remove_host_function(0)
                        .unwrap();
                }
                "replace-binding" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .register_host_function(0, binding(echo()))
                        .unwrap();
                }
                "delete-creator" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .remove(creator)
                        .unwrap();
                }
                "drain" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .inner
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .drain_one_for_test()
                        .unwrap();
                }
                "reap" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .inner
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .reap_for_test();
                }
                "cancel" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .inner
                        .as_mut()
                        .unwrap()
                        .cancel_execution_for_test();
                }
                "extract" => {
                    storage = Some(machine.take().unwrap().into_objects());
                }
                "rebuild" => {
                    machine = Some(Vm::new(storage.take().unwrap()));
                }
                "close" => {
                    drop(machine.take());
                    drop(storage.take());
                }
                _ => panic!("unknown callable command {op}"),
            }
            if let Some(vm) = &machine {
                if let Ok(cell) = vm.inner.as_ref().unwrap().returned_cell()
                    && let value::Value::Function(f) = cell.read().unwrap()
                {
                    observed = Some(f);
                }
                for value in vm.inner.as_ref().unwrap().stack().unwrap() {
                    if let vm::StackSnapshot::Value(value::Value::Function(f)) = value {
                        observed = Some(f);
                    }
                }
            }
            if let Some(f) = &observed {
                head_probes.push(Rc::downgrade(&f.0));
            }
            let entries = registry.entries.borrow();
            let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
            let frames = if let Some(vm) = &machine {
                vm.inner
                    .as_ref()
                    .unwrap()
                    .lifecycle_audit_host_for_test(&roots);
                vm.inner.as_ref().unwrap().managed_snapshot_for_test()["frames"].clone()
            } else {
                registry
                    .test_life
                    .as_ref()
                    .unwrap()
                    .audit_host(&[], &[], &roots);
                json!([])
            };
            let mut life = registry.test_life.as_ref().unwrap().snapshot();
            life["resident"] = json!(
                probes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| p.upgrade().map(|_| i))
                    .collect::<Vec<_>>()
            );
            let head=observed.as_ref().map(|f|json!({"owners":f.0.counter.owners(),"creator":f.owner().map(|o|o.get()),"bound":f.bound_count(),"captures":f.0.bound.iter().map(|c|match c.read().unwrap(){value::Value::Undefined=>"undefined".into(),value::Value::Int(n)=>format!("int:{n}"),value::Value::Array(_)=>"array".into(),value::Value::Mapping(_)=>"mapping".into(),_=>panic!("unexpected capture")}).collect::<Vec<String>>()}));
            let actual = json!({"life":life,"frames":frames,"head":head,"graph_owners":graph.as_ref().and_then(|w|w.upgrade().map(|c|c.owners())),"roots":entries.len(),"calls":calls.get(),"closed":!registry.alive.get()});
            assert_eq!(
                actual, row["expected"],
                "{} checkpoint {} ({op})",
                case["name"], steps
            );
            steps += 1;
        }
        drop(machine);
        drop(storage);
        drop(result);
        drop(function);
        drop(retained);
        drop(array);
        drop(saved.borrow_mut().take());
        drop(observed);
        assert!(head_probes.iter().all(|p| p.upgrade().is_none()));
        registry.drain();
        drop(registry);
        drop(saved);
        assert!(probes.iter().all(|p| p.upgrade().is_none()));
        assert!(weak_registry.upgrade().is_none());
        assert!(weak_life.upgrade().is_none());
        assert!(graph.is_none_or(|g| g.upgrade().is_none()));
        traces += 1;
    }
    println!(
        "CALLABLE_REPLAY {}",
        json!({"traces":traces,"steps":steps,"physical_teardowns":traces})
    );
}
