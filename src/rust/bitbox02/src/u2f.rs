// SPDX-License-Identifier: Apache-2.0

pub fn device_setup() {
    unsafe {
        bitbox02_sys::u2f_device_setup();
    }
}

pub fn process() {
    unsafe {
        bitbox02_sys::u2f_process();
    }
}
