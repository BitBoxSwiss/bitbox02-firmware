// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bitbox02_rust::async_usb::{on_next_request, spawn, waiting_for_next_request};
use bitbox02_rust::hww::process_packet;

#[no_mangle]
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
#[no_mangle]
pub unsafe extern "C" fn rust_async_usb_copy_response(out: *mut bitbox02::buffer_t) -> UsbResponse {
    use bitbox02_rust::async_usb::{copy_response, CopyResponseErr};
    let dst = core::slice::from_raw_parts_mut((*out).data, (*out).max_len);
    match copy_response(dst) {
        Ok(len) => {
            (*out).len = len as _;
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
#[no_mangle]
pub extern "C" fn rust_async_usb_on_request_hww(usb_in: crate::util::Bytes) {
    if waiting_for_next_request() {
        on_next_request(usb_in.as_ref());
    } else {
        spawn(process_packet, usb_in.as_ref());
    }
}

#[no_mangle]
pub extern "C" fn rust_async_usb_cancel() -> bool {
    bitbox02_rust::async_usb::cancel()
}
