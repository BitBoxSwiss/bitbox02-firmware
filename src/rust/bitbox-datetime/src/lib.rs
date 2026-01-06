// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(test)]
extern crate std;

// for `format!`
#[macro_use]
extern crate alloc;

use alloc::string::String;
use core::ffi::c_int;

mod ffi {
    use core::ffi::{c_int, c_long};

    #[allow(non_camel_case_types)]
    pub type time_t = c_long;

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        // glibc/musl add non-POSIX fields at the end of `struct tm`, which we must include to keep
        // the `#[repr(C)]` layout compatible when calling `gmtime()`.
        // See:
        // - glibc: https://sourceware.org/git/?p=glibc.git;a=blob;f=time/bits/types/struct_tm.h;hb=HEAD
        // - musl: https://git.musl-libc.org/cgit/musl/tree/include/time.h
        // - Rust libc (mirrors platform headers): https://docs.rs/libc/latest/libc/struct.tm.html
        #[cfg(any(target_env = "gnu", target_env = "musl"))]
        pub tm_gmtoff: c_long,
        #[cfg(any(target_env = "gnu", target_env = "musl"))]
        pub tm_zone: *const core::ffi::c_char,
    }

    unsafe extern "C" {
        pub fn gmtime(timer: *const time_t) -> *mut tm;
    }
}

pub struct Tm {
    tm: ffi::tm,
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
            let gmtime = ffi::gmtime(&(timestamp as ffi::time_t));
            if gmtime.is_null() {
                return Err(());
            }

            *gmtime
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
