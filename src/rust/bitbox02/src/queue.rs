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
pub fn pull_hww() -> Option<[u8; 64]> {
    let hww_data = unsafe { bitbox02_sys::queue_pull(bitbox02_sys::queue_hww_queue()) };
    if hww_data.is_null() {
        return None;
    }
    let mut data: [u8; 64] = [0; 64];
    unsafe { core::ptr::copy_nonoverlapping(hww_data, data.as_mut_ptr(), 64) }
    Some(data)
}

#[cfg(feature = "app-u2f")]
pub fn pull_u2f() -> Option<[u8; 64]> {
    let u2f_data = unsafe { bitbox02_sys::queue_pull(bitbox02_sys::queue_u2f_queue()) };
    if u2f_data.is_null() {
        return None;
    }
    let mut data: [u8; 64] = [0; 64];
    unsafe { core::ptr::copy_nonoverlapping(u2f_data, data.as_mut_ptr(), 64) }
    Some(data)
}
