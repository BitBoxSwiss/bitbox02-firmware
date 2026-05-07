// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let source = manifest_dir.join("bitbox03-common.ld");
    let target = out_dir.join("bitbox03-common.ld");

    println!("cargo::rerun-if-changed={}", source.display());
    std::fs::copy(source, target).expect("copy common linker script");
    println!("cargo::rustc-link-search={}", out_dir.display());
}
