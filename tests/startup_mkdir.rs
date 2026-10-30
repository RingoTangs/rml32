use rml32::startup_probe::*;
use std::{path::Path, process::Command};

#[test]
fn all_five_services_create_only_their_memory_log_directory() {
    for (service, name) in [
        (StartupService::Aaa, "aaa"),
        (StartupService::Ccs, "ccs"),
        (StartupService::Csa, "csa"),
        (StartupService::Dba, "dba"),
        (StartupService::Gs, "gs"),
    ] {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut old = StartupProbe::load(root, service, StartupProbeLimits::isolated()).unwrap();
        old.run(25).unwrap();
        let mut p = StartupProbe::load_with_slice(
            root,
            service,
            StartupProbeLimits::isolated(),
            StartupSlice::LogDirectory,
        )
        .unwrap();
        assert_eq!(p.run(100).unwrap(), StartupProbeOutcome::PrefixReached);
        let r = p.report().unwrap();
        assert_eq!((r.stop_pc, r.executed_instructions), (28, 28));
        assert!(!r.initialization_complete);
        assert_eq!(r.globals, old.report().unwrap().globals);
        let events = r.host_events.unwrap();
        assert_eq!(events.len(), 2);
        let ProbeHostEvent::Mkdir(call) = &events[1] else {
            panic!()
        };
        assert_eq!((call.slot, call.pc, call.result), (336, 26, Some(1)));
        assert_eq!(call.argument.bytes, format!("/{name}/log/").as_bytes());
        assert_eq!(
            call.normalized.as_ref().unwrap().bytes,
            format!("/{name}/log").as_bytes()
        );
        let entries = r.filesystem.unwrap().entries;
        assert_eq!(entries.len(), 7);
        assert!(
            entries
                .iter()
                .any(|e| e.path.bytes == format!("/{name}/log").as_bytes() && e.directory)
        );
        let dep = r.next_dependency.unwrap();
        assert_eq!((dep.pc, dep.slot), (30, 583));
        assert_eq!(dep.name, "xdbc_set_output_basic_file");
        assert_eq!(
            r.next_dependency_kind,
            Some(ProbeDependencyKind::GlobalFunction)
        );
        assert_eq!(
            r.next_dependency_reason.as_deref(),
            Some("atomic_function_not_enabled")
        );
    }
}

#[test]
fn cli_selects_a_slice_explicitly_and_keeps_allocation_default() {
    let bin = env!("CARGO_BIN_EXE_rml32-startup-probe");
    let run = |extra: &[&str]| {
        Command::new(bin)
            .args(["--service", "aaa", "--root", env!("CARGO_MANIFEST_DIR")])
            .args(extra)
            .output()
            .unwrap()
    };
    let default = run(&[]);
    let explicit = run(&["--slice", "allocate"]);
    assert_eq!(default.stdout, explicit.stdout);
    let old: serde_json::Value = serde_json::from_slice(&default.stdout).unwrap();
    assert_eq!(old["stop_pc"], 25);
    assert!(old.get("filesystem").is_none());
    assert!(old.get("host_events").is_none());
    let new = run(&["--slice", "log-dir"]);
    assert_eq!(
        new.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&new.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&new.stdout).unwrap();
    assert_eq!(report["stop_pc"], 28);
    assert_eq!(report["host_events"].as_array().unwrap().len(), 2);
    for args in [
        vec!["--slice"],
        vec!["--slice", "unknown"],
        vec!["--slice", "allocate", "--slice", "log-dir"],
        vec!["--slice", "log-dir", "--end-pc", "31"],
    ] {
        let out = run(&args);
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
    }
}
