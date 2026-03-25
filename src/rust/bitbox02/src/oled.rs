// SPDX-License-Identifier: Apache-2.0

use crate::ui::ugui::UG_COLOR;

pub fn clear_buffer() {
    unsafe { bitbox02_sys::oled_clear_buffer() }
}

pub fn mirror(mirror: bool) {
    unsafe { bitbox02_sys::oled_mirror(mirror) }
}

pub fn set_pixel(x: i16, y: i16, color: UG_COLOR) {
    unsafe { bitbox02_sys::oled_set_pixel(x, y, color) }
}
