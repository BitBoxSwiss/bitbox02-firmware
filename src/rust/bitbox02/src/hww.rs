// SPDX-License-Identifier: Apache-2.0

pub fn setup() {
    unsafe { bitbox02_sys::hww_setup() };
}
