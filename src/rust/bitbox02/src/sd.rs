// Copyright 2021 Shift Crypto AG
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
use alloc::string::String;
use alloc::vec::Vec;

use crate::util::str_to_cstr_vec;

struct SdList(bitbox02_sys::sd_list_t);

impl Drop for SdList {
    fn drop(&mut self) {
        unsafe { bitbox02_sys::sd_free_list(&mut self.0) }
    }
}

pub fn list_subdir(subdir: Option<&str>) -> Result<Vec<String>, ()> {
    let mut list = SdList(bitbox02_sys::sd_list_t {
        num_files: 0,
        files: core::ptr::null_mut(),
    });
    let c_subdir = subdir.map(|s| str_to_cstr_vec(s).unwrap());
    match unsafe {
        bitbox02_sys::sd_list_subdir(
            &mut list.0,
            match c_subdir.as_ref() {
                Some(s) => s.as_ptr(),
                None => core::ptr::null(),
            },
        )
    } {
        true => (0..list.0.num_files)
            .map(|i| unsafe {
                let ptr = *list.0.files.offset(i as _) as *const u8;
                crate::util::str_from_null_terminated_ptr(ptr).map(String::from)
            })
            .collect(),
        false => Err(()),
    }
}
