// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

const ALLOWLIST_VARS: &[&str] = &[
    "BASE58_CHECKSUM_LEN",
    "BIP32_SERIALIZED_LEN",
    "BIP39_WORDLIST_LEN",
    "font_font_a_11X10",
    "font_font_a_9X9",
    "font_monogram_5X9",
    "font_password_11X12",
    "INPUT_STRING_MAX_SIZE",
    "MAX_LABEL_SIZE",
    "MAX_PK_SCRIPT_SIZE",
    "MAX_VARINT_SIZE",
    "MEMORY_DEVICE_NAME_MAX_LEN",
    "MEMORY_MULTISIG_NUM_ENTRIES",
    "MEMORY_MULTISIG_NAME_MAX_LEN",
    "MEMORY_PLATFORM_BITBOX02_PLUS",
    "MEMORY_PLATFORM_BITBOX02",
    "MEMORY_SECURECHIP_TYPE_ATECC",
    "MEMORY_SECURECHIP_TYPE_OPTIGA",
    "MEMORY_SPI_BLE_FIRMWARE_1_ADDR",
    "MEMORY_SPI_BLE_FIRMWARE_2_ADDR",
    "MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE",
    "SCREEN_HEIGHT",
    "SCREEN_WIDTH",
    "secfalse_u8",
    "SD_MAX_FILE_SIZE",
    "SLIDER_POSITION_TWO_THIRD",
];

const ALLOWLIST_TYPES: &[&str] = &[
    "auto_enter_t",
    "buffer_t",
    "component_t",
    "confirm_params_t",
    "delay_t",
    "event_slider_data_t",
    "event_types",
    "ringbuffer",
    "securechip_error_t",
    "trinary_input_string_params_t",
    "UG_COLOR",
    "upside_down_t",
];

const ALLOWLIST_FNS: &[&str] = &[
    "bip32_derive_xpub",
    "bitbox02_smarteeprom_init",
    "bitbox02_smarteeprom_get_unlock_attempts",
    "bitbox02_smarteeprom_increment_unlock_attempts",
    "bitbox02_smarteeprom_reset_unlock_attempts",
    "bitbox02_smarteeprom_init",
    "confirm_create",
    "confirm_transaction_address_create",
    "confirm_transaction_fee_create",
    "delay_cancel",
    "delay_init_ms",
    "delay_is_elapsed",
    "delay_ms",
    "delay_us",
    "emit_event",
    "empty_create",
    "fake_memory_factoryreset",
    "fake_memory_nova",
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
    "printf",
    "progress_create",
    "progress_set",
    "queue_hww_queue",
    "queue_pull",
    "random_32_bytes_mcu",
    "random_32_bytes",
    "random_fake_reset",
    "reboot_to_bootloader",
    "reboot",
    "reset_ble",
    "ringbuffer_flush",
    "ringbuffer_get",
    "ringbuffer_init",
    "ringbuffer_num",
    "ringbuffer_put",
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
    "securechip_attestation_sign",
    "securechip_init_new_password",
    "securechip_kdf",
    "securechip_reset_keys",
    "securechip_model",
    "securechip_monotonic_increments_remaining",
    "securechip_stretch_password",
    "securechip_u2f_counter_set",
    "smarteeprom_bb02_config",
    "smarteeprom_disable",
    "smarteeprom_is_enabled",
    "spi_mem_full_erase",
    "spi_mem_protected_area_write",
    "status_create",
    "trinary_choice_create",
    "trinary_input_string_create",
    "trinary_input_string_set_input",
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
    "usb_processing_process",
    "usb_processing_timeout_reset",
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
    "securechip_error_t",
    "securechip_model_t",
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
    "src/queue.c",
    "src/random.c",
    "src/reset.c",
    "src/salt.c",
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

pub fn main() -> Result<(), &'static str> {
    // We could theoretically list every header file that we end up depending on, but that is hard
    // to maintain. So instead we just listen to changes on "wrapper.h" which is good enough.
    println!("cargo::rerun-if-changed=wrapper.h");

    // Check if we have `bindgen` executable
    if let Err(e) = Command::new("bindgen").spawn() {
        if e.kind() == ErrorKind::NotFound {
            return Err("`bindgen` was not found! Check your PATH!");
        }
    }

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

    let mut includes = vec![
        // $INCLUDES
        "../..",
        "../../ui/ugui",
        "../../platform",
        "../../qtouch",
        "../../usb/class",
        "../../usb/class/hid",
        "../../usb/class/hid/hww",
        "../../usb/class/hid/u2f",
        // ASF4 headers allowed in unit tests
        "../../../external/asf4-drivers/hal/utils/include",
        // fatfs
        "../../rust/fatfs-sys/depend/fatfs/source",
    ];

    // rust.h is created by cbindgen in the cmake build directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let rust_h_dir = PathBuf::from([&out_dir, "../../../../../.."].join("/"));
    println!("rust_h_dir: {:?}", rust_h_dir.canonicalize());
    includes.push(rust_h_dir.as_os_str().to_str().unwrap());

    if cross_compiling {
        includes.extend([
            // SAMD51A
            "../../../external/samd51a-ds/include",
            // ASF4-min
            "../../../external/asf4-drivers",
            "../../../external/asf4-drivers/Config",
            "../../../external/asf4-drivers/hal/include",
            "../../../external/asf4-drivers/hal/include",
            "../../../external/asf4-drivers/hpl/core",
            "../../../external/asf4-drivers/hpl/gclk",
            "../../../external/asf4-drivers/hpl/pm",
            "../../../external/asf4-drivers/hpl/port",
            "../../../external/asf4-drivers/hpl/pukcc",
            "../../../external/asf4-drivers/hpl/rtc",
            "../../../external/asf4-drivers/hpl/spi",
            "../../../external/asf4-drivers/hri",
            "../../../external/asf4-drivers/qtouch",
            "../../../external/asf4-drivers/qtouch/include",
            "../../../external/asf4-drivers/sd_mmc",
            "../../../external/asf4-drivers/usb",
            "../../../external/asf4-drivers/usb/class",
            "../../../external/asf4-drivers/usb/class/hid",
            "../../../external/asf4-drivers/usb/device",
            // ASF4
            "../../../external/asf4-drivers/diskio",
            // CMSIS
            "../../../external/CMSIS/Include",
        ]);
    } else {
        // unit test framework includes
        includes.push("../../../test/hardware-fakes/include");
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let out_path = out_path.into_os_string().into_string().unwrap();

    // Needs to match the definitions in `CMakeList.txt' files (unit tests, hardware fakes and
    // simulator)
    let mut definitions = vec![
        "-DPB_NO_PACKED_STRUCTS=1",
        "-DPB_FIELD_16BIT=1",
        "-DAPP_BTC=1",
        "-DAPP_LTC=1",
        "-DAPP_U2F=1",
        "-DAPP_ETH=1",
    ];
    definitions.extend(&extra_flags);

    let res = Command::new("bindgen")
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
        .arg("wrapper.h")
        .arg("--")
        .args(&definitions)
        .args(includes.iter().map(|s| format!("-I{s}")))
        .output()
        .expect("Failed to run bindgen");
    if !res.status.success() {
        println!(
            "bindgen-out:\n{}\n\nbindgen-err:\n{}",
            std::str::from_utf8(&res.stdout).unwrap(),
            std::str::from_utf8(&res.stderr).unwrap()
        );
        return Err("Bindgen failed");
    }

    // For the c unit tests and c simulator we build a library that mocks/fakes real user behavior
    // and logs to stdout
    // For the rust simulator (with gui) we build a library that behaves more like the real
    // hardware since we have graphical inputs and outputs.
    let excludes = if let Ok(libtype) = env::var("LIB_TYPE") {
        match libtype.as_str() {
            "c-unit-tests" => vec!["src/screen.c"],
            _ => vec![],
        }
    } else {
        vec![]
    };

    let source_includes = &[
        "test/hardware-fakes/src/fake_component.c",
        "test/hardware-fakes/src/fake_diskio.c",
        "test/hardware-fakes/src/fake_memory.c",
        "test/hardware-fakes/src/fake_qtouch.c",
        "test/hardware-fakes/src/fake_screen.c",
        "test/hardware-fakes/src/fake_securechip.c",
        "test/hardware-fakes/src/fake_smarteeprom.c",
        "test/hardware-fakes/src/fake_spi_mem.c",
    ];

    // Build the c deps for unit tests
    if !cross_compiling {
        let mdir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut builder = cc::Build::new();

        let files: Vec<String> = BITBOX02_SOURCES
            .iter()
            .chain(source_includes.iter())
            .filter(|x| !excludes.contains(x))
            .map(|s| [&mdir, "../../..", s].join("/"))
            .collect();

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
