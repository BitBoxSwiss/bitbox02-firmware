// SPDX-License-Identifier: Apache-2.0

use bitbox_usb_report_queue::UsbReportQueue;

/// Reset the USB processing timeout to the given value.
pub fn timeout_reset(value: i16) {
    unsafe {
        bitbox02_sys::usb_processing_timeout_reset(value);
    }
}

#[cfg(feature = "app-u2f")]
pub fn init(hww_queue: &mut UsbReportQueue, u2f_queue: &mut UsbReportQueue) {
    unsafe {
        bitbox02_sys::usb_processing_init(
            hww_queue
                .as_mut_ptr()
                .cast::<bitbox02_sys::RustUsbReportQueue>(),
            u2f_queue
                .as_mut_ptr()
                .cast::<bitbox02_sys::RustUsbReportQueue>(),
        )
    }
}

#[cfg(not(feature = "app-u2f"))]
pub fn init(hww_queue: &mut UsbReportQueue) {
    unsafe {
        bitbox02_sys::usb_processing_init(
            hww_queue
                .as_mut_ptr()
                .cast::<bitbox02_sys::RustUsbReportQueue>(),
            core::ptr::null_mut(),
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
