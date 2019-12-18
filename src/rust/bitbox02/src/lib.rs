// Copyright 2019 Shift Cryptosecurity AG
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

use bitbox02_sys::{self, delay_ms, delay_us};
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

// Use this for functions exported to "C"
#[allow(non_camel_case_types)]
pub type commander_error_t = bitbox02_sys::commander_error_t;
pub type CommanderError = bitbox02_sys::commander_error_t;

pub const COMMANDER_ERR_USER_ABORT: bitbox02_sys::commander_error_t =
    bitbox02_sys::commander_error_t_COMMANDER_ERR_USER_ABORT;
pub const COMMANDER_ERR_GENERIC: bitbox02_sys::commander_error_t =
    bitbox02_sys::commander_error_t_COMMANDER_ERR_GENERIC;
pub const COMMANDER_OK: bitbox02_sys::commander_error_t =
    bitbox02_sys::commander_error_t_COMMANDER_OK;

pub use bitbox02_sys::font_monogram_5X9;

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

pub fn ug_font_select() {
    unsafe { bitbox02_sys::UG_FontSelect(&bitbox02_sys::font_font_a_9X9) }
}

pub fn delay(duration: Duration) {
    if duration < Duration::from_micros(1) {
        unsafe {
            // Sleep the smallest unit of sleep we support
            delay_us(1)
        }
    } else if duration < Duration::from_millis(1) {
        unsafe {
            delay_us(duration.as_micros() as u16);
        }
    } else {
        unsafe {
            delay_ms(duration.as_millis() as u16);
        }
    }
}

// Safe wrapper for workflow_confirm
pub fn workflow_confirm(
    title: &str,
    body: &str,
    font: *const bitbox02_sys::UG_FONT,
    longtouch: bool,
    accept_only: bool,
) -> bool {
    // Create null-terminated strings for title and body
    let title_cstr = match str_to_cstr!(title, 20) {
        Ok(cstr) => cstr,
        Err(_) => {
            screen_print_debug("string didn't fit", 3000);
            return false;
        }
    };
    let body_cstr = match str_to_cstr!(body, 100) {
        Ok(cstr) => cstr,
        Err(_) => {
            screen_print_debug("string didn't fit", 3000);
            return false;
        }
    };

    unsafe {
        bitbox02_sys::workflow_confirm(
            title_cstr.as_ptr() as *const _,
            body_cstr.as_ptr() as *const _,
            font,
            longtouch,
            accept_only,
        )
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
            input.len(),
            output.as_mut_ptr(),
            output.len(),
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
