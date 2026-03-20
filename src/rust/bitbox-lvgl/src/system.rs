// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub fn init() {
    unsafe { ffi::lv_init() }
}
