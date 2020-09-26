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

#[cfg(feature = "testing")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "testing")]
pub mod testing;

pub mod commander;
pub mod keystore;

#[cfg_attr(feature = "testing", path = "memory_stub.rs")]
pub mod memory;
pub mod password;
pub mod random;
pub mod securechip;
pub mod ui;

use core::time::Duration;

// Reexport the protobuf types
pub use bitbox02_sys::BitBoxBaseConfirmPairingRequest;
pub use bitbox02_sys::BitBoxBaseDisplayStatusRequest;
pub use bitbox02_sys::BitBoxBaseHeartbeatRequest;
pub use bitbox02_sys::BitBoxBaseRequest;
pub use bitbox02_sys::BitBoxBaseSetConfigRequest;

// Reexport as u16 since this is the correct type (bindgen will generate them as u32)
#[allow(non_upper_case_globals)]
pub const BitBoxBaseSetConfigRequest_ip_tag: u16 =
    bitbox02_sys::BitBoxBaseSetConfigRequest_ip_tag as u16;

#[allow(non_upper_case_globals)]
pub const BitBoxBaseRequest_heartbeat_tag: u16 =
    bitbox02_sys::BitBoxBaseRequest_heartbeat_tag as u16;
#[allow(non_upper_case_globals)]
pub const BitBoxBaseRequest_confirm_pairing_tag: u16 =
    bitbox02_sys::BitBoxBaseRequest_confirm_pairing_tag as u16;
#[allow(non_upper_case_globals)]
pub const BitBoxBaseRequest_display_status_tag: u16 =
    bitbox02_sys::BitBoxBaseRequest_display_status_tag as u16;
#[allow(non_upper_case_globals)]
pub const BitBoxBaseRequest_set_config_tag: u16 =
    bitbox02_sys::BitBoxBaseRequest_set_config_tag as u16;

pub use bitbox02_sys::{
    Request_bitboxbase_tag, Request_btc_pub_tag, Request_btc_sign_init_tag,
    Request_btc_sign_input_tag, Request_btc_sign_output_tag, Request_btc_tag,
    Request_check_backup_tag, Request_check_sdcard_tag, Request_create_backup_tag,
    Request_device_info_tag, Request_device_language_tag, Request_device_name_tag,
    Request_electrum_encryption_key_tag, Request_eth_tag, Request_fingerprint_tag,
    Request_insert_remove_sdcard_tag, Request_list_backups_tag, Request_perform_attestation_tag,
    Request_random_number_tag, Request_reboot_tag, Request_reset_tag, Request_restore_backup_tag,
    Request_restore_from_mnemonic_tag, Request_set_mnemonic_passphrase_enabled_tag,
    Request_set_password_tag, Request_show_mnemonic_tag,
};

// Use this for functions exported to "C"
#[allow(non_camel_case_types)]
pub type commander_error_t = bitbox02_sys::commander_error_t;

pub use bitbox02_sys::font_monogram_5X9;

pub use bitbox02_sys::confirm_params_t;

pub use bitbox02_sys::buffer_t;

#[macro_use]
pub mod util;

pub fn ug_put_string(x: i16, y: i16, input: &str, inverted: bool) {
    if let Ok(buf) = str_to_cstr!(input, 128) {
        unsafe { bitbox02_sys::UG_PutString(x, y, buf.as_ptr() as *const _, inverted) }
    } else {
        screen_print_debug("string didn't fit", 3000);
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
        Err(cstr) => unsafe {
            bitbox02_sys::screen_print_debug(cstr.as_ptr() as *const _, duration)
        },
    }
}

pub fn bitboxbase_watchdog_reset() {
    unsafe { bitbox02_sys::bitboxbase_watchdog_reset() }
}

pub fn leds_turn_small_led(led: i32, enabled: bool) {
    if led < 0 || led > 4 {
        panic!("Invalid led");
    }
    unsafe { bitbox02_sys::leds_turn_small_led(led, enabled) }
}

pub enum Color {
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
}

pub fn leds_turn_big_led(led: i32, color: Option<Color>) {
    if led < 0 || led > 2 {
        panic!("Invalid led");
    }
    let c = match color {
        None => bitbox02_sys::led_color_t_LED_COLOR_NONE,
        Some(c) => match c {
            Color::White => bitbox02_sys::led_color_t_LED_COLOR_WHITE,
            Color::Red => bitbox02_sys::led_color_t_LED_COLOR_RED,
            Color::Green => bitbox02_sys::led_color_t_LED_COLOR_GREEN,
            Color::Blue => bitbox02_sys::led_color_t_LED_COLOR_BLUE,
            Color::Yellow => bitbox02_sys::led_color_t_LED_COLOR_YELLOW,
            Color::Purple => bitbox02_sys::led_color_t_LED_COLOR_PURPLE,
            Color::Cyan => bitbox02_sys::led_color_t_LED_COLOR_CYAN,
        },
    };
    unsafe { bitbox02_sys::leds_turn_big_led(led, c) }
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

pub fn bitboxbase_screensaver_reset() {
    unsafe { bitbox02_sys::bitboxbase_screensaver_reset() }
}

#[cfg(not(feature = "testing"))]
pub fn reset(status: bool) {
    unsafe { bitbox02_sys::reset_reset(status) }
}

#[cfg(feature = "testing")]
pub fn reset(status: bool) {
    let data = crate::testing::DATA.0.borrow();
    data.reset.as_ref().unwrap()(status)
}

pub fn sdcard_inserted() -> bool {
    unsafe { bitbox02_sys::sd_card_inserted() }
}
