// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;
use std::process::Command;

const ARM_NONE_EABI_GCC: &str = "arm-none-eabi-gcc";

const SOURCES: &[&str] = &["source/ff.c", "source/ffunicode.c"];

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
    let dep_dir = PathBuf::from("depend/fatfs");

    println!(
        "cargo::rerun-if-changed={}",
        dep_dir.join("source").display()
    );

    let mut build = cc::Build::new();
    let target = env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        build.flag(format!("--sysroot={}", arm_none_eabi_sysroot()));
    }
    build
        .files(SOURCES.iter().map(|s| dep_dir.join(s)))
        .include(dep_dir.join("source"))
        // Suppress all warnings in this dependency, we don't have control over them.
        .flag_if_supported("-w");

    build.compile("fatfs");
}
