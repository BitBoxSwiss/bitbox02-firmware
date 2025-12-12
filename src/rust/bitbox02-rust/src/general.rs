// SPDX-License-Identifier: Apache-2.0

#[macro_use]
pub mod screen;

/// displays the input error message on the screen and enters
/// an infinite loop.
#[allow(clippy::empty_loop)]
pub fn abort(err: &str) -> ! {
    print_screen!(0, "Error: {}", err);
    loop {}
}
