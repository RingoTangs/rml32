use rml32::startup_probe::*;
use std::{path::Path, process::Command};

#[test]
fn five_pinned_services_execute_their_real_prefix_and_only_allocate_array() {
    for (service, slots, offset, total) in [
        (StartupService::Aaa, 105, 450136, 661),
        (StartupService::Ccs, 102, 557463, 667),
        (StartupService::Csa, 98, 500201, 639),
        (StartupService::Dba, 98, 500354, 639),
        (StartupService::Gs, 359, 1793584, 4368),
    ] {
        let mut probe = StartupProbe::load(
            Path::new(env!("CARGO_MANIFEST_DIR")),
            service,
            StartupProbeLimits::isolated(),
        )
        .unwrap();
        assert_eq!(
            probe.run(usize::MAX).unwrap(),
            StartupProbeOutcome::PrefixReached
        );
        let report = probe.report().unwrap();
        assert_eq!(report.source, service.source());
        assert_eq!(report.sha256, service.sha256());
        assert_eq!(report.function_base, 572);
        assert_eq!(report.original_init_instructions, total);
        assert_eq!(
            (
                report.executed_instructions,
                report.stop_pc,
                report.stop_offset
            ),
            (25, 25, offset + 100)
        );
        assert_eq!(report.stop_reason, "prefix_boundary");
        assert!(!report.initialization_complete);
        assert_eq!(report.globals.len(), slots);
        let mut expected = vec![ProbeValue::Undefined; slots];
        expected[2] = ProbeValue::Array {
            elements: vec![ProbeValue::Undefined; 1024],
        };
        expected[3] = ProbeValue::Int { value: 2 };
        expected[4] = ProbeValue::Int { value: 0 };
        expected[5] = ProbeValue::Bytes {
            bytes_hex: "2f786462632f".into(),
        };
        expected[6] = ProbeValue::Mapping { entries: vec![] };
        expected[7] = ProbeValue::Int { value: 0 };
        assert_eq!(report.globals, expected);
        assert_eq!(
            report.host_calls,
            vec![ProbeHostCall {
                slot: 0,
                name: "allocate_array".into(),
                pc: 1,
                offset: offset + 4,
                argument: 1024
            }]
        );
        assert_eq!(
            report.next_dependency,
            Some(ProbeDependency {
                pc: 26,
                offset: offset + 104,
                slot: 336,
                name: "mkdir".into()
            })
        );
    }
}

#[test]
fn cli_has_explicit_service_selection_and_classified_exit_codes() {
    let binary = env!("CARGO_BIN_EXE_rml32-startup-probe");
    for args in [
        vec![],
        vec!["--service", "unknown"],
        vec!["--service", "aaa", "--end-pc", "26"],
        vec!["--service", "aaa", "--service", "gs"],
        vec!["--root"],
    ] {
        let out = Command::new(binary).args(args).output().unwrap();
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
    }
    let out = Command::new(binary)
        .args([
            "--service",
            "aaa",
            "--root",
            "/nonexistent-rml32-probe-root",
        ])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    let out = Command::new(binary)
        .args(["--service", "aaa", "--root", env!("CARGO_MANIFEST_DIR")])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stderr.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["stop_pc"], 25);
    assert_eq!(report["initialization_complete"], false);

    // A structurally valid package with the exact member name must still fail
    // authentication before its replacement payload can be prepared/executed.
    let root = std::env::temp_dir().join(format!("rml32-startup-hash-{}", std::process::id()));
    let package = root.join("aaa/pack_data/lib_aaa32.pak");
    std::fs::create_dir_all(package.parent().unwrap()).unwrap();
    let mut bytes = vec![0; 144];
    for (offset, word) in [
        (0, 0x11223344u32),
        (4, 64),
        (16, 1),
        (20, 80),
        (24, 64),
        (80, 0x80000000),
        (84, 144),
        (88, 1),
        (92, 1),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
    }
    bytes[36..39].copy_from_slice(b"aaa");
    bytes[100..111].copy_from_slice(b"start_aaa.o");
    bytes.push(0);
    std::fs::write(&package, bytes).unwrap();
    let out = Command::new(binary)
        .args(["--service", "aaa", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("SHA-256 mismatch"));
    std::fs::remove_dir_all(root).unwrap();
}
