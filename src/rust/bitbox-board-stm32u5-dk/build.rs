// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

const BOARD_SOURCES: &[&str] = &[
    "stm32u5-dk/Src/adc.c",
    "stm32u5-dk/Src/dsihost.c",
    "stm32u5-dk/Src/gpio.c",
    "stm32u5-dk/Src/gpu2d.c",
    "stm32u5-dk/Src/hspi.c",
    "stm32u5-dk/Src/i2c.c",
    "stm32u5-dk/Src/icache.c",
    "stm32u5-dk/Src/ltdc.c",
    "stm32u5-dk/Src/octospi.c",
    "stm32u5-dk/Src/board.c",
    "stm32u5-dk/Src/sdmmc.c",
    "stm32u5-dk/Src/system_stm32u5xx.c",
];

const ST_DEFINES: &[&str] = &[
    "USE_HAL_DRIVER",
    "STM32U5A9xx",
    "UX_INCLUDE_USER_DEFINE_FILE",
];

const ST_INCLUDES: &[&str] = &[
    "stm32u5-dk/Inc",
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

fn main() {
    println!("cargo::rerun-if-changed=memory.x");

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

    std::fs::copy(manifest_dir.join("memory.x"), out_dir.join("memory.x"))
        .expect("copy memory layout script");
    println!("cargo::rustc-link-search={}", out_dir.display());

    let target = env::var("TARGET").expect("TARGET not set");
    if !target.starts_with("thumb") {
        return;
    }

    let source_paths: Vec<PathBuf> = BOARD_SOURCES.iter().map(|p| st_root.join(p)).collect();
    let mut build = cc::Build::new();
    build.files(&source_paths);
    for define in ST_DEFINES {
        build.define(define, None);
    }
    for include in ST_INCLUDES {
        build.include(st_root.join(include));
    }
    build.flag_if_supported("-w");
    build.compile("st_board");
}
