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
use core::fmt::Write;

use super::util::{Bytes, BytesMut, CStrMut};
use core::ops::{Add, Mul};
pub use num_bigint::BigUint;

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_ethereum_keypath_is_valid_address(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    ethereum::keypath::is_valid_keypath_address(keypath, expected_coin)
}

#[no_mangle]
pub extern "C" fn rust_ethereum_address_from_pubkey_hash(recipient: Bytes, mut out: CStrMut) {
    let recipient = arrayref::array_ref!(recipient.as_ref(), 0, 20);
    ethereum::address::from_pubkey_hash(recipient, &mut out).unwrap();
}

/// Serialize `bytes` into `out`, padding with zeroes from the left to fill 32 bytes.
/// `out` must be 32 bytes and `bytes` cannot be more than 32 bytes.
fn serialize_bigendian_to_32(bytes: &[u8], mut out: BytesMut) {
    out.reset();
    let out_slice = out.as_mut();
    let len = out_slice.len();
    if bytes.len() > 32 || len != 32 {
        panic!("serialize bigendian: invalid buffer size");
    }
    out_slice[len - bytes.len()..].copy_from_slice(bytes)
}

/// `out` must be of size 32.
#[no_mangle]
pub extern "C" fn rust_ethereum_bigint_mul(left: Bytes, right: Bytes, out: BytesMut) {
    let result = BigUint::from_bytes_be(left.as_ref())
        .mul(BigUint::from_bytes_be(right.as_ref()))
        .to_bytes_be();
    serialize_bigendian_to_32(&result, out)
}

/// `out` must be of size 32. The `left`, `right` and `out` bytes can overlap.
#[no_mangle]
pub extern "C" fn rust_ethereum_bigint_add(left: Bytes, right: Bytes, out: BytesMut) {
    let result = BigUint::from_bytes_be(left.as_ref())
        .add(BigUint::from_bytes_be(right.as_ref()))
        .to_bytes_be();
    serialize_bigendian_to_32(&result, out)
}

#[no_mangle]
pub extern "C" fn rust_ethereum_bigint_format(value: Bytes, decimals: usize, mut out: CStrMut) {
    let v = format!("{}", BigUint::from_bytes_be(value.as_ref()));
    out.write_str(&util::decimal::format(&v, decimals)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{rust_util_bytes, rust_util_cstr_mut};
    use std::prelude::v1::*;

    #[test]
    fn test_rust_ethereum_address_from_pubkey_hash() {
        let recipient =
            b"\xf4\xc2\x17\x10\xef\x8b\x5a\x5e\xc4\xbd\x37\x80\xa6\x87\xfe\x08\x34\x46\xe6\x7b";
        let recipient = rust_util_bytes(recipient.as_ptr(), recipient.len());
        let mut result = vec![0; 43];
        rust_ethereum_address_from_pubkey_hash(recipient, unsafe {
            rust_util_cstr_mut(result.as_mut_ptr(), result.len())
        });
        assert_eq!(
            b"0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B\0".to_vec(),
            result
        );
    }

    #[test]
    fn test_rust_ethereum_bigint_mul() {
        let left = (43532432323452243u64).to_be_bytes();
        let right = (43532432323452243u64).to_be_bytes();
        let mut out = [0u8; 32];
        rust_ethereum_bigint_mul(
            crate::util::rust_util_bytes(left.as_ptr(), left.len()),
            crate::util::rust_util_bytes(right.as_ptr(), right.len()),
            crate::util::rust_util_bytes_mut(out.as_mut_ptr(), out.len()),
        );
        assert_eq!(&out, b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x5d\x6f\x2e\x07\x09\x63\x0c\xb5\xab\x79\xa2\x11\xe8\xe9")
    }

    #[test]
    fn test_rust_ethereum_bigint_add() {
        let left = (43532432323452243u64).to_be_bytes();
        let right = (43532432323452243u64).to_be_bytes();
        let mut out = [0u8; 32];
        rust_ethereum_bigint_add(
            crate::util::rust_util_bytes(left.as_ptr(), left.len()),
            crate::util::rust_util_bytes(right.as_ptr(), right.len()),
            crate::util::rust_util_bytes_mut(out.as_mut_ptr(), out.len()),
        );
        assert_eq!(&out, b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x35\x51\x08\x79\xb2\x3a\xa6")
    }
}
