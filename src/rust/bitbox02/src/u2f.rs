// SPDX-License-Identifier: Apache-2.0

pub fn process() {
    unsafe {
        bitbox02_sys::u2f_process();
    }
}
