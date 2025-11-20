// Copyright 2025 Shift Crypto AG
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

use crate::ringbuffer::RingBuffer;
pub use bitbox02_sys::da14531_protocol_frame;

pub fn poll(
    uart_read_buf: &mut [u8],
    uart_read_buf_len: &mut u16,
    hww_data: &mut Option<[u8; 64]>,
    uart_write_queue: &mut RingBuffer,
) -> Option<&'static da14531_protocol_frame> {
    let mut data: *const u8 = if let Some(data) = (*hww_data).as_ref() {
        data.as_ptr() as *const _
    } else {
        core::ptr::null()
    };
    let frame = unsafe {
        bitbox02_sys::da14531_protocol_poll(
            uart_read_buf.as_mut_ptr() as *mut _,
            uart_read_buf_len as *mut _,
            &mut data as *mut _,
            &mut uart_write_queue.inner as *mut _,
        )
    };
    assert!(data.is_null());
    *hww_data = None;
    if frame.is_null() {
        None
    } else {
        Some(unsafe { &*frame })
    }
}
