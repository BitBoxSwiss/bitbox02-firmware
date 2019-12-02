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

// Since we are targeting embedded we exclude the standard library by default
#![no_std]
// When compiling for testing we allow certain warnings.
#![cfg_attr(test, allow(unused_imports, dead_code))]

use core::panic::PanicInfo;

mod error;
#[macro_use]
mod general;
mod commander;
mod platform;
mod util;
mod workflow;

// A trick to convince cbindgen that an u8 is char.
// cbindgen will convert `u8` to `uint8_t` and `i8` to `int8_t` which are `unsigned char` and
// `signed char` respectively. `c_char` is converted to `char` without `signed` or `unsigned`.
#[allow(non_camel_case_types)]
type c_char = u8;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen. If we compile with `panic=abort`
// this code will never get executed.
#[cfg(not(test))]
#[cfg_attr(not(debug_assertions), allow(unused_variables))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    print_debug!(0, "Error: {}", info);
    #[cfg(not(debug_assertions))]
    print_debug!(0, "Error");
    loop {}
}

#[cfg(test)]
mod test {
    // Enable standard library for testing
    extern crate std;
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn trivial_test() {
        let a = String::from("abc");
        assert!(&a == "abc");
    }
}
