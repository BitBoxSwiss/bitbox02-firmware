// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(dead_code)]
#[path = "src/memory.rs"]
mod memory;

const ARM_NONE_EABI_GCC: &str = "arm-none-eabi-gcc";

const BOARD_SOURCES: &[&str] = &[
    "stm32u5a9j-dk/Src/adc.c",
    "stm32u5a9j-dk/Src/dsihost.c",
    "stm32u5a9j-dk/Src/gpio.c",
    "stm32u5a9j-dk/Src/gpu2d.c",
    "stm32u5a9j-dk/Src/hspi.c",
    "stm32u5a9j-dk/Src/i2c.c",
    "stm32u5a9j-dk/Src/icache.c",
    "stm32u5a9j-dk/Src/ltdc.c",
    "stm32u5a9j-dk/Src/octospi.c",
    "stm32u5a9j-dk/Src/board.c",
    "stm32u5a9j-dk/Src/sdmmc.c",
    "stm32u5a9j-dk/Src/flash.c",
    "stm32u5a9j-dk/Src/hash.c",
    "stm32u5a9j-dk/Src/usart.c",
    "stm32u5a9j-dk/Src/usb_otg.c",
    "USBX/App/app_usbx_device.c",
    "USBX/App/ux_device_customhid.c",
    "USBX/App/ux_device_descriptors.c",
];

const ST_DEFINES: &[&str] = &[
    "USE_HAL_DRIVER",
    "STM32U5A9xx",
    "UX_INCLUDE_USER_DEFINE_FILE",
];
const ST_DEBUG_DEFINES: &[(&str, &str)] = &[("USE_FULL_ASSERT", "1U")];

const ST_INCLUDES: &[&str] = &[
    "Common/Inc",
    "stm32u5a9j-dk/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
    "Drivers/CMSIS/Include",
    "USBX/App",
    "USBX/Target",
    "Middlewares/ST/usbx/common/core/inc",
    "Middlewares/ST/usbx/common/usbx_device_classes/inc",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers",
    "Middlewares/ST/usbx/ports/generic/inc",
];

fn hex_addr(addr: usize) -> String {
    format!("0x{addr:08X}")
}

fn len_expr(len: usize) -> String {
    if len.is_multiple_of(1024) {
        format!("{}K", len / 1024)
    } else {
        len.to_string()
    }
}

fn region(name: &str, attrs: &str, origin: usize, len: usize) -> String {
    format!(
        "  {name:<22} ({attrs}) : ORIGIN = {}, LENGTH = {}\n",
        hex_addr(origin),
        len_expr(len)
    )
}

fn memory_x() -> String {
    let mut out = String::from("MEMORY\n{\n");
    out.push_str(&region(
        "BOOT_ARGS",
        "xrw",
        memory::BOOT_ARGS_ADDR,
        memory::BOOT_ARGS_LEN,
    ));
    out.push_str(&region("RAM", "xrw", memory::RAM_ADDR, memory::RAM_LEN));
    out.push_str(&region(
        "SRAM4",
        "xrw",
        memory::SRAM4_ADDR,
        memory::SRAM4_LEN,
    ));
    out.push_str(&region(
        "BOOT0_FLASH",
        "rx",
        memory::BOOT0_ADDR,
        memory::BOOT0_MAX_LEN,
    ));
    out.push_str(&region(
        "BOOT1_FLASH",
        "rx",
        memory::BOOT1_ADDR,
        memory::BOOT1_MAX_LEN,
    ));
    out.push_str(&region(
        "FW_FLASH",
        "rx",
        memory::FIRMWARE_ADDR,
        memory::FIRMWARE_MAX_LEN,
    ));
    out.push_str(&region(
        "VENDOR_DATA",
        "rx",
        memory::VENDOR_DATA_ADDR,
        memory::VENDOR_DATA_LEN,
    ));
    out.push_str(&region(
        "USER_DATA",
        "rx",
        memory::USER_DATA_ADDR,
        memory::USER_DATA_LEN,
    ));
    out.push_str(&region(
        "DFU_BOOT1",
        "rx",
        memory::DFU_BOOT1_ADDR,
        memory::DFU_BOOT1_MAX_LEN,
    ));
    out.push_str(&region("GRAM", "xrw", memory::GRAM_ADDR, memory::GRAM_LEN));
    out.push_str("}\n");
    out
}

fn generate_memory_x(out_dir: &Path, manifest_dir: &Path) {
    let source = manifest_dir.join("src/memory.rs");
    println!("cargo::rerun-if-changed={}", source.display());
    std::fs::write(out_dir.join("memory.x"), memory_x()).expect("write memory layout script");
}

fn arm_none_eabi_sysroot() -> Result<String, &'static str> {
    let output = Command::new(ARM_NONE_EABI_GCC)
        .arg("--print-sysroot")
        .output()
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                "`arm-none-eabi-gcc` executable was not found. Check your PATH."
            } else {
                "failed to execute `arm-none-eabi-gcc --print-sysroot`"
            }
        })?;
    if !output.status.success() {
        return Err("`arm-none-eabi-gcc --print-sysroot` failed");
    }
    let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if sysroot.is_empty() {
        return Err("`arm-none-eabi-gcc --print-sysroot` returned an empty sysroot");
    }
    Ok(sysroot)
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-env-changed=PROFILE");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let repo_root = manifest_dir.join("../../..");
    let st_root = repo_root.join("external/ST");

    for source in BOARD_SOURCES {
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

    generate_memory_x(&out_dir, &manifest_dir);
    println!("cargo::rustc-link-search={}", out_dir.display());

    let target = env::var("TARGET").expect("TARGET not set");
    if !target.starts_with("thumb") {
        return Ok(());
    }

    let sysroot = arm_none_eabi_sysroot()?;
    let source_paths: Vec<PathBuf> = BOARD_SOURCES.iter().map(|p| st_root.join(p)).collect();
    let mut build = cc::Build::new();
    build.files(&source_paths);
    for define in ST_DEFINES {
        build.define(define, None);
    }
    if env::var("PROFILE").expect("PROFILE not set") != "release" {
        for (key, value) in ST_DEBUG_DEFINES {
            build.define(key, Some(*value));
        }
    }
    for include in ST_INCLUDES {
        build.include(st_root.join(include));
    }
    build.flag(format!("--sysroot={sysroot}"));
    build.flag_if_supported("-w");
    build.compile("st_board");
    Ok(())
}
