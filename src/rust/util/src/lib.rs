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
pub mod ascii;
pub mod bb02_async;
pub mod bip32;
pub mod bytes;
pub mod cell;
pub mod decimal;
pub mod log;
pub mod name;
mod waker_fn;

mod base58;
#[cfg(feature = "p256")]
mod p256;
#[cfg(feature = "sha2")]
mod sha2;

// for `format!`
#[macro_use]
extern crate alloc;

// include critical section implementation, needed by rtt-target
#[cfg(feature = "rtt")]
extern crate cortex_m;

/// Guaranteed to wipe the provided buffer
pub fn zero(dst: &mut [u8]) {
    for p in dst {
        unsafe { core::ptr::write_volatile(p, 0) };
    }
}

/// Survive forces T to live at least as long as lifetme 'a.
pub struct Survive<'a, T: 'a> {
    pub data: T,
    phantom: core::marker::PhantomData<&'a T>,
}

impl<T> Survive<'_, T> {
    pub fn new(data: T) -> Self {
        Survive {
            data,
            phantom: core::marker::PhantomData,
        }
    }
}

// # C interface

/// Zero a buffer using volatile writes. Accepts null-ptr and 0-length buffers and does nothing.
///
/// * `dst` - Buffer to zero
#[unsafe(no_mangle)]
pub extern "C" fn rust_util_zero(mut dst: bytes::BytesMut) {
    if dst.buf.is_null() || dst.len == 0 {
        return;
    }
    zero(dst.as_mut())
}

// # Tests

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;
    #[test]
    fn zeroing() {
        let mut buf = [1u8, 2, 3];
        assert_ne!(&buf[..], &[0, 0, 0]);
        zero(&mut buf[..]);
        assert_eq!(&buf[..], &[0, 0, 0]);
    }

    #[test]
    fn zeroing2() {
        let mut buf = [1u8, 2, 3];
        zero(&mut buf[0..1]);
        assert_eq!(&buf[..], &[0, 2, 3]);
    }

    #[test]
    fn zeroing3() {
        let mut buf = [1u8, 2, 3];
        zero(&mut buf[1..2]);
        assert_eq!(&buf[..], &[1, 0, 3]);
    }

    #[test]
    fn zeroing_ciface() {
        let mut buf = [1u8, 2, 3, 4];
        rust_util_zero(unsafe { bytes::rust_util_bytes_mut(buf.as_mut_ptr(), buf.len() - 1) });
        assert_eq!(&buf[..], &[0, 0, 0, 4]);
    }

    #[test]
    fn zeroing_ciface_empty() {
        let mut buf = [];
        rust_util_zero(unsafe { bytes::rust_util_bytes_mut(buf.as_mut_ptr(), 0) });
    }

    #[test]
    fn zeroing_ciface_null() {
        rust_util_zero(unsafe { bytes::rust_util_bytes_mut(core::ptr::null_mut(), 0) });
    }
}
