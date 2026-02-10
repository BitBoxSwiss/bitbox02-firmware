// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_bindgen(wrapper: &Path, output: &Path, clang_args: &[String]) -> Result<(), &'static str> {
    let res = Command::new("bindgen")
        .arg("--output")
        .arg(output)
        .arg("--use-core")
        .arg("--with-derive-default")
        .arg("--rustified-enum")
        .arg(".*")
        .arg(wrapper)
        .arg("--")
        .args(clang_args)
        .output()
        .expect("failed to run bindgen");

    if !res.status.success() {
        println!(
            "bindgen-out:\n{}\n\nbindgen-err:\n{}",
            std::str::from_utf8(&res.stdout).unwrap_or("invalid utf8"),
            std::str::from_utf8(&res.stderr).unwrap_or("invalid utf8"),
        );
        return Err("bindgen failed");
    }
    Ok(())
}

fn build_clang_args(target: &str, lvgl_dir: &Path, lv_conf: &Path) -> Vec<String> {
    let mut args = vec![
        format!("-I{}", lvgl_dir.display()),
        "-DLV_CONF_INCLUDE_SIMPLE".to_owned(),
        format!("-DLV_CONF_PATH=\"{}\"", lv_conf.display()),
    ];
    if target == "thumbv7em-none-eabi" {
        args.extend([
            "--target=thumbv7em-none-eabi".to_owned(),
            "-mcpu=cortex-m4".to_owned(),
            "-mthumb".to_owned(),
            "-mfloat-abi=soft".to_owned(),
        ]);
        if let Ok(sysroot) = env::var("CMAKE_SYSROOT") {
            args.push(format!("--sysroot={sysroot}"));
        }
    }
    args
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rerun-if-env-changed=LV_CONF_PATH");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");

    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found");
    }

    let lvgl_dir = repo_root.join("external/lvgl");
    let lvgl_header = lvgl_dir.join("lvgl.h");
    if !lvgl_header.is_file() {
        return Err(
            "external/lvgl/lvgl.h not found. Is the external/lvgl submodule initialized and checked out?",
        );
    }
    println!("cargo::rerun-if-changed={}", lvgl_header.display());
    println!(
        "cargo::rerun-if-changed={}",
        lvgl_dir.join("src/lvgl.h").display()
    );

    let lv_conf = match env::var("LV_CONF_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => repo_root.join("src/lvgl/lv_conf.h"),
    };
    if !lv_conf.is_file() {
        return Err("lv_conf.h not found. Set LV_CONF_PATH or provide src/lvgl/lv_conf.h.");
    }
    println!("cargo::rerun-if-changed={}", lv_conf.display());

    if let Err(err) = Command::new("bindgen").arg("--version").output() {
        if err.kind() == ErrorKind::NotFound {
            return Err("`bindgen` executable was not found. Check your PATH.");
        }
        return Err("failed to execute `bindgen --version`");
    }

    let target = env::var("TARGET").expect("TARGET not set");
    let clang_args = build_clang_args(&target, &lvgl_dir, &lv_conf);
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");
    run_bindgen(&wrapper, &out_path, &clang_args)
}
