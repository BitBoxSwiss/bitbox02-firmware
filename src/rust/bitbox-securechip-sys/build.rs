// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Output};

const ALLOWLIST_TYPES: &[&str] = &[
    "securechip_error_t",
    "securechip_interface_functions_t",
    "securechip_model_t",
    "securechip_password_stretch_algo_t",
];

const ALLOWLIST_FNS: &[&str] = &[
    "atecc_attestation_sign",
    "atecc_gen_attestation_key",
    "atecc_init_new_password",
    "atecc_kdf",
    "atecc_model",
    "atecc_monotonic_increments_remaining",
    "atecc_random",
    "atecc_reset_keys",
    "atecc_setup",
    "atecc_stretch_password",
    "atecc_u2f_counter_inc",
    "atecc_u2f_counter_set",
    "optiga_attestation_sign",
    "optiga_gen_attestation_key",
    "optiga_init_new_password",
    "optiga_kdf_external",
    "optiga_monotonic_increments_remaining",
    "optiga_random",
    "optiga_reset_keys",
    "optiga_setup",
    "optiga_stretch_password",
    "optiga_u2f_counter_inc",
    "optiga_u2f_counter_set",
];

const RUSTIFIED_ENUMS: &[&str] = &[
    "securechip_password_stretch_algo_t",
    "securechip_error_t",
    "securechip_model_t",
];

type BuildResult<T> = Result<T, String>;

pub fn main() -> BuildResult<()> {
    ensure_command_exists("bindgen")?;

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");
    let src_dir = repo_root.join("src");
    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found".into());
    }

    emit_rerun_if_changed(&wrapper);
    emit_rerun_if_changed(src_dir.join("atecc"));
    emit_rerun_if_changed(src_dir.join("optiga"));
    emit_rerun_if_changed(src_dir.join("platform"));
    emit_rerun_if_changed(src_dir.join("securechip"));
    emit_rerun_if_changed(src_dir.join("compiler_util.h"));

    let target = env::var("TARGET").expect("TARGET not set");
    let cross_compiling = target == "thumbv7em-none-eabi";

    let mut extra_flags = if cross_compiling {
        vec![
            // Generate bindings for the firmware target ABI, not the host ABI.
            "--target=thumbv7em-none-eabi",
            // The firmware C code is compiled with arm-none-eabi-gcc, which uses
            // -fshort-enums by default. Bindgen must match those enum sizes.
            "-fshort-enums",
        ]
    } else {
        vec![]
    };

    if let Ok(rustflags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        for flag in rustflags.split('\x1f') {
            if flag == "-Dwarnings" {
                extra_flags.push("-Werror");
            }
        }
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("bindings.rs");
    let out_path = out_path.into_os_string().into_string().unwrap();

    let mut definitions = vec![
        // Expose the U2F counter declarations guarded by APP_U2F in atecc.h/optiga.h.
        "-DAPP_U2F=1",
    ];
    definitions.extend(&extra_flags);

    run_command(
        Command::new("bindgen")
            .args(["--output", &out_path])
            .arg("--use-core")
            .arg("--with-derive-default")
            .args(
                ALLOWLIST_FNS
                    .iter()
                    .flat_map(|item| ["--allowlist-function", item]),
            )
            .args(
                ALLOWLIST_TYPES
                    .iter()
                    .flat_map(|item| ["--allowlist-type", item]),
            )
            .args(
                RUSTIFIED_ENUMS
                    .iter()
                    .flat_map(|item| ["--rustified-enum", item]),
            )
            .arg(&wrapper)
            .arg("--")
            .args(&definitions)
            .arg(format!("-I{}", src_dir.display())),
        "run bindgen",
    )?;

    Ok(())
}

fn emit_rerun_if_changed(path: impl AsRef<std::path::Path>) {
    println!("cargo::rerun-if-changed={}", path.as_ref().display());
}

fn ensure_command_exists(command: &str) -> BuildResult<()> {
    match Command::new(command).arg("--version").output() {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            Err(format!("`{command}` was not found! Check your PATH!"))
        }
        Err(err) => Err(format!("failed to run `{command} --version`: {err}")),
    }
}

fn run_command(command: &mut Command, context: &str) -> BuildResult<Output> {
    let output = command
        .output()
        .map_err(|err| format!("failed to {context}: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "{context} failed\nstdout:\n{}\n\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output)
}
