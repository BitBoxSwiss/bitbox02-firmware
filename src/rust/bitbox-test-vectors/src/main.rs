// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsString;
use std::path::{Path, PathBuf};

enum Mode {
    Write(PathBuf),
    Check,
    Help,
}

fn canonical_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("testdata")
        .join(bitbox_test_vectors::btc_transaction::GENERATED_FILENAME)
}

fn parse_args() -> Result<Mode, String> {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [] => Ok(Mode::Write(canonical_path())),
        [arg] if arg == "--check" => Ok(Mode::Check),
        [arg] if arg == "--help" || arg == "-h" => Ok(Mode::Help),
        [output] if !output.to_string_lossy().starts_with('-') => {
            Ok(Mode::Write(PathBuf::from(output)))
        }
        _ => Err("usage: generate-btc-test-vectors [--check | OUTPUT]".into()),
    }
}

fn write(output: &Path, generated: &str) -> Result<(), String> {
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    std::fs::write(output, generated)
        .map_err(|err| format!("failed to write {}: {err}", output.display()))?;
    println!("Wrote {}", output.display());
    Ok(())
}

fn check(generated: &str) -> Result<(), String> {
    let canonical = canonical_path();
    let committed = std::fs::read_to_string(&canonical)
        .map_err(|err| format!("failed to read {}: {err}", canonical.display()))?;
    if committed != generated {
        return Err(format!(
            "{} is stale; run generate-btc-test-vectors",
            canonical.display()
        ));
    }
    println!("{} is current", canonical.display());
    Ok(())
}

fn run() -> Result<(), String> {
    let mode = parse_args()?;
    if matches!(mode, Mode::Help) {
        println!("usage: generate-btc-test-vectors [--check | OUTPUT]");
        return Ok(());
    }

    let generated = bitbox_test_vectors::btc_transaction::try_generate_json()?;
    match mode {
        Mode::Write(output) => write(&output, &generated),
        Mode::Check => check(&generated),
        Mode::Help => unreachable!(),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
