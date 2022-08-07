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
#![feature(alloc_error_handler)] // used in alloc.rs
#![feature(lang_items)]

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
mod sha2;
#[cfg(feature = "app-ethereum")]
mod sha3;
#[cfg(feature = "firmware")]
mod workflow;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen. If we compile with `panic=abort`
// this code will never get executed.
#[cfg(not(test))]
#[cfg(not(feature = "testing"))]
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "firmware")]
    bitbox02_rust::print_debug!(0, "Error: {}", info);
    loop {}
}

// Needed to link the C unit test executables in /test/unit-test.
//
// See https://doc.rust-lang.org/unstable-book/language-features/lang-items.html#writing-an-executable-without-stdlib.
//
// One could get rid of this and also considerably shrink the binary size by compiling core instead
// of using pre-built binaries. See a proof of concept implementation here:
// https://github.com/digitalbitbox/bitbox02-firmware/tree/build-std-PoC.  We decided against doing
// this for now as the feature seems immature and because of the warnings against using it in
// production:
// https://github.com/rust-lang/wg-cargo-std-aware/tree/81765f0eb744b9c47840c16f43a32c9f61fd7f0c#mvp-implementation
#[cfg(not(feature = "dont-export-eh-personality"))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
