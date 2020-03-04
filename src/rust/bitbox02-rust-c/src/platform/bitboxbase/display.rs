// Copyright 2019 Shift Cryptosecurity AG
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

use crate::c_char;
use bitbox02_rust::platform::bitboxbase::display::write_status;
use bitbox02_rust::util::FixedCString;

#[no_mangle]
pub extern "C" fn bitboxbase_status_get(ptr: *mut c_char, ptr_len: usize) {
    let buf = unsafe { core::slice::from_raw_parts_mut(ptr, ptr_len) };
    let mut wrapper = FixedCString::new(buf);
    write_status(&mut wrapper, unsafe {
        &crate::platform::bitboxbase::config::CONFIG
    });
}
