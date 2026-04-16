use std::path::PathBuf;
use std::process::Command;

const ST_DEFINES: &[&str] = &["USE_HAL_DRIVER", "STM32U5A9xx"];

const ST_INCLUDES: &[&str] = &[
    "stm32u5-dk/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
    "Drivers/CMSIS/Include",
];

fn build_hal_overrides_object(repo_root: &PathBuf, out_dir: &PathBuf) {
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
    // Compile stm32u5xx_hal_msp.c into a standalone object instead of putting it into the
    // bitbox-platform-stm32u5-sys static archive. stm32u5xx_hal.c contains weak defaults that can
    // be pulled
    // in when the archive is searched, which then prevents the linker from extracting later
    // archive members with stronger replacements. Linking this file as a plain object avoids that
    // archive extraction behavior while keeping the ST sources themselves unmodified.
    command.arg("-c");
    command.arg(&source);
    command.arg("-o");
    command.arg(&output);

    let status = command.status().expect("compile hal_overrides.o");
    assert!(status.success(), "failed to compile hal_overrides.o");

    println!("cargo::rustc-link-arg={}", output.display());
}

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let target = std::env::var("TARGET").expect("TARGET not set");
    let repo_root = manifest_dir.join("../../../..");

    let lds_from = manifest_dir.join("bitbox03-firmware.ld");
    let lds_to = out_dir.join("bitbox03-firmware.ld");
    println!("cargo::rerun-if-changed={}", lds_from.display());
    std::fs::copy(lds_from, lds_to).expect("copy linker script");

    // Search paths to linker scripts
    println!("cargo::rustc-link-search={}", out_dir.display());

    println!(
        "cargo::rustc-link-arg=-Wl,-Map={}",
        out_dir.join("bitbox03-firmware.map").display()
    );

    if target.starts_with("thumb") {
        build_hal_overrides_object(&repo_root, &out_dir);
    }
}
