// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const ST_SOURCES: &[&str] = &[
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_cortex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gpio.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_exti.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_icache.c",
];

const ST_DEFINES: &[&str] = &["USE_HAL_DRIVER", "STM32U5A9xx"];
const ST_DEBUG_DEFINES: &[(&str, &str)] = &[("USE_FULL_ASSERT", "1U")];

const ST_INCLUDES: &[&str] = &[
    "Common/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
    "Drivers/CMSIS/Include",
];

fn run_bindgen(wrapper: &Path, output: &Path, clang_args: &[String]) -> Result<(), &'static str> {
    let res = Command::new("bindgen")
        .arg("--output")
        .arg(output)
        .arg("--use-core")
        .arg("--with-derive-default")
        .arg("--no-layout-tests")
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

fn is_release_profile() -> bool {
    env::var("PROFILE").expect("PROFILE not set") == "release"
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rerun-if-env-changed=PROFILE");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let target = env::var("TARGET").expect("TARGET not set");

    // These bindings describe STM32U5 firmware headers and are only valid for the embedded target.
    // Host builds only need the crate to exist as a dependency, so skip generating/importing them.
    if !target.starts_with("thumb") {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("bindings.rs");

    let repo_root = manifest_dir.join("../../..");
    let st_root = repo_root.join("external/ST");

    if !st_root.is_dir() {
        return Err("external/ST not found");
    }
    for source in ST_SOURCES {
        println!(
            "cargo::rerun-if-changed={}",
            st_root.join(source).as_path().display()
        );
    }
    for include in ST_INCLUDES {
        println!(
            "cargo::rerun-if-changed={}",
            st_root.join(include).as_path().display()
        );
    }

    let include_paths: Vec<PathBuf> = ST_INCLUDES.iter().map(|p| st_root.join(p)).collect();

    let release_profile = is_release_profile();
    let mut clang_args: Vec<String> = ST_DEFINES.iter().map(|d| format!("-D{d}")).collect();
    if !release_profile {
        clang_args.extend(
            ST_DEBUG_DEFINES
                .iter()
                .map(|(key, value)| format!("-D{key}={value}")),
        );
    }
    clang_args.extend(
        include_paths
            .iter()
            .map(|p| format!("-I{}", p.as_path().display())),
    );
    // Generate bindings for the active firmware target ABI, not the host ABI.
    clang_args.push(format!("--target={target}"));

    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found");
    }
    run_bindgen(&wrapper, &out_path, &clang_args)?;

    let source_paths: Vec<PathBuf> = ST_SOURCES.iter().map(|p| st_root.join(p)).collect();
    let mut build = cc::Build::new();
    build.files(&source_paths);
    for def in ST_DEFINES {
        build.define(def, None);
    }
    if !release_profile {
        for (key, value) in ST_DEBUG_DEFINES {
            build.define(key, Some(*value));
        }
    }
    build.includes(&include_paths);
    // Suppress warnings in third-party sources.
    build.flag_if_supported("-w");
    build.compile("st_drivers");
    Ok(())
}
