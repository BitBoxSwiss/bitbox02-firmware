// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};

type BuildResult<T> = Result<T, String>;

const ASF4_DRIVERS_MIN_SOURCES: &[&str] = &[
    "external/asf4-drivers/hal/utils/src/utils_syscalls.c",
    "external/asf4-drivers/hal/utils/src/utils_list.c",
    "external/asf4-drivers/hal/src/hal_atomic.c",
    "external/asf4-drivers/hal/src/hal_gpio.c",
    "external/asf4-drivers/hal/src/hal_init.c",
    "external/asf4-drivers/hal/src/hal_delay.c",
    "external/asf4-drivers/hal/src/hal_timer.c",
    "external/asf4-drivers/hal/src/hal_usb_device.c",
    "external/asf4-drivers/hal/src/hal_rand_sync.c",
    "external/asf4-drivers/hal/src/hal_flash.c",
    "external/asf4-drivers/hal/src/hal_pac.c",
    "external/asf4-drivers/hal/src/hal_io.c",
    "external/asf4-drivers/hal/src/hal_sha_sync.c",
    "external/asf4-drivers/hpl/systick/hpl_systick.c",
    "external/asf4-drivers/hal/src/hal_usart_async.c",
    "external/asf4-drivers/hal/utils/src/utils_ringbuffer.c",
    "external/asf4-drivers/hpl/gclk/hpl_gclk.c",
    "external/asf4-drivers/hpl/oscctrl/hpl_oscctrl.c",
    "external/asf4-drivers/hpl/mclk/hpl_mclk.c",
    "external/asf4-drivers/hpl/osc32kctrl/hpl_osc32kctrl.c",
    "external/asf4-drivers/hpl/core/hpl_init.c",
    "external/asf4-drivers/hpl/core/hpl_core_m4.c",
    "external/asf4-drivers/hpl/spi/spi_lite.c",
    "external/asf4-drivers/hpl/usb/hpl_usb.c",
    "external/asf4-drivers/hpl/rtc/hpl_rtc.c",
    "external/asf4-drivers/hpl/sercom/hpl_sercom.c",
    "external/asf4-drivers/hpl/trng/hpl_trng.c",
    "external/asf4-drivers/hpl/nvmctrl/hpl_nvmctrl.c",
    "external/asf4-drivers/hpl/icm/hpl_icm.c",
    "external/asf4-drivers/hpl/pac/hpl_pac.c",
    "external/asf4-drivers/usb/usb_protocol.c",
    "external/asf4-drivers/usb/device/usbdc.c",
];

const ASF4_DRIVERS_SOURCES: &[&str] = &[
    "external/asf4-drivers/hal/src/hal_mci_sync.c",
    "external/asf4-drivers/hal/src/hal_i2c_m_sync.c",
    "external/asf4-drivers/hpl/sdhc/hpl_sdhc.c",
    "external/asf4-drivers/hpl/sercom/hpl_sercom.c",
    "external/asf4-drivers/sd_mmc/sd_mmc.c",
    "external/asf4-drivers/diskio/sdmmc_diskio.c",
];

fn main() -> BuildResult<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir
        .join("../../../..")
        .canonicalize()
        .map_err(|err| format!("failed to resolve repo root: {err}"))?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    emit_rerun_if_changed_path(&manifest_dir.join("build.rs"));
    emit_rerun_if_changed_path(&manifest_dir.join("Cargo.toml"));
    emit_rerun_if_changed_path(&repo_root.join("src"));
    emit_rerun_if_changed_path(&repo_root.join("external/asf4-drivers"));
    emit_rerun_if_changed_path(&repo_root.join("external/samd51a-ds"));
    emit_rerun_if_changed_path(&repo_root.join("external/CMSIS"));

    compile_asf4_drivers(&repo_root)?;

    println!(
        "cargo:archive={}",
        out_dir.join("libbitbox_samd52_asf4.a").display()
    );
    Ok(())
}

fn compile_asf4_drivers(repo_root: &Path) -> BuildResult<()> {
    let mut build = cc::Build::new();
    build.compiler("arm-none-eabi-gcc");
    build.cargo_metadata(false);
    build.no_default_flags(true);
    build.warnings(false);
    build.flag("-std=c11");
    build.flag("-mcpu=cortex-m4");
    build.flag("-mthumb");
    build.flag("-mlong-calls");
    build.flag("-mfloat-abi=softfp");
    build.flag("-mfpu=fpv4-sp-d16");
    build.flag("-fomit-frame-pointer");
    build.flag("-ffunction-sections");
    build.flag("-fdata-sections");
    build.flag("-fstack-protector-strong");
    build.flag("-Os");
    build.flag("-DNDEBUG");
    build.flag("-D_XOPEN_SOURCE=600");
    build.flag("-D__SAMD51J20A__");
    build.flag("-Wno-incompatible-pointer-types");
    build.flag("-Wno-unused-parameter");
    build.flag("-Wno-unused-variable");
    build.flag("-Wno-missing-prototypes");
    build.flag("-Wno-missing-declarations");
    build.flag("-Wno-cast-qual");
    build.flag("-Wno-switch-default");
    build.flag("-Wno-format-nonliteral");
    build.flag("-Wno-bad-function-cast");
    build.flag("-Wno-old-style-definition");
    build.flag("-Wno-strict-prototypes");
    build.flag("-Wno-cast-align");
    build.flag("-Wno-implicit-fallthrough");
    build.flag("-Wno-pedantic");

    for include in include_dirs(repo_root) {
        build.include(include);
    }

    build.files(source_paths(repo_root, ASF4_DRIVERS_MIN_SOURCES));
    build.files(source_paths(repo_root, ASF4_DRIVERS_SOURCES));
    build.compile("bitbox_samd52_asf4");
    Ok(())
}

fn include_dirs(repo_root: &Path) -> Vec<PathBuf> {
    vec![
        repo_root.join("src"),
        repo_root.join("src/platform"),
        repo_root.join("src/qtouch"),
        repo_root.join("external/asf4-drivers"),
        repo_root.join("external/asf4-drivers/Config"),
        repo_root.join("external/asf4-drivers/hal/include"),
        repo_root.join("external/asf4-drivers/hal/utils/include"),
        repo_root.join("external/asf4-drivers/hpl/core"),
        repo_root.join("external/asf4-drivers/hpl/gclk"),
        repo_root.join("external/asf4-drivers/hpl/pm"),
        repo_root.join("external/asf4-drivers/hpl/port"),
        repo_root.join("external/asf4-drivers/hpl/pukcc"),
        repo_root.join("external/asf4-drivers/hpl/rtc"),
        repo_root.join("external/asf4-drivers/hpl/spi"),
        repo_root.join("external/asf4-drivers/hri"),
        repo_root.join("external/asf4-drivers/qtouch"),
        repo_root.join("external/asf4-drivers/qtouch/include"),
        repo_root.join("external/asf4-drivers/sd_mmc"),
        repo_root.join("external/asf4-drivers/usb"),
        repo_root.join("external/asf4-drivers/usb/class"),
        repo_root.join("external/asf4-drivers/usb/class/hid"),
        repo_root.join("external/asf4-drivers/usb/device"),
        repo_root.join("external/asf4-drivers/diskio"),
        repo_root.join("external/samd51a-ds/include"),
        repo_root.join("external/CMSIS/Include"),
    ]
}

fn source_paths(repo_root: &Path, rel_paths: &[&str]) -> Vec<PathBuf> {
    rel_paths.iter().map(|path| repo_root.join(path)).collect()
}

fn emit_rerun_if_changed_path(path: &Path) {
    println!("cargo::rerun-if-changed={}", path.display());
}
