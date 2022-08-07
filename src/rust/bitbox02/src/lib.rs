// Copyright 2019 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
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

// This crate contains safe wrappers around C functions provided by bitbox02_sys.
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;
use alloc::string::String;

#[cfg(feature = "testing")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
pub mod app_btc;
#[cfg(feature = "testing")]
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
pub mod app_btc_sign_ui;
#[cfg(feature = "app-ethereum")]
pub mod app_eth;
pub mod backup;
pub mod commander;
pub mod input;
pub mod keystore;
pub mod memory;
pub mod random;
pub mod sd;
pub mod secp256k1;
pub mod securechip;
pub mod ui;

use core::time::Duration;

pub use bitbox02_sys::{
    Request_btc_pub_tag, Request_btc_sign_init_tag, Request_btc_sign_input_tag,
    Request_btc_sign_output_tag, Request_btc_tag, Request_cardano_tag, Request_check_backup_tag,
    Request_check_sdcard_tag, Request_create_backup_tag, Request_device_info_tag,
    Request_device_language_tag, Request_device_name_tag, Request_electrum_encryption_key_tag,
    Request_eth_tag, Request_fingerprint_tag, Request_insert_remove_sdcard_tag,
    Request_list_backups_tag, Request_perform_attestation_tag, Request_reboot_tag,
    Request_reset_tag, Request_restore_backup_tag, Request_restore_from_mnemonic_tag,
    Request_set_mnemonic_passphrase_enabled_tag, Request_set_password_tag,
    Request_show_mnemonic_tag,
};

pub use bitbox02_sys::font_monogram_5X9;

pub use bitbox02_sys::confirm_params_t;

pub use bitbox02_sys::buffer_t;

#[macro_use]
pub mod util;

// ug_put_string displays a debug message on the screen for 3 sec.
pub fn ug_put_string(x: i16, y: i16, input: &str, inverted: bool) {
    match str_to_cstr!(input, 128) {
        Ok(buf) => unsafe {
            bitbox02_sys::UG_PutString(x, y, buf.as_ptr() as *const _, inverted);
        },
        Err(msg) => screen_print_debug(msg, 3000),
    }
}

pub fn ug_clear_buffer() {
    unsafe { bitbox02_sys::UG_ClearBuffer() }
}

pub fn ug_send_buffer() {
    unsafe { bitbox02_sys::UG_SendBuffer() }
}

pub fn ug_font_select_9x9() {
    unsafe { bitbox02_sys::UG_FontSelect(&bitbox02_sys::font_font_a_9X9) }
}

pub fn ug_font_select_11x10() {
    unsafe { bitbox02_sys::UG_FontSelect(&bitbox02_sys::font_font_a_11X10) }
}

#[cfg_attr(not(target_arch = "arm"), allow(unused_variables))]
pub fn delay(duration: Duration) {
    #[cfg(target_arch = "arm")]
    {
        if duration < Duration::from_micros(1) {
            unsafe {
                // Sleep the smallest unit of sleep we support
                bitbox02_sys::delay_us(1)
            }
        } else if duration < Duration::from_millis(1) {
            unsafe {
                bitbox02_sys::delay_us(duration.as_micros() as u16);
            }
        } else {
            unsafe {
                bitbox02_sys::delay_ms(duration.as_millis() as u16);
            }
        }
    }
}

pub fn screen_print_debug(msg: &str, duration: i32) {
    match str_to_cstr!(msg, 200) {
        Ok(cstr) => unsafe {
            bitbox02_sys::screen_print_debug(cstr.as_ptr() as *const _, duration)
        },
        Err(errmsg) => unsafe {
            bitbox02_sys::screen_print_debug(
                str_to_cstr_force!(errmsg, 200).as_ptr() as *const _,
                duration,
            )
        },
    }
}

pub fn sha256(input: &[u8], output: &mut [u8]) -> Result<(), ()> {
    let res = unsafe {
        bitbox02_sys::wally_sha256(
            input.as_ptr(),
            input.len() as _,
            output.as_mut_ptr(),
            output.len() as _,
        )
    };
    if res == bitbox02_sys::WALLY_OK as i32 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn reset(status: bool) {
    unsafe { bitbox02_sys::reset_reset(status) }
}

#[cfg(not(feature = "testing"))]
pub fn format_datetime(timestamp: u32, timezone_offset: i32, date_only: bool) -> String {
    let mut out = [0u8; 100];
    unsafe {
        bitbox02_sys::util_format_datetime(
            timestamp,
            timezone_offset,
            date_only,
            out.as_mut_ptr(),
            out.len() as _,
        )
    }
    crate::util::str_from_null_terminated(&out[..])
        .unwrap()
        .into()
}

#[cfg(feature = "testing")]
pub fn format_datetime(_timestamp: u32, _timezone_offset: i32, date_only: bool) -> String {
    if date_only {
        "<date>".into()
    } else {
        "<datetime>".into()
    }
}

#[cfg(not(feature = "testing"))]
pub fn version_short() -> &'static str {
    let s = unsafe {
        let ptr = bitbox02_sys::util_version_short();
        let len = crate::util::strlen_ptr(ptr);
        core::slice::from_raw_parts(ptr, len as _)
    };
    core::str::from_utf8(s).unwrap()
}

#[cfg(feature = "testing")]
pub fn version_short() -> &'static str {
    "9.2.0-testing"
}

#[cfg(not(feature = "testing"))]
pub fn reboot() -> ! {
    unsafe { bitbox02_sys::reboot() }
    loop {}
}

#[cfg(feature = "testing")]
pub fn reboot() -> ! {
    panic!("reboot called")
}
