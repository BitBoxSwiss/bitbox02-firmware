// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use core::time::Duration;

/// displays the input error message on the screen and enters
/// an infinite loop.
#[allow(clippy::empty_loop)]
pub fn abort(hal: &mut impl crate::hal::Hal, err: &str) -> ! {
    hal.ui()
        .print_screen(Duration::from_millis(0), &format!("Error: {}", err));
    loop {}
}
