use std::env;
use std::path::{Path, PathBuf};

const BINDGEN_BASE_CLANG_ARGS: &[&str] = &[
    "-std=c99",
    "-DLFS_NO_DEBUG",
    "-DLFS_NO_WARN",
    "-DLFS_NO_ERROR",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let littlefs_path = if cfg!(feature = "unstable-littlefs-patched") {
        "littlefs-patched"
    } else {
        "littlefs"
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Patch lfs.h to remove the lfs_util import because clang fails to locate the
    // libraries for the custom target (especially string.h)
    // Compilation before that succeeds because it's using gcc,
    // which comes as a distribution with these utils.
    // Turns out lfs_utils is not used in lfs.h, and clang properly finds stdint.h and stdbool,
    // but not string.h
    let lfs_h =
        std::fs::read_to_string(format!("{littlefs_path}/lfs.h")).expect("Reading lfs.h succeeds");
    println!("cargo::rerun-if-changed={littlefs_path}/lfs.h");
    let out_lfs_h = out_path.join("lfs.h");
    std::fs::write(
        &out_lfs_h,
        lfs_h.replace(
            r##"#include "lfs_util.h""##,
            "#include <stdint.h>\n#include <stdbool.h>",
        ),
    )
    .expect("Failed to write lfs.h");

    let mut builder = cc::Build::new();
    builder
        .include(&out_path)
        .include(littlefs_path)
        .file(format!("{littlefs_path}/lfs.c"))
        .file(format!("{littlefs_path}/lfs_util.c"))
        .file("string.c");

    for flag in cc_flags() {
        builder.flag(flag);
    }

    builder.compile("lfs-sys");

    generate_bindings(
        &out_lfs_h,
        &out_path.join("bindings.rs"),
        &bindgen_clang_args(),
    )?;

    Ok(())
}

fn cc_flags() -> Vec<&'static str> {
    let mut flags = Vec::from(BINDGEN_BASE_CLANG_ARGS);

    if cfg!(feature = "software-intrinsics") {
        flags.push("-DLFS_NO_INTRINSICS");
    }

    if !cfg!(feature = "assertions") {
        flags.push("-DLFS_NO_ASSERT");
    }

    if cfg!(feature = "trace") {
        flags.push("-DLFS_YES_TRACE");
    }

    if !cfg!(feature = "malloc") {
        flags.push("-DLFS_NO_MALLOC");
    }

    if cfg!(feature = "multiversion") {
        flags.push("-DLFS_MULTIVERSION");
    }

    flags
}

fn bindgen_clang_args() -> Vec<&'static str> {
    let mut args = Vec::from(BINDGEN_BASE_CLANG_ARGS);

    if cfg!(feature = "multiversion") {
        args.push("-DLFS_MULTIVERSION");
    }

    args
}

#[cfg(feature = "bindgen")]
fn generate_bindings(
    header: &Path,
    output: &Path,
    clang_args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bindgen = bindgen::Builder::default()
        .header(header.to_string_lossy().into_owned())
        .derive_default(true)
        .use_core()
        .allowlist_item("lfs_.*")
        .allowlist_item("LFS_.*");

    for arg in clang_args {
        bindgen = bindgen.clang_arg(*arg);
    }

    bindgen
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output)
        .expect("Couldn't write bindings!");

    Ok(())
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(
    header: &Path,
    output: &Path,
    clang_args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("bindgen")
        .arg("--with-derive-default")
        .arg("--use-core")
        .arg("--allowlist-item")
        .arg("lfs_.*")
        .arg("--allowlist-item")
        .arg("LFS_.*")
        .arg("-o")
        .arg(output)
        .arg(header)
        .arg("--")
        .args(clang_args)
        .status()
        .map_err(|error| {
            std::io::Error::new(
                error.kind(),
                format!("failed to run bindgen executable from PATH: {error}"),
            )
        })?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("bindgen executable failed with status {status}"),
        )
        .into());
    }

    Ok(())
}
