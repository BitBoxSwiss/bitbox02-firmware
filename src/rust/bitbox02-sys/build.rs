// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ALLOWLIST_VARS: &[&str] = &[
    "BASE58_CHECKSUM_LEN",
    "BIP32_SERIALIZED_LEN",
    "BIP39_WORDLIST_LEN",
    "da14531_handler_current_product",
    "da14531_handler_current_product_len",
    "BITBOX02_FLASH_BOOT_LEN",
    "BITBOX02_FLASH_BOOT_START",
    "font_font_a_11X10",
    "font_font_a_9X9",
    "font_monogram_5X9",
    "font_password_11X12",
    "INPUT_STRING_MAX_SIZE",
    "MAX_LABEL_SIZE",
    "MAX_PK_SCRIPT_SIZE",
    "MAX_VARINT_SIZE",
    "MEMORY_MULTISIG_NUM_ENTRIES",
    "MEMORY_PLATFORM_BITBOX02_PLUS",
    "MEMORY_PLATFORM_BITBOX02",
    "MEMORY_SECURECHIP_TYPE_ATECC",
    "MEMORY_SECURECHIP_TYPE_OPTIGA",
    "SCREEN_HEIGHT",
    "SCREEN_WIDTH",
    "secfalse_u8",
    "SD_MAX_FILE_SIZE",
    "SLIDER_POSITION_TWO_THIRD",
    "USART_0_BUFFER_SIZE",
    "USB_REPORT_SIZE",
];

const ALLOWLIST_TYPES: &[&str] = &[
    "auto_enter_t",
    "buffer_t",
    "component_t",
    "confirm_params_t",
    "da14531_protocol_frame",
    "delay_t",
    "event_slider_data_t",
    "event_types",
    "RustByteQueue",
    "RustUsbReportQueue",
    "trinary_input_string_params_t",
    "UG_COLOR",
    "upside_down_t",
];

const OPAQUE_TYPES: &[&str] = &[
    "da14531_protocol_frame",
    "RustByteQueue",
    "RustUsbReportQueue",
];

const ALLOWLIST_FNS: &[&str] = &[
    "bip32_derive_xpub",
    "bitbox02_smarteeprom_init",
    "bitbox02_smarteeprom_get_unlock_attempts",
    "bitbox02_smarteeprom_increment_unlock_attempts",
    "bitbox02_smarteeprom_reset_unlock_attempts",
    "bitbox02_smarteeprom_init",
    "confirm_create",
    "confirm_swap_create",
    "confirm_transaction_address_create",
    "confirm_transaction_fee_create",
    "delay_cancel",
    "delay_init_ms",
    "delay_ms",
    "delay_us",
    "da14531_handler",
    "da14531_protocol_poll",
    "emit_event",
    "empty_create",
    "fake_memory_factoryreset",
    "fake_memory_nova",
    "hid_hww_read",
    "hid_hww_write_poll",
    "hid_u2f_read",
    "hid_u2f_write_poll",
    "hww_setup",
    "keystore_bip39_mnemonic_to_seed",
    "keystore_get_bip39_word",
    "label_create",
    "memory_add_noise_remote_static_pubkey",
    "memory_ble_enable",
    "memory_ble_enabled",
    "memory_bootloader_hash",
    "memory_bootloader_set_flags",
    "memory_check_noise_remote_static_pubkey",
    "memory_get_attestation_bootloader_hash",
    "memory_get_attestation_pubkey_and_certificate",
    "memory_get_authorization_key",
    "memory_get_ble_metadata",
    "memory_get_device_name",
    "memory_get_encrypted_seed_and_hmac",
    "memory_get_encryption_key",
    "memory_get_io_protection_key",
    "memory_get_noise_static_private_key",
    "memory_get_optiga_config_version",
    "memory_get_platform",
    "memory_get_salt_root",
    "memory_get_securechip_type",
    "memory_get_seed_birthdate",
    "memory_is_initialized",
    "memory_is_mnemonic_passphrase_enabled",
    "memory_is_seeded",
    "memory_multisig_get_by_hash",
    "memory_multisig_set_by_hash",
    "memory_reset_hww",
    "memory_set_ble_metadata",
    "memory_set_attestation_bootloader_hash",
    "memory_set_attestation_certificate",
    "memory_set_attestation_device_pubkey",
    "memory_set_device_name",
    "memory_set_encrypted_seed_and_hmac",
    "memory_set_initialized",
    "memory_set_mnemonic_passphrase_enabled",
    "memory_set_optiga_config_version",
    "memory_set_salt_root",
    "memory_set_bootloader_hash_fake",
    "memory_set_seed_birthdate",
    "memory_setup",
    "memory_spi_get_active_ble_firmware_version",
    "menu_create",
    "orientation_arrows_create",
    "platform_product",
    "printf",
    "progress_create",
    "progress_set",
    "random_32_bytes_mcu",
    "random_32_bytes",
    "random_fake_reset",
    "reboot_to_bootloader",
    "reboot",
    "reset_ble",
    "screen_clear",
    "screen_init",
    "screen_print_debug",
    "screen_process_waiting_switch_to_lockscreen",
    "screen_process_waiting_switch_to_logo",
    "screen_process",
    "screen_rotate",
    "screen_saver_disable",
    "screen_saver_enable",
    "screen_splash",
    "sd_card_inserted",
    "sd_erase_file_in_subdir",
    "sd_format",
    "sd_free_list",
    "sd_list_subdir",
    "sd_load_bin",
    "sd_write_bin",
    "sdcard_create",
    "smarteeprom_bb02_config",
    "smarteeprom_disable",
    "smarteeprom_is_enabled",
    "spi_mem_full_erase",
    "spi_mem_protected_area_write",
    "status_create",
    "trinary_choice_create",
    "trinary_input_string_create",
    "trinary_input_string_set_input",
    "unlock_animation_play",
    "u2f_packet_init",
    "u2f_packet_process",
    "u2f_packet_timeout_get",
    "u2f_packet_timeout",
    "u2f_device_setup",
    "u2f_process",
    "uart_poll",
    "UG_ClearBuffer",
    "UG_FontSelect",
    "UG_PutString",
    "UG_SendBuffer",
    "ui_screen_stack_pop_all",
    "ui_screen_stack_pop",
    "ui_screen_stack_push",
    "unlock_animation_create",
    "usb_packet_process",
    "usb_processing_hww",
    "usb_processing_init",
    "usb_processing_init_u2f",
    "usb_processing_locked",
    "usb_processing_process",
    "usb_processing_timeout_reset",
    "usb_processing_u2f",
    "usb_processing_unlock",
    "usb_start",
    "util_format_datetime",
];

const RUSTIFIED_ENUMS: &[&str] = &[
    "event_types",
    "keystore_secp256k1_pubkey_format",
    "memory_optiga_config_version_t",
    "memory_password_stretch_algo_t",
    "memory_result_t",
    "multisig_script_type_t",
    "output_type_t",
    "simple_type_t",
    "trinary_choice_t",
];

// BITBOX02_SOURCES are only used for native builds (simulator). Avoid cross-target specific files.
const BITBOX02_SOURCES: &[&str] = &[
    "src/da14531/da14531_handler.c",
    "src/da14531/da14531_protocol.c",
    "src/da14531/da14531.c",
    "src/hardfault.c",
    "src/hww.c",
    "src/i2c_ecc.c",
    "src/memory/bitbox02_smarteeprom.c",
    "src/memory/memory_shared.c",
    "src/memory/memory_spi.c",
    "src/memory/memory.c",
    "src/platform/platform_init.c",
    "src/random.c",
    "src/reset.c",
    "src/screen.c",
    "src/sd.c",
    "src/system.c",
    "src/touch/gestures.c",
    "src/u2f.c",
    "src/u2f/u2f_app.c",
    "src/u2f/u2f_packet.c",
    "src/ui/components/button.c",
    "src/ui/components/confirm_gesture.c",
    "src/ui/components/confirm_transaction.c",
    "src/ui/components/confirm.c",
    "src/ui/components/confirm_swap.c",
    "src/ui/components/empty.c",
    "src/ui/components/icon_button.c",
    "src/ui/components/image.c",
    "src/ui/components/info_centered.c",
    "src/ui/components/keyboard_switch.c",
    "src/ui/components/knight_rider.c",
    "src/ui/components/label.c",
    "src/ui/components/left_arrow.c",
    "src/ui/components/lockscreen.c",
    "src/ui/components/unlock_animation.c",
    "src/ui/components/menu.c",
    "src/ui/components/orientation_arrows.c",
    "src/ui/components/progress.c",
    "src/ui/components/right_arrow.c",
    "src/ui/components/screensaver.c",
    "src/ui/components/sdcard.c",
    "src/ui/components/status.c",
    "src/ui/components/trinary_choice.c",
    "src/ui/components/trinary_input_char.c",
    "src/ui/components/trinary_input_string.c",
    "src/ui/components/ui_images.c",
    "src/ui/components/waiting.c",
    "src/ui/event_handler.c",
    "src/ui/fonts/font_a_11X10.c",
    "src/ui/fonts/font_a_11X12.c",
    "src/ui/fonts/font_a_13X14.c",
    "src/ui/fonts/font_a_15X16.c",
    "src/ui/fonts/font_a_17X18.c",
    "src/ui/fonts/font_a_9X9.c",
    "src/ui/fonts/monogram_5X9.c",
    "src/ui/fonts/password_11X12.c",
    "src/ui/fonts/password_9X9.c",
    "src/ui/graphics/graphics.c",
    "src/ui/oled/sh1107.c",
    "src/ui/oled/ssd1312.c",
    "src/ui/screen_process.c",
    "src/ui/screen_saver.c",
    "src/ui/screen_stack.c",
    "src/ui/ugui/ugui.c",
    "src/ui/ui_util.c",
    "src/usb/usb_frame.c",
    "src/usb/usb_packet.c",
    "src/usb/usb_processing.c",
    "src/usb/usb.c",
    "src/util.c",
    "external/asf4-drivers/hal/utils/src/utils_ringbuffer.c",
];

const FAKEHARDWARE_SOURCES: &[&str] = &[
    "test/hardware-fakes/src/fake_component.c",
    "test/hardware-fakes/src/fake_diskio.c",
    "test/hardware-fakes/src/fake_memory.c",
    "test/hardware-fakes/src/fake_qtouch.c",
    "test/hardware-fakes/src/fake_screen.c",
    "test/hardware-fakes/src/fake_smarteeprom.c",
    "test/hardware-fakes/src/fake_spi_mem.c",
];

type BuildResult<T> = Result<T, String>;

pub fn main() -> BuildResult<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .join("../../..")
        .canonicalize()
        .map_err(|err| format!("failed to find repo root: {err}"))?;

    emit_rerun_if_changed("wrapper.h");
    emit_rerun_if_changed("../bitbox02-cbindgen.toml");
    emit_rerun_if_changed("../bitbox02-rust-c/Cargo.toml");
    emit_rerun_if_changed("../bitbox02-rust-c/src");
    emit_rerun_if_changed("../bitbox02-rust/Cargo.toml");
    emit_rerun_if_changed("../bitbox02-rust/src");
    emit_rerun_if_changed("../bitbox02/Cargo.toml");
    emit_rerun_if_changed("../bitbox02/src");
    emit_rerun_if_changed("../util/Cargo.toml");
    emit_rerun_if_changed("../util/src");
    emit_rerun_if_changed("../bitbox-aes/Cargo.toml");
    emit_rerun_if_changed("../bitbox-aes/src");
    emit_rerun_if_changed("../bitbox-framed-serial-link/Cargo.toml");
    emit_rerun_if_changed("../bitbox-framed-serial-link/src");
    emit_rerun_if_changed("../bitbox-bytequeue/Cargo.toml");
    emit_rerun_if_changed("../bitbox-bytequeue/src");
    emit_rerun_if_changed("../../../versions.json");
    emit_rerun_if_changed("../../../src/version.h.tmpl");
    emit_rerun_if_changed("../../../src/bootloader/bootloader_version.h.tmpl");
    emit_rerun_if_changed("../../../scripts/generate_version_headers.py");
    emit_rerun_if_changed("../../../scripts/generate_rust_header.sh");

    // Generating version.h/bootloader_version.h depends on the current state of the git repo
    emit_git_rerun_if_changed(&repo_root);

    ensure_command_exists("bindgen")?;

    let target = env::var("TARGET").expect("TARGET not set");
    let cross_compiling = target == "thumbv7em-none-eabi";

    let arm_sysroot = env::var("CMAKE_SYSROOT").unwrap_or("/usr/local/arm-none-eabi".to_string());
    let arm_sysroot = format!("--sysroot={arm_sysroot}");

    let mut extra_flags = if cross_compiling {
        vec![
            "-D__SAMD51J20A__",
            "--target=thumbv7em-none-eabi",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-mfloat-abi=soft",
            &arm_sysroot,
            "-fshort-enums",
        ]
    } else {
        vec!["-DTESTING", "-D_UNIT_TEST_", "-DPRODUCT_BITBOX_MULTI=1"]
    };

    // If user enables -Dwarnings for rust we also want to enable -Werror for C.
    if let Ok(rustflags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        for flag in rustflags.split('\x1f') {
            if flag == "-Dwarnings" {
                extra_flags.push("-Werror");
            }
        }
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    let mut includes = vec![
        // $INCLUDES
        "../..".to_owned(),
        "../../ui/ugui".to_owned(),
        "../../platform".to_owned(),
        "../../qtouch".to_owned(),
        "../../usb/class".to_owned(),
        "../../usb/class/hid".to_owned(),
        "../../usb/class/hid/hww".to_owned(),
        "../../usb/class/hid/u2f".to_owned(),
        // ASF4 headers allowed in unit tests
        "../../../external/asf4-drivers/hal/utils/include".to_owned(),
        // fatfs
        "../../rust/fatfs-sys/depend/fatfs/source".to_owned(),
    ];

    let generated_headers_dir = if cross_compiling {
        env::var("CMAKE_CURRENT_BINARY_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| out_dir.join("../../../../../.."))
    } else {
        ensure_command_exists("cbindgen")?;
        generate_native_headers(&repo_root, &out_dir)?;
        out_dir.clone()
    };
    includes.push(generated_headers_dir.display().to_string());

    if cross_compiling {
        includes.extend([
            // SAMD51A
            "../../../external/samd51a-ds/include".to_owned(),
            // ASF4-min
            "../../../external/asf4-drivers".to_owned(),
            "../../../external/asf4-drivers/Config".to_owned(),
            "../../../external/asf4-drivers/hal/include".to_owned(),
            "../../../external/asf4-drivers/hal/include".to_owned(),
            "../../../external/asf4-drivers/hpl/core".to_owned(),
            "../../../external/asf4-drivers/hpl/gclk".to_owned(),
            "../../../external/asf4-drivers/hpl/pm".to_owned(),
            "../../../external/asf4-drivers/hpl/port".to_owned(),
            "../../../external/asf4-drivers/hpl/pukcc".to_owned(),
            "../../../external/asf4-drivers/hpl/rtc".to_owned(),
            "../../../external/asf4-drivers/hpl/spi".to_owned(),
            "../../../external/asf4-drivers/hri".to_owned(),
            "../../../external/asf4-drivers/qtouch".to_owned(),
            "../../../external/asf4-drivers/qtouch/include".to_owned(),
            "../../../external/asf4-drivers/sd_mmc".to_owned(),
            "../../../external/asf4-drivers/usb".to_owned(),
            "../../../external/asf4-drivers/usb/class".to_owned(),
            "../../../external/asf4-drivers/usb/class/hid".to_owned(),
            "../../../external/asf4-drivers/usb/device".to_owned(),
            // ASF4
            "../../../external/asf4-drivers/diskio".to_owned(),
            // CMSIS
            "../../../external/CMSIS/Include".to_owned(),
        ]);
    } else {
        // unit test framework includes
        includes.push("../../../test/hardware-fakes/include".to_owned());
    }

    let out_path = out_dir.join("bindings.rs");
    let out_path = out_path.into_os_string().into_string().unwrap();

    // Needs to match the definitions in `CMakeList.txt' files (unit tests, hardware fakes and
    // simulator)
    let mut definitions = vec!["-DAPP_U2F=1"];
    definitions.extend(&extra_flags);

    run_command(
        Command::new("bindgen")
            .args(["--output", &out_path])
            .arg("--use-core")
            .arg("--with-derive-default")
            .args(
                ALLOWLIST_FNS
                    .iter()
                    .flat_map(|s| ["--allowlist-function", s]),
            )
            .args(ALLOWLIST_TYPES.iter().flat_map(|s| ["--allowlist-type", s]))
            .args(ALLOWLIST_VARS.iter().flat_map(|s| ["--allowlist-var", s]))
            .args(RUSTIFIED_ENUMS.iter().flat_map(|s| ["--rustified-enum", s]))
            .args(OPAQUE_TYPES.iter().flat_map(|s| ["--opaque-type", s]))
            .arg("wrapper.h")
            .arg("--")
            .args(&definitions)
            .args(includes.iter().map(|s| format!("-I{s}"))),
        "run bindgen",
    )?;

    let excludes = if let Ok(libtype) = env::var("LIB_TYPE") {
        match libtype.as_str() {
            "c-unit-tests" => vec!["src/screen.c"],
            _ => vec![],
        }
    } else {
        vec![]
    };

    // Build native C deps for host builds. Keep bitbox C and hardware fakes in one archive so
    // static archive ordering cannot drop fake providers before bitbox02 consumers.
    if !cross_compiling {
        let mdir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut builder = cc::Build::new();

        let bitbox02_files = BITBOX02_SOURCES
            .iter()
            .filter(|x| !excludes.contains(x))
            .map(|s| [&mdir, "../../..", s].join("/"));
        let fakehardware_files = FAKEHARDWARE_SOURCES
            .iter()
            .map(|s| [&mdir, "../../..", s].join("/"));
        let files: Vec<String> = bitbox02_files.chain(fakehardware_files).collect();

        builder.files(&files);
        for definition in &definitions {
            builder.flag(definition);
        }
        builder.includes(&includes);

        builder.compile("bitbox02");

        for file in &files {
            println!("cargo::rerun-if-changed={file}");
        }
    }

    Ok(())
}

fn emit_rerun_if_changed(path: &str) {
    println!("cargo::rerun-if-changed={path}");
}

fn emit_git_rerun_if_changed(repo_root: &Path) {
    let Some(git_dir) = git_output(repo_root, &["rev-parse", "--absolute-git-dir"]) else {
        return;
    };
    let git_dir = PathBuf::from(git_dir);

    for path in [
        git_dir.join("HEAD"),
        git_dir.join("index"),
        git_dir.join("packed-refs"),
        git_dir.join("refs/tags"),
    ] {
        if path.exists() {
            println!("cargo::rerun-if-changed={}", path.display());
        }
    }

    if let Some(head_ref) = git_output(repo_root, &["symbolic-ref", "-q", "HEAD"]) {
        let ref_path = git_dir.join(head_ref);
        if ref_path.exists() {
            println!("cargo::rerun-if-changed={}", ref_path.display());
        }
    }
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

fn generate_native_headers(repo_root: &Path, out_dir: &Path) -> BuildResult<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let script_path = repo_root.join("scripts/generate_rust_header.sh");
    let rust_header_dir = out_dir.join("rust");

    let mut command = Command::new("bash");
    command.arg(&script_path).arg(&rust_header_dir);
    if let Ok(cargo_bin) = env::var("CARGO") {
        command.env("CARGO_BIN", cargo_bin);
    }
    if let Ok(cbindgen_bin) = env::var("CBINDGEN_BIN") {
        command.env("CBINDGEN_BIN", cbindgen_bin);
    }
    command.current_dir(manifest_dir.join(".."));
    run_command(&mut command, "generate rust.h")?;

    generate_version_headers(repo_root, out_dir)
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

fn git_output(repo_root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if value.is_empty() { None } else { Some(value) }
}
