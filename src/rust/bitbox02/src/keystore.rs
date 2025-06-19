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
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use core::convert::TryInto;

use bitbox02_sys::keystore_error_t;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;
pub const EC_PUBLIC_KEY_LEN: usize = bitbox02_sys::EC_PUBLIC_KEY_LEN as _;
pub const MAX_SEED_LENGTH: usize = bitbox02_sys::KEYSTORE_MAX_SEED_LENGTH as usize;

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

pub fn unlock(password: &str) -> Result<(), Error> {
    let mut remaining_attempts: u8 = 0;
    let mut securechip_result: i32 = 0;
    match unsafe {
        bitbox02_sys::keystore_unlock(
            crate::util::str_to_cstr_vec(password).unwrap().as_ptr(),
            &mut remaining_attempts,
            &mut securechip_result,
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_SECURECHIP => Err(Error::SecureChip(securechip_result)),
        err => Err(err.into()),
    }
}

pub fn lock() {
    unsafe { bitbox02_sys::keystore_lock() }
}

pub fn unlock_bip39(mnemonic_passphrase: &str) -> Result<(), Error> {
    if unsafe {
        bitbox02_sys::keystore_unlock_bip39(
            crate::util::str_to_cstr_vec(mnemonic_passphrase)
                .unwrap()
                .as_ptr(),
        )
    } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

pub fn create_and_store_seed(password: &str, host_entropy: &[u8]) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::keystore_create_and_store_seed(
            crate::util::str_to_cstr_vec(password).unwrap().as_ptr(),
            host_entropy.as_ptr(),
            host_entropy.len() as _,
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        err => Err(err.into()),
    }
}

pub fn copy_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
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

pub fn bip39_mnemonic_from_seed(seed: &[u8]) -> Result<zeroize::Zeroizing<String>, ()> {
    let mut mnemonic = zeroize::Zeroizing::new([0u8; 256]);
    match unsafe {
        bitbox02_sys::keystore_bip39_mnemonic_from_seed(
            seed.as_ptr(),
            seed.len() as _,
            mnemonic.as_mut_ptr(),
            mnemonic.len() as _,
        )
    } {
        false => Err(()),
        true => Ok(zeroize::Zeroizing::new(
            crate::util::str_from_null_terminated(&mnemonic[..])
                .unwrap()
                .into(),
        )),
    }
}

/// `idx` must be smaller than BIP39_WORDLIST_LEN.
pub fn get_bip39_word(idx: u16) -> Result<zeroize::Zeroizing<String>, ()> {
    let mut word_ptr: *mut u8 = core::ptr::null_mut();
    match unsafe { bitbox02_sys::keystore_get_bip39_word(idx, &mut word_ptr) } {
        false => Err(()),
        true => {
            let word = zeroize::Zeroizing::new(unsafe {
                crate::util::str_from_null_terminated_ptr(word_ptr)
                    .unwrap()
                    .into()
            });
            unsafe {
                bitbox02_sys::wally_free_string(word_ptr as _);
            }
            Ok(word)
        }
    }
}

/// An opaque C type which gives access to all BIP39 words.
pub struct Bip39Wordlist(Vec<*const u8>);

impl Bip39Wordlist {
    pub fn as_ptr(&self) -> *const *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for Bip39Wordlist {
    fn drop(&mut self) {
        for ptr in self.0.iter() {
            unsafe {
                bitbox02_sys::wally_free_string(*ptr as _);
            }
        }
    }
}

/// If indices is None, all BIP39 English words are returned, otherwise only the words of the given
/// indices in the BIP39 English wordlist.
pub fn get_bip39_wordlist(indices: Option<&[u16]>) -> Bip39Wordlist {
    let indices = match indices {
        Some(indices) => indices.to_vec(),
        None => (0..BIP39_WORDLIST_LEN).collect(),
    };
    Bip39Wordlist(
        indices
            .into_iter()
            .map(|i| {
                let mut word_ptr: *mut u8 = core::ptr::null_mut();
                match unsafe { bitbox02_sys::keystore_get_bip39_word(i, &mut word_ptr) } {
                    false => panic!("get_bip39_wordlist"),
                    true => word_ptr as _,
                }
            })
            .collect(),
    )
}

pub fn encode_xpub_at_keypath(keypath: &[u32]) -> Result<Vec<u8>, ()> {
    let mut xpub = vec![0u8; bitbox02_sys::BIP32_SERIALIZED_LEN as _];
    match unsafe {
        bitbox02_sys::keystore_encode_xpub_at_keypath(
            keypath.as_ptr(),
            keypath.len() as _,
            xpub.as_mut_ptr(),
        )
    } {
        true => Ok(xpub),
        false => Err(()),
    }
}

pub fn secp256k1_get_private_key(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut key = zeroize::Zeroizing::new(vec![0u8; 32]);
    match unsafe {
        bitbox02_sys::keystore_secp256k1_get_private_key(
            keypath.as_ptr(),
            keypath.len() as _,
            key.as_mut_ptr(),
        )
    } {
        true => Ok(key),
        false => Err(()),
    }
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

pub fn secp256k1_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let mut signature = [0u8; 64];
    let mut recid: util::c_types::c_int = 0;
    match unsafe {
        bitbox02_sys::keystore_secp256k1_sign(
            keypath.as_ptr(),
            keypath.len() as _,
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
    keypath: &[u32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    let mut signer_commitment = [0u8; EC_PUBLIC_KEY_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_nonce_commit(
            keypath.as_ptr(),
            keypath.len() as _,
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
    let mnemonic = zeroize::Zeroizing::new(crate::util::str_to_cstr_vec(mnemonic)?);
    let mut seed = zeroize::Zeroizing::new([0u8; MAX_SEED_LENGTH]);
    let mut seed_len: usize = 0;
    match unsafe {
        bitbox02_sys::keystore_bip39_mnemonic_to_seed(
            mnemonic.as_ptr(),
            seed.as_mut_ptr(),
            &mut seed_len,
        )
    } {
        true => Ok(zeroize::Zeroizing::new(seed[..seed_len].to_vec())),
        false => Err(()),
    }
}

pub fn encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::keystore_encrypt_and_store_seed(
            seed.as_ptr(),
            seed.len(),
            crate::util::str_to_cstr_vec(password).unwrap().as_ptr(),
        )
    } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        err => Err(err.into()),
    }
}

pub fn get_ed25519_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; 96].to_vec());
    match unsafe { bitbox02_sys::keystore_get_ed25519_seed(seed.as_mut_ptr()) } {
        true => Ok(seed),
        false => Err(()),
    }
}

// Currently only used in the functional tests below.
#[cfg(feature = "testing")]
pub fn get_u2f_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut seed = zeroize::Zeroizing::new([0u8; 32].to_vec());
    match unsafe { bitbox02_sys::keystore_get_u2f_seed(seed.as_mut_ptr()) } {
        true => Ok(seed),
        false => Err(()),
    }
}

pub fn secp256k1_schnorr_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    tweak: Option<&[u8; 32]>,
) -> Result<[u8; 64], ()> {
    let mut signature = [0u8; 64];

    match unsafe {
        bitbox02_sys::keystore_secp256k1_schnorr_sign(
            keypath.as_ptr(),
            keypath.len() as _,
            msg.as_ptr(),
            match tweak {
                Some(t) => t.as_ptr(),
                None => core::ptr::null() as *const _,
            },
            signature.as_mut_ptr(),
        )
    } {
        true => Ok(signature),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1;

    use crate::testing::{mock_memory, mock_unlocked, mock_unlocked_using_mnemonic};
    use util::bip32::HARDENED;

    #[test]
    fn test_secp256k1_sign() {
        lock();
        let keypath = [44 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 5];
        let msg = [0x88u8; 32];
        let host_nonce = [0x56u8; 32];

        // Fails because keystore is locked.
        assert!(secp256k1_sign(&keypath, &msg, &host_nonce).is_err());

        mock_unlocked();
        let sign_result = secp256k1_sign(&keypath, &msg, &host_nonce).unwrap();
        // Verify signature against expected pubkey.

        let secp = secp256k1::Secp256k1::new();
        let expected_pubkey = {
            let pubkey =
                hex::decode("023ffb4a4e41444d40e4e1e4c6cc329bcba2be50d0ef380aea19d490c373be58fb")
                    .unwrap();
            secp256k1::PublicKey::from_slice(&pubkey).unwrap()
        };
        let msg = secp256k1::Message::from_digest_slice(&msg).unwrap();
        // Test recid by recovering the public key from the signature and checking against the
        // expected public key.
        let recoverable_sig = secp256k1::ecdsa::RecoverableSignature::from_compact(
            &sign_result.signature,
            secp256k1::ecdsa::RecoveryId::from_i32(sign_result.recid as i32).unwrap(),
        )
        .unwrap();

        let recovered_pubkey = secp.recover_ecdsa(&msg, &recoverable_sig).unwrap();
        assert_eq!(recovered_pubkey, expected_pubkey);

        // Verify signature.
        assert!(secp
            .verify_ecdsa(&msg, &recoverable_sig.to_standard(), &expected_pubkey)
            .is_ok());
    }

    #[test]
    fn test_secp256k1_schnorr_sign() {
        mock_unlocked_using_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", "");
        let keypath = [86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        let msg = [0x88u8; 32];

        let expected_pubkey = {
            let pubkey =
                hex::decode("cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115")
                    .unwrap();
            secp256k1::XOnlyPublicKey::from_slice(&pubkey).unwrap()
        };

        // Test without tweak
        crate::random::mock_reset();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, None).unwrap();
        let secp = secp256k1::Secp256k1::new();
        assert!(secp
            .verify_schnorr(
                &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                &secp256k1::Message::from_digest_slice(&msg).unwrap(),
                &expected_pubkey
            )
            .is_ok());

        // Test with tweak
        crate::random::mock_reset();
        let tweak = {
            let tweak =
                hex::decode("a39fb163dbd9b5e0840af3cc1ee41d5b31245c5dd8d6bdc3d026d09b8964997c")
                    .unwrap();
            secp256k1::Scalar::from_be_bytes(tweak.try_into().unwrap()).unwrap()
        };
        let (tweaked_pubkey, _) = expected_pubkey.add_tweak(&secp, &tweak).unwrap();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, Some(&tweak.to_be_bytes())).unwrap();
        assert!(secp
            .verify_schnorr(
                &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                &secp256k1::Message::from_digest_slice(&msg).unwrap(),
                &tweaked_pubkey
            )
            .is_ok());
    }

    #[test]
    fn test_secp256k1_nonce_commit() {
        lock();
        let keypath = [44 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 5];
        let msg = [0x88u8; 32];
        let host_commitment = [0xabu8; 32];

        // Fails because keystore is locked.
        assert!(secp256k1_nonce_commit(&keypath, &msg, &host_commitment).is_err());

        mock_unlocked();
        let client_commitment = secp256k1_nonce_commit(&keypath, &msg, &host_commitment).unwrap();
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
    fn test_copy_seed() {
        // 12 words
        mock_unlocked_using_mnemonic(
            "trust cradle viable innocent stand equal little small junior frost laundry room",
            "",
        );
        assert_eq!(
            copy_seed().unwrap().as_slice(),
            b"\xe9\xa6\x3f\xcd\x3a\x4d\x48\x98\x20\xa6\x63\x79\x2b\xad\xf6\xdd",
        );

        // 18 words
        mock_unlocked_using_mnemonic(
            "pupil parent toe bright slam plastic spy suspect verb battle nominee loan call crystal upset razor luggage join",
            "",
        );
        assert_eq!(
            copy_seed().unwrap().as_slice(),
            b"\xad\xf4\x07\x8e\x0e\x0c\xb1\x4c\x34\xd6\xd6\xf2\x82\x6a\x57\xc1\x82\x06\x6a\xbb\xcd\x95\x84\xcf",
        );

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );
        assert_eq!(
            copy_seed().unwrap().as_slice(),
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );
    }

    #[test]
    fn test_get_bip39_word() {
        assert!(get_bip39_word(2048).is_err());

        assert_eq!(get_bip39_word(0).unwrap().as_ref() as &str, "abandon");
        assert_eq!(get_bip39_word(2047).unwrap().as_ref() as &str, "zoo");
        assert_eq!(get_bip39_word(563).unwrap().as_ref() as &str, "edit");
    }

    #[test]
    fn test_get_ed25519_seed() {
        // No seed on a locked keystore.
        lock();
        assert!(get_ed25519_seed().is_err());

        // Test vectors taken from:
        // https://github.com/cardano-foundation/CIPs/blob/6c249ef48f8f5b32efc0ec768fadf4321f3173f2/CIP-0003/Ledger.md#test-vectors
        // See also: https://github.com/cardano-foundation/CIPs/pull/132

        mock_unlocked_using_mnemonic("recall grace sport punch exhibit mad harbor stand obey short width stem awkward used stairs wool ugly trap season stove worth toward congress jaguar", "");
        assert_eq!(
            hex::encode(get_ed25519_seed().unwrap()),
            "a08cf85b564ecf3b947d8d4321fb96d70ee7bb760877e371899b14e2ccf88658104b884682b57efd97decbb318a45c05a527b9cc5c2f64f7352935a049ceea60680d52308194ccef2a18e6812b452a5815fbd7f5babc083856919aaf668fe7e4",
        );

        // Multiple loop iterations.

        mock_unlocked_using_mnemonic("correct cherry mammal bubble want mandate polar hazard crater better craft exotic choice fun tourist census gap lottery neglect address glow carry old business", "");
        assert_eq!(
            hex::encode(get_ed25519_seed().unwrap()),
            "587c6774357ecbf840d4db6404ff7af016dace0400769751ad2abfc77b9a3844cc71702520ef1a4d1b68b91187787a9b8faab0a9bb6b160de541b6ee62469901fc0beda0975fe4763beabd83b7051a5fd5cbce5b88e82c4bbaca265014e524bd",
        );

        mock_unlocked_using_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art", "foo");
        assert_eq!(
            hex::encode(get_ed25519_seed().unwrap()),
            "f053a1e752de5c26197b60f032a4809f08bb3e5d90484fe42024be31efcba7578d914d3ff992e21652fee6a4d99f6091006938fac2c0c0f9d2de0ba64b754e92a4f3723f23472077aa4cd4dd8a8a175dba07ea1852dad1cf268c61a2679c3890",
        );
    }

    #[test]
    fn test_secp256k1_get_private_key() {
        lock();
        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key(keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        assert_eq!(
            hex::encode(secp256k1_get_private_key(keypath).unwrap()),
            "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"
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
    fn test_lock() {
        lock();
        assert!(is_locked());

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(unlock("password").is_ok());
        assert!(is_locked()); // still locked, it is only unlocked after unlock_bip39.
        assert!(unlock_bip39("foo").is_ok());
        assert!(!is_locked());
        lock();
        assert!(is_locked());
    }

    #[test]
    fn test_unlock() {
        mock_memory();
        lock();

        assert!(matches!(unlock("password"), Err(Error::Unseeded)));

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        let mock_salt_root =
            hex::decode("3333333333333333444444444444444411111111111111112222222222222222")
                .unwrap();
        crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        // Loop to check that unlocking works while unlocked.
        for _ in 0..3 {
            assert!(unlock("password").is_ok());
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
            assert!(copy_seed().is_ok());
        }
        // Last attempt, triggers reset.
        assert!(matches!(
            unlock("invalid password"),
            Err(Error::MaxAttemptsExceeded),
        ));
        // Last wrong attempt locks & resets. There is no more seed.
        assert!(!crate::memory::is_seeded());
        assert!(copy_seed().is_err());
        assert!(matches!(unlock("password"), Err(Error::Unseeded)));
    }

    #[test]
    fn test_unlock_bip39() {
        mock_memory();
        lock();

        let seed = hex::decode("1111111111111111222222222222222233333333333333334444444444444444")
            .unwrap();

        let mock_salt_root =
            hex::decode("3333333333333333444444444444444411111111111111112222222222222222")
                .unwrap();
        crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(unlock("password").is_ok());
        assert!(unlock_bip39("foo").is_ok());

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
        let expected_bip39_seed = hex::decode("2b3c63de86f0f2b13cc6a36c1ba2314fbc1b40c77ab9cb64e96ba4d5c62fc204748ca6626a9f035e7d431bce8c9210ec0bdffc2e7db873dee56c8ac2153eee9a").unwrap();
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
            crate::random::mock_reset();
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
            crate::random::mock_reset();
            crate::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();
            lock();

            assert!(create_and_store_seed("password", &host_entropy[..size]).is_ok());
            assert!(unlock("password").is_ok());
            assert_eq!(copy_seed().unwrap().as_slice(), &expected_seed[..size]);
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
        lock();

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();
        let seed2 = hex::decode("c28135734876aff9ccf4f1d60df8d19a0a38fd02085883f65fc608eb769a635d")
            .unwrap();
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(unlock("password").is_ok());
        // Create new (different) seed.
        assert!(encrypt_and_store_seed(&seed2, "password").is_ok());
        assert!(unlock("password").is_ok());
        assert_eq!(copy_seed().unwrap().as_slice(), &seed2);
    }

    // Functional test to store seeds, unlock, retrieve seed.
    #[test]
    fn test_seeds() {
        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        for seed_size in [16, 24, 32] {
            mock_memory();
            lock();

            // Can repeat until initialized - initialized means backup has been created.
            for _ in 0..2 {
                assert!(encrypt_and_store_seed(&seed[..seed_size], "foo").is_ok());
            }

            // Wrong password.
            assert!(matches!(
                unlock("bar"),
                Err(Error::IncorrectPassword {
                    remaining_attempts: 9
                })
            ));

            // Can't get seed before unlock.
            assert!(copy_seed().is_err());
            // Correct password. First time: unlock. After unlock, it becomes a password check.
            for _ in 0..3 {
                assert!(unlock("foo").is_ok());
            }
            assert_eq!(copy_seed().unwrap().as_slice(), &seed[..seed_size]);

            // Can't store new seed once initialized.
            crate::memory::set_initialized().unwrap();
            assert!(matches!(
                encrypt_and_store_seed(&seed[..seed_size], "foo"),
                Err(Error::Memory)
            ));
        }
    }
}
