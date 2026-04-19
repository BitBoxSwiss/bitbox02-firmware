// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Output};

const ALLOWLIST_TYPES: &[&str] = &[
    "atecc_slot_t",
    "bool_t",
    "optiga_crypt_t",
    "optiga_hmac_type_t",
    "optiga_key_id_t",
    "optiga_key_usage_t",
    "optiga_lib_status_t",
    "optiga_rng_type_t",
    "optiga_symmetric_encryption_mode_t",
    "optiga_symmetric_key_type_t",
    "optiga_util_t",
    "securechip_error_t",
    "securechip_model_t",
    "securechip_password_stretch_algo_t",
];

const ALLOWLIST_FNS: &[&str] = &[
    "atecc_auth_compute_response",
    "atecc_cmd_read_block_response",
    "atecc_cmd_read_counter_response",
    "atecc_cmd_read_info_response",
    "atecc_cmd_read_kdf_response",
    "atecc_cmd_read_random_response",
    "atecc_cmd_read_sign_response",
    "atecc_cmd_start_checkmac",
    "atecc_cmd_start_counter_read",
    "atecc_cmd_start_derivekey_rollkey",
    "atecc_cmd_start_gendig_encryption_key",
    "atecc_cmd_start_info_revision",
    "atecc_cmd_start_kdf",
    "atecc_cmd_start_nonce_load_msgdigest",
    "atecc_cmd_start_nonce_rand",
    "atecc_cmd_start_random",
    "atecc_cmd_start_read_block",
    "atecc_cmd_start_sign_attestation",
    "atecc_cmd_start_write_encrypted_block",
    "atecc_gen_attestation_key",
    "atecc_io_apply_gendig",
    "atecc_io_decrypt_block",
    "atecc_io_prepare_encrypted_write",
    "atecc_io_prepare_tempkey",
    "atecc_kdf_decrypt",
    "atecc_ops_get_status",
    "atecc_ops_get_poll_delay_ms",
    "atecc_ops_poll",
    "atecc_setup",
    "optiga_crypt_clear_auto_state",
    "optiga_crypt_ecdsa_sign",
    "optiga_crypt_generate_auth_code",
    "optiga_crypt_hmac",
    "optiga_crypt_hmac_verify",
    "optiga_crypt_instance",
    "optiga_crypt_random",
    "optiga_crypt_symmetric_generate_key",
    "optiga_crypt_symmetric_encrypt",
    "optiga_gen_attestation_key",
    "optiga_ops_get_status",
    "optiga_ops_set_status_busy",
    "optiga_setup",
    "optiga_util_instance",
    "optiga_util_read_data",
    "optiga_util_write_data",
];

const ALLOWLIST_VARS: &[&str] = &[
    "ATECC_OPS_STATUS_BUSY",
    "ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE",
    "MONOTONIC_COUNTER_MAX_USE",
    "NONCE_NUMIN_SIZE",
    "OID_AES_SYMKEY",
    "OID_ARBITRARY_DATA",
    "OID_COUNTER",
    "OID_COUNTER_HMAC_WRITEPROTECTED",
    "OID_COUNTER_PASSWORD",
    "OID_HMAC",
    "OID_HMAC_WRITEPROTECTED",
    "OID_PASSWORD",
    "OID_PASSWORD_SECRET",
    "OPTIGA_CRYPT_SUCCESS",
    "OPTIGA_CRYPT_ERROR",
    "OPTIGA_CRYPT_ERROR_INVALID_INPUT",
    "OPTIGA_LIB_BUSY",
    "OPTIGA_LIB_SUCCESS",
    "OPTIGA_UTIL_ERROR",
    "OPTIGA_UTIL_ERROR_INVALID_INPUT",
    "OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT",
    "OPTIGA_UTIL_SUCCESS",
    "OPTIGA_UTIL_ERASE_AND_WRITE",
    "SMALL_MONOTONIC_COUNTER_MAX_USE",
];

const RUSTIFIED_ENUMS: &[&str] = &[
    "atecc_slot_t",
    "optiga_hmac_type",
    "optiga_hmac_type_t",
    "optiga_key_id",
    "optiga_key_id_t",
    "optiga_key_usage",
    "optiga_key_usage_t",
    "optiga_rng_type",
    "optiga_rng_type_t",
    "optiga_symmetric_encryption_mode",
    "optiga_symmetric_encryption_mode_t",
    "optiga_symmetric_key_type",
    "optiga_symmetric_key_type_t",
    "securechip_error_t",
    "securechip_model_t",
    "securechip_password_stretch_algo_t",
];

type BuildResult<T> = Result<T, String>;

pub fn main() -> BuildResult<()> {
    ensure_command_exists("bindgen")?;

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");
    let src_dir = repo_root.join("src");
    let external_dir = repo_root.join("external");
    let optiga_include_dir = external_dir.join("optiga-trust-m/include");
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
    emit_rerun_if_changed(external_dir.join("optiga_config.h"));
    emit_rerun_if_changed(external_dir.join("optiga-trust-m/config"));
    emit_rerun_if_changed(&optiga_include_dir);

    let target = env::var("TARGET").expect("TARGET not set");
    let cross_compiling = target == "thumbv7em-none-eabi";
    let arm_sysroot = env::var("CMAKE_SYSROOT").unwrap_or("/usr/local/arm-none-eabi".to_string());
    let arm_sysroot = format!("--sysroot={arm_sysroot}");

    let mut extra_flags = if cross_compiling {
        vec![
            // Generate bindings for the firmware target ABI, not the host ABI.
            "--target=thumbv7em-none-eabi",
            &arm_sysroot,
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
        "-DOPTIGA_LIB_EXTERNAL=\"optiga_config.h\"",
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
                ALLOWLIST_VARS
                    .iter()
                    .flat_map(|item| ["--allowlist-var", item]),
            )
            .args(
                RUSTIFIED_ENUMS
                    .iter()
                    .flat_map(|item| ["--rustified-enum", item]),
            )
            .arg(&wrapper)
            .arg("--")
            .args(&definitions)
            .arg(format!("-I{}", src_dir.display()))
            .arg(format!("-I{}", external_dir.display()))
            .arg(format!(
                "-I{}",
                external_dir.join("cryptoauthlib/lib").display()
            ))
            .arg(format!(
                "-I{}",
                external_dir.join("optiga-trust-m/config").display()
            ))
            .arg(format!("-I{}", optiga_include_dir.display()))
            .arg(format!(
                "-I{}",
                external_dir.join("optiga-trust-m/include/cmd").display()
            ))
            .arg(format!(
                "-I{}",
                external_dir.join("optiga-trust-m/include/common").display()
            ))
            .arg(format!(
                "-I{}",
                external_dir
                    .join("optiga-trust-m/include/ifx_i2c")
                    .display()
            ))
            .arg(format!(
                "-I{}",
                external_dir.join("optiga-trust-m/include/pal").display()
            ))
            .arg(format!(
                "-I{}",
                external_dir.join("optiga-trust-m/include/comms").display()
            ))
            .arg(format!(
                "-I{}",
                external_dir
                    .join("optiga-trust-m/external/mbedtls/include")
                    .display()
            )),
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
