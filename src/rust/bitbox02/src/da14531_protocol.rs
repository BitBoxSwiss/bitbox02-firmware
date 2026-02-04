// SPDX-License-Identifier: Apache-2.0

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
    if data.is_null() {
        *hww_data = None;
    }
    if frame.is_null() {
        None
    } else {
        Some(unsafe { &*frame })
    }
}
