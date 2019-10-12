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
pub use bitbox02_sys::BitBoxBaseSetConfigRequest;

// Reexport as u16 since this is the correct type (bindgen will generate them as u32)
#[allow(non_upper_case_globals)]
pub const BitBoxBaseSetConfigRequest_hostname_tag: u16 =
    bitbox02_sys::BitBoxBaseSetConfigRequest_hostname_tag as u16;
#[allow(non_upper_case_globals)]
pub const BitBoxBaseSetConfigRequest_ip_tag: u16 =
    bitbox02_sys::BitBoxBaseSetConfigRequest_ip_tag as u16;

pub mod util;

pub fn ug_put_string(x: i16, y: i16, input: &str, inverted: bool) {
    // rust strings (&str) are not null-terminated, ensure that there always is a \0 byte.
    let len = core::cmp::min(127, input.len());
    let mut buf = [0u8; 128];
    let buf = &mut buf[0..len];
    let input = &input.as_bytes()[0..len];
    buf.copy_from_slice(input);
    unsafe { bitbox02_sys::UG_PutString(x, y, buf.as_ptr() as *const _, inverted) }
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
pub fn workflow_confirm(title: &str, body: &str, longtouch: bool, accept_only: bool) -> bool {
    // Ensure valid nullterminated C-str
    // Will truncate title if it is too long
    let title_cstr = {
        const TITLE_LEN: usize = 20;
        let len = core::cmp::min(TITLE_LEN, title.len());
        let mut buf = [0u8; TITLE_LEN + 1];
        // resize title to actual length
        let title = &title.as_bytes()[0..len];
        // copy from title to buf
        buf[0..len].copy_from_slice(title);
        buf
    };
    // same as title_cstr
    let body_cstr = {
        const BODY_LEN: usize = 100;
        let len = core::cmp::min(BODY_LEN, body.len());
        let mut buf = [0u8; BODY_LEN + 1];
        // resize body to actual length
        let body = &body.as_bytes()[0..len];
        // copy from body to buf
        buf[0..len].copy_from_slice(body);
        buf
    };

    unsafe {
        bitbox02_sys::workflow_confirm(
            title_cstr.as_ptr() as *const _,
            body_cstr.as_ptr() as *const _,
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
