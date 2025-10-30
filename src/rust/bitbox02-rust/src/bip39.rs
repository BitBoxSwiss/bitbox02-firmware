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

/// Encode a seed as a BIP39 mnemonic.
pub fn mnemonic_from_seed(seed: &[u8]) -> Result<zeroize::Zeroizing<String>, ()> {
    let mnemonic = bip39::Mnemonic::from_entropy(seed).map_err(|_| ())?;
    Ok(zeroize::Zeroizing::new(mnemonic.to_string()))
}

/// Decode a BIP39 mnemonic.
pub fn mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mnemonic =
        bip39::Mnemonic::parse_in_normalized(bip39::Language::English, mnemonic).map_err(|_| ())?;
    let (seed, seed_len) = mnemonic.to_entropy_array();
    Ok(zeroize::Zeroizing::new(seed[..seed_len].to_vec()))
}

/// Derives the bip39 seed and returns it and the bip32 root fingerprint.
/// `mnemonic_passphrase` is the bip39 passphrase used in the derivation.
/// `yield_now` is called in each of the 2048 bip39 pbkdf2 iterations.
pub async fn derive_seed(
    seed: &[u8],
    mnemonic_passphrase: &str,
    yield_now: impl AsyncFn(),
) -> (zeroize::Zeroizing<[u8; 64]>, [u8; 4]) {
    let mnemonic = bip39::Mnemonic::from_entropy_in(bip39::Language::English, seed).unwrap();
    let bip39_seed: zeroize::Zeroizing<[u8; 64]> = zeroize::Zeroizing::new(
        mnemonic
            .to_seed_normalized_async(mnemonic_passphrase, yield_now)
            .await,
    );
    let root_fingerprint: [u8; 4] =
        bitcoin::bip32::Xpriv::new_master(bitcoin::NetworkKind::Main, bip39_seed.as_ref())
            .unwrap()
            .fingerprint(crate::secp256k1::SECP256K1)
            .to_bytes();

    (bip39_seed, root_fingerprint)
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
    use util::bb02_async::block_on;

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
    fn test_mnemonic_from_seed() {
        // 12 words
        let seed = b"\xae\x6a\x40\x26\x1f\x0a\xcc\x16\x57\x04\x9c\xb2\x1a\xf5\xfb\xf7";
        assert_eq!(
            mnemonic_from_seed(seed).unwrap().as_str(),
            "purpose faith another dignity proud arctic foster near rare stumble leave urge",
        );

        // 18 words
        let seed = b"\x2a\x3e\x07\xa9\xe7\x5e\xd7\x3a\xa6\xb2\xe1\xaf\x90\x3d\x50\x17\xde\x80\x4f\xdf\x2b\x45\xc2\x4b";
        assert_eq!(
            mnemonic_from_seed(seed).unwrap().as_str(),
            "clay usual tuna solid uniform outer onion found question limit favorite cook trend child lake hamster seat foot",
        );

        // 24 words
        let seed = b"\x24\x1d\x5b\x78\x35\x90\xc2\x1f\x79\x69\x8e\x7c\xe8\x92\xdd\x03\xfb\x2c\x8f\xad\xc2\x44\x0e\xc2\x3a\xa5\xde\x9e\x2d\x23\x81\xb0";
        assert_eq!(
            mnemonic_from_seed(seed).unwrap().as_str(),
            "catch turn task hen around autumn toss crack language duty resemble among ready elephant require embrace attract balcony practice rule tissue mushroom almost athlete",
        );

        // Invalid seed side
        assert!(mnemonic_from_seed(b"foo").is_err());
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

    #[test]
    fn test_derive_bip39_seed() {
        struct Test {
            seed: &'static str,
            passphrase: &'static str,
            expected_bip39_seed: &'static str,
            expected_root_fingerprint: &'static str,
        }

        let tests = &[
            // 16 byte seed
            Test {
                seed: "fb5cf00d5ea61059fa066e25a6be9544",
                passphrase: "",
                expected_bip39_seed: "f4577e463be595868060e5a763328153155b4167cd284998c8c6096d044742372020f5b052d0c41c1c5e6a6a7da2cb8a367aaaa074fab7773e8d5b2f684257ed",
                expected_root_fingerprint: "0b2fa4e5",
            },
            Test {
                seed: "fb5cf00d5ea61059fa066e25a6be9544",
                passphrase: "password",
                expected_bip39_seed: "5922fb7630bc7cb871af102f733b6bdb8f05945147cd4646a89056fde0bdad5c3a4ff5be3f9e7af535f570e7053b5b22472555b331bc89cb797c306f7eb6a5a1",
                expected_root_fingerprint: "c4062d44",
            },
            // 24 byte seed
            Test {
                seed: "23705a91b177b49822f28b3f1a60072d113fcaff4f250191",
                passphrase: "",
                expected_bip39_seed: "4a2a016a6d90eb3a79b7931ca0a172df5c5bfee3e5b47f0fd84bc0791ea3bbc9476c3d5de71cdb12c37e93c2aa3d5c303257f1992aed400fc5bbfc7da787bfa7",
                expected_root_fingerprint: "62fd19e0",
            },
            Test {
                seed: "23705a91b177b49822f28b3f1a60072d113fcaff4f250191",
                passphrase: "password",
                expected_bip39_seed: "bc317ee0f88870254be32274d63ec2b0e962bf09f3ca04287912bfc843f2fab7c556f8657cadc924f99a217b0daa91898303a8414102031a125c50023e45a80b",
                expected_root_fingerprint: "c745266d",
            },
            // 32 byte seed
            Test {
                seed: "bd83a008b3b78c8cc56c678d1b7bfc651cc5be8242f44b5c0db96a34ee297833",
                passphrase: "",
                expected_bip39_seed: "63f844e2c61ecfb20f9100de381a7a9ec875b085f5ac7735a2ba4d615a0f4147b87be402f65651969130683deeef752760c09e291604fe4b89d61ffee2630be8",
                expected_root_fingerprint: "93ba3a7b",
            },
            Test {
                seed: "bd83a008b3b78c8cc56c678d1b7bfc651cc5be8242f44b5c0db96a34ee297833",
                passphrase: "password",
                expected_bip39_seed: "42e90dacd61f3373542d212f0fb9c291dcea84a6d85034272372dde7188638a98527280d65e41599f30d3434d8ee3d4747dbb84801ff1a851d2306c7d1648374",
                expected_root_fingerprint: "b95c9318",
            },
        ];

        for test in tests {
            let seed = hex::decode(test.seed).unwrap();
            let (bip39_seed, root_fingerprint) =
                block_on(derive_seed(&seed, test.passphrase, async || {}));
            assert_eq!(hex::encode(bip39_seed).as_str(), test.expected_bip39_seed);
            assert_eq!(
                hex::encode(root_fingerprint).as_str(),
                test.expected_root_fingerprint
            );
        }
    }
}
