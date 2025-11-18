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

pub use bitbox02_sys::USB_FRAME;

pub fn init() {
    unsafe {
        bitbox02_sys::u2f_packet_init();
    }
}

pub fn timeout_get(cid: &mut u32) -> bool {
    unsafe { bitbox02_sys::u2f_packet_timeout_get(cid as *mut _) }
}

pub fn timeout(cid: u32) {
    unsafe { bitbox02_sys::u2f_packet_timeout(cid) }
}

pub fn process(packet: &[u8; 64]) -> bool {
    unsafe { bitbox02_sys::u2f_packet_process(packet.as_ptr() as *const _) }
}
