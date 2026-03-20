// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub fn set_cb(cb: Option<unsafe extern "C" fn() -> u32>) {
    unsafe { ffi::lv_tick_set_cb(cb) }
}
