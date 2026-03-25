// SPDX-License-Identifier: Apache-2.0

pub fn init() {
    unsafe { bitbox02_sys::qtouch_init() }
}
