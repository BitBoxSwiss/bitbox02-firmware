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

use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::io::ErrorKind;

pub fn main() -> Result<(), &'static str> {
    // We could theoretically list every header file that we end up depending on, but that is hard
    // to maintain. So instead we just listen to changes on "wrapper.h" which is good enough.
    println!("cargo::rerun-if-changed=wrapper.h");

    // Check if we have `bindgen` executable
    if let Err(e) = Command::new("bindgen").spawn() {
        if e.kind() == ErrorKind::NotFound {
            return Err("`bindgen` was not found! Check your PATH!")
        }
    }

    let target = env::var("TARGET").expect("TARGET not set");
    let cross_compiling = target == "thumbv7em-none-eabi";

    let arm_sysroot = env::var("CMAKE_SYSROOT").unwrap_or("/usr/local/arm-none-eabi".to_string());
    let arm_sysroot = format!("--sysroot={arm_sysroot}");

    let extra_flags = if cross_compiling {
        // APP_ vars active when generating rust declarations from C headers.  It is okay to
        // activate all of them here - Rust's 'app-' features control usage/compilation.
        vec!["-D__SAMD51J20A__", "--target=thumbv7em-none-eabi", "-mcpu=cortex-m4", "-mthumb",
         "-mfloat-abi=soft", &arm_sysroot, "-DAPP_U2F=1"]
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
        .args(["--allowlist-function", "bip32_derive_xpub"])
        .args(["--allowlist-function", "localtime"])
        .args(["--allowlist-function", "wally_free_string"])
        .args(["--allowlist-function", "mock_memory_factoryreset"])
        .args(["--allowlist-function", "memory_setup"])
        .args(["--allowlist-function", "memory_is_initialized"])
        .args(["--allowlist-function", "memory_set_initialized"])
        .args(["--allowlist-function", "memory_is_seeded"])
        .args(["--allowlist-function", "memory_is_mnemonic_passphrase_enabled"])
        .args(["--allowlist-function", "memory_get_attestation_pubkey_and_certificate"])
        .args(["--allowlist-function", "memory_get_attestation_bootloader_hash"])
        .args(["--allowlist-function", "memory_bootloader_hash"])
        .args(["--allowlist-function", "memory_get_noise_static_private_key"])
        .args(["--allowlist-function", "memory_check_noise_remote_static_pubkey"])
        .args(["--allowlist-function", "memory_add_noise_remote_static_pubkey"])
        .args(["--allowlist-function", "memory_get_device_name"])
        .args(["--allowlist-function", "memory_set_device_name"])
        .args(["--allowlist-function", "memory_set_mnemonic_passphrase_enabled"])
        .args(["--allowlist-var", "MEMORY_MULTISIG_NAME_MAX_LEN"])
        .args(["--allowlist-function", "memory_set_seed_birthdate"])
        .args(["--allowlist-function", "memory_get_seed_birthdate"])
        .args(["--allowlist-function", "memory_multisig_get_by_hash"])
        .args(["--allowlist-function", "memory_multisig_set_by_hash"])
        .args(["--allowlist-function", "smarteeprom_bb02_config"])
        .args(["--allowlist-function", "bitbox02_smarteeprom_init"])
        .args(["--rustified-enum", "memory_result_t"])
        .args(["--allowlist-var", "MEMORY_DEVICE_NAME_MAX_LEN"])
        .args(["--allowlist-function", "securechip_attestation_sign"])
        .args(["--allowlist-function", "securechip_monotonic_increments_remaining"])
        .args(["--allowlist-function", "securechip_u2f_counter_set"])
        .args(["--allowlist-function", "securechip_model"])
        .args(["--rustified-enum", "securechip_model_t"])
        .args(["--allowlist-var", "KEYSTORE_MAX_SEED_LENGTH"])
        .args(["--allowlist-function", "keystore_is_locked"])
        .args(["--allowlist-function", "keystore_unlock"])
        .args(["--allowlist-function", "keystore_unlock_bip39"])
        .args(["--allowlist-function", "keystore_lock"])
        .args(["--allowlist-function", "keystore_create_and_store_seed"])
        .args(["--allowlist-function", "keystore_copy_seed"])
        .args(["--allowlist-function", "keystore_get_bip39_mnemonic"])
        .args(["--allowlist-function", "keystore_get_bip39_word"])
        .args(["--allowlist-function", "keystore_get_ed25519_seed"])
        .args(["--allowlist-function", "keystore_bip85_bip39"])
        .args(["--allowlist-function", "keystore_bip85_ln"])
        .args(["--allowlist-function", "keystore_secp256k1_compressed_to_uncompressed"])
        .args(["--allowlist-function", "keystore_secp256k1_nonce_commit"])
        .args(["--allowlist-function", "keystore_secp256k1_sign"])
        .args(["--allowlist-function", "keystore_secp256k1_schnorr_bip86_sign"])
        .args(["--allowlist-function", "keystore_bip39_mnemonic_to_seed"])
        .args(["--allowlist-function", "keystore_mock_unlocked"])
        .args(["--allowlist-var", "EC_PUBLIC_KEY_UNCOMPRESSED_LEN"])
        .args(["--allowlist-var", "EC_PUBLIC_KEY_LEN"])
        .args(["--allowlist-function", "keystore_encode_xpub_at_keypath"])
        .args(["--allowlist-function", "keystore_encrypt_and_store_seed"])
        .args(["--allowlist-var", "XPUB_ENCODED_LEN"])
        .args(["--allowlist-var", "BIP32_SERIALIZED_LEN"])
        .args(["--allowlist-function", "lock_animation_start"])
        .args(["--allowlist-function", "lock_animation_stop"])
        .args(["--allowlist-function", "delay_us"])
        .args(["--rustified-enum", "keystore_error_t"])
        .args(["--rustified-enum", "keystore_secp256k1_pubkey_format"])
        .args(["--allowlist-function", "keystore_secp256k1_schnorr_bip86_pubkey"])
        .args(["--allowlist-function", "util_format_datetime"])
        .args(["--allowlist-type", "buffer_t"])
        .args(["--allowlist-function", "delay_ms"])
        .args(["--allowlist-function", "UG_PutString"])
        .args(["--allowlist-function", "UG_FontSelect"])
        .args(["--allowlist-function", "UG_ClearBuffer"])
        .args(["--allowlist-function", "UG_SendBuffer"])
        .args(["--allowlist-function", "screen_print_debug"])
        .args(["--allowlist-function", "ui_screen_stack_push"])
        .args(["--allowlist-function", "ui_screen_stack_pop"])
        .args(["--allowlist-function", "ui_screen_stack_pop_all"])
        .args(["--allowlist-function", "screen_saver_disable"])
        .args(["--allowlist-function", "screen_saver_enable"])
        .args(["--allowlist-function", "screen_process"])
        .args(["--allowlist-function", "label_create"])
        .args(["--allowlist-function", "confirm_create"])
        .args(["--allowlist-function", "status_create"])
        .args(["--allowlist-function", "sdcard_create"])
        .args(["--allowlist-function", "menu_create"])
        .args(["--allowlist-function", "trinary_choice_create"])
        .args(["--rustified-enum", "trinary_choice_t"])
        .args(["--allowlist-var", "BASE58_CHECKSUM_LEN"])
        .args(["--allowlist-function", "random_32_bytes_mcu"])
        .args(["--allowlist-function", "random_mock_reset"])
        .args(["--allowlist-type", "component_t"])
        .args(["--allowlist-type", "confirm_params_t"])
        .args(["--allowlist-var", "MAX_LABEL_SIZE"])
        .args(["--allowlist-var", "font_font_a_9X9"])
        .args(["--allowlist-var", "font_font_a_11X10"])
        .args(["--allowlist-var", "font_monogram_5X9"])
        .args(["--allowlist-var", "font_password_11X12"])
        .args(["--allowlist-type", "trinary_input_string_params_t"])
        .args(["--allowlist-var", "INPUT_STRING_MAX_SIZE"])
        .args(["--allowlist-function", "trinary_input_string_create"])
        .args(["--allowlist-function", "trinary_input_string_set_input"])
        .args(["--allowlist-function", "confirm_transaction_address_create"])
        .args(["--allowlist-function", "confirm_transaction_fee_create"])
        .args(["--allowlist-function", "progress_create"])
        .args(["--allowlist-function", "progress_set"])
        .args(["--allowlist-function", "empty_create"])
        .args(["--allowlist-function", "reset_reset"])
        .args(["--allowlist-function", "sd_card_inserted"])
        .args(["--allowlist-function", "sd_format"])
        .args(["--allowlist-function", "sd_list_subdir"])
        .args(["--allowlist-function", "sd_erase_file_in_subdir"])
        .args(["--allowlist-function", "sd_load_bin"])
        .args(["--allowlist-function", "sd_write_bin"])
        .args(["--allowlist-var", "SD_MAX_FILE_SIZE"])
        .args(["--allowlist-function", "sd_free_list"])
        .args(["--allowlist-var", "BIP39_WORDLIST_LEN"])
        .args(["--rustified-enum", "simple_type_t"])
        .args(["--rustified-enum", "multisig_script_type_t"])
        .args(["--rustified-enum", "output_type_t"])
        .args(["--allowlist-var", "MAX_VARINT_SIZE"])
        .args(["--allowlist-var", "MAX_PK_SCRIPT_SIZE"])
        .args(["--allowlist-function", "reboot"])
        .args(["--allowlist-function", "secp256k1_ecdsa_anti_exfil_host_commit"])
        .args(["--allowlist-function", "wally_get_secp_context"])
        .args(["--allowlist-function", "wally_hash160"])
        .args(["--allowlist-function", "wally_sha512"])
        .args(["--allowlist-function", "printf"])
        .arg("wrapper.h")
        .arg("--")
        .arg("-DPB_NO_PACKED_STRUCTS=1")
        .arg("-DPB_FIELD_16BIT=1")
        .arg("-fshort-enums")
        .args(&extra_flags)
        .args(includes.iter().map(|s| format!("-I{s}")))
        .output()
        .expect("Failed to run bindgen");
    if ! res.status.success() {
        println!("bindgen-out:\n{}\n\nbindgen-err:\n{}", std::str::from_utf8(&res.stdout).unwrap(), std::str::from_utf8(&res.stderr).unwrap());
        return Err("Bindgen failed");
    }
    Ok(())
}
