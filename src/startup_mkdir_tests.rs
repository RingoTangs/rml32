use super::*;
use memory_fs::MemoryDirectories;

fn image(path: &[u8]) -> ProgramImage {
    let bytes = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("aaa/pack_data/lib_aaa32.pak"))
        .unwrap();
    let bytes = PakArchive::parse(&bytes)
        .unwrap()
        .read_member("aaa/start_aaa.o")
        .unwrap();
    let mut image = parse_program(&bytes, ProgramMode::Global).unwrap();
    let index = image.init.nodes[25].0 as usize;
    let mut terminated = path.to_vec();
    terminated.push(0);
    image.strings[index] = Some(crate::ByteString::new(&terminated));
    // Small synthetic allocation for fault matrices; real 1024-item inputs are
    // exercised separately through the authenticated service loader.
    image.init.nodes[0].0 = 3;
    image
}
fn probe(path: &[u8]) -> StartupProbe {
    StartupProbe::new_with_slice(
        image(path),
        StartupProbeLimits::isolated(),
        StartupSlice::LogDirectory,
    )
    .unwrap()
}
fn last_call(p: &StartupProbe) -> ProbeMkdirCall {
    let events = p.host_events.as_ref().unwrap().borrow();
    let ProbeHostEvent::Mkdir(call) = events.last().unwrap() else {
        panic!()
    };
    call.clone()
}

#[test]
fn byte_paths_normalize_without_using_host_path_rules() {
    for (raw, want) in [
        (b"//aaa///log/".as_slice(), b"/aaa/log".as_slice()),
        (b"/aaa/log/\0ignored", b"/aaa/log"),
        (b"/aaa/\xff/", b"/aaa/\xff"),
        (b"/", b"/"),
    ] {
        assert_eq!(MemoryDirectories::normalize(raw).unwrap(), want);
    }
    for raw in [
        b"".as_slice(),
        b"aaa/log",
        b"/aaa/./log",
        b"/aaa/../log",
        b"/v/x",
        b"/x.pak/dir",
        b"/x.dbfs/dir",
    ] {
        assert_eq!(
            MemoryDirectories::normalize(raw).unwrap_err().kind,
            VmErrorKind::UnsupportedFeature
        );
    }
    let mut max = vec![b'x'; 255];
    max[0] = b'/';
    assert_eq!(MemoryDirectories::normalize(&max).unwrap(), max);
    max.push(b'x');
    assert_eq!(
        MemoryDirectories::normalize(&max).unwrap_err().kind,
        VmErrorKind::Bounds
    );
}

#[test]
fn directory_creation_returns_native_one_or_zero_and_preserves_other_nodes() {
    let mut fs = MemoryDirectories::new().unwrap();
    assert_eq!(fs.mkdir(b"/aaa/log").unwrap(), 1);
    let snapshot = fs.snapshot();
    assert_eq!(fs.mkdir(b"/aaa/log").unwrap(), 0);
    assert_eq!(fs.snapshot(), snapshot);
    assert_eq!(fs.mkdir(b"/missing/leaf").unwrap(), 0);
    fs.nodes.push((b"/aaa/file".to_vec(), false));
    assert_eq!(fs.mkdir(b"/aaa/file").unwrap(), 0);
    assert_eq!(fs.mkdir(b"/aaa/file/child").unwrap(), 0);
    assert_eq!(fs.mkdir(b"/").unwrap(), 0);
}

#[test]
fn directory_quota_and_reserve_failures_do_not_publish_a_node() {
    let mut fs = MemoryDirectories::new().unwrap();
    fs.max_nodes = 6;
    let before = fs.snapshot();
    assert_eq!(fs.mkdir(b"/aaa/log").unwrap_err().kind, VmErrorKind::Limit);
    assert_eq!(before, fs.snapshot());
    fs.max_nodes = 16;
    for cut in 0..2 {
        let e =
            crate::copy_plan::with_workspace_failure(cut, || fs.mkdir(b"/aaa/log")).unwrap_err();
        assert_eq!(e.kind, VmErrorKind::Limit);
        assert_eq!(before, fs.snapshot());
    }
}

#[test]
fn new_slice_respects_zero_split_and_oversized_budgets() {
    let mut full = probe(b"/aaa/log/");
    full.run(28).unwrap();
    let expected = full.report().unwrap();
    for schedule in [
        vec![1; 28],
        vec![25, 1, 1, 1],
        vec![0, 26, 0, 1, 1],
        vec![100],
    ] {
        let mut p = probe(b"/aaa/log/");
        let initial = p.report().unwrap();
        p.run(0).unwrap();
        assert_eq!(p.report().unwrap(), initial);
        for n in schedule {
            p.run(n).unwrap();
        }
        assert_eq!(p.report().unwrap(), expected);
        assert_eq!(p.run(100).unwrap(), StartupProbeOutcome::PrefixReached);
        assert_eq!(p.run(0).unwrap(), StartupProbeOutcome::PrefixReached);
        assert_eq!(p.report().unwrap(), expected);
    }
    let mut p = probe(b"/aaa/log/");
    p.run(26).unwrap();
    assert_eq!(p.host_events.as_ref().unwrap().borrow().len(), 1);
    p.run(1).unwrap();
    assert_eq!(last_call(&p).result, Some(1));
    assert_eq!(p.report().unwrap().stop_pc, 27);
    assert_eq!(p.run(1).unwrap(), StartupProbeOutcome::PrefixReached);
}

#[test]
fn preparation_and_type_checks_do_not_grant_unrelated_calls() {
    for node in [(27, 1, 6), (336, 2, 6), (583, -126, 7), (0, 0, 0x48)] {
        let mut input = image(b"/aaa/log");
        input.init.nodes[26] = node;
        let e = StartupProbe::new_with_slice(
            input,
            StartupProbeLimits::isolated(),
            StartupSlice::LogDirectory,
        )
        .err()
        .unwrap();
        assert_eq!(e.pc, Some(26));
    }
    let mut input = image(b"/aaa/log");
    input.init.nodes[25] = (2, 0, 0x3b);
    let mut p = StartupProbe::new_with_slice(
        input,
        StartupProbeLimits::isolated(),
        StartupSlice::LogDirectory,
    )
    .unwrap();
    let e = p.run(28).unwrap_err();
    assert_eq!((e.kind, e.pc), (VmErrorKind::UnsupportedType, Some(26)));
    assert_eq!(p.host_events.as_ref().unwrap().borrow().len(), 1);
    assert_eq!(p.filesystem.as_ref().unwrap().borrow().nodes.len(), 6);
}

#[test]
fn mkdir_zero_is_a_normal_return_and_invalid_paths_are_sticky_errors() {
    for path in [b"/missing/leaf/".as_slice(), b"/aaa/"] {
        let mut p = probe(path);
        assert_eq!(p.run(28).unwrap(), StartupProbeOutcome::PrefixReached);
        assert_eq!(last_call(&p).result, Some(0));
        assert_eq!(p.filesystem.as_ref().unwrap().borrow().nodes.len(), 6);
    }
    for path in [b"relative/dir".as_slice(), b"/aaa/../dir"] {
        let mut p = probe(path);
        let e = p.run(28).unwrap_err();
        assert_eq!(e.pc, Some(26));
        assert_eq!(p.run(1).unwrap_err(), e);
        assert_eq!(p.report().unwrap().stop_pc, 26);
        assert_eq!(last_call(&p).error, Some("UnsupportedFeature".into()));
    }
}

#[test]
fn every_mkdir_workspace_failure_keeps_vm_atomic_and_preserves_completed_effects() {
    let mut calibration = probe(b"/aaa/log/");
    calibration.run(26).unwrap();
    let (result, count) = crate::copy_plan::with_workspace_count(|| calibration.run(1));
    result.unwrap();
    assert!(count >= 4);
    let mut before_effect = 0;
    let mut after_effect = 0;
    for cut in 0..=count {
        let mut p = probe(b"/aaa/log/");
        p.run(26).unwrap();
        let before_stack = p.vm.stack().unwrap();
        let before_globals = p.vm.objects().global_values().unwrap();
        let result = crate::copy_plan::with_workspace_failure(cut, || p.run(1));
        if cut == count {
            result.unwrap();
            assert_eq!(last_call(&p).result, Some(1));
            continue;
        }
        let e = result.unwrap_err();
        assert_eq!((e.kind, e.pc), (VmErrorKind::Limit, Some(26)));
        assert_eq!(p.vm.stack().unwrap(), before_stack);
        assert_eq!(p.vm.objects().global_values().unwrap(), before_globals);
        let fs = p.filesystem.as_ref().unwrap().borrow();
        let created = fs.nodes.iter().any(|(n, _)| n == b"/aaa/log");
        drop(fs);
        let r = p.report().unwrap();
        let returned = r
            .host_events
            .as_ref()
            .unwrap()
            .iter()
            .any(|x| matches!(x,ProbeHostEvent::Mkdir(c) if c.result==Some(1)));
        assert_eq!(created, returned);
        if created {
            after_effect += 1
        } else {
            before_effect += 1
        }
        assert_eq!(p.run(1).unwrap_err(), e);
        assert_eq!(p.report().unwrap(), r);
    }
    assert!(before_effect > 0 && after_effect > 0);
}

#[test]
fn dropping_new_slice_releases_both_backend_and_callback_event_graphs() {
    for budget in [0, 26, 27, 28] {
        let mut p = probe(b"/aaa/log/");
        p.run(budget).unwrap();
        let fs = Rc::downgrade(p.filesystem.as_ref().unwrap());
        let events = Rc::downgrade(p.host_events.as_ref().unwrap());
        drop(p);
        assert!(fs.upgrade().is_none());
        assert!(events.upgrade().is_none());
    }
}

#[test]
#[ignore = "explicit small mkdir model replay; RML32_MKDIR_IN/OUT"]
fn replay_mkdir_cases() {
    use std::io::{BufRead, Write};
    let input =
        std::io::BufReader::new(fs::File::open(std::env::var("RML32_MKDIR_IN").unwrap()).unwrap());
    let mut output = std::io::BufWriter::new(
        fs::File::create(std::env::var("RML32_MKDIR_OUT").unwrap()).unwrap(),
    );
    let mut count = 0;
    for line in input.lines() {
        let t: serde_json::Value = serde_json::from_str(&line.unwrap()).unwrap();
        assert!(t.get("expected").is_none());
        let hex = t["path_hex"].as_str().unwrap();
        assert!(hex.len().is_multiple_of(2));
        let path: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        let mut p = probe(&path);
        {
            let mut backend = p.filesystem.as_ref().unwrap().borrow_mut();
            if let Some(nodes) = t.get("nodes") {
                backend.nodes = nodes
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|node| {
                        let raw = node["path"].as_str().unwrap().as_bytes().to_vec();
                        assert_eq!(MemoryDirectories::normalize(&raw).unwrap(), raw);
                        (raw, node["directory"].as_bool().unwrap())
                    })
                    .collect();
            }
            backend.max_nodes = t["max_nodes"].as_u64().unwrap() as usize;
            assert!(backend.nodes.len() <= backend.max_nodes && backend.max_nodes <= 16);
        }
        let initial = p.report().unwrap();
        let mut observations = Vec::new();
        for budget in t["budgets"].as_array().unwrap() {
            let error = p
                .run(budget.as_u64().unwrap() as usize)
                .err()
                .map(|e| format!("{:?}", e.kind));
            observations.push(serde_json::json!({"error":error,"report":p.report().unwrap()}));
        }
        writeln!(
            output,
            "{}",
            serde_json::json!({"id":t["id"],"initial":initial,"steps":observations})
        )
        .unwrap();
        count += 1;
    }
    output.flush().unwrap();
    println!("MKDIR_REPLAY {count}");
}
