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

extern crate alloc;
use alloc::vec::Vec;

pub fn commander(input: Vec<u8>) -> Vec<u8> {
    let input = bitbox02_sys::in_buffer_t {
        data: input.as_ptr() as *const _,
        len: input.len() as _,
    };

    // Same as (USB_DATA_MAX_LEN - 2) (1 byte reserved for HWW_RSP_* code, 1 byte for
    // OP_STATUS_SUCCESS).
    const MAX_OUT_LEN: usize = 7607;
    let mut output_vec = Vec::with_capacity(MAX_OUT_LEN);
    let mut output = bitbox02_sys::buffer_t {
        data: output_vec.as_mut_ptr() as *mut _,
        len: 0,
        max_len: output_vec.capacity() as _,
    };
    unsafe {
        // Safety:
        // input is not NULL and the data length is correct.
        // output is not NULL and has the correct capacity.
        bitbox02_sys::commander(&input as *const _, &mut output as *mut _);
        // Safety: commander is guaranteed to set the number of bytes written
        // correctly.
        output_vec.set_len(output.len as _);
    };
    output_vec
}
