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

#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)] // used in alloc.rs

#[cfg(test)]
#[macro_use]
extern crate std;

// Since util_c defines an "alloc_error_handler" we get conflicts with std when testing
#[cfg(not(test))]
mod alloc;

mod async_usb;
mod noise;
mod util;

#[cfg(feature = "platform-bitboxbase")]
pub mod bitboxbase;

#[cfg(feature = "platform-bitbox02")]
pub mod bitbox02;

#[cfg(feature = "app-ethereum")]
pub mod app_ethereum;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen. If we compile with `panic=abort`
// this code will never get executed.
#[cfg(not(test))]
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(not(feature = "bootloader"))]
    bitbox02_rust::print_debug!(0, "Error: {}", info);
    loop {}
}
