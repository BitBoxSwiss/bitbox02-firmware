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

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use util::cell::SyncUnsafeCell;

use bitcoin::secp256k1::{All, Secp256k1};

use core::convert::TryInto;

use bitbox02_sys::keystore_error_t;

/// Length of a compressed secp256k1 pubkey.
const EC_PUBLIC_KEY_LEN: usize = 33;
pub const MAX_SEED_LENGTH: usize = bitbox02_sys::KEYSTORE_MAX_SEED_LENGTH as usize;

static ROOT_FINGERPRINT: SyncUnsafeCell<Option<[u8; 4]>> = SyncUnsafeCell::new(None);

pub fn is_locked() -> bool {
    unsafe { bitbox02_sys::keystore_is_locked() }
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    MaxAttemptsExceeded,
    Unseeded,
    Memory,
    // Securechip error with the error code from securechip.c. 0 if the error is unspecified.
    SecureChip(i32),
    Salt,
    Hash,
    SeedSize,
    Encrypt,
}

impl core::convert::From<keystore_error_t> for Error {
    fn from(error: keystore_error_t) -> Self {
        match error {
            keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Error::MaxAttemptsExceeded,
            keystore_error_t::KEYSTORE_ERR_UNSEEDED => Error::Unseeded,
            keystore_error_t::KEYSTORE_ERR_MEMORY => Error::Memory,
            keystore_error_t::KEYSTORE_ERR_SEED_SIZE => Error::SeedSize,
            keystore_error_t::KEYSTORE_ERR_SECURECHIP => Error::SecureChip(0),
            keystore_error_t::KEYSTORE_ERR_SALT => Error::Salt,
            keystore_error_t::KEYSTORE_ERR_HASH => Error::Hash,
            keystore_error_t::KEYSTORE_ERR_ENCRYPT => Error::Encrypt,
            _ => panic!("cannot convert error"),
        }
    }
}

pub fn unlock(password: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    let mut remaining_attempts: u8 = 0;
    let mut securechip_result: i32 = 0;
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH].to_vec());
    let mut seed_len: usize = 0;
    match unsafe {
        bitbox02_sys::keystore_unlock(
            crate::util::str_to_cstr_vec(password)
                .unwrap()
                .as_ptr()
                .cast(),
            &mut remaining_attempts,
            &mut securechip_result,
            seed.as_mut_ptr(),
            &mut seed_len,
        )
    } {
        keystore_error_t::KEYSTORE_OK => {
            seed.truncate(seed_len);
            Ok(seed)
        }
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_SECURECHIP => Err(Error::SecureChip(securechip_result)),
        err => Err(err.into()),
    }
}

pub fn _lock() {
    unsafe { bitbox02_sys::keystore_lock() }

    unsafe { ROOT_FINGERPRINT.write(None) }
}

fn unlock_bip39_check(seed: &[u8]) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39_check(seed.as_ptr(), seed.len()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

fn unlock_bip39_finalize(bip39_seed: &[u8; 64]) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39_finalize(bip39_seed.as_ptr()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

async fn derive_bip39_seed(
    secp: &Secp256k1<All>,
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
            .fingerprint(secp)
            .to_bytes();

    (bip39_seed, root_fingerprint)
}

/// Unlocks the bip39 seed. The input seed must be the keystore seed (i.e. must match the output
/// of `keystore_copy_seed()`).
/// `mnemonic_passphrase` is the bip39 passphrase used in the derivation. Use the empty string if no
/// passphrase is needed or provided.
pub async fn unlock_bip39(
    secp: &Secp256k1<All>,
    seed: &[u8],
    mnemonic_passphrase: &str,
    yield_now: impl AsyncFn(),
) -> Result<(), Error> {
    unlock_bip39_check(seed)?;

    let (bip39_seed, root_fingerprint) =
        derive_bip39_seed(secp, seed, mnemonic_passphrase, &yield_now).await;

    let (bip39_seed_2, root_fingerprint_2) =
        derive_bip39_seed(secp, seed, mnemonic_passphrase, &yield_now).await;

    if bip39_seed != bip39_seed_2 || root_fingerprint != root_fingerprint_2 {
        return Err(Error::Memory);
    }

    unlock_bip39_finalize(bip39_seed.as_slice().try_into().unwrap())?;

    // Store root fingerprint.
    unsafe {
        ROOT_FINGERPRINT.write(Some(root_fingerprint));
    }
    Ok(())
}

pub fn root_fingerprint() -> Result<Vec<u8>, ()> {
    if is_locked() {
        return Err(());
    }
    unsafe { ROOT_FINGERPRINT.read().ok_or(()).map(|fp| fp.to_vec()) }
}

pub fn create_and_store_seed(password: &str, host_entropy: &[u8]) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::keystore_create_and_store_seed(
            crate::util::str_to_cstr_vec(password)
                .unwrap()
                .as_ptr()
                .cast(),
            host_entropy.as_ptr(),
            host_entropy.len() as _,
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        err => Err(err.into()),
    }
}

pub fn _copy_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH].to_vec());
    let mut seed_len: usize = 0;
    match unsafe { bitbox02_sys::keystore_copy_seed(seed.as_mut_ptr(), &mut seed_len) } {
        true => {
            seed.truncate(seed_len);
            Ok(seed)
        }
        false => Err(()),
    }
}

pub fn copy_bip39_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut bip39_seed = zeroize::Zeroizing::new(vec![0u8; 64]);
    match unsafe { bitbox02_sys::keystore_copy_bip39_seed(bip39_seed.as_mut_ptr()) } {
        true => Ok(bip39_seed),
        false => Err(()),
    }
}

pub fn bip39_mnemonic_from_seed(seed: &[u8]) -> Result<zeroize::Zeroizing<String>, ()> {
    let mnemonic = bip39::Mnemonic::from_entropy(seed).map_err(|_| ())?;
    Ok(zeroize::Zeroizing::new(mnemonic.to_string()))
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

pub fn _secp256k1_sign(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let mut signature = [0u8; 64];
    let mut recid: core::ffi::c_int = 0;
    match unsafe {
        bitbox02_sys::keystore_secp256k1_sign(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
            msg.as_ptr(),
            host_nonce.as_ptr(),
            signature.as_mut_ptr(),
            &mut recid,
        )
    } {
        true => Ok(SignResult {
            signature,
            recid: recid.try_into().unwrap(),
        }),
        false => Err(()),
    }
}

pub fn secp256k1_nonce_commit(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    let mut signer_commitment = [0u8; EC_PUBLIC_KEY_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_nonce_commit(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
            msg.as_ptr(),
            host_commitment.as_ptr(),
            signer_commitment.as_mut_ptr(),
        )
    } {
        true => Ok(signer_commitment),
        false => Err(()),
    }
}

pub fn bip39_mnemonic_to_seed(mnemonic: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mnemonic =
        bip39::Mnemonic::parse_in_normalized(bip39::Language::English, mnemonic).map_err(|_| ())?;
    let (seed, seed_len) = mnemonic.to_entropy_array();
    Ok(zeroize::Zeroizing::new(seed[..seed_len].to_vec()))
}

pub fn encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::keystore_encrypt_and_store_seed(
            seed.as_ptr(),
            seed.len(),
            crate::util::str_to_cstr_vec(password)
                .unwrap()
                .as_ptr()
                .cast(),
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        err => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1;

    use crate::testing::mock_memory;
    use util::bb02_async::block_on;

    #[test]
    fn test_secp256k1_nonce_commit() {
        let secp = secp256k1::Secp256k1::new();

        let private_key =
            hex::decode("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c")
                .unwrap();
        let msg = [0x88u8; 32];
        let host_commitment = [0xabu8; 32];

        let client_commitment = secp256k1_nonce_commit(
            &secp,
            &private_key.try_into().unwrap(),
            &msg,
            &host_commitment,
        )
        .unwrap();
        assert_eq!(
            hex::encode(client_commitment),
            "0381e4136251c87f2947b735159c6dd644a7b58d35b437e20c878e5129f1320e5e",
        );
    }

    #[test]
    fn test_bip39_mnemonic_to_seed() {
        assert!(bip39_mnemonic_to_seed("invalid").is_err());

        // Zero seed
        assert_eq!(
            bip39_mnemonic_to_seed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap().as_ref() as &[u8],
            &[0u8; 16],
        );

        // 12 words
        assert_eq!(
            bip39_mnemonic_to_seed(
                "trust cradle viable innocent stand equal little small junior frost laundry room"
            )
            .unwrap()
            .as_ref() as &[u8],
            b"\xe9\xa6\x3f\xcd\x3a\x4d\x48\x98\x20\xa6\x63\x79\x2b\xad\xf6\xdd",
        );

        // 18 words
        assert_eq!(
            bip39_mnemonic_to_seed("pupil parent toe bright slam plastic spy suspect verb battle nominee loan call crystal upset razor luggage join").unwrap().as_ref() as &[u8],
            b"\xad\xf4\x07\x8e\x0e\x0c\xb1\x4c\x34\xd6\xd6\xf2\x82\x6a\x57\xc1\x82\x06\x6a\xbb\xcd\x95\x84\xcf",
        );

        // 24 words
        assert_eq!(
            bip39_mnemonic_to_seed("purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay").unwrap().as_ref() as &[u8],
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );
    }

    #[test]
    fn test_bip39_mnemonic_from_seed() {
        // 12 words
        let seed = b"\xae\x6a\x40\x26\x1f\x0a\xcc\x16\x57\x04\x9c\xb2\x1a\xf5\xfb\xf7";
        assert_eq!(
            bip39_mnemonic_from_seed(seed).unwrap().as_str(),
            "purpose faith another dignity proud arctic foster near rare stumble leave urge",
        );

        // 18 words
        let seed = b"\x2a\x3e\x07\xa9\xe7\x5e\xd7\x3a\xa6\xb2\xe1\xaf\x90\x3d\x50\x17\xde\x80\x4f\xdf\x2b\x45\xc2\x4b";
        assert_eq!(
            bip39_mnemonic_from_seed(seed).unwrap().as_str(),
            "clay usual tuna solid uniform outer onion found question limit favorite cook trend child lake hamster seat foot",
        );

        // 24 words
        let seed = b"\x24\x1d\x5b\x78\x35\x90\xc2\x1f\x79\x69\x8e\x7c\xe8\x92\xdd\x03\xfb\x2c\x8f\xad\xc2\x44\x0e\xc2\x3a\xa5\xde\x9e\x2d\x23\x81\xb0";
        assert_eq!(
            bip39_mnemonic_from_seed(seed).unwrap().as_str(),
            "catch turn task hen around autumn toss crack language duty resemble among ready elephant require embrace attract balcony practice rule tissue mushroom almost athlete",
        );

        // Invalid seed side
        assert!(bip39_mnemonic_from_seed(b"foo").is_err());
    }

    #[test]
    fn test_unlock() {
        mock_memory();
        _lock();

        assert!(matches!(unlock("password"), Err(Error::Unseeded)));

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        let mock_salt_root =
            hex::decode("3333333333333333444444444444444411111111111111112222222222222222")
                .unwrap();
        crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        _lock();

        // First call: unlock. The first one does a seed rentention (1 securechip event).
        crate::securechip::fake_event_counter_reset();
        assert_eq!(unlock("password").unwrap().as_slice(), seed);
        assert_eq!(crate::securechip::fake_event_counter(), 6);

        // Loop to check that unlocking works while unlocked.
        for _ in 0..2 {
            // Further calls perform a password check.The password check does not do the retention
            // so it ends up needing one secure chip operation less.
            crate::securechip::fake_event_counter_reset();
            assert_eq!(unlock("password").unwrap().as_slice(), seed);
            assert_eq!(crate::securechip::fake_event_counter(), 5);
        }

        // Also check that the retained seed was encrypted with the expected encryption key.
        let decrypted = {
            let retained_seed_encrypted: &[u8] = unsafe {
                let mut len = 0usize;
                let ptr = bitbox02_sys::keystore_test_get_retained_seed_encrypted(&mut len);
                core::slice::from_raw_parts(ptr, len)
            };
            let expected_retained_seed_secret =
                hex::decode("b156be416530c6fc00018844161774a3546a53ac6dd4a0462608838e216008f7")
                    .unwrap();
            bitbox_aes::decrypt_with_hmac(&expected_retained_seed_secret, retained_seed_encrypted)
                .unwrap()
        };
        assert_eq!(decrypted.as_slice(), seed.as_slice());

        // First 9 wrong attempts.
        for i in 1..bitbox02_sys::MAX_UNLOCK_ATTEMPTS {
            assert!(matches!(
                unlock("invalid password"),
                Err(Error::IncorrectPassword { remaining_attempts }) if remaining_attempts
                    == (bitbox02_sys::MAX_UNLOCK_ATTEMPTS  - i) as u8
            ));
            // Still seeded.
            assert!(crate::memory::is_seeded());
            // Wrong password does not lock the keystore again if already unlocked.
            assert!(_copy_seed().is_ok());
        }
        // Last attempt, triggers reset.
        assert!(matches!(
            unlock("invalid password"),
            Err(Error::MaxAttemptsExceeded),
        ));
        // Last wrong attempt locks & resets. There is no more seed.
        assert!(!crate::memory::is_seeded());
        assert!(_copy_seed().is_err());
        assert!(matches!(unlock("password"), Err(Error::Unseeded)));
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

        let secp = secp256k1::Secp256k1::new();
        for test in tests {
            let seed = hex::decode(test.seed).unwrap();
            let (bip39_seed, root_fingerprint) = block_on(derive_bip39_seed(
                &secp,
                &seed,
                test.passphrase,
                async || {},
            ));
            assert_eq!(hex::encode(bip39_seed).as_str(), test.expected_bip39_seed);
            assert_eq!(
                hex::encode(root_fingerprint).as_str(),
                test.expected_root_fingerprint
            );
        }
    }

    #[test]
    fn test_unlock_bip39() {
        mock_memory();
        _lock();

        let seed = hex::decode("1111111111111111222222222222222233333333333333334444444444444444")
            .unwrap();

        let mock_salt_root =
            hex::decode("3333333333333333444444444444444411111111111111112222222222222222")
                .unwrap();
        crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        let secp = secp256k1::Secp256k1::new();

        assert!(root_fingerprint().is_err());
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(root_fingerprint().is_err());
        // Incorrect seed passed
        assert!(
            block_on(unlock_bip39(
                &secp,
                b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "foo",
                async || {}
            ))
            .is_err()
        );
        // Correct seed passed.
        crate::securechip::fake_event_counter_reset();
        assert!(block_on(unlock_bip39(&secp, &seed, "foo", async || {})).is_ok());
        assert_eq!(crate::securechip::fake_event_counter(), 1);
        assert_eq!(root_fingerprint(), Ok(vec![0xf1, 0xbc, 0x3c, 0x46]),);

        let expected_bip39_seed = hex::decode("2b3c63de86f0f2b13cc6a36c1ba2314fbc1b40c77ab9cb64e96ba4d5c62fc204748ca6626a9f035e7d431bce8c9210ec0bdffc2e7db873dee56c8ac2153eee9a").unwrap();

        assert_eq!(
            copy_bip39_seed().unwrap().as_slice(),
            expected_bip39_seed.as_slice()
        );

        // Check that the retained bip39 seed was encrypted with the expected encryption key.
        let decrypted = {
            let retained_bip39_seed_encrypted: &[u8] = unsafe {
                let mut len = 0usize;
                let ptr = bitbox02_sys::keystore_test_get_retained_bip39_seed_encrypted(&mut len);
                core::slice::from_raw_parts(ptr, len)
            };
            let expected_retained_bip39_seed_secret =
                hex::decode("856d9a8c1ea42a69ae76324244ace674397ff1360a4ba4c85ffbd42cee8a7f29")
                    .unwrap();
            bitbox_aes::decrypt_with_hmac(
                &expected_retained_bip39_seed_secret,
                retained_bip39_seed_encrypted,
            )
            .unwrap()
        };
        assert_eq!(decrypted.as_slice(), expected_bip39_seed.as_slice());
    }

    #[test]
    fn test_create_and_store_seed() {
        let mock_salt_root =
            hex::decode("3333333333333333444444444444444411111111111111112222222222222222")
                .unwrap();

        let host_entropy =
            hex::decode("25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe")
                .unwrap();

        // Invalid seed lengths
        for size in [8, 24, 40] {
            assert!(matches!(
                create_and_store_seed("password", &host_entropy[..size]),
                Err(Error::SeedSize)
            ));
        }

        // Hack to get the random bytes that will be used.
        let seed_random = {
            crate::random::fake_reset();
            crate::random::random_32_bytes()
        };

        // Derived from mock_salt_root and "password".
        let password_salted_hashed =
            hex::decode("e8c70a20d9108fbb9454b1b8e2d7373e78cbaf9de025ab2d4f4d3c7a6711694c")
                .unwrap();

        // expected_seed = seed_random ^ host_entropy ^ password_salted_hashed
        let expected_seed: Vec<u8> = seed_random
            .into_iter()
            .zip(host_entropy.iter())
            .zip(password_salted_hashed)
            .map(|((a, &b), c)| a ^ b ^ c)
            .collect();

        for size in [16, 32] {
            mock_memory();
            crate::random::fake_reset();
            crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();
            _lock();

            assert!(create_and_store_seed("password", &host_entropy[..size]).is_ok());
            assert_eq!(_copy_seed().unwrap().as_slice(), &expected_seed[..size]);
            // Check the seed has been stored encrypted with the expected encryption key.
            // Decrypt and check seed.
            let cipher = crate::memory::get_encrypted_seed_and_hmac().unwrap();

            // Same as Python:
            // import hmac, hashlib; hmac.digest(b"unit-test", b"password", hashlib.sha256).hex()
            // See also: mock_securechip.c
            let expected_encryption_key =
                hex::decode("e56de448f5f1d29cdcc0e0099007309afe4d5a3ef2349e99dcc41840ad98409e")
                    .unwrap();
            let decrypted =
                bitbox_aes::decrypt_with_hmac(&expected_encryption_key, &cipher).unwrap();
            assert_eq!(decrypted.as_slice(), &expected_seed[..size]);
        }
    }

    // This tests that you can create a keystore, unlock it, and then do this again. This is an
    // expected workflow for when the wallet setup process is restarted after seeding and unlocking,
    // but before creating a backup, in which case a new seed is created.
    #[test]
    fn test_create_and_unlock_twice() {
        mock_memory();
        _lock();

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();
        let seed2 = hex::decode("c28135734876aff9ccf4f1d60df8d19a0a38fd02085883f65fc608eb769a635d")
            .unwrap();
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        // Create new (different) seed.
        assert!(encrypt_and_store_seed(&seed2, "password").is_ok());
        assert_eq!(_copy_seed().unwrap().as_slice(), &seed2);
    }

    // Functional test to store seeds, lock/unlock, retrieve seed.
    #[test]
    fn test_seeds() {
        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        for seed_size in [16, 24, 32] {
            mock_memory();
            _lock();

            // Can repeat until initialized - initialized means backup has been created.
            for _ in 0..2 {
                assert!(encrypt_and_store_seed(&seed[..seed_size], "foo").is_ok());
            }
            // Also unlocks, so we can get the retained seed.
            assert_eq!(_copy_seed().unwrap().as_slice(), &seed[..seed_size]);

            _lock();
            // Can't get seed before unlock.
            assert!(_copy_seed().is_err());

            // Wrong password.
            assert!(matches!(
                unlock("bar"),
                Err(Error::IncorrectPassword {
                    remaining_attempts: 9
                })
            ));

            // Correct password. First time: unlock. After unlock, it becomes a password check.
            for _ in 0..3 {
                assert_eq!(unlock("foo").unwrap().as_slice(), &seed[..seed_size]);
            }
            assert_eq!(_copy_seed().unwrap().as_slice(), &seed[..seed_size]);

            // Can't store new seed once initialized.
            crate::memory::set_initialized().unwrap();
            assert!(matches!(
                encrypt_and_store_seed(&seed[..seed_size], "foo"),
                Err(Error::Memory)
            ));
        }
    }
}
