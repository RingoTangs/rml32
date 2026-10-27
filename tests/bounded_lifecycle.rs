//! D08 through the normal library build, without cfg(test) adapters.
use rml32::*;
include!("boundary_support.rs.inc");
fn limits() -> LifecycleLimits {
    LifecycleLimits {
        max_objects: 4,
        max_function_heads: 4,
        max_ordinary_refs_per_object: 16,
        max_exec_refs_per_object: 4,
        max_owners_per_header: 16,
    }
}
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
fn store(q: LifecycleLimits) -> ObjectStore {
    ObjectStore::with_bounded_lifecycle(ValueLimits::default(), q).unwrap()
}
fn program(s: &mut ObjectStore, vars: usize) -> ProgramId {
    s.register_program(
        prepare_program_with_container_captures(
            image(
                &[
                    fun(0, 0, 0, 2, &[(42, 0, 0x3b)]),
                    fun(1, 0, 0, 9, &[(0, 0, 0x17)]),
                    fun(2, 1, 0, 255, &[(0, 0, 0x37)]),
                    fun(3, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 5)]),
                    fun(4, 1, 0, 255, &[(0, 0, 0x37), (0, 1, 6)]),
                ],
                vars,
                &[],
            ),
            GlobalBankSchema { slots: 0 },
        )
        .unwrap(),
    )
    .unwrap()
}
fn finish(vm: &mut Vm) -> HostValue {
    assert_eq!(
        vm.run(100).unwrap(),
        RunOutcome::Finished(CompletionMeta::Returned)
    );
    vm.take_returned().unwrap()
}
#[test]
fn limits_reject_invalid_ranges_and_allow_zero_global_capacities() {
    for q in [
        LifecycleLimits {
            max_ordinary_refs_per_object: 1,
            ..limits()
        },
        LifecycleLimits {
            max_exec_refs_per_object: 0,
            ..limits()
        },
        LifecycleLimits {
            max_owners_per_header: 0,
            ..limits()
        },
    ] {
        assert_eq!(
            ObjectStore::with_bounded_lifecycle(ValueLimits::default(), q)
                .unwrap_err()
                .kind,
            VmErrorKind::Limit
        );
    }
    let mut s = store(LifecycleLimits {
        max_objects: 0,
        max_function_heads: 0,
        ..limits()
    });
    let p = program(&mut s, 0);
    assert_eq!(s.create(p, &[]).unwrap_err().kind, VmErrorKind::Limit);
    assert!(s.array(&[]).is_ok());
    assert!(s.mapping(&[]).is_ok());
}
#[test]
fn legacy_maintenance_is_unsupported_and_removal_remains_immediate() {
    let mut s = ObjectStore::new().unwrap();
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    assert_eq!(
        s.maintain_lifecycle(ZERO).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    s.remove(a).unwrap();
    assert!(s.get(a).is_err());
    let mut vm = Vm::new(s);
    assert_eq!(
        vm.maintain_lifecycle(ALL).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn freed_records_keep_quota_until_reap_and_ids_never_reuse() {
    let mut s = store(LifecycleLimits {
        max_objects: 1,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    s.remove(a).unwrap();
    assert_eq!(s.create(p, &[]).unwrap_err().kind, VmErrorKind::Limit);
    let idle = s.maintain_lifecycle(ZERO).unwrap();
    assert_eq!(idle.reaped_objects, 0);
    assert!(!idle.scan_complete);
    let done = s.maintain_lifecycle(ALL).unwrap();
    assert_eq!(done.reaped_objects, 1);
    assert_eq!(done.stop, LifecycleStop::Complete);
    let b = s.create(p, &[]).unwrap();
    assert!(b.get() > a.get());
    assert!(s.value(ValueInput::Object(a)).is_err());
}
#[test]
fn shell_raw_transport_retains_until_last_root_and_cannot_resurrect() {
    let mut s = store(LifecycleLimits {
        max_objects: 1,
        max_ordinary_refs_per_object: 2,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let root = s.value(ValueInput::Object(a)).unwrap();
    assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::Limit);
    s.remove(a).unwrap();
    let copy = root.retain().unwrap();
    assert_eq!(copy.snapshot().unwrap(), ValueSnapshot::Object(a));
    assert_eq!(s.maintain_lifecycle(ALL).unwrap().reaped_objects, 0);
    drop(root);
    drop(copy);
    assert_eq!(s.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
    assert!(s.value(ValueInput::Object(a)).is_err());
}
#[test]
fn header_quota_counts_transport_and_container_members_at_peak() {
    let s = store(LifecycleLimits {
        max_owners_per_header: 1,
        ..limits()
    });
    let a = s.array(&[ValueInput::Int(3)]).unwrap();
    assert_eq!(a.retain().unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(
        s.array(&[ValueInput::Handle(&a)]).unwrap_err().kind,
        VmErrorKind::Limit
    );
    assert!(matches!(a.snapshot().unwrap(), ValueSnapshot::Array { .. }));
    drop(a);
    assert!(s.array(&[]).is_ok());
}
#[test]
fn pending_exit_enqueues_once_and_zero_budget_does_not_drain() {
    let mut s = store(limits());
    let p = program(&mut s, 1);
    let a = s.create(p, &[ValueInput::Int(7)]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 0, &[]).unwrap();
    vm.objects_mut().remove(a).unwrap();
    assert!(vm.objects().get(a).is_ok());
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    let value = finish(&mut vm);
    assert_eq!(value.snapshot().unwrap(), ValueSnapshot::Int(42));
    assert!(vm.objects().get(a).is_err());
    let idle = vm.maintain_lifecycle(ZERO).unwrap();
    assert_eq!(idle.queued_objects, 1);
    assert_eq!(idle.drained_objects, 0);
    let done = vm.maintain_lifecycle(ALL).unwrap();
    assert_eq!(done.drained_objects, 1);
    assert_eq!(done.reaped_objects, 1);
}
#[test]
fn execution_handoff_reserves_before_old_lease_is_released() {
    let mut s = store(LifecycleLimits {
        max_exec_refs_per_object: 1,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 1, &[]).unwrap();
    let function = finish(&mut vm);
    vm.start(a, 3, &[ValueInput::Handle(&function)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(vm.frame_depth(), 1);
    assert_eq!(vm.stack().unwrap(), before);
    let mut vm = Vm::new(vm.into_objects());
    vm.start(a, 0, &[]).unwrap();
    assert_eq!(finish(&mut vm).snapshot().unwrap(), ValueSnapshot::Int(42));
}
#[test]
fn function_head_quota_is_returned_by_metadata_reclamation() {
    let mut s = store(LifecycleLimits {
        max_function_heads: 1,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 1, &[]).unwrap();
    let first = finish(&mut vm);
    let ValueSnapshot::Function(meta) = first.snapshot().unwrap() else {
        panic!()
    };
    vm.start(a, 1, &[]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    drop(first);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reclaimed_heads, 1);
    vm.start(a, 1, &[]).unwrap();
    let next = finish(&mut vm);
    let ValueSnapshot::Function(other) = next.snapshot().unwrap() else {
        panic!()
    };
    assert_ne!(meta.id, other.id);
}
#[test]
fn script_copy_normalizes_shell_but_host_transport_stays_raw() {
    let mut s = store(limits());
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let b = s.create(p, &[]).unwrap();
    let root = s.value(ValueInput::Object(b)).unwrap();
    s.remove(b).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 2, &[ValueInput::Handle(&root)]).unwrap();
    assert_eq!(
        finish(&mut vm).snapshot().unwrap(),
        ValueSnapshot::Undefined
    );
    assert_eq!(root.snapshot().unwrap(), ValueSnapshot::Object(b));
}
#[test]
fn transfer_preserves_queue_cursor_and_origin_isolation() {
    let mut s = store(limits());
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 0, &[]).unwrap();
    vm.objects_mut().remove(a).unwrap();
    let mut s = vm.into_objects();
    assert_eq!(s.maintain_lifecycle(ZERO).unwrap().queued_objects, 1);
    s.maintain_lifecycle(LifecycleBudget {
        metadata_entries: 1,
        ..ZERO
    })
    .unwrap();
    let mut vm = Vm::new(s);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().drained_objects, 1);
    let other = store(limits());
    let foreign = other.array(&[]).unwrap();
    assert_eq!(
        vm.objects().retain(&foreign).unwrap_err().kind,
        VmErrorKind::InvalidState
    );
    assert_eq!(
        other.snapshot(&foreign).unwrap(),
        foreign.snapshot().unwrap()
    );
}
#[test]
fn zero_ordinary_owners_do_not_delete_live_objects() {
    let mut s = store(limits());
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    assert_eq!(s.maintain_lifecycle(ALL).unwrap().reaped_objects, 0);
    assert!(s.get(a).is_ok());
}
#[test]
fn close_rejects_new_root_use_but_structural_retirement_is_safe() {
    let mut s = store(limits());
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let root = s.array(&[ValueInput::Object(a)]).unwrap();
    drop(s);
    assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::InvalidState);
    drop(root);
}

#[test]
fn copies_of_one_head_never_duplicate_the_creator_lease() {
    let mut s = store(LifecycleLimits {
        max_ordinary_refs_per_object: 2,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 1, &[]).unwrap();
    let f = finish(&mut vm);
    let copy = f.retain().unwrap();
    vm.start(a, 1, &[]).unwrap();
    assert_eq!(vm.run(100).unwrap_err().kind, VmErrorKind::Limit);
    drop(f);
    drop(copy);
    vm.maintain_lifecycle(ALL).unwrap();
    vm.start(a, 1, &[]).unwrap();
    assert!(matches!(
        finish(&mut vm).snapshot().unwrap(),
        ValueSnapshot::Function(_)
    ));
}

#[test]
fn callback_quota_failure_keeps_operands_and_never_replays_the_effect() {
    use std::{cell::Cell, rc::Rc};
    let mut s = store(LifecycleLimits {
        max_owners_per_header: 3,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let calls = Rc::new(Cell::new(0));
    let count = calls.clone();
    s.register_host_function(
        0,
        HostFunction {
            name: "once".into(),
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
    let array = s.array(&[]).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 4, &[ValueInput::Handle(&array)]).unwrap();
    vm.run(3).unwrap();
    let before = vm.stack().unwrap();
    let failure = vm.run(1).unwrap_err();
    assert_eq!(failure.kind, VmErrorKind::Limit);
    assert_eq!(calls.get(), 1);
    assert_eq!(vm.stack().unwrap(), before);
    assert_eq!(vm.run(100).unwrap_err(), failure);
    assert_eq!(calls.get(), 1);
}

#[test]
fn managed_pending_delete_waits_for_positive_step_then_keeps_one_queue_owner() {
    let mut s = store(limits());
    let mut code = image(&[], 1, &[]);
    code.init.nodes = vec![(7, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33), (0, 0, 0x48)];
    code.init.nodes_offset = 9000;
    let mut catalog = ProgramCatalog::new();
    catalog.insert("init", code).unwrap();
    let p = s
        .register_program(
            prepare_initializable_program(&catalog, "init", LinkLimits::default()).unwrap(),
        )
        .unwrap();
    let mut vm = Vm::new(s);
    let a = vm.begin_initialize(p).unwrap();
    vm.objects_mut().remove(a).unwrap();
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(vm.maintain_lifecycle(ZERO).unwrap().queued_objects, 0);
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::InvalidObject);
    assert_eq!(vm.maintain_lifecycle(ZERO).unwrap().queued_objects, 1);
    let done = vm.maintain_lifecycle(ALL).unwrap();
    assert_eq!(done.drained_objects, 1);
    assert_eq!(done.reaped_objects, 1);
}

#[test]
fn metadata_budget_resumes_and_requires_a_pass_after_reclamation() {
    let mut s = store(limits());
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    s.create(p, &[]).unwrap();
    let one = LifecycleBudget {
        metadata_entries: 1,
        ..ZERO
    };
    let scan = s
        .maintain_lifecycle(LifecycleBudget {
            metadata_entries: 2,
            ..ZERO
        })
        .unwrap();
    assert_eq!(scan.metadata_examined, 2);
    assert!(scan.scan_complete);
    assert_eq!(s.maintain_lifecycle(one).unwrap().metadata_examined, 0);
    s.remove(a).unwrap();
    assert!(!s.maintain_lifecycle(ZERO).unwrap().scan_complete);
    let scan = s.maintain_lifecycle(one).unwrap();
    assert_eq!(scan.reaped_objects, 1);
    assert!(!scan.scan_complete);
    assert!(!s.maintain_lifecycle(one).unwrap().scan_complete);
    assert!(s.maintain_lifecycle(one).unwrap().scan_complete);
    let root = s.array(&[]).unwrap();
    drop(root);
    let mut pruned = 0;
    for _ in 0..16 {
        let step = s.maintain_lifecycle(one).unwrap();
        assert!(step.metadata_examined <= 1);
        pruned += step.pruned_weak_entries;
        if step.scan_complete {
            break;
        }
    }
    assert!(pruned > 0);
    assert!(s.maintain_lifecycle(ZERO).unwrap().scan_complete);
}

#[test]
fn queue_reference_uses_the_reserved_unit_at_the_ordinary_ceiling() {
    let mut s = store(LifecycleLimits {
        max_ordinary_refs_per_object: 2,
        ..limits()
    });
    let p = program(&mut s, 0);
    let a = s.create(p, &[]).unwrap();
    let root = s.value(ValueInput::Object(a)).unwrap();
    let mut vm = Vm::new(s);
    vm.start(a, 0, &[]).unwrap();
    vm.objects_mut().remove(a).unwrap();
    finish(&mut vm);
    assert_eq!(vm.maintain_lifecycle(ZERO).unwrap().queued_objects, 1);
    assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::Limit);
    let done = vm.maintain_lifecycle(ALL).unwrap();
    assert_eq!(done.drained_objects, 1);
    assert_eq!(done.reaped_objects, 0);
    drop(root);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().reaped_objects, 1);
}

#[test]
fn initialization_failure_keeps_a_single_queue_responsibility() {
    let mut s = store(limits());
    let mut code = image(&[], 1, &[]);
    code.init.nodes = vec![(0, 0, 0x33), (0, 0, 0x48)];
    code.init.nodes_offset = 9000;
    let mut catalog = ProgramCatalog::new();
    catalog.insert("fail", code).unwrap();
    let p = s
        .register_program(
            prepare_initializable_program(&catalog, "fail", LinkLimits::default()).unwrap(),
        )
        .unwrap();
    let mut vm = Vm::new(s);
    vm.begin_initialize(p).unwrap();
    let error = vm.run(1).unwrap_err();
    assert_eq!(vm.run(100).unwrap_err(), error);
    let pending = vm.maintain_lifecycle(ZERO).unwrap();
    assert_eq!(pending.queued_objects, 1);
    assert_eq!(pending.drained_objects, 0);
    assert_eq!(vm.maintain_lifecycle(ALL).unwrap().drained_objects, 1);
}
