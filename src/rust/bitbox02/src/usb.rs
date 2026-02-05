// SPDX-License-Identifier: Apache-2.0

pub use bitbox02_sys::USB_REPORT_SIZE;

pub fn start() {
    unsafe {
        bitbox02_sys::usb_start();
    }
}
