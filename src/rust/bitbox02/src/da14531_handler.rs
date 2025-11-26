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
use bitbox02_sys::da14531_protocol_frame;

pub fn handler(frame: &'static da14531_protocol_frame, uart_write_queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_handler(frame as *const _, &mut uart_write_queue.inner);
    }
}

pub fn set_product(product: &'static str, len: u16) {
    unsafe {
        bitbox02_sys::da14531_handler_current_product = product.as_bytes().as_ptr();
        bitbox02_sys::da14531_handler_current_product_len = len;
    }
}
