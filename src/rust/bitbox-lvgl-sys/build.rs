// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

fn run_bindgen(
    wrapper: &PathBuf,
    output: &PathBuf,
    lvgl_dir: &PathBuf,
    lv_conf: &PathBuf,
) -> Result<(), &'static str> {
    let lv_conf_arg = format!("-DLV_CONF_PATH=\"{}\"", lv_conf.display());

    let res = Command::new("bindgen")
        .arg("--output")
        .arg(output)
        .arg("--use-core")
        .arg("--with-derive-default")
        .arg("--rustified-enum")
        .arg(".*")
        .arg(wrapper)
        .arg("--")
        .arg(format!("-I{}", lvgl_dir.display()))
        .arg("-DLV_CONF_INCLUDE_SIMPLE")
        .arg(lv_conf_arg)
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

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");
    run_bindgen(&wrapper, &out_path, &lvgl_dir, &lv_conf)
}
