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

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

// Since util_c defines an "alloc_error_handler" we get conflicts with std when testing
#[cfg(not(test))]
// for `format!`
#[macro_use]
mod alloc;

mod util;

#[cfg(feature = "firmware")]
mod async_usb;
#[cfg(feature = "firmware")]
mod noise;
#[cfg(feature = "firmware")]
mod p256;
#[cfg(feature = "firmware")]
mod sha2;
#[cfg(feature = "firmware")]
mod workflow;

#[cfg(feature = "firmware")]
mod der;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen and over RTT. If we compile with
// `panic=abort` this code will never get executed.
#[cfg(not(test))]
#[cfg(not(feature = "testing"))]
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ::util::log::log!("{}", info);
    #[cfg(feature = "firmware")]
    bitbox02_rust::print_screen!(0, "Error: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_rtt_init() {
    ::util::log::rtt_init();
}

#[no_mangle]
pub extern "C" fn rust_rtt_flush() {
    ::util::log::rtt_flush();
}

/// # Safety
///
/// The pointer `ptr` must point to a null terminated string
#[no_mangle]
#[cfg_attr(not(feature = "rtt"), allow(unused))]
pub unsafe extern "C" fn rust_log(ptr: *const ::util::c_types::c_char) {
    #[cfg(feature = "rtt")]
    {
        if ptr.is_null() {
            panic!("`ptr` must be a valid pointer");
        }
        let s = unsafe { core::ffi::CStr::from_ptr(ptr as _) };
        let s = unsafe { core::str::from_utf8_unchecked(s.to_bytes()) };
        ::util::log::rtt_target::rprintln!("{}", s);
    }
}
