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

pub fn process_hww() {
    unsafe { bitbox02_sys::usb_processing_process(bitbox02_sys::usb_processing_hww()) }
}

#[cfg(feature = "app-u2f")]
pub fn process_u2f() {
    unsafe { bitbox02_sys::usb_processing_process(bitbox02_sys::usb_processing_u2f()) }
}

#[cfg(feature = "app-u2f")]
pub fn locked_u2f() -> bool {
    unsafe { bitbox02_sys::usb_processing_locked(bitbox02_sys::usb_processing_u2f()) }
}

pub fn unlock() {
    unsafe { bitbox02_sys::usb_processing_unlock() }
}
