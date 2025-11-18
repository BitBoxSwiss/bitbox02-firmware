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
pub use bitbox02_sys::USART_0_BUFFER_SIZE;

pub fn poll(
    uart_read_buf: Option<&mut [u8]>,
    uart_read_buf_len: Option<&mut u16>,
    uart_write_queue: Option<&mut RingBuffer>,
) {
    let (uart_read_buf, cap) = if let Some(uart_read_buf) = uart_read_buf {
        (
            uart_read_buf as *mut _ as *mut _,
            uart_read_buf.len() as u16,
        )
    } else {
        (core::ptr::null_mut(), 0u16)
    };
    let uart_read_buf_len = if let Some(len) = uart_read_buf_len {
        len as *mut _
    } else {
        core::ptr::null_mut()
    };
    let uart_write_queue = if let Some(uart_write_queue) = uart_write_queue {
        &mut uart_write_queue.inner as *mut _
    } else {
        core::ptr::null_mut()
    };

    unsafe {
        bitbox02_sys::uart_poll(uart_read_buf, cap, uart_read_buf_len, uart_write_queue);
    }
}
