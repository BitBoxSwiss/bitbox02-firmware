// SPDX-License-Identifier: Apache-2.0

use crate::ringbuffer::RingBuffer;
use bitbox02_sys::da14531_protocol_frame;

pub fn handler(frame: &'static da14531_protocol_frame, uart_write_queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_handler(frame as *const _, &mut uart_write_queue.inner);
    }
}

pub fn set_product(product: &'static str) {
    let product = product.as_bytes();
    unsafe {
        bitbox02_sys::da14531_handler_current_product = product.as_ptr();
        bitbox02_sys::da14531_handler_current_product_len = product.len() as u16;
    }
}
