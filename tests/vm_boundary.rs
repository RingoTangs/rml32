//! External users exercise only the checked public boundary.
use rml32::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
include!("boundary_support.rs.inc");

fn machine(functions: &[Fun], vars: usize) -> (Vm, ObjectId) {
    let mut store = ObjectStore::new().unwrap();
    let p = store
        .register_program(
            prepare_program_with_callable_values(
                image(functions, vars, &[]),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )
        .unwrap();
    let inputs = vec![ValueInput::Undefined; vars];
    let o = store.create(p, &inputs).unwrap();
    (Vm::new(store), o)
}
fn done(vm: &mut Vm) -> HostValue {
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    vm.take_returned().unwrap()
}
fn info(value: &HostValue) -> FunctionSnapshot {
    let ValueSnapshot::Function(info) = value.snapshot().unwrap() else {
        panic!("expected function")
    };
    info
}
fn functions() -> Vec<Fun> {
    vec![
        fun(0, 0, 0, 9, &[(1, 0, 0x17)]),
        fun(1, 0, 0, 2, &[(17, 0, 0x3b)]),
        fun(2, 1, 0, 255, &[(0, 0, 0x37)]),
        fun(3, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
        fun(4, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)]),
    ]
}
fn binding(callback: HostCallback) -> HostFunction {
    HostFunction {
        name: "boundary-test".into(),
        signature: HostSignature {
            required: vec![HostType::Any],
            optional: vec![],
            rest: None,
            returns: HostReturn::Mixed,
        },
        callback,
    }
}

#[test]
fn completion_observation_take_and_retain_have_explicit_owner_effects() {
    let (mut vm, o) = machine(&functions(), 0);
    assert_eq!(
        vm.take_returned().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    vm.start(o, 0, &[]).unwrap();
    for _ in 0..4 {
        assert_eq!(
            vm.run(100).unwrap(),
            RunOutcome::Finished(CompletionMeta::Returned)
        );
    }
    let f = vm.take_returned().unwrap();
    assert_eq!(info(&f).owners, 1);
    assert_eq!(info(&f).bound_count, 0);
    assert_eq!(info(&f).function, 1);
    assert!(info(&f).program.is_some());
    let frozen = f.snapshot().unwrap();
    let second = f.retain().unwrap();
    assert_eq!(info(&f).owners, 2);
    assert_eq!(info(&f).id, info(&second).id);
    assert_eq!(
        vm.take_returned().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        vm.run(0).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    let _debug = format!("{f:?}");
    drop(second);
    assert_eq!(info(&f).owners, 1);
    assert_eq!(frozen, f.snapshot().unwrap());
    vm.start(o, 3, &[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(done(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(17));
    assert_eq!(info(&f).owners, 1);
}
#[test]
fn containers_observe_without_owners_and_export_independent_roots() {
    let (mut vm, o) = machine(&functions(), 0);
    vm.start(o, 0, &[]).unwrap();
    let f = done(&mut vm);
    let array = vm.objects().array(&[ValueInput::Handle(&f)]).unwrap();
    assert_eq!(info(&f).owners, 2);
    let alias = array.retain().unwrap();
    for _ in 0..3 {
        array.snapshot().unwrap();
        array.elements().unwrap();
    }
    assert_eq!(info(&f).owners, 2);
    let member = array.export_member(0).unwrap();
    assert_eq!(info(&f).owners, 3);
    drop(array);
    assert_eq!(info(&f).owners, 3);
    drop(alias);
    assert_eq!(info(&f).owners, 2);
    drop(member);
    assert_eq!(info(&f).owners, 1);
    let map = vm
        .objects()
        .mapping(&[(ValueInput::Bytes(b"key"), ValueInput::Handle(&f))])
        .unwrap();
    assert_eq!(info(&f).owners, 2);
    let entries = map.entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(map.get(ValueInput::Int(404)).unwrap(), None);
    assert!(map.get_owned(ValueInput::Int(404)).unwrap().is_none());
    assert_eq!(map.entries().unwrap(), entries);
    let exported = map.get_owned(ValueInput::Bytes(b"key")).unwrap().unwrap();
    assert_eq!(info(&f).owners, 3);
    drop(map);
    assert_eq!(info(&f).owners, 2);
    drop(exported);
    assert_eq!(info(&f).owners, 1);
    assert!(
        vm.objects()
            .mapping(&[(ValueInput::Handle(&f), ValueInput::Int(0))])
            .is_err()
    );
}
#[test]
fn variables_globals_and_stack_observe_slots_without_new_roots() {
    let (mut vm, o) = machine(&functions(), 1);
    vm.start(o, 0, &[]).unwrap();
    let f = done(&mut vm);
    let p = vm.objects().get(o).unwrap().program();
    let second = vm
        .objects_mut()
        .create(p, &[ValueInput::Handle(&f)])
        .unwrap();
    vm.objects_mut()
        .register_globals(&[ValueInput::Handle(&f)])
        .unwrap();
    assert_eq!(info(&f).owners, 3);
    vm.objects().get(second).unwrap().variables().unwrap();
    vm.objects().global_values().unwrap();
    assert_eq!(info(&f).owners, 3);
    let a = vm
        .objects()
        .get(second)
        .unwrap()
        .export_variable(0)
        .unwrap();
    let b = vm.objects().export_global(0).unwrap();
    assert_eq!(info(&f).owners, 5);
    drop((a, b));
    vm.start(o, 2, &[ValueInput::Handle(&f)]).unwrap();
    for _ in 0..3 {
        vm.stack().unwrap();
    }
    assert_eq!(info(&f).owners, 4);
    let result = done(&mut vm);
    assert_eq!(info(&f).owners, 4);
    drop(result);
    vm.objects_mut().remove(second).unwrap();
    vm.objects_mut().remove_globals().unwrap();
    assert_eq!(info(&f).owners, 1);
}
#[test]
fn foreign_store_inputs_ids_and_leases_fail_before_mutation() {
    let (mut a, oa) = machine(&functions(), 0);
    let (mut b, ob) = machine(&functions(), 0);
    assert_eq!(oa.get(), ob.get());
    assert_ne!(oa, ob);
    a.start(oa, 0, &[]).unwrap();
    let f = done(&mut a);
    assert_eq!(
        b.start(ob, 2, &[ValueInput::Handle(&f)]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        b.start(oa, 2, &[]).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert!(b.objects().value(ValueInput::Object(oa)).is_err());
    assert!(b.objects().retain(&f).is_err());
    assert!(b.objects().snapshot(&f).is_err());
    assert!(b.objects().array(&[ValueInput::Handle(&f)]).is_err());
    assert!(
        b.objects_mut()
            .create(a.objects().get(oa).unwrap().program(), &[])
            .is_err()
    );
    assert!(
        b.objects_mut()
            .register_globals(&[ValueInput::Handle(&f)])
            .is_err()
    );
    assert!(b.objects().global_values().is_err());
    assert_eq!(b.frame_depth(), 0);
    assert_eq!(info(&f).owners, 1);
}
#[test]
fn into_objects_preserves_origin_and_store_close_invalidates_roots() {
    let (mut vm, o) = machine(&functions(), 0);
    vm.start(o, 0, &[]).unwrap();
    let f = done(&mut vm);
    let store = vm.into_objects();
    let root = store.retain(&f).unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 3, &[ValueInput::Handle(&root)]).unwrap();
    assert_eq!(done(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(17));
    let frozen = f.snapshot().unwrap();
    drop(vm);
    assert!(f.retain().is_err());
    assert!(root.snapshot().is_err());
    assert!(matches!(frozen, ValueSnapshot::Function(_)));
    drop((f, root));
}
#[test]
fn callback_can_retain_borrowed_function_without_reentry_authority() {
    let (mut vm, o) = machine(&functions(), 0);
    vm.start(o, 0, &[]).unwrap();
    let f = done(&mut vm);
    let saved = Rc::new(RefCell::new(None));
    let observed = Rc::new(Cell::new(0));
    let (saved_in, calls) = (saved.clone(), observed.clone());
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(move |context, scope, args| {
                calls.set(calls.get() + 1);
                assert_eq!(context.slot, 0);
                assert_eq!(args.len(), 1);
                assert!(matches!(
                    args[0].snapshot().unwrap(),
                    ValueSnapshot::Function(_)
                ));
                *saved_in.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
                Ok(HostResult::Borrowed(0))
            })),
        )
        .unwrap();
    vm.start(o, 4, &[ValueInput::Handle(&f)]).unwrap();
    let result = done(&mut vm);
    assert_eq!(observed.get(), 1);
    assert_eq!(info(&result).id, info(&f).id);
    assert_eq!(info(&f).owners, 3);
    drop(result);
    saved.borrow_mut().take();
    assert_eq!(info(&f).owners, 1);
}
#[test]
fn callback_owned_construction_and_failure_have_single_side_effects() {
    let (mut vm, o) = machine(&functions(), 0);
    let calls = Rc::new(Cell::new(0));
    let counted = calls.clone();
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(move |_, scope, _| {
                counted.set(counted.get() + 1);
                Ok(HostResult::Owned(
                    scope.array(&[ValueInput::Int(5)]).unwrap(),
                ))
            })),
        )
        .unwrap();
    vm.start(o, 4, &[ValueInput::Int(2)]).unwrap();
    let result = done(&mut vm);
    assert_eq!(result.elements().unwrap(), vec![ValueSnapshot::Int(5)]);
    assert_eq!(calls.get(), 1);
    vm.objects_mut().remove_host_function(0).unwrap();
    let counted = calls.clone();
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(move |_, _, _| {
                counted.set(counted.get() + 1);
                Ok(HostResult::Borrowed(9))
            })),
        )
        .unwrap();
    vm.start(o, 4, &[ValueInput::Int(2)]).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Bounds);
    assert!(e.function.is_some());
    assert_eq!(vm.run(100).unwrap_err(), e);
    assert_eq!(calls.get(), 2);
    assert!(vm.take_returned().is_err());
}
#[test]
fn callback_owned_foreign_return_and_signature_failure_preserve_saved_root() {
    let foreign = ObjectStore::new().unwrap();
    let root = foreign.value(ValueInput::Int(8)).unwrap();
    let returned = Rc::new(RefCell::new(Some(root)));
    let (mut vm, o) = machine(&functions(), 0);
    let data = returned.clone();
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(move |_, _, _| {
                Ok(HostResult::Owned(data.borrow_mut().take().unwrap()))
            })),
        )
        .unwrap();
    vm.start(o, 4, &[ValueInput::Int(1)]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::InvalidState);
    vm.objects_mut().remove_host_function(0).unwrap();
    let saved = Rc::new(RefCell::new(None));
    let data = saved.clone();
    let mut f = binding(Rc::new(move |_, scope, args| {
        *data.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
        Ok(HostResult::Void)
    }));
    f.signature.returns = HostReturn::Exact(HostType::Int);
    vm.objects_mut().register_host_function(0, f).unwrap();
    vm.start(o, 4, &[ValueInput::Int(9)]).unwrap();
    assert!(vm.run(100).is_err());
    assert_eq!(
        saved.borrow().as_ref().unwrap().snapshot().unwrap(),
        ValueSnapshot::Int(9)
    );
}
#[test]
fn root_graph_item_and_depth_limits_fail_without_consuming_existing_leases() {
    let store = ObjectStore::with_limits(ValueLimits {
        max_host_roots: 1,
        ..ValueLimits::default()
    })
    .unwrap();
    let root = store.value(ValueInput::Int(3)).unwrap();
    assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(root.snapshot().unwrap(), ValueSnapshot::Int(3));
    drop(root);
    store.value(ValueInput::Int(4)).unwrap();
    let store = ObjectStore::with_limits(ValueLimits {
        max_graph_nodes: 2,
        max_items: 1,
        max_depth: 1,
        ..ValueLimits::default()
    })
    .unwrap();
    assert!(
        store
            .array(&[ValueInput::Int(0), ValueInput::Int(1)])
            .is_err()
    );
    let a = store.array(&[ValueInput::Int(0)]).unwrap();
    assert!(store.array(&[ValueInput::Handle(&a)]).is_err());
    assert_eq!(a.elements().unwrap(), vec![ValueSnapshot::Int(0)]);
}
#[test]
fn snapshots_freeze_aliases_and_raw_deleted_object_transport() {
    let (mut vm, o) = machine(&functions(), 0);
    let a = vm.objects().array(&[ValueInput::Object(o)]).unwrap();
    let shared = vm
        .objects()
        .array(&[ValueInput::Handle(&a), ValueInput::Handle(&a)])
        .unwrap();
    let ValueSnapshot::Array { elements, .. } = shared.snapshot().unwrap() else {
        panic!()
    };
    assert!(matches!(elements[1], ValueSnapshot::Shared { .. }));
    vm.objects_mut().remove(o).unwrap();
    assert_eq!(a.elements().unwrap(), vec![ValueSnapshot::Object(o)]);
    let root = a.export_member(0).unwrap();
    assert_eq!(root.snapshot().unwrap(), ValueSnapshot::Object(o));
    let store = vm.into_objects();
    drop(store);
    assert!(root.retain().is_err());
}
#[test]
fn failed_take_reservation_keeps_completion_available_for_retry() {
    let mut store = ObjectStore::with_limits(ValueLimits {
        max_host_roots: 1,
        ..ValueLimits::default()
    })
    .unwrap();
    let p = store
        .register_program(
            prepare_program_with_callable_values(
                image(&functions(), 0, &[]),
                GlobalBankSchema { slots: 0 },
            )
            .unwrap(),
        )
        .unwrap();
    let o = store.create(p, &[]).unwrap();
    let occupied = store.value(ValueInput::Int(1)).unwrap();
    let mut vm = Vm::new(store);
    vm.start(o, 0, &[]).unwrap();
    vm.run(100).unwrap();
    assert_eq!(vm.take_returned().unwrap_err().kind, VmErrorKind::Limit);
    drop(occupied);
    let f = vm.take_returned().unwrap();
    assert_eq!(info(&f).owners, 1);
    assert_eq!(
        vm.take_returned().unwrap_err().kind,
        VmErrorKind::InvalidState
    );
}
#[test]
fn callback_failure_keeps_explicit_saved_root_and_can_restart() {
    let (mut vm, o) = machine(&functions(), 0);
    vm.start(o, 0, &[]).unwrap();
    let f = done(&mut vm);
    let saved = Rc::new(RefCell::new(None));
    let into = saved.clone();
    let calls = Rc::new(Cell::new(0));
    let counted = calls.clone();
    vm.objects_mut()
        .register_host_function(
            0,
            binding(Rc::new(move |_, scope, args| {
                counted.set(counted.get() + 1);
                *into.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
                Err(HostError {
                    kind: VmErrorKind::Arithmetic,
                    message: "callback failure".into(),
                })
            })),
        )
        .unwrap();
    vm.start(o, 4, &[ValueInput::Handle(&f)]).unwrap();
    assert!(vm.run(100).is_err());
    let before = info(&f).owners;
    vm.stack().unwrap();
    assert_eq!(info(&f).owners, before);
    assert!(vm.run(100).is_err());
    assert_eq!(calls.get(), 1);
    vm.start(o, 1, &[]).unwrap();
    drop(done(&mut vm));
    assert_eq!(info(&f).owners, 2);
    saved.borrow_mut().take();
    assert_eq!(info(&f).owners, 1);
}
