// Copyright 2022 Shift Crypto AG
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

use alloc::vec::Vec;

/// Decode a base58 encoded string. Errors if the decoded output would be larget than 1024 bytes.
pub fn decode(s: &str) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; 1024];
    let mut written: util::c_types::c_uint = 0;
    let res = unsafe {
        bitbox02_sys::wally_base58_to_bytes(
            crate::util::str_to_cstr_vec(s)?.as_ptr(),
            0,
            out.as_mut_ptr(),
            out.len() as _,
            &mut written,
        )
    };
    if res == bitbox02_sys::WALLY_OK as i32 {
        Ok(out[..written as usize].to_vec())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert!(decode("").is_err());
        assert!(decode("invalid").is_err());

        // First testcase of https://github.com/ThePiachu/Bitcoin-Unit-Tests/blob/715efcbb457b71e78377d41131d789aa43cc2435/Address/Address%20Generation%20Test%201.txt (line 10 decoded yields line 9).
        // Python: base58.b58decode('16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM').hex()
        assert_eq!(
            decode("16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM"),
            Ok(b"\x00\x01\x09\x66\x77\x60\x06\x95\x3d\x55\x67\x43\x9e\x5e\x39\xf8\x6a\x0d\x27\x3b\xee\xd6\x19\x67\xf6".to_vec()),
        );
    }
}
