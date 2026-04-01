// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub fn handler() {
    unsafe {
        ffi::lv_timer_handler();
    }
}
