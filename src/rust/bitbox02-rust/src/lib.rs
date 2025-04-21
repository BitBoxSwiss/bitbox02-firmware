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
    include!("./shiftcrypto.bitbox02.rs");
}
mod pb_backup {
    include!("./shiftcrypto.bitbox02.backups.rs");
}

#[macro_use]
pub mod general;
pub mod async_usb;
pub mod attestation;
pub mod backup;
pub mod bb02_async;
mod bip32;
pub mod hal;
pub mod hww;
pub mod keystore;
mod version;
mod waker_fn;
pub mod workflow;
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
mod xpubcache;

// for `format!`
#[macro_use]
extern crate alloc;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trivial_test() {
        let a = alloc::string::String::from("abc");
        assert!(&a == "abc");
    }
}
