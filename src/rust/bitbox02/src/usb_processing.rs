// SPDX-License-Identifier: Apache-2.0

use bitbox_usb_report_queue::UsbReportQueue;

/// Reset the USB processing timeout to the given value.
pub fn timeout_reset(value: i16) {
    unsafe {
        bitbox02_sys::usb_processing_timeout_reset(value);
    }
}

pub fn init<const HWW_MAX_SIZE: usize>(hww_queue: &mut UsbReportQueue<HWW_MAX_SIZE>) {
    unsafe {
        bitbox02_sys::usb_processing_init(
            hww_queue
                .as_mut_ptr()
                .cast::<bitbox02_sys::RustUsbReportQueue>(),
        )
    }
}

#[cfg(feature = "app-u2f")]
pub fn init_u2f<const U2F_MAX_SIZE: usize>(u2f_queue: &mut UsbReportQueue<U2F_MAX_SIZE>) {
    unsafe {
        bitbox02_sys::usb_processing_init_u2f(
            u2f_queue
                .as_mut_ptr()
                .cast::<bitbox02_sys::RustUsbReportQueue>(),
        )
    }
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
