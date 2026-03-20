// SPDX-License-Identifier: Apache-2.0

use crate::{LvColor, ffi};

pub fn make(red: u8, green: u8, blue: u8) -> LvColor {
    unsafe { ffi::lv_color_make(red, green, blue) }
}

pub fn hex(color: u32) -> LvColor {
    unsafe { ffi::lv_color_hex(color) }
}

pub fn hex3(color: u32) -> LvColor {
    unsafe { ffi::lv_color_hex3(color) }
}

pub fn white() -> LvColor {
    unsafe { ffi::lv_color_white() }
}

pub fn black() -> LvColor {
    unsafe { ffi::lv_color_black() }
}
