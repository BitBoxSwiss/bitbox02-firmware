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
    "EC_PUBLIC_KEY_UNCOMPRESSED_LEN",
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
    "keystore_bip85_bip39",
    "keystore_bip85_ln",
    "keystore_copy_seed",
    "keystore_create_and_store_seed",
    "keystore_encode_xpub_at_keypath",
    "keystore_encrypt_and_store_seed",
    "keystore_get_bip39_mnemonic",
    "keystore_get_bip39_word",
    "keystore_get_ed25519_seed",
    "keystore_is_locked",
    "keystore_lock",
    "keystore_mock_unlocked",
    "keystore_secp256k1_compressed_to_uncompressed",
    "keystore_secp256k1_get_private_key",
    "keystore_secp256k1_nonce_commit",
    "keystore_secp256k1_schnorr_bip86_pubkey",
    "keystore_secp256k1_schnorr_sign",
    "keystore_secp256k1_sign",
    "keystore_unlock",
    "keystore_unlock_bip39",
    "label_create",
    "localtime",
    "lock_animation_start",
    "lock_animation_stop",
    "memory_add_noise_remote_static_pubkey",
    "memory_bootloader_hash",
    "memory_check_noise_remote_static_pubkey",
    "memory_get_attestation_bootloader_hash",
    "memory_get_attestation_pubkey_and_certificate",
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
    "menu_create",
    "mock_memory_factoryreset",
    "spi_mem_full_erase",
    "printf",
    "progress_create",
    "progress_set",
    "random_32_bytes_mcu",
    "random_mock_reset",
    "reboot",
    "reset_reset",
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
    "smarteeprom_bb02_config",
    "status_create",
    "trinary_choice_create",
    "trinary_input_string_create",
    "trinary_input_string_set_input",
    "ui_screen_stack_pop",
    "ui_screen_stack_pop_all",
    "ui_screen_stack_push",
    "util_format_datetime",
    "wally_free_string",
    "wally_get_secp_context",
    "wally_hash160",
    "wally_sha512",
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
        // APP_ vars active when generating rust declarations from C headers.  It is okay to
        // activate all of them here - Rust's 'app-' features control usage/compilation.
        vec![
            "-D__SAMD51J20A__",
            "--target=thumbv7em-none-eabi",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-mfloat-abi=soft",
            &arm_sysroot,
            "-DAPP_U2F=1",
        ]
    } else {
        vec!["-DTESTING=1"]
    };

    let mut includes = vec![
        // $INCLUDES
        "../..",
        "../../platform",
        "../../qtouch",
        "../../usb/class",
        "../../usb/class/hid",
        "../../usb/class/hid/hww",
        "../../usb/class/hid/u2f",
        // $WALLY_INCLUDES
        "../../../external/libwally-core/include",
        // $SECP256k1_INCLUDES
        "../../../external/libwally-core/src/secp256k1/include",
    ];

    if cross_compiling {
        includes.extend([
            // SAMD51A
            "../../../external/samd51a-ds/include",
            // ASF4-min
            "../../../external/asf4-drivers",
            "../../../external/asf4-drivers/Config",
            "../../../external/asf4-drivers/hal/include",
            "../../../external/asf4-drivers/hal/include",
            "../../../external/asf4-drivers/hal/utils/include",
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
        includes.push("../../../test/unit-test/framework/includes")
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let out_path = out_path.into_os_string().into_string().unwrap();
    let res = Command::new("bindgen")
        .args(["--output", &out_path])
        .arg("--use-core")
        .arg("--with-derive-default")
        .args(["--ctypes-prefix", "util::c_types"])
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
        .arg("-DPB_NO_PACKED_STRUCTS=1")
        .arg("-DPB_FIELD_16BIT=1")
        .arg("-fshort-enums")
        .args(&extra_flags)
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
    Ok(())
}
