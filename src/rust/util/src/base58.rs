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

/// Base58Check-encode the input.
#[cfg(feature = "c-unit-testing")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_base58_encode_check(
    buf: crate::bytes::Bytes,
    mut out: crate::bytes::BytesMut,
) -> bool {
    if buf.is_empty() {
        return false;
    }
    let encoded = bitcoin::base58::encode_check(buf.as_ref());
    out.as_mut()[..encoded.len()].copy_from_slice(encoded.as_bytes());
    // Null-terminator.
    out.as_mut()[encoded.len()] = 0;
    true
}

#[cfg(all(test, feature = "c-unit-testing"))]
mod tests {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn test_rust_base58_encode_check() {
        let buf = b"test";
        let mut result_buf = [0u8; 100];
        assert!(rust_base58_encode_check(
            unsafe { crate::bytes::rust_util_bytes(buf.as_ptr(), buf.len()) },
            unsafe { crate::bytes::rust_util_bytes_mut(result_buf.as_mut_ptr(), result_buf.len()) },
        ));
        let expected = b"LUC1eAJa5jW\0";
        assert_eq!(&result_buf[..expected.len()], expected);
    }
}
