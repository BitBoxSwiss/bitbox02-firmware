// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(test), no_std)]
pub mod ascii;
pub mod bb02_async;
pub mod bip32;
pub mod bytes;
pub mod cell;
pub mod datetime;
pub mod decimal;
pub mod display;
pub mod futures;
pub mod log;
pub mod name;
pub mod strings;

#[cfg(feature = "p256")]
mod p256;
#[cfg(feature = "sha2")]
pub mod sha2;

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

// # C interface

/// Zero a buffer using volatile writes. Accepts null-ptr and 0-length buffers and does nothing.
///
/// * `dst` - Buffer to zero
///
/// # Safety
///
/// `dst` must point to a writable memory area of size `len`, unless it is null or `len == 0`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_util_zero(dst: *mut u8, len: usize) {
    if dst.is_null() || len == 0 {
        return;
    }
    for i in 0..len {
        unsafe { core::ptr::write_volatile(dst.add(i), 0) };
    }
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
        unsafe { rust_util_zero(buf.as_mut_ptr(), buf.len() - 1) };
        assert_eq!(&buf[..], &[0, 0, 0, 4]);
    }

    #[test]
    fn zeroing_ciface_empty() {
        let mut buf = [];
        unsafe { rust_util_zero(buf.as_mut_ptr(), 0) };
    }

    #[test]
    fn zeroing_ciface_null() {
        unsafe { rust_util_zero(core::ptr::null_mut(), 0) };
    }
}
