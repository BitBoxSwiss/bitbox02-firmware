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

/// This wraps hww_api_process_packet() in a safe interface.
pub fn process_packet(in_req: Vec<u8>) -> Vec<u8> {
    let in_req = bitbox02_sys::in_buffer_t {
        data: in_req.as_ptr() as *const _,
        len: in_req.len() as _,
    };

    // Same as (USB_DATA_MAX_LEN - 1) (1 byte reserved for HWW_RSP_* code).
    const MAX_OUT_LEN: usize = 7608;

    let mut out_rsp_vec = Vec::with_capacity(MAX_OUT_LEN);
    let mut out_rsp = bitbox02_sys::buffer_t {
        data: out_rsp_vec.as_mut_ptr() as *mut _,
        len: 0,
        max_len: out_rsp_vec.capacity() as _,
    };
    unsafe {
        // Safety:
        // in_req is not NULL and the data length is correct.
        // out_rsp is not NULL and has the correct capacity.
        bitbox02_sys::hww_api_process_packet(&in_req as *const _, &mut out_rsp as *mut _);
        // Safety: hww_api_process_packet is guaranteed to set the number of bytes written
        // correctly.
        out_rsp_vec.set_len(out_rsp.len as _);
    }
    out_rsp_vec
}
