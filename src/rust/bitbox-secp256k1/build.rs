// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

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
