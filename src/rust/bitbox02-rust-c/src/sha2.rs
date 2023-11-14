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

extern crate alloc;

use alloc::boxed::Box;
use sha2::Digest;
use sha2::Sha256;
use util::c_types::{c_uchar, c_void};

/// Result must be freed by calling `rust_sha256_finish()` or `rust_sha256_free()`.
#[no_mangle]
pub extern "C" fn rust_sha256_new() -> *mut c_void {
    Box::into_raw(Box::new(Sha256::new())) as *mut _
}

/// Safety: ctx must be a valid sha256 context produced by `rust_sha256_new()`. `data` must be a
/// valid buffer for `len` bytes.
// NOTE: we specifically do not use util::Bytes, as it disallows NULL. Our data can be 0 though, as
// the booloader starts at 0 and is hashed.
#[no_mangle]
pub unsafe extern "C" fn rust_sha256_update(ctx: *mut c_void, data: *const c_void, len: usize) {
    let data = core::slice::from_raw_parts(data as *const u8, len);
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let ctx = ctx as *mut Sha256;
    (*ctx).update(data);
}

/// Safety: ctx must be a pointer to a valid sha256 context produced by `rust_sha256_new()`.
/// `out` must be 32 bytes long.
/// After this, the hasher is dropped and `ctx` is set to NULL and must not be used anymore.
#[no_mangle]
pub unsafe extern "C" fn rust_sha256_finish(ctx: *mut *mut c_void, out: *mut c_uchar) {
    let out = core::slice::from_raw_parts_mut(out, 32);
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let hasher = Box::from_raw(*ctx as *mut Sha256); // dropped at the end
    let hash = hasher.finalize();
    out.copy_from_slice(&hash[..]);
    *ctx = core::ptr::null_mut();
}

/// Safety: data must be a valid buffer for `len` bytes. `out` must be 32 bytes long.
#[no_mangle]
pub unsafe extern "C" fn rust_sha256(data: *const c_void, len: usize, out: *mut c_uchar) {
    let out = core::slice::from_raw_parts_mut(out, 32);
    let data = core::slice::from_raw_parts(data as *const u8, len);
    let hash = Sha256::digest(data);
    out.copy_from_slice(&hash[..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn test_sha256_streaming() {
        let mut ctx = rust_sha256_new();
        unsafe {
            let data = b"foo";
            rust_sha256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        unsafe {
            let data = b" abc def xyz";
            rust_sha256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        unsafe {
            let data = b" bar";
            rust_sha256_update(ctx, data.as_ptr() as *const _, data.len());
        }
        let result = unsafe {
            let mut result = [0u8; 32];
            rust_sha256_finish(&mut ctx, result.as_mut_ptr());
            result
        };
        assert_eq!(result, &Sha256::digest(b"foo abc def xyz bar")[..]);
        assert!(ctx.is_null());
    }

    #[test]
    fn test_sha256() {
        let data = b"foo abc def xyz bar";
        let mut result = [0u8; 32];
        unsafe {
            rust_sha256(data.as_ptr() as *const _, data.len(), result.as_mut_ptr());
        }
        assert_eq!(result, &Sha256::digest(b"foo abc def xyz bar")[..]);
    }

    /// Test that input and output can be the same buffer.
    #[test]
    fn test_overlapping() {
        let mut input_and_output = *b"12345678901234567890123456789012";
        unsafe {
            rust_sha256(
                input_and_output.as_ptr() as *const _,
                input_and_output.len(),
                input_and_output.as_mut_ptr(),
            );
        }
        assert_eq!(
            &input_and_output,
            &Sha256::digest(b"12345678901234567890123456789012")[..],
        );
    }
}
