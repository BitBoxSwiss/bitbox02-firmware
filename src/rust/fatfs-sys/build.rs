// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

const SOURCES: &[&str] = &["source/ff.c", "source/ffunicode.c"];

fn main() {
    let dep_dir = PathBuf::from("depend/fatfs");
    let target = std::env::var("TARGET").expect("TARGET not set");
    let cross_compiling = target == "thumbv7em-none-eabi";

    println!(
        "cargo::rerun-if-changed={}",
        dep_dir.join("source").display()
    );

    let mut build = cc::Build::new();
    build
        .files(SOURCES.iter().map(|s| dep_dir.join(s)))
        .include(dep_dir.join("source"))
        // Suppress all warnings in this dependency, we don't have control over them.
        .flag_if_supported("-w");

    if !cross_compiling {
        // Provide the disk_* symbols used by ff.c in host/unit-test builds.
        build.file("../../../test/hardware-fakes/src/fake_diskio.c");
        println!("cargo::rerun-if-changed=../../../test/hardware-fakes/src/fake_diskio.c");
    }

    build.compile("fatfs");
}
