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

pub fn set_name(name: &str, queue: &mut RingBuffer) {
    let name = crate::util::str_to_cstr_vec(name).unwrap();
    unsafe { bitbox02_sys::da14531_set_name(name.as_ptr(), &mut queue.inner as *mut _) };
}

pub fn set_product(product: &str, queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_set_product(
            product.as_bytes().as_ptr() as *const _,
            product.len() as u16,
            &mut queue.inner,
        )
    }
}

pub fn power_down(queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_power_down(&mut queue.inner as *mut _);
    }
}
