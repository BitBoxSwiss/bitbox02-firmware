// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("../../..")
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = repo_root(&manifest_dir);

    for path in [
        "messages",
        "scripts/generate-protobuf-rust.sh",
        "tools/prost-build-proto/src/main.rs",
    ] {
        println!("cargo:rerun-if-changed={}", repo_root.join(path).display());
    }

    let status = Command::new("bash")
        .arg(repo_root.join("scripts/generate-protobuf-rust.sh"))
        .status()
        .expect("failed to invoke protobuf generator script");
    assert!(status.success(), "protobuf generation failed");
}
