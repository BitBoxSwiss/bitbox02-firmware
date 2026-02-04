// SPDX-License-Identifier: Apache-2.0

use crate::ringbuffer::RingBuffer;
pub use bitbox02_sys::USART_0_BUFFER_SIZE;

pub fn poll(
    uart_read_buf: Option<&mut [u8]>,
    uart_read_buf_len: Option<&mut u16>,
    uart_write_queue: &mut RingBuffer,
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

    unsafe {
        bitbox02_sys::uart_poll(
            uart_read_buf,
            cap,
            uart_read_buf_len,
            &mut uart_write_queue.inner,
        );
    }
}
