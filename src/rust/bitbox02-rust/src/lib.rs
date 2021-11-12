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

mod pb {
    include!(concat!(env!("OUT_DIR"), "/shiftcrypto.bitbox02.rs"));
}

#[macro_use]
pub mod general;
pub mod apps;
pub mod async_usb;
pub mod attestation;
pub mod bb02_async;
pub mod hww;
pub mod keystore;
pub mod util;
mod waker_fn;
pub mod workflow;

// for `format!`
#[macro_use]
extern crate alloc;

// reexport arrayvec because it is used in our macro "print_debug"
pub extern crate arrayvec;

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
