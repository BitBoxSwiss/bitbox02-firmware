// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use alloc::vec::Vec;
use bitbox02_rust::async_usb::{on_next_request, spawn, waiting_for_next_request};
use core::sync::atomic::{AtomicPtr, Ordering};

static ASYNC_USB_HAL: AtomicPtr<crate::BitBox02HAL> = AtomicPtr::new(core::ptr::null_mut());

async fn process_packet_with_hal(usb_in: Vec<u8>) -> Vec<u8> {
    let hal = ASYNC_USB_HAL.load(Ordering::Relaxed);
    bitbox02_rust::hww::process_packet(unsafe { crate::bitbox02hal_mut(hal) }, usb_in).await
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_async_usb_spin() {
    bitbox02_rust::async_usb::spin();
}

#[repr(C)]
pub enum UsbResponse {
    UsbResponseAck,
    UsbResponseNotReady,
    UsbResponseNack,
}

/// Polls for a result of an async usb task. If a result is available, it is copied to `out`.
///
/// Returns:
/// `UsbResponseNack` if on ask is running.
/// `UsbResponseAck` if the result was copied.
/// `UsbResponseNotReady` if a task is running but not yet complete.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_async_usb_copy_response(out: *mut bitbox02::buffer_t) -> UsbResponse {
    use bitbox02_rust::async_usb::{CopyResponseErr, copy_response};
    let dst = unsafe { core::slice::from_raw_parts_mut((*out).data, (*out).max_len) };
    match copy_response(dst) {
        Ok(len) => {
            unsafe { (*out).len = len as _ };
            UsbResponse::UsbResponseAck
        }
        Err(CopyResponseErr::NotReady) => UsbResponse::UsbResponseNotReady,
        Err(CopyResponseErr::NotRunning) => UsbResponse::UsbResponseNack,
    }
}

/// Spawns the async HWW api processor (api level, HWW_REQ_*
/// arbitration level should be taken care of before).
///
/// `usb_in` are the api request bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_async_usb_on_request_hww(
    hal: *mut crate::BitBox02HAL,
    usb_in: util::bytes::Bytes,
) {
    ASYNC_USB_HAL.store(hal, Ordering::Relaxed);
    if waiting_for_next_request() {
        on_next_request(usb_in.as_ref());
    } else {
        spawn(process_packet_with_hal, usb_in.as_ref());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_async_usb_cancel() -> bool {
    bitbox02_rust::async_usb::cancel()
}
