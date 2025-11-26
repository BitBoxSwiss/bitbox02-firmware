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

#[macro_use]
mod alloc;

#[cfg(feature = "firmware")]
pub mod async_usb;
#[cfg(feature = "firmware")]
mod der;

// Expose C interface defined in bitbox_aes
#[cfg(feature = "firmware")]
extern crate bitbox_aes;

// Expose C interface defined in util
extern crate util;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen and over RTT. If we compile with
// `panic=abort` this code will never get executed.
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "firmware")]
    ::util::log::log!("{}", info);
    #[cfg(feature = "firmware")]
    bitbox02_rust::print_screen!(0, "Error: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}
