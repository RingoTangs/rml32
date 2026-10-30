use rml32::startup_probe::{
    StartupProbe, StartupProbeLimits, StartupProbeOutcome, StartupService, StartupSlice,
};
use std::{
    io::{self, Write},
    path::PathBuf,
};
const HELP: &str = "rml32-startup-probe --service aaa|ccs|csa|dba|gs [--root DIRECTORY] [--slice allocate|log-dir]\nExecutes Global init PC 0..25 (default), or 0..28 with memory-only mkdir. Never starts a service.\n";
fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut service = None;
    let mut root = None;
    let mut slice = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(0);
            }
            "--service" if service.is_none() => {
                service = args.next().as_deref().and_then(StartupService::parse);
                if service.is_none() {
                    eprint!("{HELP}");
                    return Ok(2);
                }
            }
            "--root" if root.is_none() => {
                root = args.next().map(PathBuf::from);
                if root.is_none() {
                    eprint!("{HELP}");
                    return Ok(2);
                }
            }
            "--slice" if slice.is_none() => {
                slice = args.next().as_deref().and_then(StartupSlice::parse);
                if slice.is_none() {
                    eprint!("{HELP}");
                    return Ok(2);
                }
            }
            _ => {
                eprint!("{HELP}");
                return Ok(2);
            }
        }
    }
    let Some(service) = service else {
        eprint!("{HELP}");
        return Ok(2);
    };
    let slice = slice.unwrap_or(StartupSlice::AllocationPrefix);
    let mut probe = StartupProbe::load_with_slice(
        &root.unwrap_or_else(|| PathBuf::from(".")),
        service,
        StartupProbeLimits::isolated(),
        slice,
    )?;
    if probe.run(slice.end_pc())? != StartupProbeOutcome::PrefixReached {
        return Err("startup prefix did not reach its boundary".into());
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut out, &probe.report()?)?;
    writeln!(out)?;
    out.flush()?;
    Ok(0)
}
fn main() {
    std::process::exit(match run() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    });
}
