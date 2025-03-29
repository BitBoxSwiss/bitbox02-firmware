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
pub mod testing;

pub mod bip32;
pub mod keystore;
pub mod memory;
pub mod random;
pub mod screen_saver;
pub mod sd;
pub mod secp256k1;
pub mod securechip;
pub mod ui;

use ::util::c_types::c_int;
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

pub struct Tm {
    tm: bitbox02_sys::tm,
}

fn range(low: c_int, item: c_int, high: c_int) -> c_int {
    core::cmp::max(low, core::cmp::min(item, high))
}

impl Tm {
    /// Returns the weekday, one of "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
    pub fn weekday(&self) -> String {
        // Same as '%a' in strftime:
        // https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#435
        let wday = self.tm.tm_wday;
        if !(0..=6).contains(&wday) {
            return "?".into();
        }
        ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][wday as usize].into()
    }

    /// Returns 'year-month-day', e.g. 2024-07-16, equivalent of '%Y-%m-%d' in strftime.
    pub fn date(&self) -> String {
        // Same as strftime:
        // %Y - https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#L712
        // %m - https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#L600
        // %d - https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#L498
        format!(
            "{}-{:02}-{:02}",
            1900 + self.tm.tm_year,
            range(0, self.tm.tm_mon, 11) + 1,
            range(1, self.tm.tm_mday, 31)
        )
    }

    /// Returns the zero-padded hour from 00-23, e.g. "07".
    pub fn hour(&self) -> String {
        // Same as '%H' in strftime:
        // https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#582
        format!("{:02}", range(0, self.tm.tm_hour, 23))
    }

    /// Returns the zero-padded minute from 00-59, e.g. "07".
    pub fn minute(&self) -> String {
        // Same as '%M' in strftime:
        // https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#L605
        format!("{:02}", range(0, self.tm.tm_min, 59))
    }

    /// Returns the zero-padded second from 00-60, e.g. "07".
    pub fn second(&self) -> String {
        // Same as '%S' in strftime:
        // https://github.com/arnoldrobbins/strftime/blob/2011b7e82365d25220b8949e252eb5f28c0994cd/strftime.c#L645
        format!("{:02}", range(0, self.tm.tm_sec, 60))
    }
}

pub fn get_datetime(timestamp: u32) -> Result<Tm, ()> {
    Ok(Tm {
        tm: unsafe {
            let localtime = bitbox02_sys::localtime(&(timestamp as bitbox02_sys::time_t));
            if localtime.is_null() {
                return Err(());
            }

            *localtime
        },
    })
}

/// Formats the timestamp in the local timezone.
/// timestamp is the unix timestamp in seconds.
/// timezone_offset is added to the timestamp, timezone part.
/// date_only: if true, only the date is formatted. If false, both date and time are.
pub fn format_datetime(
    timestamp: u32,
    timezone_offset: i32,
    date_only: bool,
) -> Result<String, ()> {
    const MAX_EAST_UTC_OFFSET: i32 = 50400; // 14 hours in seconds
    const MAX_WEST_UTC_OFFSET: i32 = -43200; // 12 hours in seconds

    if !(MAX_WEST_UTC_OFFSET..=MAX_EAST_UTC_OFFSET).contains(&timezone_offset) {
        return Err(());
    }
    let ts = ((timestamp as i64) + (timezone_offset as i64)) as u32;
    let tm = get_datetime(ts)?;
    Ok(if date_only {
        format!("{} {}", tm.weekday(), tm.date())
    } else {
        format!(
            "{} {}\n{}:{}",
            tm.weekday(),
            tm.date(),
            tm.hour(),
            tm.minute()
        )
    })
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

pub fn sha512(msg: &[u8]) -> [u8; 64] {
    let mut result = [0u8; 64];
    unsafe {
        bitbox02_sys::wally_sha512(
            msg.as_ptr(),
            msg.len() as _,
            result.as_mut_ptr(),
            result.len() as _,
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime() {
        assert_eq!(
            format_datetime(1601281809, 0, true),
            Ok("Mon 2020-09-28".into())
        );
        assert_eq!(
            format_datetime(1601281809, 0, false),
            Ok("Mon 2020-09-28\n08:30".into()),
        );
        assert_eq!(
            format_datetime(1601281809, 18000, false),
            Ok("Mon 2020-09-28\n13:30".into()),
        );
        assert_eq!(
            format_datetime(1601281809, -32400, false),
            Ok("Sun 2020-09-27\n23:30".into()),
        );

        assert!(format_datetime(1601281809, 50401, false).is_err());
        assert!(format_datetime(1601281809, -43201, false).is_err());
    }
}
