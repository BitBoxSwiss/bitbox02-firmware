// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};

type BuildResult<T> = Result<T, String>;

const SOURCES: &[&str] = &[
    "external/cryptoauthlib/lib/atca_cfgs.c",
    "external/cryptoauthlib/lib/atca_command.c",
    "external/cryptoauthlib/lib/atca_device.c",
    "external/cryptoauthlib/lib/atca_iface.c",
    "external/cryptoauthlib/lib/hal/atca_hal.c",
    "external/cryptoauthlib/lib/hal/hal_timer_start.c",
    "external/cryptoauthlib/lib/atca_basic.c",
    "external/cryptoauthlib/lib/atca_debug.c",
    "external/cryptoauthlib/lib/calib/calib_basic.c",
    "external/cryptoauthlib/lib/calib/calib_command.c",
    "external/cryptoauthlib/lib/calib/calib_execution.c",
    "external/cryptoauthlib/lib/calib/calib_counter.c",
    "external/cryptoauthlib/lib/calib/calib_gendig.c",
    "external/cryptoauthlib/lib/calib/calib_nonce.c",
    "external/cryptoauthlib/lib/calib/calib_checkmac.c",
    "external/cryptoauthlib/lib/calib/calib_info.c",
    "external/cryptoauthlib/lib/calib/calib_derivekey.c",
    "external/cryptoauthlib/lib/calib/calib_random.c",
    "external/cryptoauthlib/lib/calib/calib_selftest.c",
    "external/cryptoauthlib/lib/calib/calib_read.c",
    "external/cryptoauthlib/lib/calib/calib_privwrite.c",
    "external/cryptoauthlib/lib/calib/calib_verify.c",
    "external/cryptoauthlib/lib/calib/calib_write.c",
    "external/cryptoauthlib/lib/calib/calib_updateextra.c",
    "external/cryptoauthlib/lib/calib/calib_lock.c",
    "external/cryptoauthlib/lib/calib/calib_kdf.c",
    "external/cryptoauthlib/lib/calib/calib_genkey.c",
    "external/cryptoauthlib/lib/calib/calib_sign.c",
    "external/cryptoauthlib/lib/host/atca_host.c",
    "external/cryptoauthlib/lib/crypto/hashes/sha2_routines.c",
    "external/cryptoauthlib/lib/crypto/atca_crypto_sw_sha2.c",
];

fn main() -> BuildResult<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir
        .join("../../..")
        .canonicalize()
        .map_err(|err| format!("failed to resolve repo root: {err}"))?;
    emit_rerun_if_changed_path(&manifest_dir.join("build.rs"));
    emit_rerun_if_changed_path(&manifest_dir.join("Cargo.toml"));
    emit_rerun_if_changed_path(&repo_root.join("external/cryptoauthlib"));
    emit_rerun_if_changed_path(&repo_root.join("external/atca_config.h"));
    emit_rerun_if_changed_path(&repo_root.join("external/asf4-drivers"));
    emit_rerun_if_changed_path(&repo_root.join("external/samd51a-ds"));
    emit_rerun_if_changed_path(&repo_root.join("external/CMSIS"));
    emit_rerun_if_changed_path(&repo_root.join("src"));

    compile_cryptoauthlib(&repo_root)?;
    Ok(())
}

fn compile_cryptoauthlib(repo_root: &Path) -> BuildResult<()> {
    let mut build = cc::Build::new();
    build.compiler("arm-none-eabi-gcc");
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

    build.files(source_paths(repo_root, SOURCES));
    build.compile("cryptoauthlib");
    Ok(())
}

fn include_dirs(repo_root: &Path) -> Vec<PathBuf> {
    vec![
        repo_root.join("src"),
        repo_root.join("external"),
        repo_root.join("external/cryptoauthlib/lib"),
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
