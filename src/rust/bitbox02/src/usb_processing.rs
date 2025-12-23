// SPDX-License-Identifier: Apache-2.0

/// Reset the USB processing timeout to the given value.
pub fn timeout_reset(value: i16) {
    unsafe {
        bitbox02_sys::usb_processing_timeout_reset(value);
    }
}

#[cfg(feature = "simulator-graphical")]
pub fn init() {
    unsafe { bitbox02_sys::usb_processing_init() }
}

#[cfg(feature = "simulator-graphical")]
pub fn process_hww() {
    unsafe { bitbox02_sys::usb_processing_process(bitbox02_sys::usb_processing_hww()) }
}
