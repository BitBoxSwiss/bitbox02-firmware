// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::process::Command;

const ST_DEFINES: &[&str] = &["USE_HAL_DRIVER", "STM32U5A9xx"];

const ST_INCLUDES: &[&str] = &[
    "stm32u5-dk/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
    "Drivers/CMSIS/Include",
];

pub fn build_hal_overrides_object(repo_root: &Path, out_dir: &Path) {
    let st_root = repo_root.join("external/ST");
    let source = st_root.join("stm32u5-dk/Src/stm32u5xx_hal_msp.c");
    let output = out_dir.join("hal_overrides.o");

    println!("cargo::rerun-if-changed={}", source.display());
    for include in ST_INCLUDES {
        println!(
            "cargo::rerun-if-changed={}",
            st_root.join(include).display()
        );
    }

    let mut build = cc::Build::new();
    for define in ST_DEFINES {
        build.define(define, None);
    }
    for include in ST_INCLUDES {
        build.include(st_root.join(include));
    }
    build.flag_if_supported("-w");

    let compiler = build.get_compiler();
    let mut command: Command = compiler.to_command();
    // Compile stm32u5xx_hal_msp.c into a standalone object instead of putting it into a static
    // archive. The final binaries already pull in a weak HAL_MspInit from stm32u5xx_hal.c via the
    // platform sys archive, so a separate libhal_overrides.a linked with rustc-link-lib would
    // usually not be extracted: by that point the symbol is no longer unresolved. Linking a raw
    // .o through each top-level build script makes the strong HAL_MspInit participate in the final
    // link unconditionally, which reliably overrides the weak default.
    command.arg("-c");
    command.arg(&source);
    command.arg("-o");
    command.arg(&output);

    let status = command.status().expect("compile hal_overrides.o");
    assert!(status.success(), "failed to compile hal_overrides.o");

    println!("cargo::rustc-link-arg={}", output.display());
}
