use rml32::{
    AbiProfile, ByteString, ErrorKind, PakArchive, ProgramDescriptor, ProgramMode, lzss,
    parse_program,
    program::{Inherit, parse_program_with_profile},
    sha256, validate_inheritance,
};
use std::{fs, process::Command};

fn word(v: &mut Vec<u8>, n: i32) {
    v.extend(n.to_le_bytes());
}
fn short(v: &mut Vec<u8>, n: i16) {
    v.extend(n.to_le_bytes());
}
fn string(v: &mut Vec<u8>, bytes: &[u8]) {
    word(v, bytes.len() as i32);
    v.extend(bytes);
}

/// Small independently constructed format examples; never interpreted.
fn program(object: bool, nodes: &[(i16, i8, u8)], metadata: &[u8], strings: &[&[u8]]) -> Vec<u8> {
    let mut v = Vec::new();
    word(&mut v, 0x10001);
    word(&mut v, -123);
    word(&mut v, 0);
    word(&mut v, 0); // defines
    word(&mut v, strings.len() as i32);
    for s in strings {
        string(&mut v, s);
    }
    word(&mut v, 0);
    word(&mut v, 0); // names
    short(&mut v, 0);
    short(&mut v, 0);
    short(&mut v, 559);
    short(&mut v, 0);
    word(&mut v, 0); // hidden
    if object {
        short(&mut v, 1);
        word(&mut v, 0);
    }
    word(&mut v, 0); // DLT
    word(&mut v, 0); // symbols
    word(&mut v, nodes.len() as i32);
    for (a, b, op) in nodes {
        short(&mut v, *a);
        v.extend([*b as u8, *op]);
    }
    word(&mut v, metadata.len() as i32);
    v.extend(metadata);
    word(&mut v, 0); // runtime
    v
}
fn finish() -> [(i16, i8, u8); 1] {
    [(0, 0, 0x1d)]
}
fn pak(payload: &[u8], attrib: u32, stored: usize, original: usize, name: &[u8]) -> Vec<u8> {
    let mut v = vec![0; 80];
    v[..4].copy_from_slice(&0x11223344u32.to_le_bytes());
    v[4..8].copy_from_slice(&64u32.to_le_bytes());
    v[16..20].copy_from_slice(&attrib.to_le_bytes());
    v[20..24].copy_from_slice(&80u32.to_le_bytes());
    v[24..28].copy_from_slice(&(stored as u32).to_le_bytes());
    v[28..32].copy_from_slice(&(original as u32).to_le_bytes());
    v[36..36 + name.len()].copy_from_slice(name);
    v.extend(payload);
    v
}

#[test]
fn parses_global_and_object_and_preserves_opaque_header() {
    for object in [false, true] {
        let bytes = program(object, &finish(), &[], &[]);
        let p = parse_program(&bytes, ProgramMode::Auto).unwrap();
        assert_eq!(
            p.mode,
            if object {
                ProgramMode::Object
            } else {
                ProgramMode::Global
            }
        );
        assert_eq!(p.opaque_header, -123);
        assert_eq!(p.bytes_consumed, bytes.len());
    }
}
#[test]
fn rejects_every_truncation_and_trailing_bytes() {
    let mut bytes = program(true, &finish(), &[], &[]);
    for end in 0..bytes.len() {
        assert!(
            parse_program(&bytes[..end], ProgramMode::Auto).is_err(),
            "accepted truncation {end}"
        );
    }
    bytes.push(0);
    assert!(
        parse_program(&bytes, ProgramMode::Object)
            .unwrap_err()
            .message
            .contains("trailing")
    );
}

#[test]
fn rejects_a_genuinely_ambiguous_object_global_image() {
    // Both layouts consume this synthetic file to EOF. The six-byte displacement
    // of object-only fields makes its code overlap the global metadata blob.
    let mut v = Vec::new();
    for n in [0x10001, 0, 0, 0, 0, 1, 65539] {
        word(&mut v, n);
    }
    let mut names = vec![0; 65539];
    names[12..14].copy_from_slice(b"o\0");
    names[65536..65538].copy_from_slice(b"g\0");
    v.extend(names);
    for n in [0, 0, 559, 0] {
        short(&mut v, n);
    }
    word(&mut v, 0);
    word(&mut v, 1);
    let mut dlt = [0; 20];
    dlt[..4].copy_from_slice(&65536u32.to_le_bytes());
    dlt[4..8].copy_from_slice(&0xc0000u32.to_le_bytes());
    dlt[10..14].copy_from_slice(&0xc0000u32.to_le_bytes());
    v.extend(dlt);
    word(&mut v, 0);
    word(&mut v, 1);
    v.extend([0, 0, 0, 0x1d]);
    word(&mut v, 65536);
    let mut metadata = vec![0; 65536];
    metadata[1] = 0x1d;
    for i in 1..7424 {
        metadata[2 + (i - 1) * 4 + 3] = 0x1d;
    }
    metadata[29694..29698].copy_from_slice(&35838i32.to_le_bytes());
    v.extend(metadata);
    word(&mut v, 0);
    assert!(parse_program(&v, ProgramMode::Global).is_ok());
    assert!(parse_program(&v, ProgramMode::Object).is_ok());
    assert_eq!(
        parse_program(&v, ProgramMode::Auto).unwrap_err().kind,
        ErrorKind::Ambiguous
    );
}

#[test]
fn rejects_dlt_pool_bounds_and_preserves_function_records() {
    let mut v = Vec::new();
    for n in [0x10001, 0, 0, 0, 0, 1, 2] {
        word(&mut v, n);
    }
    v.extend(b"f\0");
    for n in [0, 0, 559, 1] {
        short(&mut v, n);
    }
    word(&mut v, 0);
    word(&mut v, 1);
    let dlt_start = v.len();
    let mut raw = [0; 20];
    raw[4..8].copy_from_slice(&0x8000000u32.to_le_bytes());
    v.extend(raw);
    let mut raw = [0; 64];
    raw[13] = 4;
    raw[14] = 1;
    raw[15] = 1;
    v.extend(raw);
    string(&mut v, b"f");
    short(&mut v, 1);
    let mut raw = [0; 20];
    raw[5] = 4;
    v.extend(raw);
    string(&mut v, b"arg");
    short(&mut v, 0);
    for _ in 0..2 {
        word(&mut v, 0);
        word(&mut v, 1);
        v.extend([0, 0, 0, 0x1d]);
        word(&mut v, 0);
        word(&mut v, 0);
    }
    let p = parse_program(&v, ProgramMode::Global).unwrap();
    assert_eq!(p.functions.len(), 1);
    assert_eq!(p.functions[0].arguments[0].r#type, 4);
    assert_eq!(p.functions[0].raw_hex.len(), 128);
    assert_eq!(p.functions[0].index, Some(0));
    v[dlt_start..dlt_start + 4].copy_from_slice(&99u32.to_le_bytes());
    assert!(
        parse_program(&v, ProgramMode::Global)
            .unwrap_err()
            .message
            .contains("name offset")
    );
}
#[test]
fn rejects_negative_and_excessive_counts() {
    for count in [-1i32, 1_000_001] {
        let mut bytes = program(false, &finish(), &[], &[]);
        bytes[12..16].copy_from_slice(&count.to_le_bytes());
        assert_eq!(
            parse_program(&bytes, ProgramMode::Global).unwrap_err().kind,
            ErrorKind::Limit
        );
    }
}
#[test]
fn rejects_missing_finish_and_invalid_opcodes() {
    for opcode in [0, 86, 87, 240, 255] {
        let bytes = program(false, &[(0, 0, opcode), (0, 0, 0x1d)], &[], &[]);
        assert_eq!(
            parse_program(&bytes, ProgramMode::Global).unwrap_err().kind,
            ErrorKind::UnsupportedFormat
        );
    }
    let bytes = program(false, &[(0, 0, 1)], &[], &[]);
    assert!(
        parse_program(&bytes, ProgramMode::Global)
            .unwrap_err()
            .message
            .contains("finish")
    );
}
#[test]
fn retains_unused_legal_opcode_and_signed_operands() {
    let bytes = program(
        false,
        &[(0, 0, 9), (-32768, -128, 0x3b), (0, 0, 0x1d)],
        &[0; 8],
        &[],
    );
    let p = parse_program(&bytes, ProgramMode::Global).unwrap();
    assert_eq!(p.init.nodes[1], (-32768, -128, 0x3b));
}
#[test]
fn rejects_invalid_metadata_offsets() {
    for (a, b) in [(-1, 0), (0, -1), (1, 0)] {
        let bytes = program(false, &[(a, b, 0x3c), (0, 0, 0x1d)], &[0; 4], &[]);
        assert_eq!(
            parse_program(&bytes, ProgramMode::Global).unwrap_err().kind,
            ErrorKind::Bounds
        );
    }
}
#[test]
fn keeps_non_utf8_and_embedded_nul_string_bytes() {
    let raw = b"\xff\x00abc";
    let bytes = program(false, &[(0, 0, 64), (0, 0, 0x1d)], &[], &[raw]);
    let p = parse_program(&bytes, ProgramMode::Global).unwrap();
    assert_eq!(p.strings[0].as_ref().unwrap().bytes, raw);
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(json["strings"][0]["bytes_hex"], "ff00616263");
}
#[test]
fn rejects_invalid_string_and_efun_indices() {
    for nodes in [
        [(-1, 0, 64), (0, 0, 0x1d)],
        [(0, 0, 64), (0, 0, 0x1d)],
        [(559, 0, 6), (0, 0, 0x1d)],
        [(-1, 0, 6), (0, 0, 0x1d)],
    ] {
        let bytes = program(false, &nodes, &[], &[]);
        assert_eq!(
            parse_program(&bytes, ProgramMode::Global).unwrap_err().kind,
            ErrorKind::Bounds
        );
    }
}
#[test]
fn rejects_runtime_count_mismatch() {
    let mut bytes = program(false, &finish(), &[], &[]);
    let end = bytes.len();
    bytes[end - 4..].copy_from_slice(&2i32.to_le_bytes());
    assert!(
        parse_program(&bytes, ProgramMode::Global)
            .unwrap_err()
            .message
            .contains("runtime count")
    );
}
#[test]
fn explicit_legacy_profile_and_known_artifacts_are_unsupported() {
    let bytes = program(false, &finish(), &[], &[]);
    assert_eq!(
        parse_program_with_profile(&bytes, ProgramMode::Global, AbiProfile::LegacyAuth389)
            .unwrap_err()
            .kind,
        ErrorKind::UnsupportedFormat
    );
    for name in ["auth_srv.o", "pwd.o"] {
        let bytes = fs::read(format!("{}/1/service/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        assert_eq!(
            parse_program(&bytes, ProgramMode::Auto).unwrap_err().kind,
            ErrorKind::UnsupportedFormat
        );
    }
}
#[test]
fn lzss_literals_and_overlapping_backreference() {
    assert_eq!(lzss::decompress(&[7, b'A', b'B', b'C'], 3).unwrap(), b"ABC");
    assert_eq!(
        lzss::decompress(&[1, b'A', 0xee, 0xf2], 6).unwrap(),
        b"AAAAAA"
    );
    assert_eq!(lzss::decompress(&[0, 0, 0], 3).unwrap(), b"   ");
}
#[test]
fn lzss_wraps_the_ring() {
    let mut encoded = Vec::new();
    for start in (0..4096).step_by(8) {
        encoded.push(0xff);
        for i in start..start + 8 {
            encoded.push((i % 251) as u8);
        }
    }
    encoded.extend([0, 0xfe, 0xf1]); // backreference across physical ring end
    let decoded = lzss::decompress(&encoded, 4100).unwrap();
    let expected: Vec<_> = (0..4096).map(|i| (i % 251) as u8).collect();
    assert_eq!(&decoded[..4096], expected);
    // Source 0xffe wraps through 0xfff to 0, containing literal indices 16..19.
    assert_eq!(&decoded[4096..], &[16, 17, 18, 19]);
}
#[test]
fn lzss_rejects_truncation_and_length_mismatch() {
    for (payload, size) in [
        (&[1][..], 1),
        (&[0, 0][..], 3),
        (&[1, b'A'][..], 0),
        (&[][..], 5),
    ] {
        assert_eq!(
            lzss::decompress(payload, size).unwrap_err().kind,
            ErrorKind::Compression
        );
    }
}
#[test]
fn pak_raw_length_case_insensitive_lookup_and_compressed_data() {
    let bytes = pak(b"ABCD", 0x80000000, 1, 4, b"Test.bin");
    let archive = PakArchive::parse(&bytes).unwrap();
    assert_eq!(archive.read_member("/test.BIN").unwrap(), b"ABCD");
    let bytes = pak(&[7, b'A', b'B', b'C'], 0, 4, 3, b"a");
    assert_eq!(
        PakArchive::parse(&bytes).unwrap().read_member("a").unwrap(),
        b"ABC"
    );
}
#[test]
fn pak_rejects_truncated_signature_bounds_and_names() {
    let valid = pak(b"abc", 0x80000000, 3, 3, b"a");
    for end in 0..80 {
        assert!(PakArchive::parse(&valid[..end]).is_err());
    }
    for name in [b"".as_slice(), b"..", b"a/b", b"a\\b"] {
        assert!(PakArchive::parse(&pak(b"", 0, 0, 0, name)).is_err());
    }
    let mut bytes = valid.clone();
    bytes[0] = 0;
    assert!(PakArchive::parse(&bytes).is_err());
    let mut bytes = valid;
    bytes[28..32].copy_from_slice(&99u32.to_le_bytes());
    assert!(PakArchive::parse(&bytes).unwrap().read_member("a").is_err());
}
#[test]
fn pak_rejects_directory_cycles_and_directory_reads() {
    let mut bytes = pak(b"", 1, 64, 0, b"dir");
    bytes[20..24].copy_from_slice(&16u32.to_le_bytes());
    assert!(
        PakArchive::parse(&bytes)
            .unwrap_err()
            .message
            .contains("cyclic")
    );
    let bytes = pak(b"", 1, 0, 0, b"empty");
    let archive = PakArchive::parse(&bytes).unwrap();
    assert!(
        archive
            .read_member("empty")
            .unwrap_err()
            .message
            .contains("directory")
    );
}
#[test]
fn pak_case_collisions_are_ambiguous() {
    let first = pak(b"", 0x80000000, 0, 0, b"A");
    let mut bytes = first[..80].to_vec();
    bytes[4..8].copy_from_slice(&128u32.to_le_bytes());
    let mut second = bytes[16..80].to_vec();
    second[20] = b'a';
    bytes.extend(second);
    assert_eq!(
        PakArchive::parse(&bytes)
            .unwrap()
            .read_member("a")
            .unwrap_err()
            .kind,
        ErrorKind::Ambiguous
    );
}
#[test]
fn inheritance_is_package_local_and_checks_counts() {
    let parent = ProgramDescriptor {
        source: "aaa/lib.pak/feature/base.o".into(),
        counts: [1, 2, 3],
        inherits: vec![],
    };
    let child = ProgramDescriptor {
        source: "aaa/lib.pak/child.o".into(),
        counts: [1, 0, 0],
        inherits: vec![Inherit {
            name: Some(ByteString::new(b"/feature/base.c")),
            offsets: [0, 0, 0, 1, 2, 3],
        }],
    };
    let report = validate_inheritance(&[child.clone(), parent.clone()]);
    assert_eq!(report.matches, 1);
    let mut wrong = parent.clone();
    wrong.counts[1] = 9;
    assert_eq!(
        validate_inheritance(&[child.clone(), wrong]).checks[0].status,
        "count-mismatch"
    );
    let mut other_package = parent.clone();
    other_package.source = "gs/lib.pak/feature/base.o".into();
    assert_eq!(
        validate_inheritance(&[child.clone(), other_package]).checks[0].status,
        "missing"
    );
    assert_eq!(
        validate_inheritance(&[child, parent.clone(), parent]).checks[0].status,
        "ambiguous"
    );
}
#[test]
fn cli_help_usage_missing_input_and_known_error() {
    let bin = env!("CARGO_BIN_EXE_rml32-inspect");
    assert!(
        Command::new(bin)
            .arg("--help")
            .output()
            .unwrap()
            .status
            .success()
    );
    assert_eq!(
        Command::new(bin)
            .args(["program", "x", "--mode", "invalid"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
    let missing = Command::new(bin)
        .args(["pak", "does-not-exist.pak"])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(1));
    let v: serde_json::Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert_eq!(v["data"]["kind"], "io");
    let output = Command::new(bin)
        .args(["pak", "dba/pack_data/etc.pak"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let lines: Vec<serde_json::Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(lines.last().unwrap()["data"]["files"], 10);
    assert_eq!(lines.last().unwrap()["data"]["decoded_files"], 9);
    assert_eq!(lines.last().unwrap()["data"]["errors"], 1);
}
#[test]
fn uses_sha2_known_digest_and_serde_json_escaping() {
    assert_eq!(
        sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let raw = ByteString::new(b"\xff\n\"\\");
    let output = serde_json::to_vec(&raw).unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(v["bytes_hex"], "ff0a225c");
}

#[test]
fn corpus_reports_changed_input_and_continues_after_missing_packages() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!("test-corpus-{}", std::process::id()));
    let input = root.join("aaa/pack_data/etc.pak");
    fs::create_dir_all(input.parent().unwrap()).unwrap();
    fs::write(&input, b"changed baseline").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rml32-inspect"))
        .args(["corpus", "--root"])
        .arg(&root)
        .output()
        .unwrap();
    fs::remove_file(input).unwrap();
    fs::remove_dir(root.join("aaa/pack_data")).unwrap();
    fs::remove_dir(root.join("aaa")).unwrap();
    fs::remove_dir(&root).unwrap();
    assert_eq!(output.status.code(), Some(1));
    let lines: Vec<serde_json::Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(lines[0]["data"]["kind"], "integrity");
    assert_eq!(lines.last().unwrap()["kind"], "summary");
    assert_eq!(lines.last().unwrap()["data"]["errors"], 12);
}
