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

/// Reset the USB processing timeout to the given value.
pub fn timeout_reset(value: i16) {
    unsafe {
        bitbox02_sys::usb_processing_timeout_reset(value);
    }
}

#[cfg(feature = "simulator-graphical")]
pub fn init() {
    unsafe { bitbox02_sys::usb_processing_init() }
}

pub fn process_hww() {
    unsafe { bitbox02_sys::usb_processing_process(bitbox02_sys::usb_processing_hww()) }
}

#[cfg(feature = "app-u2f")]
pub fn process_u2f() {
    unsafe { bitbox02_sys::usb_processing_process(bitbox02_sys::usb_processing_u2f()) }
}

#[cfg(feature = "app-u2f")]
pub fn locked_u2f() -> bool {
    unsafe { bitbox02_sys::usb_processing_locked(bitbox02_sys::usb_processing_u2f()) }
}

pub fn unlock() {
    unsafe { bitbox02_sys::usb_processing_unlock() }
}
