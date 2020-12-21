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

use super::util::{Bytes, CStrMut};

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
}
