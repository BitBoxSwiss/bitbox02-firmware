// Copyright 2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

const ALLOWLIST_VARS: &[&str] = &[
    "BASE58_CHECKSUM_LEN",
    "BIP32_SERIALIZED_LEN",
    "BIP39_WORDLIST_LEN",
    "EC_PUBLIC_KEY_LEN",
    "INPUT_STRING_MAX_SIZE",
    "KEYSTORE_MAX_SEED_LENGTH",
    "MAX_LABEL_SIZE",
    "MAX_PK_SCRIPT_SIZE",
    "MAX_VARINT_SIZE",
    "MEMORY_DEVICE_NAME_MAX_LEN",
    "MEMORY_MULTISIG_NAME_MAX_LEN",
    "SD_MAX_FILE_SIZE",
    "XPUB_ENCODED_LEN",
    "font_font_a_11X10",
    "font_font_a_9X9",
    "font_monogram_5X9",
    "font_password_11X12",
    "MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE",
    "MEMORY_SPI_BLE_FIRMWARE_1_ADDR",
    "MEMORY_SPI_BLE_FIRMWARE_2_ADDR",
    "MEMORY_PLATFORM_BITBOX02",
    "MEMORY_PLATFORM_BITBOX02_PLUS",
    "MEMORY_SECURECHIP_TYPE_ATECC",
    "MEMORY_SECURECHIP_TYPE_OPTIGA",
    "MAX_UNLOCK_ATTEMPTS",
];

const ALLOWLIST_TYPES: &[&str] = &[
    "buffer_t",
    "component_t",
    "confirm_params_t",
    "trinary_input_string_params_t",
];

const ALLOWLIST_FNS: &[&str] = &[
    "UG_ClearBuffer",
    "UG_FontSelect",
    "UG_PutString",
    "UG_SendBuffer",
    "bip32_derive_xpub",
    "bitbox02_smarteeprom_init",
    "bitbox_secp256k1_dleq_prove",
    "bitbox_secp256k1_dleq_verify",
    "confirm_create",
    "confirm_transaction_address_create",
    "confirm_transaction_fee_create",
    "delay_ms",
    "delay_us",
    "empty_create",
    "keystore_bip39_mnemonic_to_seed",
    "keystore_copy_seed",
    "keystore_copy_bip39_seed",
    "keystore_create_and_store_seed",
    "keystore_encrypt_and_store_seed",
    "keystore_get_bip39_word",
    "keystore_is_locked",
    "keystore_lock",
    "keystore_mock_unlocked",
    "keystore_secp256k1_nonce_commit",
    "keystore_secp256k1_sign",
    "keystore_unlock",
    "keystore_unlock_bip39_check",
    "keystore_unlock_bip39_finalize",
    "keystore_test_get_retained_seed_encrypted",
    "keystore_test_get_retained_bip39_seed_encrypted",
    "label_create",
    "localtime",
    "lock_animation_start",
    "lock_animation_stop",
    "memory_set_salt_root",
    "memory_add_noise_remote_static_pubkey",
    "memory_bootloader_hash",
    "memory_check_noise_remote_static_pubkey",
    "memory_get_attestation_bootloader_hash",
    "memory_get_attestation_pubkey_and_certificate",
    "memory_get_encrypted_seed_and_hmac",
    "memory_get_device_name",
    "memory_get_noise_static_private_key",
    "memory_get_seed_birthdate",
    "memory_is_initialized",
    "memory_is_mnemonic_passphrase_enabled",
    "memory_is_seeded",
    "memory_multisig_get_by_hash",
    "memory_multisig_set_by_hash",
    "memory_set_device_name",
    "memory_set_initialized",
    "memory_set_mnemonic_passphrase_enabled",
    "memory_set_seed_birthdate",
    "memory_setup",
    "memory_ble_enabled",
    "memory_ble_enable",
    "memory_get_ble_metadata",
    "memory_set_ble_metadata",
    "memory_get_platform",
    "memory_get_securechip_type",
    "memory_spi_get_active_ble_firmware_version",
    "spi_mem_protected_area_write",
    "menu_create",
    "fake_memory_factoryreset",
    "spi_mem_full_erase",
    "printf",
    "progress_create",
    "progress_set",
    "random_32_bytes_mcu",
    "random_32_bytes",
    "random_fake_reset",
    "reboot_to_bootloader",
    "reset_reset",
    "reset_ble",
    "screen_print_debug",
    "screen_process",
    "screen_saver_disable",
    "screen_saver_enable",
    "sd_card_inserted",
    "sd_erase_file_in_subdir",
    "sd_format",
    "sd_free_list",
    "sd_list_subdir",
    "sd_load_bin",
    "sd_write_bin",
    "sdcard_create",
    "secp256k1_ecdsa_anti_exfil_host_commit",
    "securechip_attestation_sign",
    "securechip_model",
    "securechip_monotonic_increments_remaining",
    "securechip_u2f_counter_set",
    "fake_securechip_event_counter",
    "fake_securechip_event_counter_reset",
    "smarteeprom_is_enabled",
    "smarteeprom_disable",
    "smarteeprom_bb02_config",
    "status_create",
    "trinary_choice_create",
    "trinary_input_string_create",
    "trinary_input_string_set_input",
    "ui_screen_stack_pop",
    "ui_screen_stack_pop_all",
    "ui_screen_stack_push",
    "util_format_datetime",
    "communication_mode_ble_enabled",
];

const RUSTIFIED_ENUMS: &[&str] = &[
    "keystore_error_t",
    "keystore_secp256k1_pubkey_format",
    "memory_result_t",
    "multisig_script_type_t",
    "output_type_t",
    "securechip_model_t",
    "simple_type_t",
    "trinary_choice_t",
];

// BITBOX02_SOURCES are only used for native builds (simulator). Avoid cross-target specific files.
const BITBOX02_SOURCES: &[&str] = &[
    "src/cipher/cipher.c",
    "src/communication_mode.c",
    "src/da14531/crc.c",
    "src/da14531/da14531_handler.c",
    "src/da14531/da14531_protocol.c",
    "src/da14531/da14531.c",
    "src/hardfault.c",
    "src/hww.c",
    "src/i2c_ecc.c",
    "src/keystore.c",
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
    "src/ui/components/confirm_button.c",
    "src/ui/components/confirm_gesture.c",
    "src/ui/components/confirm_transaction.c",
    "src/ui/components/confirm.c",
    "src/ui/components/empty.c",
    "src/ui/components/entry_screen.c",
    "src/ui/components/icon_button.c",
    "src/ui/components/image.c",
    "src/ui/components/info_centered.c",
    "src/ui/components/keyboard_switch.c",
    "src/ui/components/knight_rider.c",
    "src/ui/components/label.c",
    "src/ui/components/left_arrow.c",
    "src/ui/components/lockscreen.c",
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
    "src/ui/graphics/lock_animation.c",
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
    "src/workflow/orientation_screen.c",
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

    let extra_flags = if cross_compiling {
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
        // $SECP256k1_INCLUDES
        "../../../external/secp256k1-zkp/include",
        // ASF4 headers allowed in unit tests
        "../../../external/asf4-drivers/hal/utils/include",
        // fatfs
        "../../../external/fatfs/source",
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
        "test/hardware-fakes/src/fake_cipher.c",
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
