// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

const SOURCES: &[&str] = &["source/ff.c", "source/ffunicode.c"];

fn main() -> Result<(), String> {
    let dep_dir = PathBuf::from("depend/fatfs");
    let _target = env::var("TARGET").expect("TARGET not set");

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

    build.compile("fatfs");
    Ok(())
}
