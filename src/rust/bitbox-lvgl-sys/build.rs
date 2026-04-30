// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

include!("lvgl_sources.rs");

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

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rerun-if-changed=lvgl_sources.rs");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_ST_LTDC");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_CONFIG_FIRMWARE");

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
    println!("cargo::rerun-if-changed={}", lvgl_dir.display());

    let st_ltdc_enabled = env::var_os("CARGO_FEATURE_ST_LTDC").is_some();
    let firmware_enabled = env::var_os("CARGO_FEATURE_CONFIG_FIRMWARE").is_some();
    let target = env::var("TARGET").expect("TARGET not set");

    let mut cflags = vec![
        format!("-I{}", lvgl_dir.display()),
        "-DLV_KCONFIG_IGNORE".to_owned(),
        "-DLV_LVGL_H_INCLUDE_SIMPLE".to_owned(),
    ];
    if st_ltdc_enabled {
        cflags.push("-DLV_USE_ST_LTDC=1".to_owned());
        cflags.push("-DUSE_HAL_DRIVER".to_owned());
        cflags.push("-DSTM32U5A9xx".to_owned());
    }

    let lv_conf = manifest_dir.join("lv_conf.h");
    println!("cargo::rerun-if-changed={}", lv_conf.display());

    if st_ltdc_enabled {
        const INCLUDES: &[&str] = &[
            "stm32u5-dk/Inc",
            "Drivers/STM32U5xx_HAL_Driver/Inc",
            "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
            "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
            "Drivers/CMSIS/Include",
        ];
        let st_root = repo_root.join("external/ST");
        for inc in INCLUDES {
            let inc_full = st_root.join(inc);
            cflags.push(format!("-I{}", inc_full.display()));
        }
    }
    let debug = env::var("PROFILE").unwrap() == "debug";

    if target.starts_with("thumb") {
        let nema_gfx_include = lvgl_dir.join("libs/nema_gfx/include");
        if nema_gfx_include.join("nema_core.h").is_file() {
            cflags.push(format!("-I{}", nema_gfx_include.display()));
        }
        cflags.push("-Os".to_owned());
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

    // LV_* variables are set using CMake when building the library but by us here when running
    // bindgen.
    let mut clang_args = vec![format!("--target={target}")];
    let mut c_args = vec![
        format!("-DLV_CONF_PATH=\"{}\"", lv_conf.display()),
        "-DLV_CONF_INCLUDE_SIMPLE".to_owned(),
    ];

    if firmware_enabled {
        c_args.push("-DBITBOX_LVGL_CONF_FIRMWARE".to_owned());
    }

    c_args.extend(cflags.iter().cloned());

    clang_args.extend(c_args.iter().cloned());

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");

    let mut lvgl_build = cc::Build::new();
    for source in LVGL_C_FILES {
        lvgl_build.file(lvgl_dir.join(source));
    }
    if st_ltdc_enabled {
        for source in LVGL_ST_LTDC_C_FILES {
            lvgl_build.file(lvgl_dir.join(source));
        }
    }
    for flag in &c_args {
        lvgl_build.flag(flag);
    }
    lvgl_build.warnings(false);
    lvgl_build.extra_warnings(false);
    lvgl_build.debug(debug);
    lvgl_build.compile("lvgl");

    let mut fonts = cc::Build::new();
    fonts.file(manifest_dir.join("../../ui/fonts/inter_regular_32.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_regular_48.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_bold_32.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_bold_48.c"));
    for flag in &c_args {
        fonts.flag(flag);
    }
    fonts.compile("lvgl_fonts");
    run_bindgen(&wrapper, &out_path, &clang_args)
}
