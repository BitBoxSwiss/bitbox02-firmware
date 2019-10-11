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

use core::time::Duration;

use bitbox02::{delay, ug_clear_buffer, ug_font_select, ug_put_string, ug_send_buffer};

pub fn print_debug_internal(duration: Duration, msg: &str) {
    ug_clear_buffer();
    ug_font_select();
    ug_put_string(0, 0, msg, false);
    ug_send_buffer();
    delay(duration);
}

/// This is a convenience macro for printing to the screen.
///
/// Example usage:
///
/// ```
/// print_debug!(1000, "{}", &my_str);
/// ```
#[macro_export]
macro_rules! print_debug {
    ($duration:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        let duration = core::time::Duration::from_millis($duration);
        let mut buf = arrayvec::ArrayString::<[_; 256]>::new();
        let _ = write!(buf, $($arg)*);
        $crate::general::screen::print_debug_internal(duration, &buf);
    })
}
