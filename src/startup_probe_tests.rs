use super::*;

fn image() -> ProgramImage {
    let service = StartupService::Aaa;
    let (package, member) = service.source().split_once(".pak/").unwrap();
    let bytes =
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("{package}.pak"))).unwrap();
    let bytes = PakArchive::parse(&bytes)
        .unwrap()
        .read_member(member)
        .unwrap();
    parse_program(&bytes, ProgramMode::Global).unwrap()
}
fn probe() -> StartupProbe {
    StartupProbe::new(image(), StartupProbeLimits::isolated()).unwrap()
}

#[test]
fn zero_budget_and_all_budget_partitions_preserve_the_exact_fence() {
    let mut baseline = probe();
    let initial = baseline.report().unwrap();
    assert_eq!(
        baseline.run(0).unwrap(),
        StartupProbeOutcome::BudgetExhausted
    );
    assert_eq!(baseline.report().unwrap(), initial);
    baseline.run(25).unwrap();
    let expected = baseline.report().unwrap();
    for schedule in [
        vec![1; 25],
        vec![24, 1],
        vec![0, 2, 0, 23],
        vec![25],
        vec![usize::MAX],
    ] {
        let mut p = probe();
        for budget in schedule {
            p.run(budget).unwrap();
        }
        assert_eq!(p.report().unwrap(), expected);
        assert_eq!(
            p.run(usize::MAX).unwrap(),
            StartupProbeOutcome::PrefixReached
        );
        assert_eq!(p.run(0).unwrap(), StartupProbeOutcome::PrefixReached);
        assert_eq!(p.report().unwrap(), expected);
    }
}

#[test]
fn array_sizes_are_undefined_filled_and_obey_the_rust_item_limit() {
    for size in [0, 1, 1023, 1024, 4096] {
        let mut image = image();
        image.init.nodes[0].0 = size;
        let mut p = StartupProbe::new(image, StartupProbeLimits::isolated()).unwrap();
        assert_eq!(p.run(25).unwrap(), StartupProbeOutcome::PrefixReached);
        let ValueSnapshot::Array { elements, .. } = &p.vm.objects().global_values().unwrap()[2]
        else {
            panic!()
        };
        assert_eq!(elements.len(), size as usize);
        assert!(elements.iter().all(|v| *v == ValueSnapshot::Undefined));
    }
}

#[test]
fn failed_allocation_keeps_pc_and_globals_and_never_replays_callback() {
    for (size, kind) in [(-1, VmErrorKind::Bounds), (4097, VmErrorKind::Limit)] {
        let mut image = image();
        let offset = image.init.nodes_offset;
        image.init.nodes[0].0 = size;
        let mut p = StartupProbe::new(image, StartupProbeLimits::isolated()).unwrap();
        let e = p.run(25).unwrap_err();
        assert_eq!(
            (
                e.kind,
                e.pc,
                e.offset,
                e.initialization,
                e.function,
                e.object
            ),
            (kind, Some(1), Some(offset + 4), Some(0), None, None)
        );
        assert_eq!(p.run(0).unwrap_err(), e);
        let r = p.report().unwrap();
        assert_eq!(r.stop_reason, "failed");
        assert_eq!(r.host_calls.len(), 1);
        assert!(r.globals.iter().all(|v| *v == ProbeValue::Undefined));
    }
}

#[test]
fn preparation_rejects_calls_control_transfer_and_invalid_operand_indices() {
    for (pc, node) in [
        (1, (336, 1, 0x06)),
        (1, (0, 2, 0x06)),
        (0, (0, 0, 0x07)),
        (0, (0, 0, 0x1f)),
        (0, (0, 0, 0x48)),
        (2, (105, 0, 0x36)),
        (13, (32767, 0, 0x40)),
    ] {
        let mut image = image();
        let offset = image.init.nodes_offset;
        image.init.nodes[pc] = node;
        let e = StartupProbe::new(image, StartupProbeLimits::isolated())
            .err()
            .unwrap();
        assert_eq!(
            (e.pc, e.offset, e.initialization),
            (Some(pc), Some(offset + 4 * pc), Some(0))
        );
    }
}

#[test]
fn image_kind_capacities_and_short_prefix_are_rejected() {
    for variant in 0..5 {
        let mut image = image();
        match variant {
            0 => image.mode = ProgramMode::Object,
            1 => image.efuns_count = 389,
            2 => image.max_vars = -1,
            3 => image.function_base = 558,
            _ => image.init.nodes.truncate(24),
        }
        assert!(StartupProbe::new(image, StartupProbeLimits::isolated()).is_err());
    }
    let mut image = image();
    image.init.nodes_offset = usize::MAX;
    assert!(StartupProbe::new(image, StartupProbeLimits::isolated()).is_err());
}

#[test]
fn missing_string_and_unbalanced_stack_fail_at_the_original_location() {
    let mut input = image();
    let offset = input.init.nodes_offset;
    let index = input.init.nodes[13].0 as usize;
    input.strings[index] = None;
    let mut p = StartupProbe::new(input, StartupProbeLimits::isolated()).unwrap();
    let e = p.run(25).unwrap_err();
    assert_eq!((e.pc, e.offset), (Some(13), Some(offset + 52)));
    let mut input = image();
    input.init.nodes[24] = (1, 0, 0x3b);
    let mut p = StartupProbe::new(input, StartupProbeLimits::isolated()).unwrap();
    let e = p.run(25).unwrap_err();
    assert_eq!(
        (e.kind, e.pc, e.offset),
        (VmErrorKind::InvalidProgram, Some(25), Some(offset + 100))
    );
    assert_eq!(p.run(1).unwrap_err(), e);
}

#[test]
fn stack_frame_and_root_limits_are_effective_without_charging_global_reference_cells() {
    for variant in 0..3 {
        let mut limits = StartupProbeLimits::isolated();
        match variant {
            0 => limits.vm.max_frames = 1,
            1 => limits.values.max_host_roots = 0,
            _ => limits.vm.max_stack = 1,
        }
        let mut p = StartupProbe::new(image(), limits).unwrap();
        let e = p.run(25).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::Limit);
        assert_eq!(e.pc, Some(if variant < 2 { 1 } else { 2 }));
        let report = p.report().unwrap();
        assert_eq!(Some(report.stop_pc), e.pc);
        assert_eq!(Some(report.stop_offset), e.offset);
        assert_eq!(report.executed_instructions, report.stop_pc);
        assert_eq!(p.calls.borrow().len(), usize::from(variant != 0));
        assert!(
            p.vm.objects()
                .global_values()
                .unwrap()
                .iter()
                .all(|v| *v == ValueSnapshot::Undefined)
        );
    }
    // The existing reference-cell quota concerns temporary owned cells, not
    // references to the already allocated stable Global bank.
    let mut limits = StartupProbeLimits::isolated();
    limits.vm.max_reference_cells = 0;
    let mut p = StartupProbe::new(image(), limits).unwrap();
    assert_eq!(p.run(25).unwrap(), StartupProbeOutcome::PrefixReached);
}

#[test]
fn every_observed_workspace_failure_is_atomic_and_sticky() {
    // Calibrate the actual reserve count for each effectful instruction, then
    // fail each reserve individually and compare the complete global snapshot.
    for pc in [1, 3, 7, 11, 15, 17, 19, 23] {
        let small = || {
            let mut input = image();
            input.init.nodes[0].0 = 3;
            StartupProbe::new(input, StartupProbeLimits::isolated()).unwrap()
        };
        let mut calibration = small();
        calibration.run(pc).unwrap();
        let (result, count) = crate::copy_plan::with_workspace_count(|| calibration.run(1));
        result.unwrap();
        assert!(count > 0, "no calibrated workspace at {pc}");
        for after in 0..count {
            let mut p = small();
            p.run(pc).unwrap();
            let before = p.vm.objects().global_values().unwrap();
            let e = crate::copy_plan::with_workspace_failure(after, || p.run(1)).unwrap_err();
            assert_eq!((e.kind, e.pc), (VmErrorKind::Limit, Some(pc)));
            assert_eq!(p.vm.objects().global_values().unwrap(), before);
            let calls = p.calls.borrow().len();
            assert_eq!(p.run(1).unwrap_err(), e);
            assert_eq!(p.calls.borrow().len(), calls);
        }
    }
}

#[test]
fn dropping_paused_finished_and_failed_probes_releases_their_callback_graph() {
    for budget in [0, 2, 3, 20, 25] {
        let mut p = probe();
        p.run(budget).unwrap();
        let weak = Rc::downgrade(&p.calls);
        drop(p);
        assert!(weak.upgrade().is_none());
    }
    let mut input = image();
    input.init.nodes[0].0 = -1;
    let mut p = StartupProbe::new(input, StartupProbeLimits::isolated()).unwrap();
    p.run(25).unwrap_err();
    let weak = Rc::downgrade(&p.calls);
    drop(p);
    assert!(weak.upgrade().is_none());
}

#[test]
fn preparation_does_not_mark_the_global_prefix_as_complete_initialization() {
    let input = image();
    let p = crate::prepared::prepare_startup_prefix(&input, "fixture").unwrap();
    assert!(!p.is_initializable());
    assert!(!p.is_constructible());
    assert!(p.functions.is_empty());
    assert_eq!(
        p.initializations.as_ref().unwrap()[0].instructions.len(),
        25
    );
    assert_eq!(input.init.nodes.len(), 661);
}
