//! JSON Lines inspector. All artifact access is read-only.
use rml32::{
    ErrorKind, PakArchive, ParseError, ProgramDescriptor, ProgramMode, parse_program, sha256,
    validate_inheritance,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env, fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

const HELP: &str = "rml32-inspect (read-only; never executes code)\n\
  pak <package.pak>\n\
  program <file.o|package.pak/member.o> [--mode auto|object|global] [--full]\n\
  corpus [--root DIRECTORY]\n\
stdout: JSON Lines. stderr: diagnostics. Exit 0: success; 1: input/check errors; 2: usage.\n\
corpus verifies the fixed package manifest and continues after member errors.\n";

fn error(source: &str, message: impl Into<String>, kind: ErrorKind) -> ParseError {
    ParseError::new(0, kind, message).with_source(source)
}
fn read(path: &Path) -> Result<Vec<u8>, ParseError> {
    fs::read(path).map_err(|e| error(&path.to_string_lossy(), e.to_string(), ErrorKind::Io))
}
fn emit(out: &mut impl Write, kind: &str, data: impl Serialize) -> io::Result<()> {
    serde_json::to_writer(&mut *out, &json!({"kind":kind,"data":data}))
        .map_err(io::Error::other)?;
    out.write_all(b"\n")
}
fn diagnostic(out: &mut impl Write, e: &ParseError) -> io::Result<()> {
    eprintln!("{e}");
    emit(out, "error", e)
}
#[derive(Default, Serialize)]
struct Summary {
    packages: usize,
    members: usize,
    files: usize,
    decoded_files: usize,
    programs: usize,
    functions: usize,
    instructions: usize,
    inheritance_matches: usize,
    errors: usize,
}
#[derive(Deserialize)]
struct Input {
    path: String,
    sha256: String,
}

fn scan_package(
    out: &mut impl Write,
    label: &str,
    bytes: &[u8],
    parse: bool,
    summary: &mut Summary,
    descriptors: &mut Vec<ProgramDescriptor>,
) -> io::Result<()> {
    let archive = match PakArchive::parse(bytes) {
        Ok(p) => p,
        Err(e) => {
            summary.errors += 1;
            return diagnostic(out, &e.with_source(label));
        }
    };
    summary.packages += 1;
    emit(
        out,
        "package",
        json!({"package":label,"sha256":sha256(bytes),"header":archive.header}),
    )?;
    for member in &archive.members {
        summary.members += 1;
        let mut entry = serde_json::to_value(member).map_err(io::Error::other)?;
        entry["package"] = label.into();
        if member.directory {
            emit(out, "member", entry)?;
            continue;
        }
        summary.files += 1;
        let source = format!("{label}/{}", String::from_utf8_lossy(&member.path.bytes));
        let data = match archive.read_entry(member) {
            Ok(data) => data,
            Err(e) => {
                let e = e.with_source(&source);
                entry["decode_error"] = json!(&e);
                emit(out, "member", entry)?;
                summary.errors += 1;
                diagnostic(out, &e)?;
                continue;
            }
        };
        summary.decoded_files += 1;
        entry["sha256"] = sha256(&data).into();
        let is_program = data.starts_with(&0x10001u32.to_le_bytes());
        entry["format"] = if is_program { "vm-bytecode" } else { "other" }.into();
        emit(out, "member", entry)?;
        if parse && is_program {
            match parse_program(&data, ProgramMode::Auto) {
                Ok(image) => {
                    summary.programs += 1;
                    summary.functions += image.functions.len() + image.hidden.len();
                    summary.instructions += image.init.nodes.len()
                        + image
                            .functions
                            .iter()
                            .chain(&image.hidden)
                            .map(|f| f.code.nodes.len())
                            .sum::<usize>();
                    descriptors.push(ProgramDescriptor::from_image(source.clone(), &image));
                    let mut image = serde_json::to_value(image).map_err(io::Error::other)?;
                    image["source"] = source.into();
                    emit(out, "program", image)?;
                }
                Err(e) => {
                    summary.errors += 1;
                    diagnostic(out, &e.with_source(&source))?;
                }
            }
        }
    }
    Ok(())
}
fn corpus(out: &mut impl Write, root: &Path) -> io::Result<i32> {
    // Embedded canonical evidence fixes the input set, independent of current working directory.
    let manifest: Vec<Input> = serde_json::from_str(include_str!(
        "../docs/compatibility-evidence/input-manifest.json"
    ))
    .map_err(io::Error::other)?;
    let mut inputs: Vec<_> = manifest
        .into_iter()
        .filter(|i| i.path.ends_with(".pak"))
        .collect();
    inputs.sort_by(|a, b| a.path.cmp(&b.path));
    let mut summary = Summary::default();
    let mut descriptors = Vec::new();
    for input in inputs {
        let bytes = match read(&root.join(&input.path)) {
            Ok(b) => b,
            Err(e) => {
                summary.errors += 1;
                diagnostic(out, &e.with_source(&input.path))?;
                continue;
            }
        };
        if sha256(&bytes) != input.sha256 {
            summary.errors += 1;
            diagnostic(
                out,
                &error(
                    &input.path,
                    "input differs from fixed baseline",
                    ErrorKind::Integrity,
                ),
            )?;
            continue;
        }
        eprintln!("Inspecting {}", input.path);
        scan_package(
            out,
            &input.path,
            &bytes,
            true,
            &mut summary,
            &mut descriptors,
        )?;
    }
    let inheritance = validate_inheritance(&descriptors);
    summary.inheritance_matches = inheritance.matches;
    summary.errors += inheritance.errors;
    for check in inheritance.checks {
        emit(out, "inheritance", check)?;
    }
    emit(out, "summary", &summary)?;
    Ok(i32::from(summary.errors > 0))
}
fn program(out: &mut impl Write, path: &str, mode: ProgramMode, full: bool) -> io::Result<i32> {
    let bytes = if let Some((package, member)) = path.split_once(".pak/") {
        let filename = format!("{package}.pak");
        read(Path::new(&filename)).and_then(|bytes| {
            PakArchive::parse(&bytes)
                .and_then(|p| p.read_member(member))
                .map_err(|e| e.with_source(path))
        })
    } else {
        read(Path::new(path))
    };
    let image = match bytes.and_then(|b| parse_program(&b, mode).map_err(|e| e.with_source(path))) {
        Ok(image) => image,
        Err(e) => {
            diagnostic(out, &e)?;
            return Ok(1);
        }
    };
    let mut result = if full {
        serde_json::to_value(&image).map_err(io::Error::other)?
    } else {
        json!({"mode":image.mode,"sha256":image.sha256,"bytes_consumed":image.bytes_consumed,
        "function_base":image.function_base,"efuns_count":image.efuns_count,"functions":image.functions.len(),
        "hidden":image.hidden.len(),"strings":image.strings.len(),"inherits":image.inherits.len(),
        "init_nodes":image.init.nodes.len()})
    };
    result["source"] = path.into();
    emit(out, "program", result)?;
    Ok(0)
}
fn usage() -> i32 {
    eprintln!("{HELP}");
    2
}
fn run(out: &mut impl Write, args: &[String]) -> io::Result<i32> {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => {
            out.write_all(HELP.as_bytes())?;
            Ok(0)
        }
        Some("pak") if args.len() == 2 => {
            let bytes = match read(Path::new(&args[1])) {
                Ok(b) => b,
                Err(e) => {
                    diagnostic(out, &e)?;
                    return Ok(1);
                }
            };
            let mut summary = Summary::default();
            scan_package(out, &args[1], &bytes, false, &mut summary, &mut Vec::new())?;
            emit(out, "summary", &summary)?;
            Ok(i32::from(summary.errors > 0))
        }
        Some("program") if args.len() >= 2 => {
            let mut mode = ProgramMode::Auto;
            let mut full = false;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--full" => full = true,
                    "--mode" if i + 1 < args.len() => {
                        i += 1;
                        mode = match args[i].as_str() {
                            "auto" => ProgramMode::Auto,
                            "object" => ProgramMode::Object,
                            "global" => ProgramMode::Global,
                            _ => return Ok(usage()),
                        };
                    }
                    _ => return Ok(usage()),
                }
                i += 1;
            }
            program(out, &args[1], mode, full)
        }
        Some("corpus") => {
            let root = match args.len() {
                1 => PathBuf::from("."),
                3 if args[1] == "--root" => PathBuf::from(&args[2]),
                _ => return Ok(usage()),
            };
            corpus(out, &root)
        }
        _ => Ok(usage()),
    }
}
fn main() {
    let mut out = BufWriter::new(io::stdout().lock());
    let args: Vec<_> = env::args().skip(1).collect();
    let result = run(&mut out, &args).and_then(|status| out.flush().map(|()| status));
    let status = match result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("output error: {e}");
            1
        }
    };
    std::process::exit(status);
}
