// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

use bitbox02::{delay, screen_clear, ug_font_select_9x9, ug_put_string, ug_send_buffer};

pub fn print_debug_internal(duration: Duration, msg: &str) {
    screen_clear();
    ug_font_select_9x9();
    ug_put_string(0, 0, msg, false);
    ug_send_buffer();
    delay(duration);
}

/// This is a convenience macro for printing to the screen.
///
/// Example usage:
///
/// ```no_run
/// # #[macro_use] extern crate bitbox02_rust; fn main() {
/// let my_str = "abc";
/// print_screen!(1000, "{}", &my_str);
/// # }
/// ```
#[macro_export]
macro_rules! print_screen {
    ($duration:expr, $($arg:tt)*) => ({
        extern crate alloc;
        let duration = core::time::Duration::from_millis($duration);
        $crate::general::screen::print_debug_internal(duration, &alloc::format!($($arg)*));
    })
}
