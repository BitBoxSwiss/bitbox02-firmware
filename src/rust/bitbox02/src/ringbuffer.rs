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

use bitbox02_sys::{ringbuffer, ringbuffer_init};

/// A wrapper around ASF4 `ringbuffer` type
pub struct RingBuffer<'a> {
    // For now we don't use `buf`, but when we implement push/pull we will need to.
    _buf: &'a mut [u8],
    pub inner: ringbuffer,
}

impl<'a> RingBuffer<'a> {
    /// `buf` length must be a power of 2
    pub fn new(buf: &'a mut [u8]) -> Self {
        debug_assert!(buf.len().is_power_of_two());
        let mut inner = ringbuffer {
            buf: core::ptr::null_mut(),
            size: 0,
            read_index: 0,
            write_index: 0,
        };
        unsafe {
            ringbuffer_init(
                &mut inner as *mut _,
                buf as *mut _ as *mut _,
                buf.len() as u32,
            );
        };
        RingBuffer { _buf: buf, inner }
    }

    /// Bytes currently used
    pub fn len(&self) -> u32 {
        unsafe { bitbox02_sys::ringbuffer_num(&self.inner as *const _) }
    }
}
