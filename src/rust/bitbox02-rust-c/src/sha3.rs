// Copyright 2021 Shift Crypto AG
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

extern crate alloc;

use alloc::boxed::Box;
use sha3::{Digest, Keccak256};
use util::c_types::{c_uchar, c_void};

/// Result must be freed by calling `rust_keccak256_finish()`.
#[no_mangle]
pub extern "C" fn rust_keccak256_new() -> *mut c_void {
    Box::into_raw(Box::new(Keccak256::new())) as *mut _
}

/// Safety: ctx must be a valid keccak256 context produced by `rust_keccak256_new()`. `data` must be a
/// valid buffer for `len` bytes.
// NOTE: we specifically do not use util::Bytes, as it disallows NULL. Our data can be 0 though, as
// the booloader starts at 0 and is hashed.
#[no_mangle]
pub unsafe extern "C" fn rust_keccak256_update(ctx: *mut c_void, data: *const c_void, len: usize) {
    let data = core::slice::from_raw_parts(data as *const u8, len);
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let ctx = ctx as *mut Keccak256;
    (*ctx).update(data);
}

/// Safety: ctx must be a pointer to a valid keccak256 context produced by `rust_keccak256_new()`.
/// `out` must be 32 bytes long.
/// After this, the hasher is dropped and `ctx` is set to NULL and must not be used anymore.
#[no_mangle]
pub unsafe extern "C" fn rust_keccak256_finish(ctx: *mut *mut c_void, out: *mut c_uchar) {
    let out = core::slice::from_raw_parts_mut(out, 32);
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let hasher = Box::from_raw(*ctx as *mut Keccak256); // dropped at the end
    let hash = hasher.finalize();
    out.copy_from_slice(&hash[..]);
    *ctx = core::ptr::null_mut();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn test_keccak256_streaming() {
        let mut ctx = rust_keccak256_new();
        unsafe {
            let data = b"foo";
            rust_keccak256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        unsafe {
            let data = b" abc def xyz";
            rust_keccak256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        unsafe {
            let data = b" bar";
            rust_keccak256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        let result = unsafe {
            let mut result = [0u8; 32];
            rust_keccak256_finish(&mut ctx, result.as_mut_ptr());
            result
        };
        assert_eq!(
            result,
            *b"\xf2\x80\x80\x9c\x57\x2c\xc7\x15\x8c\x09\xc4\x2b\xf1\x2a\xac\x3a\xe3\x7f\x5a\x9b\x1b\x45\xf0\x45\x73\x61\x66\x4a\x76\xa1\x9f\x54"
        );
        assert!(ctx.is_null());
    }
}
