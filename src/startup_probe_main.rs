use rml32::startup_probe::{StartupProbe, StartupProbeLimits, StartupProbeOutcome, StartupService};
use std::{
    io::{self, Write},
    path::PathBuf,
};
const HELP: &str = "rml32-startup-probe --service aaa|ccs|csa|dba|gs [--root DIRECTORY]\nExecutes only Global init PC 0..25 in an isolated Rust VM. Never starts a service.\n";
fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut service = None;
    let mut root = None;
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
    let mut probe = StartupProbe::load(
        &root.unwrap_or_else(|| PathBuf::from(".")),
        service,
        StartupProbeLimits::isolated(),
    )?;
    if probe.run(25)? != StartupProbeOutcome::PrefixReached {
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
