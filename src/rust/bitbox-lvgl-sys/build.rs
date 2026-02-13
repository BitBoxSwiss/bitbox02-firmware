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

fn build_clang_args(lvgl_dir: &Path, lv_conf: &Path) -> Vec<String> {
    let mut args = vec![
        format!("-I{}", lvgl_dir.display()),
        "-DLV_CONF_INCLUDE_SIMPLE".to_owned(),
        format!("-DLV_CONF_PATH=\"{}\"", lv_conf.display()),
    ];

    if let Ok(sysroot) = env::var("CMAKE_SYSROOT") {
        args.push(format!("--sysroot={sysroot}"));
    } else if let Ok(output) = cc::Build::new()
        .get_compiler()
        .to_command()
        .arg("-print-sysroot")
        .output()
    {
        let sysroot = String::from_utf8(output.stdout).expect("invalid utf-8");
        let sysroot = sysroot.trim();
        args.push(format!("--sysroot={sysroot}"));
    } else {
        panic!("Could not determine sysroot");
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

    let Ok(lv_conf) = env::var("LV_CONF_PATH") else {
        return Err("lv_conf.h not found. Set LV_CONF_PATH or provide src/lvgl/lv_conf.h.");
    };
    let lv_conf = PathBuf::from(lv_conf);
    println!("cargo::rerun-if-changed={}", lv_conf.display());

    let mut cmake_build = cmake::Config::new(&lvgl_dir);
    cmake_build.define("LV_BUILD_CONF_PATH", &lv_conf);
    // TODO: check if cross compiling
    //if target.startswith("thumb") {
    const INCLUDES: &[&str] = &[
        "Core/Inc",
        "Drivers/STM32U5xx_HAL_Driver/Inc",
        "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
        "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
        "Drivers/CMSIS/Include",
    ];
    let st_root = repo_root.join("external/ST");
    for inc in INCLUDES {
        let inc_full = st_root.join(inc);
        cmake_build.cflag(format!("-I{}", inc_full.display()));
    }
    let nema_gfx_include = lvgl_dir.join("libs/nema_gfx/include");
    if nema_gfx_include.join("nema_core.h").is_file() {
        cmake_build.cflag(format!("-I{}", nema_gfx_include.display()));
    }
    //cmake_build.cflag("--specs=nosys.specs");
    //cmake_build.cflag("--specs=nano.specs");
    cmake_build.cflag("-DUSE_HAL_DRIVER");
    cmake_build.cflag("-DSTM32U5A9xx");
    cmake_build.define(
        "CMAKE_EXE_LINKER_FLAGS",
        "--specs=nosys.specs --specs=nano.specs",
    );
    //}
    let dst = cmake_build.build();
    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-lib=static=lvgl");
    let target = env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        let nema_gfx_lib_dir = lvgl_dir.join("libs/nema_gfx/lib/core/cortex_m33_revC/gcc");
        let nema_gfx_lib = nema_gfx_lib_dir.join("libnemagfx-float-abi-hard.a");
        if nema_gfx_lib.is_file() {
            println!("cargo::rerun-if-changed={}", nema_gfx_lib.display());
            println!(
                "cargo::rustc-link-search=native={}",
                nema_gfx_lib_dir.display()
            );
            println!("cargo::rustc-link-lib=static=nemagfx-float-abi-hard");
        }
    }

    if let Err(err) = Command::new("bindgen").arg("--version").output() {
        if err.kind() == ErrorKind::NotFound {
            return Err("`bindgen` executable was not found. Check your PATH.");
        }
        return Err("failed to execute `bindgen --version`");
    }

    let clang_args = build_clang_args(&lvgl_dir, &lv_conf);
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");
    run_bindgen(&wrapper, &out_path, &clang_args)
}
