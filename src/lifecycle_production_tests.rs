//! Private borrow/fault probes for the same bounded state used by public clients.
use super::*;
use crate::{
    LifecycleBudget, LifecycleLimits, LifecycleStop, ProgramImage, ProgramMode, parse_program,
    prepare_program,
};
include!("../tests/boundary_support.rs.inc");
const ZERO: LifecycleBudget = LifecycleBudget {
    host_roots: 0,
    objects: 0,
    metadata_entries: 0,
};
const ALL: LifecycleBudget = LifecycleBudget {
    host_roots: 64,
    objects: 64,
    metadata_entries: 4096,
};
fn store() -> (ObjectStore, ProgramId) {
    let mut s = ObjectStore::with_bounded_lifecycle(
        ValueLimits::default(),
        LifecycleLimits {
            max_objects: 4,
            max_function_heads: 1,
            max_ordinary_refs_per_object: 16,
            max_exec_refs_per_object: 4,
            max_owners_per_header: 16,
        },
    )
    .unwrap();
    let p = s
        .register_program(
            prepare_program(image(&[fun(0, 0, 0, 2, &[(9, 0, 0x3b)])], 1, &[])).unwrap(),
        )
        .unwrap();
    (s, p)
}
fn enqueue(s: ObjectStore, id: ObjectId) -> ObjectStore {
    let mut vm = Vm::new(s);
    vm.start(id, 0, &[]).unwrap();
    vm.objects_mut().remove(id).unwrap();
    vm.into_objects()
}
#[test]
fn zero_budget_does_not_call_registry_check_or_retire_roots() {
    let (mut s, _) = store();
    let root = s.value(ValueInput::Int(1)).unwrap();
    let guard = s.registry.entries.borrow();
    drop(root);
    drop(guard);
    let p = s.maintain_lifecycle(ZERO).unwrap();
    assert_eq!(p.retired_roots, 0);
    assert_eq!(p.pending_roots, 1);
    assert_eq!(s.registry.entries.borrow().len(), 1);
    let p = s
        .maintain_lifecycle(LifecycleBudget {
            host_roots: 1,
            ..ZERO
        })
        .unwrap();
    assert_eq!(p.retired_roots, 1);
    assert_eq!(p.pending_roots, 0);
}
#[test]
fn retirement_budget_uses_ascending_lease_ids() {
    let (mut s, _) = store();
    let a = s.value(ValueInput::Int(1)).unwrap();
    let b = s.value(ValueInput::Int(2)).unwrap();
    let c = s.value(ValueInput::Int(3)).unwrap();
    let first = a.id;
    let guard = s.registry.entries.borrow();
    drop(c);
    drop(a);
    drop(b);
    drop(guard);
    let p = s
        .maintain_lifecycle(LifecycleBudget {
            host_roots: 1,
            ..ZERO
        })
        .unwrap();
    assert_eq!(p.retired_roots, 1);
    assert_eq!(p.pending_roots, 2);
    assert!(s.registry.entries.borrow().iter().all(|e| e.id > first));
}
#[test]
fn borrow_block_keeps_lifo_head_after_previous_object_committed() {
    let (mut s, p) = store();
    let a = s.create(p, &[ValueInput::Int(1)]).unwrap();
    let b = s.create(p, &[ValueInput::Int(2)]).unwrap();
    let mut s = enqueue(enqueue(s, a), b);
    let cell = s.inner.as_ref().unwrap().get(a.slot).unwrap_err();
    assert_eq!(cell.kind, VmErrorKind::InvalidObject);
    // Queued records remain resident but are intentionally hidden by get().
    let cell = s
        .inner
        .as_ref()
        .unwrap()
        .objects_for_audit_for_test(a.slot)
        .unwrap()[0]
        .clone();
    let p = cell.with_read_borrow_for_test(|| s.maintain_lifecycle(ALL).unwrap());
    assert_eq!(p.stop, LifecycleStop::BorrowBlocked);
    assert_eq!(p.drained_objects, 1);
    assert_eq!(p.queued_objects, 1);
    assert_eq!(p.metadata_examined, 0);
    let p = s.maintain_lifecycle(ALL).unwrap();
    assert_eq!(p.drained_objects, 1);
    assert_eq!(p.reaped_objects, 2);
}
#[test]
fn workspace_block_returns_partial_success_and_preserves_cleanup_responsibility() {
    let (mut s, p) = store();
    let a = s.create(p, &[ValueInput::Int(1)]).unwrap();
    let b = s.create(p, &[ValueInput::Int(2)]).unwrap();
    let mut s = enqueue(enqueue(s, a), b);
    let p = crate::copy_plan::with_workspace_failure(1, || s.maintain_lifecycle(ALL).unwrap());
    assert_eq!(p.stop, LifecycleStop::WorkspaceLimit);
    assert_eq!(p.drained_objects, 1);
    assert_eq!(p.queued_objects, 1);
    assert_eq!(s.maintain_lifecycle(ALL).unwrap().drained_objects, 1);
}
#[test]
fn epoch_wrap_does_not_fail_mutation_and_requires_rescan() {
    let (mut s, p) = store();
    s.create(p, &[ValueInput::Int(1)]).unwrap();
    assert!(s.maintain_lifecycle(ALL).unwrap().scan_complete);
    let epoch = s.registry.test_life.as_ref().unwrap().ctx.epoch.clone();
    epoch.value.set(u64::MAX);
    let root = s.array(&[]).unwrap();
    assert!(epoch.force.get());
    assert!(!s.maintain_lifecycle(ZERO).unwrap().scan_complete);
    assert!(epoch.force.get());
    assert!(s.maintain_lifecycle(ALL).unwrap().scan_complete);
    assert!(!epoch.force.get());
    drop(root);
}
#[test]
fn stale_source_and_destination_versions_reject_without_publication() {
    let (s, _) = store();
    let life = s.registry.test_life.as_ref().unwrap();
    let a = life.cell(value::Value::Int(1)).unwrap();
    let b = life.cell(value::Value::Int(2)).unwrap();
    let mut plan = life.plan();
    let v = plan.copy(a.clone(), |v| Ok(v.clone())).unwrap();
    plan.write(b.clone(), v).unwrap();
    a.replace(value::Value::Int(3)).unwrap();
    assert_eq!(plan.commit().unwrap_err().kind, VmErrorKind::InvalidState);
    assert_eq!(b.read().unwrap(), value::Value::Int(2));
    let mut plan = life.plan();
    plan.write(b.clone(), value::Value::Int(7)).unwrap();
    b.replace(value::Value::Int(8)).unwrap();
    assert_eq!(plan.commit().unwrap_err().kind, VmErrorKind::InvalidState);
    assert_eq!(b.read().unwrap(), value::Value::Int(8));
}
#[test]
fn closed_epoch_fails_before_retiring_or_draining() {
    let (mut s, _) = store();
    s.registry.alive.set(false);
    assert_eq!(
        s.maintain_lifecycle(ALL).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    s.registry.alive.set(true);
}
#[test]
fn final_container_owner_clears_members_even_while_physical_view_survives() {
    for mapping in [false, true] {
        let (mut s, p) = store();
        let a = s.create(p, &[ValueInput::Int(1)]).unwrap();
        let root = if mapping {
            s.mapping(&[(ValueInput::Int(0), ValueInput::Object(a))])
                .unwrap()
        } else {
            s.array(&[ValueInput::Object(a)]).unwrap()
        };
        let physical = s.registry.raw(&root).unwrap();
        s.remove(a).unwrap();
        drop(root);
        assert_eq!(s.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
        let members = crate::container::lifecycle_members(&physical).unwrap();
        assert_eq!(members[0].read().unwrap(), value::Value::Undefined);
        drop(physical);
    }
}

#[test]
fn unpublished_head_reservation_aborts_on_output_workspace_failure() {
    let (mut s, _) = store();
    let code = image(
        &[
            fun(0, 0, 0, 9, &[(1, 0, 0x17)]),
            fun(1, 0, 0, 2, &[(9, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    let p = s
        .register_program(
            crate::prepare_program_with_callable_values(code, crate::GlobalBankSchema { slots: 0 })
                .unwrap(),
        )
        .unwrap();
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 0, &[]).unwrap();
    vm.run(2).unwrap();
    let error = crate::copy_plan::with_workspace_failure(0, || vm.run(1)).unwrap_err();
    assert_eq!(error.kind, VmErrorKind::Limit);
    vm.start(a, 0, &[]).unwrap();
    vm.run(100).unwrap();
    let f = vm.take_returned().unwrap();
    assert!(matches!(f.snapshot().unwrap(), ValueSnapshot::Function(_)));
}

#[test]
fn retired_function_metadata_stays_charged_while_a_physical_view_survives() {
    let (mut s, _) = store();
    let code = image(
        &[
            fun(0, 0, 0, 9, &[(1, 0, 0x17)]),
            fun(1, 0, 0, 2, &[(9, 0, 0x3b)]),
        ],
        0,
        &[],
    );
    let p = s
        .register_program(
            crate::prepare_program_with_callable_values(code, crate::GlobalBankSchema { slots: 0 })
                .unwrap(),
        )
        .unwrap();
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 0, &[]).unwrap();
    vm.run(100).unwrap();
    let f = vm.take_returned().unwrap();
    let physical = vm.registry.raw(&f).unwrap();
    drop(f);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reclaimed_heads, 0);
    vm.start(a, 0, &[]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    drop(physical);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reclaimed_heads, 1);
}

#[test]
fn host_root_keeps_closed_arena_until_retirement_and_then_releases_control() {
    let (s, _) = store();
    let root = s.array(&[]).unwrap();
    let registry = Rc::downgrade(&s.registry);
    let context = Rc::downgrade(&s.registry.test_life.as_ref().unwrap().ctx);
    drop(s);
    assert!(registry.upgrade().is_some());
    assert!(context.upgrade().is_some());
    drop(root);
    assert!(registry.upgrade().is_none());
    assert!(context.upgrade().is_none());
}

#[test]
fn closing_bounded_store_breaks_callback_root_cycles() {
    let (mut s, _) = store();
    let root = s.array(&[]).unwrap();
    let registry = Rc::downgrade(&s.registry);
    let context = Rc::downgrade(&s.registry.test_life.as_ref().unwrap().ctx);
    s.register_host_function(
        0,
        HostFunction {
            name: "cycle".into(),
            signature: crate::HostSignature {
                required: vec![],
                optional: vec![],
                rest: None,
                returns: crate::HostReturn::Void,
            },
            callback: Rc::new(move |_, _, _| {
                let _ = root.snapshot();
                Ok(HostResult::Void)
            }),
        },
    )
    .unwrap();
    drop(s);
    assert!(registry.upgrade().is_none());
    assert!(context.upgrade().is_none());
}

#[test]
fn raw_transfer_succeeds_at_the_header_owner_ceiling() {
    let s = ObjectStore::with_bounded_lifecycle(
        ValueLimits::default(),
        LifecycleLimits {
            max_objects: 0,
            max_function_heads: 0,
            max_ordinary_refs_per_object: 2,
            max_exec_refs_per_object: 1,
            max_owners_per_header: 1,
        },
    )
    .unwrap();
    let life = s.registry.test_life.as_ref().unwrap();
    let value = life.array(vec![value::Value::Int(7)]).unwrap();
    let counter = value.counter().unwrap();
    let source = life.cell(value).unwrap();
    let mut plan = life.plan();
    let target = plan.transfer(source.clone()).unwrap();
    plan.commit().unwrap();
    assert_eq!(counter.owners(), 1);
    assert_eq!(source.read().unwrap(), value::Value::Undefined);
    drop(target);
    assert_eq!(counter.owners(), 0);
}
