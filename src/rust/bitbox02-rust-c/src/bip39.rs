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

/// # Safety
///
/// The passphrase must be not NULL and null-terminated.
///
/// `seed` must be 16, 24 or 32 bytes long.
/// `out` must be exactly 64 bytes long.
#[no_mangle]
pub unsafe extern "C" fn rust_derive_bip39_seed(
    seed: crate::util::Bytes,
    passphrase: *const core::ffi::c_char,
    mut out: crate::util::BytesMut,
) {
    let mnemonic =
        bip39::Mnemonic::from_entropy_in(bip39::Language::English, seed.as_ref()).unwrap();
    let passphrase = core::ffi::CStr::from_ptr(passphrase);
    let bip39_seed =
        zeroize::Zeroizing::new(mnemonic.to_seed_normalized(passphrase.to_str().unwrap()));
    out.as_mut().clone_from_slice(&bip39_seed[..]);
}

#[no_mangle]
pub extern "C" fn rust_get_bip39_word(idx: u16, mut out: crate::util::BytesMut) -> bool {
    let word = match bitbox02_rust::bip39::get_word(idx) {
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
    fn test_rust_derive_bip39_seed() {
        struct Test {
            seed: &'static str,
            passphrase: &'static core::ffi::CStr,
            expected_bip39_seed: &'static str,
        }

        let tests = &[
            // 16 byte seed
            Test {
                seed: "fb5cf00d5ea61059fa066e25a6be9544",
                passphrase: c"",
                expected_bip39_seed: "f4577e463be595868060e5a763328153155b4167cd284998c8c6096d044742372020f5b052d0c41c1c5e6a6a7da2cb8a367aaaa074fab7773e8d5b2f684257ed",
            },
            Test {
                seed: "fb5cf00d5ea61059fa066e25a6be9544",
                passphrase: c"password",
                expected_bip39_seed: "5922fb7630bc7cb871af102f733b6bdb8f05945147cd4646a89056fde0bdad5c3a4ff5be3f9e7af535f570e7053b5b22472555b331bc89cb797c306f7eb6a5a1",
            },
            // 24 byte seed
            Test {
                seed: "23705a91b177b49822f28b3f1a60072d113fcaff4f250191",
                passphrase: c"",
                expected_bip39_seed: "4a2a016a6d90eb3a79b7931ca0a172df5c5bfee3e5b47f0fd84bc0791ea3bbc9476c3d5de71cdb12c37e93c2aa3d5c303257f1992aed400fc5bbfc7da787bfa7",
            },
            Test {
                seed: "23705a91b177b49822f28b3f1a60072d113fcaff4f250191",
                passphrase: c"password",
                expected_bip39_seed: "bc317ee0f88870254be32274d63ec2b0e962bf09f3ca04287912bfc843f2fab7c556f8657cadc924f99a217b0daa91898303a8414102031a125c50023e45a80b",
            },
            // 32 byte seed
            Test {
                seed: "bd83a008b3b78c8cc56c678d1b7bfc651cc5be8242f44b5c0db96a34ee297833",
                passphrase: c"",
                expected_bip39_seed: "63f844e2c61ecfb20f9100de381a7a9ec875b085f5ac7735a2ba4d615a0f4147b87be402f65651969130683deeef752760c09e291604fe4b89d61ffee2630be8",
            },
            Test {
                seed: "bd83a008b3b78c8cc56c678d1b7bfc651cc5be8242f44b5c0db96a34ee297833",
                passphrase: c"password",
                expected_bip39_seed: "42e90dacd61f3373542d212f0fb9c291dcea84a6d85034272372dde7188638a98527280d65e41599f30d3434d8ee3d4747dbb84801ff1a851d2306c7d1648374",
            },
        ];

        for test in tests {
            let seed = hex::decode(test.seed).unwrap();
            let mut bip39_seed = [0u8; 64];
            unsafe {
                rust_derive_bip39_seed(
                    crate::util::rust_util_bytes(seed.as_ptr(), seed.len()),
                    test.passphrase.as_ptr(),
                    crate::util::rust_util_bytes_mut(bip39_seed.as_mut_ptr(), bip39_seed.len()),
                );
            }
            assert_eq!(hex::encode(bip39_seed).as_str(), test.expected_bip39_seed);
        }
    }

    #[test]
    fn test_rust_get_bip39_word() {
        let mut word = [1u8; 10];
        assert!(!rust_get_bip39_word(2048, unsafe {
            crate::util::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));

        let mut word = [1u8; 10];
        // 7 is too short, missing the null terminator.
        assert!(!rust_get_bip39_word(0, unsafe {
            crate::util::rust_util_bytes_mut(word.as_mut_ptr(), 7)
        }));
        // 8 is just enough.
        assert!(rust_get_bip39_word(0, unsafe {
            crate::util::rust_util_bytes_mut(word.as_mut_ptr(), 8)
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "abandon"
        );
        let mut word = [1u8; 10];
        assert!(rust_get_bip39_word(2047, unsafe {
            crate::util::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "zoo"
        );
        let mut word = [1u8; 10];
        assert!(rust_get_bip39_word(563, unsafe {
            crate::util::rust_util_bytes_mut(word.as_mut_ptr(), word.len())
        }));
        assert_eq!(
            bitbox02::util::str_from_null_terminated(&word).unwrap(),
            "edit"
        );
    }
}
