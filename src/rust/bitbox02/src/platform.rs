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

pub fn product() -> (&'static str, u16) {
    unsafe {
        let mut len = 0;
        let s = bitbox02_sys::platform_product(&mut len as *mut _) as *const u8;
        let s = core::slice::from_raw_parts(s, len);
        let s = str::from_utf8_unchecked(s);
        (s as _, len as u16)
    }
}
