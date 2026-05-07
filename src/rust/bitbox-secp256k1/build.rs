// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;
use std::process::Command;

const ARM_NONE_EABI_GCC: &str = "arm-none-eabi-gcc";

fn arm_none_eabi_sysroot() -> String {
    let output = Command::new(ARM_NONE_EABI_GCC)
        .arg("--print-sysroot")
        .output()
        .expect("get arm-none-eabi-gcc sysroot");
    assert!(
        output.status.success(),
        "arm-none-eabi-gcc --print-sysroot failed"
    );
    let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    assert!(
        !sysroot.is_empty(),
        "arm-none-eabi-gcc --print-sysroot returned an empty sysroot"
    );
    sysroot
}

fn main() {
    let secp_dir = PathBuf::from("depend/secp256k1-zkp");
    let callbacks = PathBuf::from("src/default_callbacks.c");

    println!(
        "cargo::rerun-if-changed={}",
        secp_dir.join("include").display()
    );
    println!("cargo::rerun-if-changed={}", secp_dir.join("src").display());
    println!("cargo::rerun-if-changed={}", callbacks.display());

    let mut build = cc::Build::new();
    let target = env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        build.flag(format!("--sysroot={}", arm_none_eabi_sysroot()));
    }
    build
        .file(secp_dir.join("src/secp256k1.c"))
        .file(secp_dir.join("src/precomputed_ecmult.c"))
        .file(secp_dir.join("src/precomputed_ecmult_gen.c"))
        .file(&callbacks)
        .include(secp_dir.join("include"))
        // Suppress all warnings in this dependency, we don't have control over them.
        .flag_if_supported("-w")
        .define("ECMULT_WINDOW_SIZE", Some("2"))
        .define("ECMULT_GEN_PREC_BITS", Some("2"))
        .define("USE_EXTERNAL_DEFAULT_CALLBACKS", Some("1"))
        .define("ENABLE_MODULE_RECOVERY", Some("1")) // needed only in Rust unit tests.
        .define("ENABLE_MODULE_EXTRAKEYS", Some("1"))
        .define("ENABLE_MODULE_SCHNORRSIG", Some("1"))
        .define("ENABLE_MODULE_ECDSA_ADAPTOR", Some("1"))
        .define("ENABLE_MODULE_ECDSA_S2C", Some("1"));

    build.compile("secp256k1");
}
