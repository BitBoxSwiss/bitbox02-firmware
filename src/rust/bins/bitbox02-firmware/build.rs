use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

type BuildResult<T> = Result<T, String>;

const DBB_FIRMWARE_SOURCES: &[&str] = &[
    "src/delay.c",
    "src/random.c",
    "src/hardfault.c",
    "src/util.c",
    "src/sd.c",
    "src/system.c",
    "src/hww.c",
    "src/memory/bitbox02_smarteeprom.c",
    "src/memory/memory.c",
    "src/memory/memory_shared.c",
    "src/memory/mpu.c",
    "src/memory/nvmctrl.c",
    "src/memory/spi_mem.c",
    "src/memory/memory_spi.c",
    "src/memory/smarteeprom.c",
    "src/i2c_ecc.c",
    "src/touch/gestures.c",
    "src/reset.c",
    "src/queue.c",
    "src/usb/usb_processing.c",
];

const DBB_FIRMWARE_UI_SOURCES: &[&str] = &[
    "src/screen.c",
    "src/ui/graphics/graphics.c",
    "src/ui/ugui/ugui.c",
    "src/ui/fonts/font_a_9X9.c",
    "src/ui/fonts/font_a_11X10.c",
    "src/ui/fonts/font_a_11X12.c",
    "src/ui/fonts/font_a_13X14.c",
    "src/ui/fonts/font_a_15X16.c",
    "src/ui/fonts/font_a_17X18.c",
    "src/ui/fonts/monogram_5X9.c",
    "src/ui/fonts/password_9X9.c",
    "src/ui/fonts/password_11X12.c",
    "src/ui/screen_saver.c",
    "src/ui/screen_stack.c",
    "src/ui/screen_process.c",
    "src/ui/event_handler.c",
    "src/ui/ui_util.c",
    "src/ui/components/trinary_choice.c",
    "src/ui/components/trinary_input_char.c",
    "src/ui/components/trinary_input_string.c",
    "src/ui/components/waiting.c",
    "src/ui/components/screensaver.c",
    "src/ui/components/knight_rider.c",
    "src/ui/components/right_arrow.c",
    "src/ui/components/left_arrow.c",
    "src/ui/components/icon_button.c",
    "src/ui/components/confirm_gesture.c",
    "src/ui/components/label.c",
    "src/ui/components/confirm.c",
    "src/ui/components/keyboard_switch.c",
    "src/ui/components/orientation_arrows.c",
    "src/ui/components/info_centered.c",
    "src/ui/components/lockscreen.c",
    "src/ui/components/unlock_animation.c",
    "src/ui/components/menu.c",
    "src/ui/components/status.c",
    "src/ui/components/image.c",
    "src/ui/components/button.c",
    "src/ui/components/empty.c",
    "src/ui/components/progress.c",
    "src/ui/components/sdcard.c",
    "src/ui/components/ui_images.c",
    "src/ui/components/confirm_transaction.c",
];

const DRIVER_SOURCES: &[&str] = &[
    "src/platform/platform_init.c",
    "src/platform/driver_init.c",
    "src/ui/oled/oled.c",
    "src/ui/oled/oled_writer.c",
];

const QTOUCH_SOURCES: &[&str] = &["src/qtouch/qtouch.c"];

const PLATFORM_BITBOX02_PLUS_SOURCES: &[&str] = &[
    "src/da14531/da14531.c",
    "src/da14531/da14531_protocol.c",
    "src/da14531/da14531_handler.c",
    "src/uart.c",
];

const PLATFORM_BITBOX02_SOURCES: &[&str] = &[
    "src/sd_mmc/sd_mmc_start.c",
    "src/sd_mmc/sd_mmc_ext.c",
    "src/usb/class/hid/hid.c",
    "src/usb/class/hid/hww/hid_hww.c",
    "src/ui/oled/sh1107.c",
    "src/ui/oled/ssd1312.c",
    "src/usb/usb.c",
    "src/usb/usb_frame.c",
    "src/usb/usb_packet.c",
    "src/u2f/u2f_packet.c",
];

const FIRMWARE_U2F_DRIVER_SOURCES: &[&str] = &["src/usb/class/hid/u2f/hid_u2f.c"];

const FIRMWARE_U2F_SOURCES: &[&str] = &["src/u2f.c", "src/u2f/u2f_app.c"];

const SECURECHIP_SOURCES: &[&str] = &[
    "src/atecc/atecc.c",
    "src/securechip/securechip.c",
    "src/optiga/pal/pal.c",
    "src/optiga/pal/pal_gpio.c",
    "src/optiga/pal/pal_i2c.c",
    "src/optiga/pal/pal_ifx_i2c_config.c",
    "src/optiga/pal/pal_logger.c",
    "src/optiga/pal/pal_os_datastore.c",
    "src/optiga/pal/pal_os_event.c",
    "src/optiga/pal/pal_os_lock.c",
    "src/optiga/pal/pal_os_timer.c",
    "src/optiga/pal/pal_os_memory.c",
    "src/optiga/optiga_ops.c",
    "src/optiga/optiga.c",
];

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

const CRYPTOAUTHLIB_SOURCES: &[&str] = &[
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

const EMBEDDED_SWD_SOURCES: &[&str] = &[
    "external/embedded-swd/dap.c",
    "external/embedded-swd/dap_target.c",
];

const STARTUP_SOURCES: &[&str] = &[
    "external/samd51a-ds/gcc/system_samd51.c",
    "external/samd51a-ds/gcc/gcc/startup_samd51.c",
];

fn main() -> BuildResult<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir
        .join("../../../..")
        .canonicalize()
        .map_err(|err| format!("failed to resolve repo root: {err}"))?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let variant = variant()?;

    emit_rerun_if_changed_path(&manifest_dir.join("build.rs"));
    emit_rerun_if_changed_path(&manifest_dir.join("Cargo.toml"));
    emit_rerun_if_changed_path(&manifest_dir.join("src"));
    emit_rerun_if_changed_path(&repo_root.join("src"));
    emit_rerun_if_changed_path(&repo_root.join("external/asf4-drivers"));
    emit_rerun_if_changed_path(&repo_root.join("external/samd51a-ds"));
    emit_rerun_if_changed_path(&repo_root.join("external/cryptoauthlib"));
    emit_rerun_if_changed_path(&repo_root.join("external/embedded-swd"));
    emit_rerun_if_changed_path(&repo_root.join("external/optiga-trust-m"));
    emit_rerun_if_changed_path(&repo_root.join("scripts/generate_rust_header.sh"));
    emit_rerun_if_changed_path(&repo_root.join("scripts/generate_version_headers.py"));
    emit_rerun_if_changed_path(&repo_root.join("versions.json"));

    let generated_headers_dir = out_dir.join("generated");
    let rust_header_dir = generated_headers_dir.join("rust");
    fs::create_dir_all(&rust_header_dir).map_err(|err| {
        format!(
            "failed to create generated headers dir {}: {err}",
            rust_header_dir.display()
        )
    })?;

    ensure_command_exists("cbindgen")?;
    generate_rust_header(&repo_root, &rust_header_dir)?;
    generate_version_headers(&repo_root, &generated_headers_dir)?;
    copy_linker_script(&manifest_dir, &out_dir)?;
    compile_c_firmware(&repo_root, &generated_headers_dir, variant)?;
    emit_linker_args(&out_dir, &repo_root, variant);

    Ok(())
}

fn variant() -> BuildResult<&'static str> {
    let multi = env::var_os("CARGO_FEATURE_MULTI").is_some();
    let btc_only = env::var_os("CARGO_FEATURE_BTC_ONLY").is_some();
    match (multi, btc_only) {
        (true, false) => Ok("firmware"),
        (false, true) => Ok("firmware-btc"),
        (true, true) => Err("both `multi` and `btc-only` features are enabled".to_string()),
        (false, false) => Err("no firmware variant feature is enabled".to_string()),
    }
}

fn compile_c_firmware(
    repo_root: &Path,
    generated_headers_dir: &Path,
    variant: &str,
) -> BuildResult<()> {
    let mut build = cc::Build::new();
    build.compiler("arm-none-eabi-gcc");
    build.cargo_metadata(false);
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
    build.flag("-DPB_NO_PACKED_STRUCTS=1");
    build.flag("-DPB_FIELD_16BIT=1");
    build.flag("-DOPTIGA_LIB_EXTERNAL=\"optiga_config.h\"");
    build.flag("-DMBEDTLS_USER_CONFIG_FILE=\"mbedtls_config.h\"");
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
    build.flag(&format!(
        "-DSOURCE_PATH_SIZE={}",
        repo_root.display().to_string().len() + 1
    ));

    match variant {
        "firmware" => {
            build.flag("-DPRODUCT_BITBOX_MULTI=1");
            build.flag("-DAPP_BTC=1");
            build.flag("-DAPP_LTC=1");
            build.flag("-DAPP_ETH=1");
            build.flag("-DAPP_U2F=1");
        }
        "firmware-btc" => {
            build.flag("-DPRODUCT_BITBOX_BTCONLY=1");
            build.flag("-DAPP_BTC=1");
            build.flag("-DAPP_LTC=0");
            build.flag("-DAPP_ETH=0");
            build.flag("-DAPP_U2F=0");
        }
        _ => return Err(format!("unsupported firmware variant: {variant}")),
    }

    for include in include_dirs(repo_root, generated_headers_dir) {
        build.include(include);
    }

    let mut sources = Vec::new();
    sources.extend(source_paths(repo_root, STARTUP_SOURCES));
    sources.extend(source_paths(repo_root, DBB_FIRMWARE_SOURCES));
    sources.extend(source_paths(repo_root, DBB_FIRMWARE_UI_SOURCES));
    sources.extend(source_paths(repo_root, DRIVER_SOURCES));
    sources.extend(source_paths(repo_root, QTOUCH_SOURCES));
    sources.extend(source_paths(repo_root, SECURECHIP_SOURCES));
    sources.extend(source_paths(repo_root, PLATFORM_BITBOX02_PLUS_SOURCES));
    sources.extend(source_paths(repo_root, PLATFORM_BITBOX02_SOURCES));
    sources.extend(source_paths(repo_root, ASF4_DRIVERS_MIN_SOURCES));
    sources.extend(source_paths(repo_root, ASF4_DRIVERS_SOURCES));
    sources.extend(source_paths(repo_root, CRYPTOAUTHLIB_SOURCES));
    sources.extend(source_paths(repo_root, EMBEDDED_SWD_SOURCES));
    sources.extend(optiga_sources(repo_root)?);
    sources.push(repo_root.join("src/common_main.c"));
    sources.push(repo_root.join("src/firmware.c"));
    if variant == "firmware" {
        sources.extend(source_paths(repo_root, FIRMWARE_U2F_SOURCES));
        sources.extend(source_paths(repo_root, FIRMWARE_U2F_DRIVER_SOURCES));
    }

    for source in sources {
        build.file(source);
    }
    build.compile("bitbox02_firmware_c");
    Ok(())
}

fn include_dirs(repo_root: &Path, generated_headers_dir: &Path) -> Vec<PathBuf> {
    vec![
        repo_root.join("src"),
        repo_root.join("src/ui/ugui"),
        repo_root.join("src/platform"),
        repo_root.join("src/qtouch"),
        repo_root.join("src/usb/class"),
        repo_root.join("src/usb/class/hid"),
        repo_root.join("src/usb/class/hid/hww"),
        repo_root.join("src/usb/class/hid/u2f"),
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
        repo_root.join("external/cryptoauthlib/lib"),
        repo_root.join("external"),
        repo_root.join("external/embedded-swd"),
        repo_root.join("external/optiga-trust-m/config"),
        repo_root.join("external/optiga-trust-m/include"),
        repo_root.join("external/optiga-trust-m/include/cmd"),
        repo_root.join("external/optiga-trust-m/include/common"),
        repo_root.join("external/optiga-trust-m/include/ifx_i2c"),
        repo_root.join("external/optiga-trust-m/include/pal"),
        repo_root.join("external/optiga-trust-m/include/comms"),
        repo_root.join("external/optiga-trust-m/external/mbedtls/include"),
        repo_root.join("src/rust/fatfs-sys/depend/fatfs/source"),
        generated_headers_dir.to_path_buf(),
    ]
}

fn source_paths(repo_root: &Path, rel_paths: &[&str]) -> Vec<PathBuf> {
    rel_paths.iter().map(|path| repo_root.join(path)).collect()
}

fn optiga_sources(repo_root: &Path) -> BuildResult<Vec<PathBuf>> {
    let mut result = Vec::new();
    for dir in [
        "external/optiga-trust-m/src/cmd",
        "external/optiga-trust-m/src/common",
        "external/optiga-trust-m/external/mbedtls/library",
        "external/optiga-trust-m/src/comms/ifx_i2c",
        "external/optiga-trust-m/src/crypt",
        "external/optiga-trust-m/src/util",
    ] {
        result.extend(list_c_files(&repo_root.join(dir))?);
    }
    result.push(repo_root.join("external/optiga-trust-m/src/comms/optiga_comms_ifx_i2c.c"));
    result.push(repo_root.join("external/optiga-trust-m/extras/pal/pal_crypt_mbedtls.c"));
    Ok(result)
}

fn list_c_files(dir: &Path) -> BuildResult<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)
        .map_err(|err| format!("failed to read directory {}: {err}", dir.display()))?
    {
        let entry =
            entry.map_err(|err| format!("failed to read entry in {}: {err}", dir.display()))?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("c")) {
            result.push(path);
        }
    }
    result.sort();
    Ok(result)
}

fn emit_linker_args(out_dir: &Path, repo_root: &Path, variant: &str) {
    println!("cargo::rustc-link-search={}", out_dir.display());
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR did not contain a Cargo profile directory");
    let fatfs_archive = find_native_archive(&profile_dir.join("build"), "fatfs-sys-", "libfatfs.a")
        .expect("failed to locate libfatfs.a produced by fatfs-sys");
    println!(
        "cargo::rustc-link-arg=-Wl,-Map={}",
        profile_dir.join(format!("{variant}.map")).display()
    );
    let qtouch_dir = repo_root.join("external/asf4-drivers/qtouch/lib/gcc");
    let firmware_archive = out_dir.join("libbitbox02_firmware_c.a");
    emit_rerun_if_changed_path(&firmware_archive);
    println!("cargo::rustc-link-arg=-Wl,--start-group");
    println!("cargo::rustc-link-arg={}", firmware_archive.display());
    emit_rerun_if_changed_path(&fatfs_archive);
    println!("cargo::rustc-link-arg={}", fatfs_archive.display());
    for lib in [
        "qtm_acq_samd51_0x000f",
        "qtm_binding_layer_cm4_0x0005",
        "qtm_touch_key_cm4_0x0002",
    ] {
        let qtouch_lib = qtouch_dir.join(format!("lib{lib}.a"));
        emit_rerun_if_changed_path(&qtouch_lib);
        println!("cargo::rustc-link-arg={}", qtouch_lib.display());
    }
    println!("cargo::rustc-link-arg=-lc");
    println!("cargo::rustc-link-arg=-lnosys");
    println!("cargo::rustc-link-arg=-lm");
    println!("cargo::rustc-link-arg=-lgcc");
    println!("cargo::rustc-link-arg=-Wl,--end-group");
}

fn find_native_archive(
    build_dir: &Path,
    package_prefix: &str,
    archive_name: &str,
) -> BuildResult<PathBuf> {
    let entries = fs::read_dir(build_dir).map_err(|err| {
        format!(
            "failed to read Cargo build dir {}: {err}",
            build_dir.display()
        )
    })?;
    for entry in entries {
        let path = entry
            .map_err(|err| format!("failed to read entry in {}: {err}", build_dir.display()))?
            .path();
        let Some(dir_name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };
        if !dir_name.starts_with(package_prefix) {
            continue;
        }
        let archive_path = path.join("out").join(archive_name);
        if archive_path.is_file() {
            return Ok(archive_path);
        }
    }
    Err(format!(
        "could not find {archive_name} in any {}* build output under {}",
        package_prefix,
        build_dir.display()
    ))
}

fn copy_linker_script(manifest_dir: &Path, out_dir: &Path) -> BuildResult<()> {
    let src = manifest_dir.join("bitbox02-firmware.ld");
    let dst = out_dir.join("bitbox02-firmware.ld");
    emit_rerun_if_changed_path(&src);
    fs::copy(&src, &dst).map_err(|err| {
        format!(
            "failed to copy linker script {} -> {}: {err}",
            src.display(),
            dst.display()
        )
    })?;
    Ok(())
}

fn ensure_command_exists(command: &str) -> BuildResult<()> {
    match Command::new(command).arg("--version").output() {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            Err(format!("`{command}` was not found! Check your PATH!"))
        }
        Err(err) => Err(format!("failed to run `{command} --version`: {err}")),
    }
}

fn generate_rust_header(repo_root: &Path, rust_header_dir: &Path) -> BuildResult<()> {
    let script_path = repo_root.join("scripts/generate_rust_header.sh");
    let mut command = Command::new("bash");
    command.arg(&script_path).arg(rust_header_dir);
    if let Ok(cargo_bin) = env::var("CARGO") {
        command.env("CARGO_BIN", cargo_bin);
    }
    if let Ok(cbindgen_bin) = env::var("CBINDGEN_BIN") {
        command.env("CBINDGEN_BIN", cbindgen_bin);
    }
    command.current_dir(repo_root.join("src/rust"));
    run_command(&mut command, "generate rust.h")?;
    Ok(())
}

fn generate_version_headers(repo_root: &Path, out_dir: &Path) -> BuildResult<()> {
    let script_path = repo_root.join("scripts/generate_version_headers.py");
    run_command(
        Command::new("python3")
            .arg(&script_path)
            .arg("generate")
            .arg("--repo-root")
            .arg(repo_root)
            .arg("--output-dir")
            .arg(out_dir),
        "generate version headers",
    )?;
    Ok(())
}

fn run_command(command: &mut Command, context: &str) -> BuildResult<Output> {
    let output = command
        .output()
        .map_err(|err| format!("failed to {context}: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "{context} failed\nstdout:\n{}\n\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output)
}

fn emit_rerun_if_changed_path(path: &Path) {
    println!("cargo::rerun-if-changed={}", path.display());
}
