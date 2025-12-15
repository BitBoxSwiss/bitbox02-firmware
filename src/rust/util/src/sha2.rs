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
use core::ffi::{c_uchar, c_void};
use sha2::Digest;
use sha2::Sha256;

/// Result must be freed by calling `rust_sha256_finish()` or `rust_sha256_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn rust_sha256_new() -> *mut c_void {
    Box::into_raw(Box::new(Sha256::new())) as *mut _
}

/// Safety: ctx must be a valid sha256 context produced by `rust_sha256_new()`. `data` must be a
/// valid buffer for `len` bytes.
// NOTE: we specifically do not use util::Bytes, as it disallows NULL. Our data can be 0 though, as
// the booloader starts at 0 and is hashed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_sha256_update(ctx: *mut c_void, data: *const c_void, len: usize) {
    let data = unsafe { core::slice::from_raw_parts(data as *const u8, len) };
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let ctx = ctx as *mut Sha256;
    unsafe { (*ctx).update(data) };
}

/// Safety: ctx must be a pointer to a valid sha256 context produced by `rust_sha256_new()`.
/// `out` must be 32 bytes long.
/// After this, the hasher is dropped and `ctx` is set to NULL and must not be used anymore.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_sha256_finish(ctx: *mut *mut c_void, out: *mut c_uchar) {
    let out = unsafe { core::slice::from_raw_parts_mut(out, 32) };
    #[allow(clippy::cast_ptr_alignment)] // ctx is properly aligned, see `Box::into_raw`.
    let hasher = unsafe { Box::from_raw(*ctx as *mut Sha256) }; // dropped at the end
    let hash = hasher.finalize();
    out.copy_from_slice(&hash[..]);
    unsafe { *ctx = core::ptr::null_mut() };
}

/// Safety: data must be valid buffer for `len` bytes. `out` must be 32 bytes long.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_sha256(data: *const c_void, len: usize, out: *mut c_uchar) {
    let hash = {
        let data = unsafe { core::slice::from_raw_parts(data as *const u8, len) };
        Sha256::digest(data)
    };

    let out = unsafe { core::slice::from_raw_parts_mut(out, 32) };
    out.copy_from_slice(&hash[..]);
}

/// Safety: `key` and `data` must be valid buffers of the corresponding sizes. `out` must be 32
/// bytes long.
///
/// `out` may overlap with `data` (and/or `key`). This is supported safely: the HMAC is computed
/// first and only then written to `out`.
#[cfg(feature = "firmware")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_hmac_sha256(
    key: *const c_void,
    key_len: usize,
    data: *const c_void,
    data_len: usize,
    out: *mut c_uchar,
) {
    use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha256};

    let result: [u8; 32] = {
        let key = unsafe { core::slice::from_raw_parts(key as *const u8, key_len) };
        let data = unsafe { core::slice::from_raw_parts(data as *const u8, data_len) };

        let mut engine = HmacEngine::<sha256::Hash>::new(key);
        engine.input(data);
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        hmac_result.to_byte_array()
    };

    let out = unsafe { core::slice::from_raw_parts_mut(out, 32) };
    out.copy_from_slice(&result);
}

/// Safety: `key` and `data` must be a valid buffers of the corresponding sizes. `out` must be 64
/// bytes long.
///
/// `out` may overlap with `data` (and/or `key`). This is supported safely: the HMAC is computed
/// first and only then written to `out`.
#[cfg(feature = "firmware")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_hmac_sha512(
    key: *const c_void,
    key_len: usize,
    data: *const c_void,
    data_len: usize,
    out: *mut c_uchar,
) {
    use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha512};

    let result: [u8; 64] = {
        let key = unsafe { core::slice::from_raw_parts(key as *const u8, key_len) };
        let data = unsafe { core::slice::from_raw_parts(data as *const u8, data_len) };

        let mut engine = HmacEngine::<sha512::Hash>::new(key);
        engine.input(data);
        let hmac_result: Hmac<sha512::Hash> = Hmac::from_engine(engine);
        hmac_result.to_byte_array()
    };

    let out = unsafe { core::slice::from_raw_parts_mut(out, 64) };
    out.copy_from_slice(&result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;
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
    fn test_sha256_overlapping() {
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

    #[test]
    fn test_hmac_sha256() {
        let key = [0x0b_u8; 20];
        let data = b"Hi There";
        let mut out = [0u8; 32];
        unsafe {
            rust_hmac_sha256(
                key.as_ptr() as *const _,
                key.len(),
                data.as_ptr() as *const _,
                data.len(),
                out.as_mut_ptr(),
            );
        }
        let expected = hex!("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");
        assert_eq!(out, expected);
    }

    #[test]
    fn test_hmac_sha256_overlapping() {
        let key = [0x0b_u8; 20];
        let data = b"Hi There";
        let expected = hex!("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");

        // out == data
        let mut input_and_output = [0u8; 32];
        input_and_output[..data.len()].copy_from_slice(data);
        unsafe {
            rust_hmac_sha256(
                key.as_ptr() as *const _,
                key.len(),
                input_and_output.as_ptr() as *const _,
                data.len(),
                input_and_output.as_mut_ptr(),
            );
        }
        assert_eq!(input_and_output, expected);

        // out overlaps with data, but is not the same start address.
        let mut buf = [0u8; 64];
        buf[1..1 + data.len()].copy_from_slice(data);
        unsafe {
            rust_hmac_sha256(
                key.as_ptr() as *const _,
                key.len(),
                buf[1..].as_ptr() as *const _,
                data.len(),
                buf.as_mut_ptr(),
            );
        }
        assert_eq!(&buf[..32], &expected);
    }

    #[test]
    fn test_hmac_sha256_overlapping_key() {
        let mut key = [0x0b_u8; 32];
        let data = b"Hi There";
        let expected = hex!("198a607eb44bfbc69903a0f1cf2bbdc5ba0aa3f3d9ae3c1c7a3b1696a0b68cf7");

        // out == key
        unsafe {
            rust_hmac_sha256(
                key.as_ptr() as *const _,
                key.len(),
                data.as_ptr() as *const _,
                data.len(),
                key.as_mut_ptr(),
            );
        }
        assert_eq!(key, expected);
    }

    #[test]
    fn test_hmac_sha512() {
        let key = [0x0b_u8; 20];
        let data = b"Hi There";
        let mut out = [0u8; 64];
        unsafe {
            rust_hmac_sha512(
                key.as_ptr() as *const _,
                key.len(),
                data.as_ptr() as *const _,
                data.len(),
                out.as_mut_ptr(),
            );
        }
        let expected = hex!(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        );
        assert_eq!(out, expected);
    }

    #[test]
    fn test_hmac_sha512_overlapping() {
        let key = [0x0b_u8; 20];
        let data = b"Hi There";
        let expected = hex!(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        );

        // out == data
        let mut input_and_output = [0u8; 64];
        input_and_output[..data.len()].copy_from_slice(data);
        unsafe {
            rust_hmac_sha512(
                key.as_ptr() as *const _,
                key.len(),
                input_and_output.as_ptr() as *const _,
                data.len(),
                input_and_output.as_mut_ptr(),
            );
        }
        assert_eq!(input_and_output, expected);

        // out overlaps with data, but is not the same start address.
        let mut buf = [0u8; 96];
        buf[1..1 + data.len()].copy_from_slice(data);
        unsafe {
            rust_hmac_sha512(
                key.as_ptr() as *const _,
                key.len(),
                buf[1..].as_ptr() as *const _,
                data.len(),
                buf.as_mut_ptr(),
            );
        }
        assert_eq!(&buf[..64], &expected);
    }

    #[test]
    fn test_hmac_sha512_overlapping_key() {
        let mut key = [0x0b_u8; 64];
        let data = b"Hi There";
        let expected = hex!(
            "637edc6e01dce7e6742a99451aae82df23da3e92439e590e43e761b33e910fb8ac2878ebd5803f6f0b61dbce5e251ff8789a4722c1be65aea45fd464e89f8f5b"
        );

        // out == key
        unsafe {
            rust_hmac_sha512(
                key.as_ptr() as *const _,
                key.len(),
                data.as_ptr() as *const _,
                data.len(),
                key.as_mut_ptr(),
            );
        }
        assert_eq!(key, expected);
    }
}
