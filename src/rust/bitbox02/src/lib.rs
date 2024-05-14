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

// allow unused as we use format!() only in some build configs (for
// the simulator), but replicating the conditions under which it is
// used here is not worth it.
#[allow(unused_imports)]
// for `format!`
#[macro_use]
extern crate alloc;

use alloc::string::String;

#[cfg(feature = "testing")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "testing")]
pub mod testing;

pub mod bip32;
pub mod input;
pub mod keystore;
pub mod memory;
pub mod random;
pub mod screen_saver;
pub mod sd;
pub mod secp256k1;
pub mod securechip;
pub mod ui;

use core::time::Duration;

pub use bitbox02_sys::buffer_t;

#[macro_use]
pub mod util;

pub fn ug_put_string(x: i16, y: i16, input: &str, inverted: bool) {
    unsafe {
        bitbox02_sys::UG_PutString(
            x,
            y,
            crate::util::str_to_cstr_vec(input).unwrap().as_ptr(),
            inverted,
        );
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
    unsafe {
        bitbox02_sys::screen_print_debug(
            crate::util::str_to_cstr_vec(msg).unwrap().as_ptr(),
            duration,
        )
    }
}

pub fn reset(status: bool) {
    unsafe { bitbox02_sys::reset_reset(status) }
}

pub fn strftime(timestamp: u32, format: &str) -> String {
    let mut out = [0u8; 100];
    unsafe {
        bitbox02_sys::strftime(
            out.as_mut_ptr(),
            out.len() as _,
            crate::util::str_to_cstr_vec(format).unwrap().as_ptr(),
            bitbox02_sys::localtime(&(timestamp as bitbox02_sys::time_t)),
        );
    }
    crate::util::str_from_null_terminated(&out[..])
        .unwrap()
        .into()
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
pub fn reboot() -> ! {
    unsafe { bitbox02_sys::reboot() }
    loop {}
}

pub fn hash160(data: &[u8]) -> [u8; 20] {
    let mut out = [0u8; 20];
    unsafe {
        bitbox02_sys::wally_hash160(
            data.as_ptr(),
            data.len() as _,
            out.as_mut_ptr(),
            out.len() as _,
        );
    }
    out
}

#[cfg(feature = "testing")]
pub fn reboot() -> ! {
    panic!("reboot called")
}

#[cfg(any(feature = "testing", feature = "c-unit-testing"))]
pub fn print_stdout(msg: &str) {
    unsafe {
        bitbox02_sys::printf(crate::util::str_to_cstr_vec(msg).unwrap().as_ptr());
    }
}

#[cfg(any(feature = "testing", feature = "c-unit-testing"))]
pub fn println_stdout(msg: &str) {
    unsafe {
        bitbox02_sys::printf(crate::util::str_to_cstr_vec(msg).unwrap().as_ptr());
        bitbox02_sys::printf(crate::util::str_to_cstr_vec("\n").unwrap().as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strftime() {
        assert_eq!(
            strftime(1601281809, "%a %Y-%m-%d\n%H:%M").as_str(),
            "Mon 2020-09-28\n08:30",
        );
    }
}
