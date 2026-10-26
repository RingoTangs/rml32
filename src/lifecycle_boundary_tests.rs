//! Actual checked Host boundary in a private immutable lifecycle mode.
use super::*;
use crate::{ProgramImage, ProgramMode, parse_program};
use serde_json::{Value as Json, json};
include!("../tests/boundary_support.rs.inc");

fn store() -> ObjectStore {
    ObjectStore::with_test_host_lifecycle(ValueLimits::default()).unwrap()
}
fn object(store: &mut ObjectStore) -> ObjectId {
    let p = store
        .register_program(
            crate::prepare_program(image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37)])], 0, &[])).unwrap(),
        )
        .unwrap();
    store.create(p, &[]).unwrap()
}
fn roots(registry: &Registry) -> std::cell::Ref<'_, Vec<Entry>> {
    registry.entries.borrow()
}
fn audit_store(store: &ObjectStore) -> Json {
    let entries = roots(&store.registry);
    let owned = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    store
        .inner
        .as_ref()
        .unwrap()
        .test_life
        .as_ref()
        .unwrap()
        .audit_host(&[], &[], &owned);
    store.inner.as_ref().unwrap().lifecycle_snapshot_for_test()
}
fn audit_vm(vm: &Vm) -> Json {
    let entries = roots(&vm.registry);
    let owned = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    vm.inner
        .as_ref()
        .unwrap()
        .lifecycle_audit_host_for_test(&owned);
    vm.inner
        .as_ref()
        .unwrap()
        .objects()
        .lifecycle_snapshot_for_test()
}
fn refs(s: &Json, id: ObjectId) -> u64 {
    s["objects"][id.get().to_string()]["refs"].as_u64().unwrap()
}
fn state(s: &Json, id: ObjectId) -> &str {
    s["objects"][id.get().to_string()]["state"]
        .as_str()
        .unwrap()
}
fn binding(callback: HostCallback) -> HostFunction {
    HostFunction {
        name: "2be-synthetic".into(),
        signature: host::HostSignature {
            required: vec![host::HostType::Any],
            optional: vec![],
            rest: None,
            returns: host::HostReturn::Mixed,
        },
        callback,
    }
}
fn host_machine(callback: HostCallback) -> (Vm, ObjectId) {
    let mut s = store();
    let p = s
        .register_program(
            crate::prepare_program_with_host_calls(
                image(&[fun(0, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)])], 0, &[]),
                None,
            )
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    s.register_host_function(0, binding(callback)).unwrap();
    (Vm::new(s), o)
}
fn finished(vm: &mut Vm) {
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
}
fn function_root(s: ObjectStore) -> (Vm, ObjectId, HostValue) {
    let mut s = s;
    let p = s
        .register_program(
            crate::prepare_program_with_function_values(
                image(
                    &[
                        fun(0, 0, 0, 9, &[(1, 0, 0x17)]),
                        fun(1, 0, 0, 2, &[(17, 0, 0x3b)]),
                    ],
                    0,
                    &[],
                ),
                None,
            )
            .unwrap(),
        )
        .unwrap();
    let o = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[]).unwrap();
    finished(&mut vm);
    let f = vm.take_returned().unwrap();
    (vm, o, f)
}
#[test]
fn roots_retain_and_observation_have_exact_owner_effects() {
    let mut s = store();
    let o = object(&mut s);
    let root = s.value(ValueInput::Object(o)).unwrap();
    assert_eq!(refs(&audit_store(&s), o), 1);
    root.snapshot().unwrap();
    let _debug = format!("{root:?}");
    assert_eq!(refs(&audit_store(&s), o), 1);
    let second = root.retain().unwrap();
    assert_eq!(refs(&audit_store(&s), o), 2);
    drop(second);
    drop(root);
    assert_eq!(refs(&audit_store(&s), o), 0);
    assert_eq!(state(&audit_store(&s), o), "alive");
}
#[test]
fn borrowed_registry_defers_exactly_one_root_release() {
    let mut s = store();
    let o = object(&mut s);
    let r = s.value(ValueInput::Object(o)).unwrap();
    let guard = s.registry.entries.borrow();
    drop(r);
    assert!(guard[0].retired.get());
    assert_eq!(refs(&audit_store(&s), o), 1);
    assert!(s.value(ValueInput::Int(1)).is_err());
    drop(guard);
    s.registry.check().unwrap();
    assert_eq!(refs(&audit_store(&s), o), 0);
    assert!(s.registry.entries.borrow().is_empty());
}
#[test]
fn raw_retention_preserves_shell_until_script_copy() {
    let mut s = store();
    let o = object(&mut s);
    let r = s.value(ValueInput::Object(o)).unwrap();
    let caller = object(&mut s);
    s.remove(o).unwrap();
    let second = r.retain().unwrap();
    assert_eq!(refs(&audit_store(&s), o), 2);
    assert_eq!(r.snapshot().unwrap(), ValueSnapshot::Object(o));
    let mut vm = Vm::new(s);
    vm.start(caller, 0, &[ValueInput::Handle(&second)]).unwrap();
    finished(&mut vm);
    assert_eq!(
        vm.take_returned().unwrap().snapshot().unwrap(),
        ValueSnapshot::Undefined
    );
    assert_eq!(r.snapshot().unwrap(), ValueSnapshot::Object(o));
    assert_eq!(state(&audit_vm(&vm), o), "shell");
    drop(second);
    drop(r);
    assert_eq!(state(&audit_vm(&vm), o), "freed");
}
#[test]
fn freed_ids_and_foreign_roots_cannot_be_imported() {
    let mut a = store();
    let o = object(&mut a);
    a.remove(o).unwrap();
    assert!(a.value(ValueInput::Object(o)).is_err());
    let b = store();
    let r = b.value(ValueInput::Int(9)).unwrap();
    assert!(a.value(ValueInput::Handle(&r)).is_err());
    assert!(b.value(ValueInput::Object(o)).is_err());
    audit_store(&a);
}
#[test]
fn containers_bind_real_members_and_exports_own_independent_slots() {
    let mut s = store();
    let o = object(&mut s);
    let a = s
        .array(&[ValueInput::Object(o), ValueInput::Object(o)])
        .unwrap();
    assert_eq!(refs(&audit_store(&s), o), 2);
    let alias = a.retain().unwrap();
    assert_eq!(refs(&audit_store(&s), o), 2);
    let member = a.export_member(0).unwrap();
    assert_eq!(refs(&audit_store(&s), o), 3);
    drop(a);
    drop(alias);
    assert_eq!(refs(&audit_store(&s), o), 1);
    let m = s
        .mapping(&[(ValueInput::Int(1), ValueInput::Handle(&member))])
        .unwrap();
    let export = m.get_owned(ValueInput::Int(1)).unwrap().unwrap();
    assert_eq!(refs(&audit_store(&s), o), 3);
    drop(m);
    drop(export);
    drop(member);
    assert_eq!(refs(&audit_store(&s), o), 0);
}
#[test]
fn function_copies_keep_one_creator_and_real_container_members() {
    let (vm, o, f) = function_root(store());
    assert_eq!(refs(&audit_vm(&vm), o), 1);
    let second = f.retain().unwrap();
    let a = vm
        .objects()
        .array(&[ValueInput::Handle(&f), ValueInput::Handle(&f)])
        .unwrap();
    let ValueSnapshot::Function(info) = f.snapshot().unwrap() else {
        panic!()
    };
    assert_eq!(info.owners, 4);
    assert_eq!(refs(&audit_vm(&vm), o), 1);
    drop(second);
    drop(a);
    drop(f);
    assert_eq!(refs(&audit_vm(&vm), o), 0);
}
#[test]
fn completion_transfer_uses_existing_lease_and_claims_once() {
    let mut s = store();
    let o = object(&mut s);
    let r = s.value(ValueInput::Object(o)).unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Handle(&r)]).unwrap();
    finished(&mut vm);
    let before = refs(&audit_vm(&vm), o);
    let result = vm.take_returned().unwrap();
    assert_eq!(refs(&audit_vm(&vm), o), before);
    assert!(vm.take_returned().is_err());
    assert_eq!(result.snapshot().unwrap(), ValueSnapshot::Object(o));
}
#[test]
fn completion_capacity_failure_is_retryable_without_consuming_value() {
    let mut s = ObjectStore::with_test_host_lifecycle(ValueLimits {
        max_host_roots: 1,
        ..ValueLimits::default()
    })
    .unwrap();
    let o = object(&mut s);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Handle(&root)]).unwrap();
    finished(&mut vm);
    let before = audit_vm(&vm);
    assert_eq!(vm.take_returned().unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(audit_vm(&vm), before);
    drop(root);
    let r = vm.take_returned().unwrap();
    assert_eq!(r.snapshot().unwrap(), ValueSnapshot::Object(o));
}
#[test]
fn completion_borrow_failure_is_retryable() {
    let mut s = store();
    let o = object(&mut s);
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Int(4)]).unwrap();
    finished(&mut vm);
    let registry = vm.registry.clone();
    let guard = registry.entries.borrow();
    assert!(vm.take_returned().is_err());
    drop(guard);
    assert_eq!(
        vm.take_returned().unwrap().snapshot().unwrap(),
        ValueSnapshot::Int(4)
    );
    audit_vm(&vm);
}
#[test]
fn root_capacity_and_id_exhaustion_do_not_publish_or_leak() {
    let mut s = ObjectStore::with_test_host_lifecycle(ValueLimits {
        max_host_roots: 1,
        ..ValueLimits::default()
    })
    .unwrap();
    let o = object(&mut s);
    let r = s.value(ValueInput::Object(o)).unwrap();
    let before = audit_store(&s);
    assert!(r.retain().is_err());
    assert_eq!(audit_store(&s), before);
    drop(r);
    s.registry.next.set(u64::MAX);
    assert!(s.value(ValueInput::Object(o)).is_err());
    assert_eq!(refs(&audit_store(&s), o), 0);
}
#[test]
fn store_close_retains_records_until_final_host_root() {
    let mut s = store();
    let o = object(&mut s);
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let registry = Rc::downgrade(&s.registry);
    drop(s);
    assert!(probe.upgrade().is_some());
    assert!(root.snapshot().is_err());
    assert!(root.retain().is_err());
    drop(root);
    assert!(probe.upgrade().is_none());
    assert!(registry.upgrade().is_none());
}
#[test]
fn close_without_roots_releases_storage_immediately() {
    let mut s = store();
    let o = object(&mut s);
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    drop(s);
    assert!(probe.upgrade().is_none());
}
#[test]
fn close_while_registry_borrowed_preserves_retired_root_and_storage() {
    let mut s = store();
    let o = object(&mut s);
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let registry = s.registry.clone();
    let guard = registry.entries.borrow();
    drop(root);
    drop(s);
    assert!(probe.upgrade().is_some());
    drop(guard);
    assert!(registry.drain());
    assert!(probe.upgrade().is_none());
    assert!(registry.closed_store.borrow().is_none());
}
#[test]
fn vm_close_exits_pending_frames_then_retains_storage_for_root() {
    let mut s = store();
    let o = object(&mut s);
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Int(1)]).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let life = vm.registry.test_life.clone().unwrap();
    drop(vm);
    assert_eq!(life.snapshot()["objects"]["0"]["execs"], 0);
    assert!(probe.upgrade().is_some());
    drop(root);
    assert!(probe.upgrade().is_none());
}
#[test]
fn transfer_and_rebuild_keep_origin_mode_queue_and_callbacks() {
    let (mut vm, o) = host_machine(Rc::new(|_, _, _| Ok(HostResult::Borrowed(0))));
    let root = vm.objects().value(ValueInput::Object(o)).unwrap();
    vm.start(o, 0, &[ValueInput::Int(2)]).unwrap();
    vm.objects_mut().remove(o).unwrap();
    let identity = vm.registry.identity.id;
    let s = vm.into_objects();
    assert_eq!(s.registry.identity.id, identity);
    assert_eq!(state(&audit_store(&s), o), "queued");
    assert_eq!(
        s.inner.as_ref().unwrap().lifecycle_snapshot_for_test()["queue"],
        json!([0])
    );
    assert!(s.registry.alive.get());
    let mut vm = Vm::new(s);
    assert!(vm.start(o, 0, &[]).is_err());
    assert_eq!(root.snapshot().unwrap(), ValueSnapshot::Object(o));
    audit_vm(&vm);
}
#[test]
fn callback_borrowed_result_and_saved_root_have_exact_ownership() {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, scope, args| {
        c.set(c.get() + 1);
        *out.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
        Ok(HostResult::Borrowed(0))
    }));
    vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
    finished(&mut vm);
    assert_eq!(calls.get(), 1);
    let result = vm.take_returned().unwrap();
    assert_eq!(result.snapshot().unwrap(), ValueSnapshot::Object(o));
    audit_vm(&vm);
    vm.run(0).unwrap();
    assert_eq!(calls.get(), 1);
    drop(saved.borrow_mut().take());
    audit_vm(&vm);
}
#[test]
fn callback_owned_container_result_binds_members() {
    let (mut vm, o) = host_machine(Rc::new(|_, scope, args| {
        let r = scope.retain(&args[0]).map_err(host_error)?;
        Ok(HostResult::Owned(
            scope.array(&[ValueInput::Handle(&r)]).map_err(host_error)?,
        ))
    }));
    vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
    finished(&mut vm);
    let r = vm.take_returned().unwrap();
    assert_eq!(r.elements().unwrap(), vec![ValueSnapshot::Object(o)]);
    audit_vm(&vm);
}
#[test]
fn callback_void_result_is_valid_and_has_no_extra_object_owner() {
    let (mut vm, o) = host_machine(Rc::new(|_, _, _| Ok(HostResult::Void)));
    vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
    finished(&mut vm);
    assert_eq!(
        vm.take_returned().unwrap().snapshot().unwrap(),
        ValueSnapshot::Void
    );
    audit_vm(&vm);
}
#[test]
fn callback_error_is_sticky_and_saved_roots_survive() {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, scope, args| {
        c.set(c.get() + 1);
        *out.borrow_mut() = Some(scope.retain(&args[0]).unwrap());
        Err(host::HostError {
            kind: VmErrorKind::InvalidState,
            message: "once".into(),
        })
    }));
    vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
    let first = vm.run(100).unwrap_err();
    assert_eq!(vm.run(100).unwrap_err(), first);
    assert_eq!(calls.get(), 1);
    audit_vm(&vm);
    assert!(saved.borrow().as_ref().unwrap().snapshot().is_ok());
}
#[test]
fn callback_post_effect_invalid_result_never_replays() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, _, _| {
        c.set(c.get() + 1);
        Ok(HostResult::Borrowed(9))
    }));
    vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
    assert!(vm.run(100).is_err());
    assert!(vm.run(100).is_err());
    assert_eq!(calls.get(), 1);
    audit_vm(&vm);
}
#[test]
fn callback_preflight_failure_has_zero_effects() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, _, _| {
        c.set(c.get() + 1);
        Ok(HostResult::Void)
    }));
    vm.start(o, 0, &[ValueInput::Int(1)]).unwrap();
    vm.run(3).unwrap();
    crate::cell::with_allocation_failure(0, || assert!(vm.run(1).is_err()));
    assert_eq!(calls.get(), 0);
    audit_vm(&vm);
}
#[test]
fn closing_store_detaches_callbacks_that_capture_host_roots() {
    let mut s = store();
    let o = object(&mut s);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let registry = Rc::downgrade(&s.registry);
    s.register_host_function(
        0,
        binding(Rc::new(move |_, _, _| {
            let _ = &root;
            Ok(HostResult::Void)
        })),
    )
    .unwrap();
    drop(s);
    assert!(probe.upgrade().is_none());
    assert!(registry.upgrade().is_none());
}
#[test]
fn missing_registry_root_is_detected_by_independent_audit() {
    let mut s = store();
    let o = object(&mut s);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let life = s.registry.test_life.as_ref().unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| life.audit(&[], &[]))).is_err()
    );
    audit_store(&s);
    drop(root);
}
#[test]
fn host_retirement_does_not_drain_object_queue() {
    let mut s = store();
    let o = object(&mut s);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Int(1)]).unwrap();
    vm.objects_mut().remove(o).unwrap();
    finished(&mut vm);
    drop(root);
    assert_eq!(state(&audit_vm(&vm), o), "queued");
    assert_eq!(audit_vm(&vm)["queue"], json!([0]));
    vm.inner
        .as_mut()
        .unwrap()
        .objects_mut()
        .drain_one_for_test()
        .unwrap();
    assert_eq!(state(&audit_vm(&vm), o), "freed");
}
#[test]
fn unbound_container_graph_is_rejected_without_adoption() {
    let s = store();
    let graph = value::Value::array(vec![value::Value::Int(1)]).unwrap();
    assert!(s.registry.publish(graph).is_err());
    assert!(s.registry.entries.borrow().is_empty());
    audit_store(&s);
}
#[test]
fn callback_post_effect_workspace_failure_rolls_back_and_never_replays() {
    for after in 0..32 {
        let calls = Rc::new(Cell::new(0));
        let c = calls.clone();
        let (mut vm, o) = host_machine(Rc::new(move |_, _, _| {
            c.set(c.get() + 1);
            Ok(HostResult::Borrowed(0))
        }));
        vm.start(o, 0, &[ValueInput::Object(o)]).unwrap();
        vm.run(3).unwrap();
        let before = audit_vm(&vm);
        let result = crate::copy_plan::with_workspace_failure(after, || vm.run(1));
        if result.is_err() && calls.get() == 1 {
            assert_eq!(audit_vm(&vm), before);
            assert!(vm.run(1).is_err());
            assert_eq!(calls.get(), 1);
            return;
        }
    }
    panic!("no injected post-callback failure reached");
}

#[test]
fn callback_foreign_owned_result_is_rejected_after_one_effect() {
    let foreign_store = store();
    let foreign = foreign_store.value(ValueInput::Int(3)).unwrap();
    let foreign = Rc::new(foreign);
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, _, _| {
        c.set(c.get() + 1);
        Ok(HostResult::Owned(foreign.retain().unwrap()))
    }));
    vm.start(o, 0, &[ValueInput::Int(1)]).unwrap();
    assert!(vm.run(100).is_err());
    assert!(vm.run(100).is_err());
    assert_eq!(calls.get(), 1);
    audit_vm(&vm);
}
#[test]
fn callback_raw_deleted_object_argument_and_result_are_not_normalized() {
    let (mut vm, caller) = host_machine(Rc::new(|_, _, _| Ok(HostResult::Borrowed(0))));
    let program = vm.objects().get(caller).unwrap().program();
    let other = vm.objects_mut().create(program, &[]).unwrap();
    vm.start(caller, 0, &[ValueInput::Object(other)]).unwrap();
    vm.run(3).unwrap();
    vm.objects_mut().remove(other).unwrap();
    finished(&mut vm);
    let root = vm.take_returned().unwrap();
    assert_eq!(root.snapshot().unwrap(), ValueSnapshot::Object(other));
    assert_eq!(state(&audit_vm(&vm), other), "shell");
    drop(root);
    assert_eq!(state(&audit_vm(&vm), other), "freed");
}
#[test]
fn callback_bindings_survive_transfer_and_reexecute_only_after_explicit_restart() {
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    let (mut vm, o) = host_machine(Rc::new(move |_, _, _| {
        c.set(c.get() + 1);
        Ok(HostResult::Borrowed(0))
    }));
    vm.start(o, 0, &[ValueInput::Int(3)]).unwrap();
    finished(&mut vm);
    let root = vm.take_returned().unwrap();
    let s = vm.into_objects();
    let mut vm = Vm::new(s);
    vm.start(o, 0, &[ValueInput::Handle(&root)]).unwrap();
    finished(&mut vm);
    assert_eq!(calls.get(), 2);
    assert_eq!(
        vm.take_returned().unwrap().snapshot().unwrap(),
        ValueSnapshot::Int(3)
    );
    audit_vm(&vm);
}
#[test]
fn host_function_values_and_managed_execution_stay_gated_in_host_mode() {
    let mut s = store();
    let p = crate::prepare_program_with_callable_values(
        image(&[fun(0, 0, 0, 255, &[(7, 0, 0x3b)])], 0, &[]),
        crate::GlobalBankSchema { slots: 0 },
    )
    .unwrap();
    assert_eq!(
        s.register_program(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    let o = object(&mut s);
    let p = s.view().get(o).unwrap().program();
    let mut vm = Vm::new(s);
    assert_eq!(
        vm.begin_initialize(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    audit_vm(&vm);
}
#[test]
#[ignore = "explicit Host timeline worker; requires RML32_HOST_LIFECYCLE_TRACES"]
fn replay_host_lifecycle_traces() {
    use std::io::BufRead;
    let input = std::fs::File::open(
        std::env::var_os("RML32_HOST_LIFECYCLE_TRACES").expect("explicit trace input required"),
    )
    .unwrap();
    let mut cases = 0;
    let mut steps = 0;
    for line in std::io::BufReader::new(input).lines() {
        let case: Json = serde_json::from_str(&line.unwrap()).unwrap();
        let calls = Rc::new(Cell::new(0));
        let count = calls.clone();
        let saved = Rc::new(RefCell::new(None));
        let target = saved.clone();
        let mode = case["callback"].as_str().unwrap().to_owned();
        let callback = Rc::new(
            move |_: &HostCallContext, scope: &HostScope<'_>, args: &[BorrowedValue<'_>]| {
                count.set(count.get() + 1);
                if matches!(mode.as_str(), "save" | "error") {
                    *target.borrow_mut() = Some(scope.retain(&args[0]).map_err(host_error)?);
                }
                match mode.as_str() {
                    "error" => Err(host::HostError {
                        kind: VmErrorKind::InvalidState,
                        message: "trace sticky failure".into(),
                    }),
                    "invalid" => Ok(HostResult::Borrowed(9)),
                    "void" => Ok(HostResult::Void),
                    _ => Ok(HostResult::Borrowed(0)),
                }
            },
        );
        let mut s = store();
        let p = s
            .register_program(
                crate::prepare_program_with_host_calls(
                    image(
                        &[
                            fun(0, 1, 0, 255, &[(0, 0, 0x37)]),
                            fun(1, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)]),
                        ],
                        0,
                        &[],
                    ),
                    None,
                )
                .unwrap(),
            )
            .unwrap();
        let a = s.create(p, &[]).unwrap();
        let b = s.create(p, &[]).unwrap();
        let ids = [a, b];
        let probes = ids
            .iter()
            .map(|id| s.inner.as_ref().unwrap().probe_for_test(id.slot))
            .collect::<Vec<_>>();
        s.register_host_function(0, binding(callback)).unwrap();
        let registry = s.registry.clone();
        let weak_registry = Rc::downgrade(&registry);
        let weak_life = Rc::downgrade(registry.test_life.as_ref().unwrap());
        let mut storage = Some(s);
        let mut machine: Option<Vm> = None;
        let mut handles = std::collections::BTreeMap::<String, HostValue>::new();
        let mut guard = None;
        for (index, command) in case["commands"].as_array().unwrap().iter().enumerate() {
            let op = command[0].as_str().unwrap();
            let name = || command[1].as_str().unwrap().to_owned();
            match op {
                "root" => {
                    let id = ids[command[2].as_u64().unwrap() as usize];
                    let r = registry.publish(value::Value::Object(id.slot)).unwrap();
                    handles.insert(name(), r);
                }
                "retain" => {
                    let r = handles[command[2].as_str().unwrap()].retain().unwrap();
                    handles.insert(name(), r);
                }
                "drop" => {
                    drop(handles.remove(&name()).unwrap());
                }
                "borrow" => {
                    guard = Some(registry.entries.borrow());
                }
                "unborrow" => {
                    drop(guard.take());
                }
                "host-drain" => {
                    assert!(registry.drain());
                }
                "check-rejected" => {
                    assert!(registry.check().is_err());
                }
                "delete" => {
                    let id = ids[command[1].as_u64().unwrap() as usize];
                    if let Some(vm) = machine.as_mut() {
                        vm.objects_mut().remove(id).unwrap()
                    } else {
                        storage.as_mut().unwrap().remove(id).unwrap()
                    }
                }
                "start" => {
                    if machine.is_none() {
                        machine = Some(Vm::new(storage.take().unwrap()));
                    }
                    machine
                        .as_mut()
                        .unwrap()
                        .start(
                            a,
                            command[1].as_i64().unwrap() as i16,
                            &[ValueInput::Object(b)],
                        )
                        .unwrap();
                }
                "run" => {
                    let result = machine
                        .as_mut()
                        .unwrap()
                        .run(command[1].as_u64().unwrap() as usize);
                    let outcome = match result {
                        Ok(RunOutcome::BudgetExhausted) => "budget",
                        Ok(RunOutcome::Finished(_)) => "finished",
                        Err(_) => "error",
                    };
                    assert_eq!(outcome, command[2].as_str().unwrap());
                }
                "take" => {
                    let r = machine.as_mut().unwrap().take_returned().unwrap();
                    handles.insert(name(), r);
                }
                "transfer" => {
                    storage = Some(machine.take().unwrap().into_objects());
                }
                "rebuild" => {
                    machine = Some(Vm::new(storage.take().unwrap()));
                }
                "drain" => {
                    if let Some(vm) = machine.as_mut() {
                        vm.inner
                            .as_mut()
                            .unwrap()
                            .objects_mut()
                            .drain_one_for_test()
                            .unwrap()
                    } else {
                        storage
                            .as_mut()
                            .unwrap()
                            .inner
                            .as_mut()
                            .unwrap()
                            .drain_one_for_test()
                            .unwrap()
                    }
                }
                "reap" => {
                    if let Some(vm) = machine.as_mut() {
                        vm.inner.as_mut().unwrap().objects_mut().reap_for_test()
                    } else {
                        storage
                            .as_mut()
                            .unwrap()
                            .inner
                            .as_mut()
                            .unwrap()
                            .reap_for_test()
                    }
                }
                "drop-saved" => {
                    drop(saved.borrow_mut().take());
                }
                "close" => {
                    drop(machine.take());
                    drop(storage.take());
                }
                _ => panic!("unknown command {op}"),
            }
            let entries = registry.entries.borrow();
            let owned = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
            if let Some(vm) = &machine {
                vm.inner
                    .as_ref()
                    .unwrap()
                    .lifecycle_audit_host_for_test(&owned)
            } else {
                registry
                    .test_life
                    .as_ref()
                    .unwrap()
                    .audit_host(&[], &[], &owned)
            }
            let mut actual = registry.test_life.as_ref().unwrap().snapshot();
            actual["resident"] = json!(
                probes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| p.upgrade().map(|_| i))
                    .collect::<Vec<_>>()
            );
            actual["host_roots"] = json!(entries.len());
            actual["retired"] = json!(entries.iter().filter(|e| e.retired.get()).count());
            actual["closed"] = json!(!registry.alive.get());
            actual["frames"] = json!(machine.as_ref().map_or(0, Vm::frame_depth));
            actual["calls"] = json!(calls.get());
            assert_eq!(
                actual, case["expected"][index],
                "{} checkpoint {} ({op})",
                case["name"], index
            );
            steps += 1;
        }
        drop(guard);
        drop(handles);
        drop(saved.borrow_mut().take());
        drop(machine);
        drop(storage);
        registry.drain();
        drop(registry);
        drop(saved);
        assert!(probes.iter().all(|p| p.upgrade().is_none()));
        assert!(weak_registry.upgrade().is_none());
        assert!(weak_life.upgrade().is_none());
        cases += 1;
    }
    println!(
        "HOST_REPLAY {}",
        json!({"traces":cases,"steps":steps,"physical_teardowns":cases})
    );
}
#[test]
fn final_registry_drop_releases_borrow_deferred_closed_storage() {
    let mut s = store();
    let o = object(&mut s);
    let probe = s.inner.as_ref().unwrap().probe_for_test(o.slot);
    let root = s.value(ValueInput::Object(o)).unwrap();
    let registry = s.registry.clone();
    let weak = Rc::downgrade(&registry);
    let guard = registry.entries.borrow();
    drop(root);
    drop(s);
    assert!(probe.upgrade().is_some());
    drop(guard);
    // No later check/drain: field teardown must release roots before the arena.
    drop(registry);
    assert!(probe.upgrade().is_none());
    assert!(weak.upgrade().is_none());
}
