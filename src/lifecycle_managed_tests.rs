//! Actual prepared managed tasks through the checked boundary, test mode only.
use super::*;
use crate::{ByteString, ProgramImage, ProgramMode, parse_program};
use serde_json::{Value as Json, json};
include!("../tests/boundary_support.rs.inc");
fn store() -> ObjectStore {
    ObjectStore::with_test_managed_lifecycle(ValueLimits::default()).unwrap()
}
fn set_init(p: &mut ProgramImage, nodes: &[Node]) {
    p.init.nodes = nodes.to_vec();
    p.init.nodes_offset = 9000;
    p.init.metadata_hex.clear();
}
fn inherit(child: &mut ProgramImage, parent: &ProgramImage, name: &[u8], bases: [i16; 3]) {
    child.inherits.push(crate::program::Inherit {
        name: Some(ByteString::new(name)),
        offsets: [
            bases[0],
            bases[1],
            bases[2],
            parent.constitution_count.unwrap(),
            parent.max_functions,
            parent.max_vars,
        ],
    });
}
fn log(marker: i16) -> Vec<Node> {
    vec![
        (0, 0, 0x3e),
        (0, 0, 0x3b),
        (0, 0, 0x44),
        (1, 0, 0x22),
        (0, 0, 0x3e),
        (marker, 0, 0x3b),
        (0, 0, 0x44),
        (0, 0, 0x3d),
        (0, 0, 0x3b),
        (0, 0, 0x43),
        (0, 0, 0x04),
        (0, 0, 0x33),
    ]
}
fn layer(
    create: i16,
    destruct: i16,
    marker: i16,
    vars: usize,
    fail_init: bool,
    fail_create: bool,
    fail_destruct: bool,
) -> ProgramImage {
    let mut init = vec![(7, 0, 0x3b), (0, 0, 0x4e), (0, 0, 0x33)];
    if fail_init {
        init.push((0, 0, 0x33));
    }
    init.push((0, 0, 0x48));
    let mut c = vec![(0, 0, 0x37), (0, 0, 0x4e), (0, 0, 0x33)];
    c.extend(log(marker));
    if fail_create {
        c.push((0, 0, 0x33));
    }
    let mut d = log(marker + 10);
    if fail_destruct {
        d.push((0, 0, 0x33));
    }
    let mut p = image(
        &[fun(create, 1, 0, 1, &c), fun(destruct, 0, 0, 1, &d)],
        vars,
        &[],
    );
    p.functions[0].name = Some(ByteString::new(b"create\0"));
    p.functions[1].name = Some(ByteString::new(b"destruct\0"));
    set_init(&mut p, &init);
    p
}
fn family(
    init_fail: Option<usize>,
    create_fail: Option<usize>,
    destruct_fail: bool,
) -> crate::ProgramCatalog {
    let a = layer(
        0,
        1,
        1,
        1,
        init_fail == Some(0),
        create_fail == Some(0),
        destruct_fail,
    );
    let b = layer(
        0,
        1,
        2,
        1,
        init_fail == Some(1),
        create_fail == Some(1),
        destruct_fail,
    );
    let mut root = layer(
        4,
        5,
        3,
        3,
        init_fail == Some(2),
        create_fail == Some(2),
        destruct_fail,
    );
    root.constitution_count = Some(3);
    inherit(&mut root, &a, b"a", [1, 0, 1]);
    inherit(&mut root, &b, b"b", [2, 2, 2]);
    let mut c = crate::ProgramCatalog::new();
    for (key, p, create, destruct) in [("a", a, 0, 1), ("b", b, 0, 1), ("root", root, 4, 5)] {
        c.insert(key, p).unwrap();
        c.bind_lifecycle(
            key,
            crate::LifecycleBinding {
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
fn machine(catalog: &crate::ProgramCatalog) -> (Vm, ProgramId) {
    let mut s = store();
    let p = s
        .register_program(
            crate::prepare_constructible_program(catalog, "root", Default::default()).unwrap(),
        )
        .unwrap();
    (Vm::new(s), p)
}
fn trace(vm: &Vm) -> HostValue {
    vm.objects()
        .mapping(&[(ValueInput::Int(0), ValueInput::Int(0))])
        .unwrap()
}
fn mark(trace: &HostValue, key: i32) -> Option<ValueSnapshot> {
    trace.get(ValueInput::Int(key)).unwrap()
}
fn audit(vm: &Vm) -> Json {
    let entries = vm.registry.entries.borrow();
    let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    let inner = vm.inner.as_ref().unwrap();
    inner.lifecycle_audit_host_for_test(&roots);
    json!({"life":inner.objects().lifecycle_snapshot_for_test(),"managed":inner.managed_snapshot_for_test(),"host_roots":entries.len()})
}
fn state(s: &Json, id: ObjectId) -> &str {
    s["life"]["objects"][id.get().to_string()]["state"]
        .as_str()
        .unwrap()
}
fn until_phase(vm: &mut Vm, phase: &str) {
    for _ in 0..1000 {
        if audit(vm)["managed"]["construct"]["phase"] == phase {
            return;
        }
        assert_eq!(vm.run(1).unwrap(), RunOutcome::BudgetExhausted);
    }
    panic!("phase not reached");
}
#[test]
fn initialization_replaces_real_frame_ids_and_releases_last_exec() {
    let (mut vm, p) = machine(&family(None, None, false));
    let o = vm.begin_initialize(p).unwrap();
    let mut ids = std::collections::BTreeSet::new();
    loop {
        let s = audit(&vm);
        assert_eq!(s["life"]["objects"]["0"]["execs"], 1);
        ids.insert(s["managed"]["frames"][0]["id"].as_u64().unwrap());
        if vm.run(1).unwrap() == RunOutcome::Finished(CompletionMeta::Initialized(o)) {
            break;
        }
    }
    assert_eq!(ids.len(), 3);
    assert_eq!(audit(&vm)["life"]["objects"]["0"]["execs"], 0);
    assert_eq!(state(&audit(&vm), o), "alive");
}
#[test]
fn construct_runs_all_initializers_before_all_create_layers() {
    let (mut vm, p) = machine(&family(None, None, false));
    let t = trace(&vm);
    let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    assert_eq!(audit(&vm)["managed"]["construct"]["parameter_owners"], 2);
    until_phase(&mut vm, "Create");
    for key in 1..=3 {
        assert_eq!(mark(&t, key), None);
    }
    assert_eq!(
        vm.run(1000).unwrap(),
        RunOutcome::Finished(CompletionMeta::Constructed(o))
    );
    assert_eq!(mark(&t, 1), Some(ValueSnapshot::Int(1)));
    assert_eq!(mark(&t, 2), Some(ValueSnapshot::Int(2)));
    assert_eq!(mark(&t, 3), Some(ValueSnapshot::Int(3)));
    assert_eq!(state(&audit(&vm), o), "alive");
}
#[test]
fn zero_budget_preserves_task_frame_parameter_and_cursor() {
    let (mut vm, p) = machine(&family(None, None, false));
    let t = trace(&vm);
    vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    let before = audit(&vm);
    assert_eq!(vm.run(0).unwrap(), RunOutcome::BudgetExhausted);
    assert_eq!(audit(&vm), before);
}
#[test]
fn initialization_failures_at_each_layer_queue_without_destruct() {
    for layer in 0..3 {
        let (mut vm, p) = machine(&family(Some(layer), None, false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        let error = vm.run(1000).unwrap_err();
        assert_eq!(error.kind, VmErrorKind::StackUnderflow);
        assert!(error.initialization.is_some());
        assert_eq!(
            error.lifecycle.unwrap().phase,
            crate::LifecyclePhase::Initialize
        );
        assert_eq!(state(&audit(&vm), o), "queued");
        assert_eq!(audit(&vm)["managed"]["frames"], json!([]));
        assert_eq!(mark(&t, 0), Some(ValueSnapshot::Int(0)));
        vm.inner
            .as_mut()
            .unwrap()
            .objects_mut()
            .drain_one_for_test()
            .unwrap();
        assert_eq!(state(&audit(&vm), o), "freed");
    }
}
#[test]
fn each_failed_create_rolls_back_visited_layers_in_reverse() {
    for layer in 0..3 {
        let (mut vm, p) = machine(&family(None, Some(layer), false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        let e = vm.run(1000).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::StackUnderflow);
        assert_eq!(e.lifecycle.unwrap().phase, crate::LifecyclePhase::Create);
        for i in 0..=layer {
            assert_eq!(
                mark(&t, i as i32 + 1),
                Some(ValueSnapshot::Int(i as i32 + 1))
            );
            assert_eq!(
                mark(&t, i as i32 + 11),
                Some(ValueSnapshot::Int((2 * (layer + 1) - i) as i32))
            );
        }
        assert_eq!(state(&audit(&vm), o), "queued");
        assert!(vm.inner.as_ref().unwrap().rollback_errors().is_empty());
    }
}
#[test]
fn rollback_errors_continue_and_primary_error_is_sticky() {
    let (mut vm, p) = machine(&family(None, Some(2), true));
    let t = trace(&vm);
    let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    let first = vm.run(1000).unwrap_err();
    assert_eq!(
        first.lifecycle.as_ref().unwrap().phase,
        crate::LifecyclePhase::Create
    );
    assert_eq!(vm.inner.as_ref().unwrap().rollback_errors().len(), 3);
    assert_eq!(vm.run(1).unwrap_err(), first);
    assert_eq!(mark(&t, 13), Some(ValueSnapshot::Int(4)));
    assert_eq!(mark(&t, 12), Some(ValueSnapshot::Int(5)));
    assert_eq!(mark(&t, 11), Some(ValueSnapshot::Int(6)));
    assert_eq!(state(&audit(&vm), o), "queued");
}
#[test]
fn deletion_in_each_managed_phase_aborts_at_next_positive_budget() {
    for phase in ["Initialize", "Create", "RollbackDestruct"] {
        let (mut vm, p) = machine(&family(None, Some(2), false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        until_phase(&mut vm, phase);
        vm.objects_mut().remove(o).unwrap();
        let before = audit(&vm);
        assert_eq!(state(&before, o), "pending");
        vm.run(0).unwrap();
        assert_eq!(audit(&vm), before);
        let original = t.snapshot().unwrap();
        let e = vm.run(1).unwrap_err();
        assert_eq!(state(&audit(&vm), o), "queued");
        assert_eq!(t.snapshot().unwrap(), original);
        if phase == "RollbackDestruct" {
            assert_eq!(e.kind, VmErrorKind::StackUnderflow);
            assert_eq!(
                vm.inner
                    .as_ref()
                    .unwrap()
                    .rollback_errors()
                    .last()
                    .unwrap()
                    .kind,
                VmErrorKind::InvalidObject
            );
        } else {
            assert_eq!(e.kind, VmErrorKind::InvalidObject);
        }
    }
}
#[test]
fn extraction_cancels_each_phase_and_preserves_origin_mode_and_queue() {
    for phase in ["Initialize", "Create", "RollbackDestruct"] {
        let (mut vm, p) = machine(&family(None, Some(2), false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        until_phase(&mut vm, phase);
        let origin = vm.registry.identity.id;
        let mut s = vm.into_objects();
        assert_eq!(s.registry.identity.id, origin);
        assert!(s.registry.alive.get());
        let snap = s.inner.as_ref().unwrap().lifecycle_snapshot_for_test();
        assert_eq!(snap["objects"]["0"]["state"], "queued");
        s.inner.as_mut().unwrap().drain_one_for_test().unwrap();
        let mut rebuilt = Vm::new(s);
        assert!(rebuilt.start(o, 0, &[]).is_err());
        audit(&rebuilt);
    }
}
#[test]
fn task_parameter_graph_survives_host_root_drop_until_explicit_cleanup() {
    let (mut vm, p) = machine(&family(None, Some(0), false));
    let other = vm
        .objects_mut()
        .create(p, &[ValueInput::Undefined; 3])
        .unwrap();
    let root = vm
        .objects()
        .mapping(&[
            (ValueInput::Int(0), ValueInput::Int(0)),
            (ValueInput::Int(99), ValueInput::Object(other)),
        ])
        .unwrap();
    let o = vm
        .begin_construct(p, Some(ValueInput::Handle(&root)))
        .unwrap();
    drop(root);
    assert_eq!(audit(&vm)["life"]["objects"]["0"]["refs"], 1);
    vm.run(1000).unwrap_err();
    assert_eq!(state(&audit(&vm), o), "queued");
    assert_eq!(audit(&vm)["life"]["objects"]["0"]["refs"], 1);
    vm.inner
        .as_mut()
        .unwrap()
        .objects_mut()
        .drain_one_for_test()
        .unwrap();
    assert_eq!(audit(&vm)["life"]["objects"]["0"]["refs"], 0);
}
#[test]
fn missing_task_parameter_is_detected_by_graph_root_audit() {
    let (mut vm, p) = machine(&family(None, None, false));
    let t = trace(&vm);
    vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    let registry = vm.registry.clone();
    let entries = registry.entries.borrow();
    let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| vm
            .inner
            .as_ref()
            .unwrap()
            .lifecycle_audit_omitting_task_for_test(&roots)))
        .is_err()
    );
    drop(entries);
    audit(&vm);
}
#[test]
fn missing_initialized_frame_is_detected_by_exec_audit() {
    let (mut vm, p) = machine(&family(None, None, false));
    vm.begin_initialize(p).unwrap();
    let life = vm.registry.test_life.as_ref().unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| life.audit(&[], &[]))).is_err()
    );
    audit(&vm);
}
#[test]
fn prepublication_failures_preserve_prior_completion_and_no_new_records() {
    let (mut vm, p) = machine(&family(None, None, false));
    let first = vm.begin_initialize(p).unwrap();
    vm.run(1000).unwrap();
    let before = audit(&vm);
    crate::copy_plan::with_workspace_failure(0, || assert!(vm.begin_initialize(p).is_err()));
    assert_eq!(audit(&vm), before);
    crate::cell::with_allocation_failure(0, || assert!(vm.begin_initialize(p).is_err()));
    assert_eq!(audit(&vm), before);
    assert!(vm.objects().get(first).is_ok());
}
#[test]
fn typed_parameter_rejection_preserves_completed_task() {
    let (mut vm, p) = machine(&family(None, None, false));
    let o = vm.begin_initialize(p).unwrap();
    vm.run(1000).unwrap();
    let before = audit(&vm);
    assert!(vm.begin_construct(p, Some(ValueInput::Int(2))).is_err());
    assert_eq!(audit(&vm), before);
    assert!(vm.objects().get(o).is_ok());
}
#[test]
fn closing_active_vm_keeps_actual_records_until_last_host_root() {
    for phase in ["Initialize", "Create", "RollbackDestruct"] {
        let (mut vm, p) = machine(&family(None, Some(2), false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        until_phase(&mut vm, phase);
        let root = vm.objects().value(ValueInput::Object(o)).unwrap();
        let probe = vm.inner.as_ref().unwrap().objects().probe_for_test(o.slot);
        let registry = Rc::downgrade(&vm.registry);
        drop(vm);
        assert!(probe.upgrade().is_some());
        assert!(root.snapshot().is_err());
        drop(root);
        drop(t);
        assert!(probe.upgrade().is_none());
        assert!(registry.upgrade().is_none());
    }
}
fn single(p: ProgramImage, binding: crate::LifecycleBinding) -> (Vm, ProgramId) {
    let mut c = crate::ProgramCatalog::new();
    c.insert("root", p).unwrap();
    c.bind_lifecycle("root", binding).unwrap();
    machine(&c)
}
#[test]
fn initialization_helpers_have_one_lease_per_installed_frame() {
    let mut p = image(
        &[
            fun(0, 0, 0, 1, &[(1, 0, 7), (0, 0, 0x33)]),
            fun(1, 0, 0, 1, &[]),
        ],
        0,
        &[],
    );
    set_init(&mut p, &[(0, 0, 7), (0, 0, 0x33), (0, 0, 0x48)]);
    let (mut vm, p) = single(p, Default::default());
    let o = vm.begin_initialize(p).unwrap();
    let mut maximum = 0;
    for _ in 0..30 {
        let s = audit(&vm);
        maximum = maximum.max(s["managed"]["frames"].as_array().unwrap().len());
        assert_eq!(s["life"]["objects"]["0"]["execs"], json!(vm.frame_depth()));
        if vm.run(1).unwrap() == RunOutcome::Finished(CompletionMeta::Initialized(o)) {
            break;
        }
    }
    assert_eq!(maximum, 3);
    assert_eq!(audit(&vm)["life"]["objects"]["0"]["execs"], 0);
}
#[test]
fn nested_frame_limit_failure_queues_and_unwinds_all_task_frames() {
    let mut p = image(&[fun(0, 0, 0, 1, &[])], 0, &[]);
    set_init(&mut p, &[(0, 0, 7), (0, 0, 0x33), (0, 0, 0x48)]);
    let (vm, p) = single(p, Default::default());
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        vm::VmLimits {
            max_frames: 1,
            ..Default::default()
        },
    );
    let o = vm.begin_initialize(p).unwrap();
    assert_eq!(vm.run(1).unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(state(&audit(&vm), o), "queued");
    assert_eq!(vm.frame_depth(), 0);
}
#[test]
fn zero_frame_create_entry_failure_still_owns_exactly_one_cleanup_node() {
    let mut p = image(&[fun(0, 1, 0, 1, &[])], 0, &[]);
    p.functions[0].name = Some(ByteString::new(b"create"));
    set_init(&mut p, &[(0, 0, 0x48)]);
    let (vm, p) = single(
        p,
        crate::LifecycleBinding {
            create: Some(0),
            destruct: None,
        },
    );
    let mut vm = Vm::with_limits(
        vm.into_objects(),
        vm::VmLimits {
            max_stack: 0,
            ..Default::default()
        },
    );
    let t = trace(&vm);
    let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    let e = vm.run(1).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.lifecycle.unwrap().phase, crate::LifecyclePhase::Create);
    let s = audit(&vm);
    assert_eq!(state(&s, o), "queued");
    assert_eq!(s["life"]["objects"]["0"]["refs"], 1);
    assert_eq!(vm.frame_depth(), 0);
}
#[test]
fn missing_create_layer_still_visits_destruct_in_failure_rollback() {
    let mut parent = image(&[fun(0, 0, 0, 1, &[(0, 0, 0x33)])], 0, &[]);
    parent.functions[0].name = Some(ByteString::new(b"destruct"));
    set_init(&mut parent, &[(0, 0, 0x48)]);
    let mut root = image(&[fun(1, 0, 0, 1, &[(0, 0, 0x33)])], 0, &[]);
    root.functions[0].name = Some(ByteString::new(b"create"));
    set_init(&mut root, &[(0, 0, 0x48)]);
    root.constitution_count = Some(2);
    inherit(&mut root, &parent, b"parent", [1, 0, 0]);
    let mut c = crate::ProgramCatalog::new();
    c.insert("parent", parent).unwrap();
    c.insert("root", root).unwrap();
    c.bind_inherit("root", b"parent", "parent");
    c.bind_lifecycle(
        "parent",
        crate::LifecycleBinding {
            create: None,
            destruct: Some(0),
        },
    )
    .unwrap();
    c.bind_lifecycle(
        "root",
        crate::LifecycleBinding {
            create: Some(1),
            destruct: None,
        },
    )
    .unwrap();
    let (mut vm, p) = machine(&c);
    let o = vm.begin_construct(p, None).unwrap();
    let e = vm.run(100).unwrap_err();
    assert_eq!(e.source, "root");
    let errors = vm.inner.as_ref().unwrap().rollback_errors();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].source, "parent");
    assert_eq!(errors[0].function, Some(0));
    assert_eq!(state(&audit(&vm), o), "queued");
}
#[test]
fn no_lifecycle_functions_complete_without_an_ordinary_object_owner() {
    let mut p = image(&[], 0, &[]);
    set_init(&mut p, &[(0, 0, 0x48)]);
    let (mut vm, p) = single(p, Default::default());
    let o = vm.begin_construct(p, None).unwrap();
    assert_eq!(
        vm.run(1).unwrap(),
        RunOutcome::Finished(CompletionMeta::Constructed(o))
    );
    let s = audit(&vm);
    assert_eq!(s["life"]["objects"]["0"]["refs"], 0);
    assert_eq!(s["life"]["objects"]["0"]["execs"], 0);
    assert_eq!(state(&s, o), "alive");
}
#[test]
fn explicit_cancel_is_idempotent_in_every_phase() {
    for phase in ["Initialize", "Create", "RollbackDestruct"] {
        let (mut vm, p) = machine(&family(None, Some(2), false));
        let t = trace(&vm);
        let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        until_phase(&mut vm, phase);
        vm.inner.as_mut().unwrap().cancel_execution_for_test();
        let first = audit(&vm);
        assert_eq!(state(&first, o), "queued");
        vm.inner.as_mut().unwrap().cancel_execution_for_test();
        assert_eq!(audit(&vm), first);
    }
}
#[test]
fn failed_drain_keeps_queue_variables_and_cleanup_responsibility() {
    let (mut vm, p) = machine(&family(None, Some(2), false));
    let t = trace(&vm);
    let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    vm.run(1000).unwrap_err();
    let before = audit(&vm);
    let cell = vm
        .inner
        .as_ref()
        .unwrap()
        .objects()
        .objects_for_audit_for_test(o.slot)
        .unwrap()[0]
        .clone();
    cell.with_read_borrow_for_test(|| {
        assert!(
            vm.inner
                .as_mut()
                .unwrap()
                .objects_mut()
                .drain_one_for_test()
                .is_err()
        )
    });
    assert_eq!(audit(&vm), before);
    crate::copy_plan::with_workspace_failure(0, || {
        assert!(
            vm.inner
                .as_mut()
                .unwrap()
                .objects_mut()
                .drain_one_for_test()
                .is_err()
        )
    });
    assert_eq!(audit(&vm), before);
    vm.inner
        .as_mut()
        .unwrap()
        .objects_mut()
        .drain_one_for_test()
        .unwrap();
    assert_eq!(state(&audit(&vm), o), "freed");
}
#[test]
fn failed_initialization_self_reference_is_broken_only_at_drain() {
    let mut p = image(&[], 1, &[]);
    set_init(
        &mut p,
        &[
            (0, 0, 0x41),
            (0, 0, 0x4e),
            (0, 0, 0x33),
            (0, 0, 0x33),
            (0, 0, 0x48),
        ],
    );
    let (mut vm, p) = single(p, Default::default());
    let o = vm.begin_initialize(p).unwrap();
    let probe = vm.inner.as_ref().unwrap().objects().probe_for_test(o.slot);
    vm.run(100).unwrap_err();
    let s = audit(&vm);
    assert_eq!(s["life"]["objects"]["0"]["refs"], 2);
    assert_eq!(s["life"]["objects"]["0"]["cleaned"], false);
    assert!(probe.upgrade().is_some());
    vm.inner
        .as_mut()
        .unwrap()
        .objects_mut()
        .drain_one_for_test()
        .unwrap();
    assert_eq!(state(&audit(&vm), o), "freed");
    assert!(probe.upgrade().is_none());
}
#[test]
fn new_task_after_sticky_failure_preserves_old_queue_and_unique_ids() {
    let (mut vm, p) = machine(&family(Some(0), None, false));
    let old = vm.begin_initialize(p).unwrap();
    vm.run(100).unwrap_err();
    let new = vm.begin_initialize(p).unwrap();
    assert!(new.get() > old.get());
    let s = audit(&vm);
    assert_eq!(state(&s, old), "queued");
    assert_eq!(s["life"]["objects"][new.get().to_string()]["execs"], 1);
}
#[test]
fn first_create_argument_allocation_failure_rolls_back_with_primary_limit() {
    let (mut vm, p) = machine(&family(None, None, false));
    let t = trace(&vm);
    let o = vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
    vm.run(11).unwrap();
    assert_eq!(audit(&vm)["managed"]["init"]["position"], 2);
    crate::cell::with_allocation_failure(0, || {
        let _ = vm.run(1);
    });
    let e = vm.run(1000).unwrap_err();
    assert_eq!(e.kind, VmErrorKind::Limit);
    assert_eq!(e.lifecycle.unwrap().phase, crate::LifecyclePhase::Create);
    assert_eq!(state(&audit(&vm), o), "queued");
}
#[test]
fn core_and_host_modes_continue_to_reject_prepared_managed_tasks() {
    let c = family(None, None, false);
    let mut s = ObjectStore::with_test_host_lifecycle(ValueLimits::default()).unwrap();
    let p = s
        .register_program(
            crate::prepare_constructible_program(&c, "root", Default::default()).unwrap(),
        )
        .unwrap();
    let mut vm = Vm::new(s);
    assert_eq!(
        vm.begin_initialize(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(
        vm.begin_construct(p, None).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
    assert_eq!(vm.frame_depth(), 0);
    let mut raw = object::ObjectStore::with_test_lifecycle(ValueLimits::default()).unwrap();
    let p = raw
        .register_program(
            crate::prepare_constructible_program(&c, "root", Default::default()).unwrap(),
        )
        .unwrap();
    assert_eq!(
        vm::Vm::new(raw).begin_initialize(p).unwrap_err().kind,
        VmErrorKind::UnsupportedFeature
    );
}
#[test]
fn error_context_tracks_real_source_constitution_and_function() {
    for i in 0..3 {
        let (mut vm, p) = machine(&family(None, Some(i), false));
        let t = trace(&vm);
        vm.begin_construct(p, Some(ValueInput::Handle(&t))).unwrap();
        let e = vm.run(1000).unwrap_err();
        assert_eq!(e.source, ["a", "b", "root"][i]);
        assert_eq!(e.function, Some((i * 2) as i16));
        assert_eq!(e.lifecycle.unwrap().constitution, [1, 2, 0][i]);
        assert!(e.pc.is_some());
        assert!(e.offset.is_some());
        audit(&vm);
    }
}
fn minimal_catalog(
    layers: usize,
    init_fail: bool,
    create_fail: Option<usize>,
    destruct_fail: bool,
    no_destruct: bool,
) -> crate::ProgramCatalog {
    let mut parts = Vec::new();
    for i in 0..layers {
        let base = if i + 1 == layers { (2 * i) as i16 } else { 0 };
        let mut functions = vec![fun(
            base,
            1,
            0,
            1,
            if create_fail == Some(i) {
                &[(0, 0, 0x33)]
            } else {
                &[]
            },
        )];
        if !no_destruct {
            functions.push(fun(
                base + 1,
                0,
                0,
                1,
                if destruct_fail { &[(0, 0, 0x33)] } else { &[] },
            ));
        }
        let mut p = image(&functions, 0, &[]);
        p.functions[0].name = Some(ByteString::new(b"create"));
        if !no_destruct {
            p.functions[1].name = Some(ByteString::new(b"destruct"));
        }
        set_init(
            &mut p,
            if init_fail && i == 0 {
                &[(0, 0, 0x33), (0, 0, 0x48)]
            } else {
                &[(0, 0, 0x48)]
            },
        );
        parts.push(p);
    }
    let mut root = parts.pop().unwrap();
    root.constitution_count = Some(layers as i16);
    for (i, parent) in parts.iter().enumerate() {
        inherit(
            &mut root,
            parent,
            format!("p{i}").as_bytes(),
            [(i + 1) as i16, (2 * i) as i16, 0],
        );
    }
    let mut c = crate::ProgramCatalog::new();
    for (i, p) in parts.into_iter().enumerate() {
        let name = format!("p{i}");
        c.insert(&name, p).unwrap();
        c.bind_lifecycle(
            &name,
            crate::LifecycleBinding {
                create: Some(0),
                destruct: (!no_destruct).then_some(1),
            },
        )
        .unwrap();
        c.bind_inherit("root", name.as_bytes(), &name);
    }
    c.insert("root", root).unwrap();
    c.bind_lifecycle(
        "root",
        crate::LifecycleBinding {
            create: Some((2 * (layers - 1)) as i16),
            destruct: (!no_destruct).then_some((2 * (layers - 1) + 1) as i16),
        },
    )
    .unwrap();
    c
}
#[test]
#[ignore = "explicit managed timeline worker; requires RML32_MANAGED_TRACES"]
fn replay_managed_lifecycle_traces() {
    use std::io::BufRead;
    let file = std::fs::File::open(
        std::env::var_os("RML32_MANAGED_TRACES").expect("explicit trace input required"),
    )
    .unwrap();
    let mut traces = 0;
    let mut steps = 0;
    for line in std::io::BufReader::new(file).lines() {
        let case: Json = serde_json::from_str(&line.unwrap()).unwrap();
        let catalog = minimal_catalog(
            case["layers"].as_u64().unwrap() as usize,
            case["init_fail"].as_bool().unwrap_or(false),
            case["create_fail"].as_u64().map(|n| n as usize),
            case["destruct_fail"].as_bool().unwrap_or(false),
            case["no_destruct"].as_bool().unwrap_or(false),
        );
        let (vm, p) = machine(&catalog);
        let mut vm = Vm::with_limits(
            vm.into_objects(),
            vm::VmLimits {
                max_stack: case["stack_limit"].as_u64().unwrap_or(65536) as usize,
                ..Default::default()
            },
        );
        let sentinel = vm.objects_mut().create(p, &[]).unwrap();
        assert_eq!(sentinel.get(), 0);
        let root = vm
            .objects()
            .mapping(&[(ValueInput::Int(0), ValueInput::Object(sentinel))])
            .unwrap();
        let registry = vm.registry.clone();
        let weak_registry = Rc::downgrade(&registry);
        let weak_life = Rc::downgrade(registry.test_life.as_ref().unwrap());
        let counter = Rc::downgrade(
            &registry.entries.borrow()[0]
                .payload
                .value
                .counter()
                .unwrap(),
        );
        let mut probes = vec![
            vm.inner
                .as_ref()
                .unwrap()
                .objects()
                .probe_for_test(sentinel.slot),
        ];
        let mut machine = Some(vm);
        let mut storage: Option<ObjectStore> = None;
        let mut task = None;
        for row in case["steps"].as_array().unwrap() {
            let cmd = &row["command"];
            let op = cmd[0].as_str().unwrap();
            match op {
                "begin" => {
                    let vm = machine.as_mut().unwrap();
                    let o = if case["construct"].as_bool().unwrap() {
                        vm.begin_construct(p, Some(ValueInput::Handle(&root)))
                            .unwrap()
                    } else {
                        vm.begin_initialize(p).unwrap()
                    };
                    assert_eq!(o.get(), 1);
                    probes.push(vm.inner.as_ref().unwrap().objects().probe_for_test(o.slot));
                    task = Some(o);
                }
                "run" => {
                    let outcome = match machine
                        .as_mut()
                        .unwrap()
                        .run(cmd[1].as_u64().unwrap() as usize)
                    {
                        Ok(RunOutcome::BudgetExhausted) => "budget",
                        Ok(RunOutcome::Finished(CompletionMeta::Initialized(_))) => "initialized",
                        Ok(RunOutcome::Finished(CompletionMeta::Constructed(_))) => "constructed",
                        Err(_) => "error",
                        _ => panic!("unexpected completion"),
                    };
                    assert_eq!(outcome, cmd[2].as_str().unwrap(), "{}", case["name"]);
                }
                "delete" => {
                    machine
                        .as_mut()
                        .unwrap()
                        .objects_mut()
                        .remove(task.unwrap())
                        .unwrap();
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
                "close" => {
                    drop(machine.take());
                    drop(storage.take());
                }
                "inspect" => {}
                _ => panic!("unknown managed command {op}"),
            }
            let entries = registry.entries.borrow();
            let roots = entries.iter().map(|e| &e.payload).collect::<Vec<_>>();
            let managed = if let Some(vm) = &machine {
                vm.inner
                    .as_ref()
                    .unwrap()
                    .lifecycle_audit_host_for_test(&roots);
                vm.inner.as_ref().unwrap().managed_snapshot_for_test()
            } else {
                registry
                    .test_life
                    .as_ref()
                    .unwrap()
                    .audit_host(&[], &[], &roots);
                json!({"init":null,"construct":null,"frames":[],"failure":null,"rollback_errors":[]})
            };
            let mut life = registry.test_life.as_ref().unwrap().snapshot();
            life["resident"] = json!(
                probes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| p.upgrade().map(|_| i))
                    .collect::<Vec<_>>()
            );
            let actual = json!({"life":life,"managed":managed,"parameter_owners":counter.upgrade().map(|c|c.owners()),"closed":!registry.alive.get(),"host_roots":entries.len()});
            assert_eq!(
                actual, row["expected"],
                "{} checkpoint {} ({op})",
                case["name"], steps
            );
            steps += 1;
        }
        drop(machine);
        drop(storage);
        drop(root);
        registry.drain();
        drop(registry);
        assert!(probes.iter().all(|p| p.upgrade().is_none()));
        assert!(weak_registry.upgrade().is_none());
        assert!(weak_life.upgrade().is_none());
        assert!(counter.upgrade().is_none());
        traces += 1;
    }
    println!(
        "MANAGED_REPLAY {}",
        json!({"traces":traces,"steps":steps,"physical_teardowns":traces})
    );
}
