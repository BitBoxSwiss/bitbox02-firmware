// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const ST_SOURCES: &[&str] = &[
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal.c",
    "Core/Src/platform.c",
    "Core/Src/stm32u5xx_it.c",
    "Core/Src/aps512xx.c",
    "Core/Src/stm32u5xx_hal_msp.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_adc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_adc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_i2c.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_i2c_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_cortex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_xspi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gpio.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_exti.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gtzc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_icache.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_xspi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_dlyb.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ospi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_sdmmc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sd.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sd_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_mmc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_mmc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sdio.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_uart.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_uart_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_hcd.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_usb.c",
    "Core/Src/system_stm32u5xx.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gpu2d.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ltdc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ltdc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dsi.c",
];

const ST_DEFINES: &[&str] = &["USE_HAL_DRIVER", "STM32U5A9xx"];

const ST_INCLUDES: &[&str] = &[
    "Core/Inc",
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

fn detect_sysroot() -> Option<String> {
    if let Ok(sysroot) = env::var("CMAKE_SYSROOT") {
        if !sysroot.is_empty() {
            return Some(sysroot);
        }
    }
    let output = Command::new("arm-none-eabi-gcc")
        .arg("--print-sysroot")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sysroot = String::from_utf8(output.stdout).ok()?;
    let sysroot = sysroot.trim();
    if sysroot.is_empty() {
        None
    } else {
        Some(sysroot.to_owned())
    }
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
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

    if let Err(err) = Command::new("bindgen").arg("--version").output() {
        if err.kind() == ErrorKind::NotFound {
            return Err("`bindgen` executable was not found. Check your PATH.");
        }
        return Err("failed to execute `bindgen --version`");
    }

    let include_paths: Vec<PathBuf> = ST_INCLUDES.iter().map(|p| st_root.join(p)).collect();

    let mut clang_args: Vec<String> = ST_DEFINES.iter().map(|d| format!("-D{d}")).collect();
    clang_args.extend(
        include_paths
            .iter()
            .map(|p| format!("-I{}", p.as_path().display())),
    );
    // Parse headers as Cortex-M33, matching the ST project that generated these files.
    clang_args.push("--target=thumbv8m.main-none-eabihf".to_owned());
    if let Some(sysroot) = detect_sysroot() {
        clang_args.push(format!("--sysroot={sysroot}"));
    }

    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found");
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("bindings.rs");
    run_bindgen(&wrapper, &out_path, &clang_args)?;

    // ST sources are Cortex-M only and do not compile as host objects.
    let target = env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        let source_paths: Vec<PathBuf> = ST_SOURCES.iter().map(|p| st_root.join(p)).collect();
        let mut build = cc::Build::new();
        build.files(&source_paths);
        for def in ST_DEFINES {
            build.define(def, None);
        }
        build.includes(&include_paths);
        // Suppress warnings in third-party sources.
        build.flag_if_supported("-w");
        build.compile("st_drivers");
    }
    Ok(())
}
