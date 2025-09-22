// Copyright 2025 Shift Crypto AG
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

use alloc::string::{String, ToString};

/// `idx` must be smaller than BIP39_WORDLIST_LEN.
pub fn get_word(idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    Ok(zeroize::Zeroizing::new(
        bip39::Language::English
            .word_list()
            .get(idx as usize)
            .ok_or(())?
            .to_string(),
    ))
}

// C API

#[unsafe(no_mangle)]
pub extern "C" fn rust_get_bip39_word(idx: u16, mut out: util::bytes::BytesMut) -> bool {
    let word = match get_word(idx) {
        Err(()) => return false,
        Ok(w) => w,
    };
    let bytes = word.as_bytes();
    let out = out.as_mut();
    if out.len() < bytes.len() + 1 {
        return false;
    }
    out[..bytes.len()].clone_from_slice(bytes);
    out[bytes.len()] = 0;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_get_bip39_word() {
        let mut word = [1u8; 10];
        assert!(!rust_get_bip39_word(2048, unsafe {
            util::bytes::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));

        let mut word = [1u8; 10];
        // 7 is too short, missing the null terminator.
        assert!(!rust_get_bip39_word(0, unsafe {
            util::bytes::rust_util_bytes_mut(word.as_mut_ptr(), 7)
        }));
        // 8 is just enough.
        assert!(rust_get_bip39_word(0, unsafe {
            util::bytes::rust_util_bytes_mut(word.as_mut_ptr(), 8)
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "abandon"
        );
        let mut word = [1u8; 10];
        assert!(rust_get_bip39_word(2047, unsafe {
            util::bytes::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "zoo"
        );
        let mut word = [1u8; 10];
        assert!(rust_get_bip39_word(563, unsafe {
            util::bytes::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "edit"
        );
    }

    #[test]
    fn test_get_word() {
        assert!(get_word(2048).is_err());

        assert_eq!(get_word(0).unwrap().as_ref() as &str, "abandon");
        assert_eq!(get_word(2047).unwrap().as_ref() as &str, "zoo");
        assert_eq!(get_word(563).unwrap().as_ref() as &str, "edit");
    }
}
