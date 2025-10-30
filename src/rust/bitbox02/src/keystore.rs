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

pub fn _is_locked() -> bool {
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

pub fn _unlock(password: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
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

#[cfg(feature = "testing")]
pub fn test_get_retained_seed_encrypted() -> &'static [u8] {
    unsafe {
        let mut len = 0usize;
        let ptr = bitbox02_sys::keystore_test_get_retained_seed_encrypted(&mut len);
        core::slice::from_raw_parts(ptr, len)
    }
}

#[cfg(feature = "testing")]
pub fn test_get_retained_bip39_seed_encrypted() -> &'static [u8] {
    unsafe {
        let mut len = 0usize;
        let ptr = bitbox02_sys::keystore_test_get_retained_bip39_seed_encrypted(&mut len);
        core::slice::from_raw_parts(ptr, len)
    }
}

/// Unlocks the bip39 seed. The input seed must be the keystore seed (i.e. must match the output
/// of `keystore_copy_seed()`).
/// `mnemonic_passphrase` is the bip39 passphrase used in the derivation. Use the empty string if no
/// passphrase is needed or provided.
pub async fn _unlock_bip39(
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
    if _is_locked() {
        return Err(());
    }
    unsafe { ROOT_FINGERPRINT.read().ok_or(()).map(|fp| fp.to_vec()) }
}

pub fn _create_and_store_seed(password: &str, host_entropy: &[u8]) -> Result<(), Error> {
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

pub fn _copy_bip39_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
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

pub fn _secp256k1_nonce_commit(
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

pub fn _encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), Error> {
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

#[cfg(feature = "testing")]
pub fn mock_unlocked(seed: &[u8]) {
    unsafe {
        bitbox02_sys::keystore_mock_unlocked(seed.as_ptr(), seed.len() as _, core::ptr::null())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1;

    use util::bb02_async::block_on;

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
}
