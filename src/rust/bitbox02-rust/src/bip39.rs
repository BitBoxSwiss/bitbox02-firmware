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
use alloc::vec::Vec;

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

/// Decode a BIP39 mnemonic.
pub fn mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mnemonic =
        bip39::Mnemonic::parse_in_normalized(bip39::Language::English, mnemonic).map_err(|_| ())?;
    let (seed, seed_len) = mnemonic.to_entropy_array();
    Ok(zeroize::Zeroizing::new(seed[..seed_len].to_vec()))
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

    #[test]
    fn test_mnemonic_to_seed() {
        assert!(mnemonic_to_seed("invalid").is_err());

        // Zero seed
        assert_eq!(
            mnemonic_to_seed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap().as_ref() as &[u8],
            &[0u8; 16],
        );

        // 12 words
        assert_eq!(
            mnemonic_to_seed(
                "trust cradle viable innocent stand equal little small junior frost laundry room"
            )
            .unwrap()
            .as_ref() as &[u8],
            b"\xe9\xa6\x3f\xcd\x3a\x4d\x48\x98\x20\xa6\x63\x79\x2b\xad\xf6\xdd",
        );

        // 18 words
        assert_eq!(
            mnemonic_to_seed("pupil parent toe bright slam plastic spy suspect verb battle nominee loan call crystal upset razor luggage join").unwrap().as_ref() as &[u8],
            b"\xad\xf4\x07\x8e\x0e\x0c\xb1\x4c\x34\xd6\xd6\xf2\x82\x6a\x57\xc1\x82\x06\x6a\xbb\xcd\x95\x84\xcf",
        );

        // 24 words
        assert_eq!(
            mnemonic_to_seed("purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay").unwrap().as_ref() as &[u8],
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );
    }
}
