// Copyright 2021 Shift Crypto AG
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

#[cfg(feature = "ed25519")]
pub mod ed25519;

use alloc::string::String;
use alloc::vec::Vec;

use crate::bip32;
use crate::hal::{Random, SecureChip};
use bitbox02::keystore;
pub use bitbox02::keystore::SignResult;

use util::bip32::HARDENED;
use util::cell::SyncCell;

use crate::secp256k1::SECP256K1;

use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha256, sha512};

/// Length of a compressed secp256k1 pubkey.
const EC_PUBLIC_KEY_LEN: usize = 33;

/// aes256cbc-hmac cipher adds 16 bytes IV, 16 bytes padding, 32 bytes hmac.
const ENCRYPTION_OVERHEAD: usize = 64;

// Unlocking the keystore takes longer than the default 500 ms watchdog. Bump the watchdog timeout
// to roughly seven seconds so we don't assume communication was lost mid-unlock.
const LONG_TIMEOUT: i16 = -70;

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword,
    MaxAttemptsExceeded,
    Unseeded,
    Memory,
    // Securechip error with the error code from securechip.c. 0 if the error is unspecified.
    SecureChip(i32),
    SeedSize,
    Salt,
    Decrypt,
}

impl core::convert::From<bitbox02::securechip::Error> for Error {
    fn from(error: bitbox02::securechip::Error) -> Self {
        match error {
            bitbox02::securechip::Error::SecureChip(
                bitbox02::securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            ) => Error::IncorrectPassword,
            bitbox02::securechip::Error::SecureChip(sc_err) => Error::SecureChip(sc_err as i32),
            bitbox02::securechip::Error::Status(status) => Error::SecureChip(status),
        }
    }
}

#[derive(Copy, Clone)]
struct ReadOnlyBuffer {
    // 64 is the biggest retained buffer (bip39 seed) we will store, and 64 is added for the
    // aes256cbc-hmac overhead.
    data: [u8; 64 + ENCRYPTION_OVERHEAD],
    len: usize,
}

impl ReadOnlyBuffer {
    fn from_slice(data: &[u8]) -> Self {
        let mut result = ReadOnlyBuffer {
            data: [0; 64 + ENCRYPTION_OVERHEAD],
            len: data.len(),
        };
        result.data[..data.len()].copy_from_slice(data);
        result
    }

    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

/// Helper struct for retaining the seed and bip39 seed.
#[derive(Copy, Clone)]
struct RetainedEncryptedBuffer {
    // Stores a random key which, after stretching, is used to encrypt the retained (bip39) seed.
    unstretched_encryption_key: [u8; 32],
    // Stores the encrypted (bip39) seed using aes256cbc.
    encrypted_seed: ReadOnlyBuffer,
    purpose: &'static str,
}

impl RetainedEncryptedBuffer {
    fn from_buffer(
        hal: &mut impl crate::hal::Hal,
        data: &[u8],
        purpose: &'static str,
    ) -> Result<Self, Error> {
        let rand: [u8; 32] = hal
            .random()
            .random_32_bytes()
            .as_slice()
            .try_into()
            .unwrap();
        let encryption_key = stretch_retained_seed_encryption_key(
            hal,
            &rand,
            &format!("{}_in", purpose),
            &format!("{}_out", purpose),
        )?;
        let iv_rand = hal.random().random_32_bytes();
        let iv: &[u8; 16] = iv_rand.first_chunk::<16>().unwrap();
        let encrypted = bitbox_aes::encrypt_with_hmac(iv, &encryption_key, data);
        Ok(RetainedEncryptedBuffer {
            unstretched_encryption_key: rand,
            encrypted_seed: ReadOnlyBuffer::from_slice(&encrypted),
            purpose,
        })
    }

    fn decrypt(
        &self,
        hal: &mut impl crate::hal::Hal,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        let encryption_key = stretch_retained_seed_encryption_key(
            hal,
            &self.unstretched_encryption_key,
            &format!("{}_in", self.purpose),
            &format!("{}_out", self.purpose),
        )?;
        bitbox_aes::decrypt_with_hmac(&encryption_key, self.encrypted_seed.as_slice())
            .map_err(|_| Error::Decrypt)
    }
}

// Stores the encrypted seed after unlock.
static RETAINED_SEED: SyncCell<Option<RetainedEncryptedBuffer>> = SyncCell::new(None);
// A hash of the unencrypted retained seed, used for comparing seeds without knowing their
// plaintext.
static RETAINED_SEED_HASH: SyncCell<Option<[u8; 32]>> = SyncCell::new(None);
// Stores the encrypted BIP-39 seed after bip39-unlock.
static RETAINED_BIP39_SEED: SyncCell<Option<RetainedEncryptedBuffer>> = SyncCell::new(None);

static ROOT_FINGERPRINT: SyncCell<Option<[u8; 4]>> = SyncCell::new(None);

/// Locks the keystore (resets to state before `unlock()`).
pub fn lock() {
    ROOT_FINGERPRINT.write(None);
    RETAINED_SEED.write(None);
    RETAINED_SEED_HASH.write(None);
    RETAINED_BIP39_SEED.write(None);
}

/// Returns false if the keystore is unlocked (unlock() followed by unlock_bip39()), true otherwise.
pub fn is_locked() -> bool {
    let unlocked = RETAINED_SEED.read().is_some() && RETAINED_BIP39_SEED.read().is_some();
    !unlocked
}

fn verify_seed(encryption_key: &[u8], expected_seed: &[u8]) -> bool {
    if encryption_key.len() != 32 {
        return false;
    }

    let cipher = match bitbox02::memory::get_encrypted_seed_and_hmac() {
        Ok(cipher) => cipher,
        Err(_) => return false,
    };
    let decrypted = match bitbox_aes::decrypt_with_hmac(encryption_key, &cipher) {
        Ok(decrypted) => decrypted,
        Err(_) => return false,
    };

    decrypted.as_slice() == expected_seed
}

fn hash_seed(seed: &[u8]) -> Result<[u8; 32], Error> {
    let salted_key =
        crate::salt::hash_data(&[], "keystore_retain_seed_hash").map_err(|_| Error::Salt)?;

    let mut engine = HmacEngine::<sha256::Hash>::new(salted_key.as_slice());
    engine.input(seed);
    Ok(Hmac::<sha256::Hash>::from_engine(engine).to_byte_array())
}

fn retain_seed(hal: &mut impl crate::hal::Hal, seed: &[u8]) -> Result<(), Error> {
    RETAINED_SEED.write(Some(RetainedEncryptedBuffer::from_buffer(
        hal,
        seed,
        "keystore_retained_seed_access",
    )?));
    RETAINED_SEED_HASH.write(Some(hash_seed(seed)?));
    Ok(())
}

/// Restores a seed. This also unlocks the keystore with this seed.
/// `password` is the password with which we encrypt the seed.
pub fn encrypt_and_store_seed(
    hal: &mut impl crate::hal::Hal,
    seed: &[u8],
    password: &str,
) -> Result<(), Error> {
    if bitbox02::memory::is_initialized() {
        return Err(Error::Memory);
    }

    if !matches!(seed.len(), 16 | 24 | 32) {
        return Err(Error::SeedSize);
    }

    lock();

    bitbox02::usb_processing::timeout_reset(LONG_TIMEOUT);

    hal.securechip().init_new_password(password)?;

    let secret = hal.securechip().stretch_password(password)?;

    let iv_rand = hal.random().random_32_bytes();
    let iv: &[u8; 16] = iv_rand.first_chunk::<16>().unwrap();
    let encrypted = bitbox_aes::encrypt_with_hmac(iv, &secret, seed);

    if encrypted.len() > u8::MAX as usize {
        panic!("encrypted seed length overflow");
    }

    bitbox02::memory::set_encrypted_seed_and_hmac(&encrypted).map_err(|_| Error::Memory)?;

    if !verify_seed(&secret, seed) {
        bitbox02::memory::reset_hww().map_err(|_| Error::Memory)?;
        return Err(Error::Memory);
    }

    retain_seed(hal, seed)
}

// Checks if the retained seed matches the passed seed.
fn check_retained_seed(seed: &[u8]) -> Result<(), ()> {
    if RETAINED_SEED.read().is_none() {
        return Err(());
    }
    if hash_seed(seed).map_err(|_| ())? != RETAINED_SEED_HASH.read().ok_or(())? {
        return Err(());
    }
    Ok(())
}

fn get_and_decrypt_seed(
    hal: &mut impl crate::hal::Hal,
    password: &str,
) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    let encrypted = bitbox02::memory::get_encrypted_seed_and_hmac().map_err(|_| Error::Memory)?;
    // Our Optiga securechip implementation fails password stretching if the password is
    // wrong, so it already returns an error here. The ATECC stretches the password without checking
    // if the password is correct, and we determine if it is correct in the seed decryption
    // step below.
    let secret = hal.securechip().stretch_password(password)?;
    let seed = match bitbox_aes::decrypt_with_hmac(&secret, &encrypted) {
        Ok(seed) => seed,
        Err(()) => return Err(Error::IncorrectPassword),
    };

    if !matches!(seed.len(), 16 | 24 | 32) {
        return Err(Error::SeedSize);
    }
    Ok(seed)
}

pub fn unlock(
    hal: &mut impl crate::hal::Hal,
    password: &str,
) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    if !bitbox02::memory::is_seeded() {
        return Err(Error::Unseeded);
    }
    if get_remaining_unlock_attempts() == 0 {
        //
        //  We reset the device as soon as the MAX_UNLOCK_ATTEMPTSth attempt
        //  is made. So we should never enter this branch...
        //  This is just an extraordinary measure for added resilience.
        //
        bitbox02::reset(false);
        return Err(Error::MaxAttemptsExceeded);
    }
    bitbox02::usb_processing::timeout_reset(LONG_TIMEOUT);
    bitbox02::memory::smarteeprom_increment_unlock_attempts();
    let seed = match get_and_decrypt_seed(hal, password) {
        Ok(seed) => seed,
        err @ Err(_) => {
            if get_remaining_unlock_attempts() == 0 {
                bitbox02::reset(false);
                return Err(Error::MaxAttemptsExceeded);
            }
            return err;
        }
    };

    if RETAINED_SEED.read().is_some() {
        // Already unlocked. Fail if the seed changed under our feet (should never happen).
        if check_retained_seed(&seed).is_err() {
            panic!("Seed has suddenly changed. This should never happen.");
        }
    } else {
        retain_seed(hal, &seed)?;
    }
    bitbox02::memory::smarteeprom_reset_unlock_attempts();
    Ok(seed)
}

/// Returns the number of remaining unlock attempts (calls to `unlock()`) that are allowed before
/// the device resets itself.
pub fn get_remaining_unlock_attempts() -> u8 {
    let failed_attempts: u8 = bitbox02::memory::smarteeprom_get_unlock_attempts();
    bitbox02::memory::MAX_UNLOCK_ATTEMPTS.saturating_sub(failed_attempts)
}

/// Unlocks the bip39 seed. The input seed must be the keystore seed (i.e. must match the output
/// of `keystore_copy_seed()`).
/// `mnemonic_passphrase` is the bip39 passphrase used in the derivation. Use the empty string if no
/// passphrase is needed or provided.
pub async fn unlock_bip39(
    hal: &mut impl crate::hal::Hal,
    seed: &[u8],
    mnemonic_passphrase: &str,
    yield_now: impl AsyncFn(),
) -> Result<(), Error> {
    check_retained_seed(seed).map_err(|_| Error::CannotUnlockBIP39)?;

    let (bip39_seed, root_fingerprint) =
        crate::bip39::derive_seed(seed, mnemonic_passphrase, &yield_now).await;

    let (bip39_seed_2, root_fingerprint_2) =
        crate::bip39::derive_seed(seed, mnemonic_passphrase, &yield_now).await;

    if bip39_seed != bip39_seed_2 || root_fingerprint != root_fingerprint_2 {
        return Err(Error::Memory);
    }

    RETAINED_BIP39_SEED.write(Some(RetainedEncryptedBuffer::from_buffer(
        hal,
        bip39_seed.as_slice(),
        "keystore_retained_bip39_seed_access",
    )?));

    // Store root fingerprint.
    ROOT_FINGERPRINT.write(Some(root_fingerprint));
    Ok(())
}

/// Returns a copy of the retained seed. Errors if the keystore is locked.
pub fn copy_seed(hal: &mut impl crate::hal::Hal) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    RETAINED_SEED.read().ok_or(())?.decrypt(hal).map_err(|_| ())
}

/// Returns a copy of the retained bip39 seed. Errors if the keystore is locked.
pub fn copy_bip39_seed(hal: &mut impl crate::hal::Hal) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    RETAINED_BIP39_SEED
        .read()
        .ok_or(())?
        .decrypt(hal)
        .map_err(|_| ())
}

/// Generates the seed, mixes it with host_entropy, and stores it encrypted with the
/// password. The size of the host entropy determines the size of the seed. Can be either 16 or 32
/// bytes, resulting in 12 or 24 BIP39 recovery words.
/// This also unlocks the keystore with the new seed.
pub fn create_and_store_seed(
    hal: &mut impl crate::hal::Hal,
    password: &str,
    host_entropy: &[u8],
) -> Result<(), Error> {
    let seed_len = host_entropy.len();
    if !matches!(seed_len, 16 | 32) {
        return Err(Error::SeedSize);
    }

    let mut seed_vec = hal.random().random_32_bytes();
    let seed = &mut seed_vec[..seed_len];

    // Mix in host entropy.
    for (i, &entropy_byte) in host_entropy.iter().enumerate() {
        seed[i] ^= entropy_byte;
    }

    // Mix in entropy derived from the user password.
    let password_salted_hashed =
        crate::salt::hash_data(password.as_bytes(), "keystore_seed_generation")
            .map_err(|_| Error::Salt)?;

    for (i, &hash_byte) in password_salted_hashed.iter().take(seed_len).enumerate() {
        seed[i] ^= hash_byte;
    }

    encrypt_and_store_seed(hal, seed, password)
}

/// Returns the keystore's seed encoded as a BIP-39 mnemonic.
pub fn get_bip39_mnemonic(
    hal: &mut impl crate::hal::Hal,
) -> Result<zeroize::Zeroizing<String>, ()> {
    crate::bip39::mnemonic_from_seed(&copy_seed(hal)?)
}

fn get_xprv(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<bip32::Xprv, ()> {
    let bip39_seed = copy_bip39_seed(hal)?;
    let xprv: bip32::Xprv =
        bitcoin::bip32::Xpriv::new_master(bitcoin::NetworkKind::Main, &bip39_seed)
            .map_err(|_| ())?
            .into();
    Ok(xprv
        .xprv
        .derive_priv(SECP256K1, &bip32::keypath_from_slice(keypath))
        .map_err(|_| ())?
        .into())
}

/// Get the private key at the keypath.
pub fn secp256k1_get_private_key(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let xprv = get_xprv(hal, keypath)?;
    Ok(zeroize::Zeroizing::new(
        xprv.xprv.private_key.secret_bytes().to_vec(),
    ))
}

/// Get the private key at the keypath, computed twice to mitigate the risk of bitflips.
pub fn secp256k1_get_private_key_twice(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let privkey = secp256k1_get_private_key(hal, keypath)?;
    if privkey == secp256k1_get_private_key(hal, keypath)? {
        Ok(privkey)
    } else {
        Err(())
    }
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_once(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let xpriv = get_xprv(hal, keypath)?;
    let xpub = bitcoin::bip32::Xpub::from_priv(SECP256K1, &xpriv.xprv);
    Ok(bip32::Xpub::from(xpub))
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_twice(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let res1 = get_xpub_once(hal, keypath)?;
    let res2 = get_xpub_once(hal, keypath)?;
    if res1 != res2 {
        return Err(());
    }
    Ok(res1)
}

/// Gets multiple xpubs at once. This is better than multiple calls to `get_xpub_twice()` as it only
/// uses two secure chip operations in total, instead of two per xpub.
pub fn get_xpubs_twice(
    hal: &mut impl crate::hal::Hal,
    keypaths: &[&[u32]],
) -> Result<Vec<bip32::Xpub>, ()> {
    if is_locked() {
        return Err(());
    }
    if keypaths.is_empty() {
        return Ok(vec![]);
    }
    // We get the root xprv as a starting point (twice to mitigate bitflips), afterwards we don't
    // need the securechip anymore.
    let xprv = get_xprv(hal, &[])?;
    let xprv2 = get_xprv(hal, &[])?;

    let mut out = Vec::with_capacity(keypaths.len());
    for keypath in keypaths {
        if xprv != xprv2 {
            return Err(());
        }

        let derive_xpub = || -> Result<bip32::Xpub, ()> {
            let derived_xprv = xprv
                .xprv
                .derive_priv(SECP256K1, &bip32::keypath_from_slice(keypath))
                .map_err(|_| ())?;
            Ok(bip32::Xpub::from(bitcoin::bip32::Xpub::from_priv(
                SECP256K1,
                &derived_xprv,
            )))
        };
        let derived_xpub = derive_xpub()?;
        if derived_xpub != derive_xpub()? {
            return Err(());
        }
        out.push(derived_xpub);
    }

    Ok(out)
}

/// Returns fingerprint of the root public key at m/, which are the first four bytes of its hash160
/// according to:
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
pub fn root_fingerprint() -> Result<Vec<u8>, ()> {
    if is_locked() {
        return Err(());
    }
    ROOT_FINGERPRINT.read().ok_or(()).map(|fp| fp.to_vec())
}

/// Stretches the given encryption_key using the securechip. The resulting key is used to encrypt
/// the retained seed or bip39 seed.
pub fn stretch_retained_seed_encryption_key(
    hal: &mut impl crate::hal::Hal,
    encryption_key: &[u8; 32],
    purpose_in: &str,
    purpose_out: &str,
) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    let salted_in = crate::salt::hash_data(encryption_key, purpose_in).map_err(|_| Error::Salt)?;

    let kdf = hal.securechip().kdf(salted_in.as_slice())?;

    let salted_out =
        crate::salt::hash_data(encryption_key, purpose_out).map_err(|_| Error::Salt)?;

    let mut engine = HmacEngine::<sha256::Hash>::new(salted_out.as_slice());
    engine.input(kdf.as_slice());
    let stretched = Hmac::<sha256::Hash>::from_engine(engine).to_byte_array();

    Ok(zeroize::Zeroizing::new(stretched.to_vec()))
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_lock() {
    lock()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_is_locked() -> bool {
    is_locked()
}

fn bip85_entropy(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let priv_key = secp256k1_get_private_key_twice(hal, keypath)?;

    let mut engine = HmacEngine::<sha512::Hash>::new(b"bip-entropy-from-k");
    engine.input(&priv_key);
    Ok(zeroize::Zeroizing::new(
        Hmac::from_engine(engine).to_byte_array().to_vec(),
    ))
}

/// Computes a BIP39 mnemonic according to BIP-85:
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
/// `words` must be 12, 18 or 24.
/// `index` must be smaller than `bip32::HARDENED`.
pub fn bip85_bip39(
    hal: &mut impl crate::hal::Hal,
    words: u32,
    index: u32,
) -> Result<zeroize::Zeroizing<String>, ()> {
    if index >= HARDENED {
        return Err(());
    }

    let seed_size: usize = match words {
        12 => 16,
        18 => 24,
        24 => 32,
        _ => return Err(()),
    };

    let keypath = [
        83696968 + HARDENED,
        39 + HARDENED,
        0 + HARDENED,
        words + HARDENED,
        index + HARDENED,
    ];

    let entropy = bip85_entropy(hal, &keypath)?;
    crate::bip39::mnemonic_from_seed(&entropy[..seed_size])
}

/// Computes a 16 byte deterministic seed specifically for Lightning hot wallets according to BIP-85.
/// It is the same as BIP-85 with app number 39', but instead using app number 19534' (= 0x4c4e =
/// 'LN'). https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
/// Restricted to 16 byte output entropy.
/// `index` must be smaller than `bip32::HARDENED`.
pub fn bip85_ln(
    hal: &mut impl crate::hal::Hal,
    index: u32,
) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    if index >= HARDENED {
        return Err(());
    }
    let keypath = [
        83696968 + HARDENED,
        19534 + HARDENED,
        0 + HARDENED,
        12 + HARDENED,
        index + HARDENED,
    ];

    let mut entropy = bip85_entropy(hal, &keypath)?;
    entropy.truncate(16);
    Ok(entropy)
}

/// Sign message with private key using the given private key.
///
/// Details about `host_nonce`, the host nonce contribution.  Instead of using plain rfc6979 to
/// generate the nonce in this signature, the following formula is used:
///
///     r = rfc6979(..., additional_data=Hash_d(host_nonce))
///     R = r * G (pubkey to secret r)
///     nonce = r + Hash_p(R, host_nonce)
/// `Hash_d(msg)` and `Hash_p(msg)` are tagged hashes: `sha256(sha256(tag)||sha256(tag)||msg)`.
/// Tag for `Hash_d`: "s2c/ecdsa/data".
/// Tag for `Hash_p`: "s2c/ecdsa/point".
/// This is part of the ECDSA Anti-Klepto protocol, preventing this function to leak any secrets via
/// the signatures (see the ecdsa-s2c module in secp256k1-zpk for more details).
///
/// # Arguments
/// * `private_key` - 32 byte private key
/// * `msg` - 32 byte message to sign
/// * `host_nonce` - 32 byte nonce contribution. Cannot be NULL.
///   Intended to be a contribution by the host. If there is none available, use 32 zero bytes.
///
/// # Returns
/// * `Ok(SignResult)` containing signature in compact format and recoverable id on success
/// * `Err(())` if the keystore is locked
pub fn secp256k1_sign(
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    keystore::_secp256k1_sign(SECP256K1, private_key, msg, host_nonce)
}

/// Get a commitment to the original nonce before tweaking it with the host nonce. This is part of
/// the ECDSA Anti-Klepto Protocol. For more details, check the docs of
/// `secp256k1_ecdsa_anti_exfil_signer_commit`.
///
/// # Arguments
/// * `private_key` - 32 byte private key
/// * `msg` - 32 byte message which will be signed by `secp256k1_sign`
/// * `host_commitment` - must be `sha256(sha256(tag)||sha256(tag)||host_nonce)` where
///   host_nonce is passed to `secp256k1_sign()`. See `secp256k1_ecdsa_anti_exfil_host_commit()`.
///
/// # Returns
/// * `Ok([u8; EC_PUBLIC_KEY_LEN])` - EC_PUBLIC_KEY_LEN bytes compressed signer nonce pubkey on success
/// * `Err(())` on failure
pub fn secp256k1_nonce_commit(
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    keystore::_secp256k1_nonce_commit(SECP256K1, private_key, msg, host_commitment)
}

/// Sign a message using the private key at the keypath, which is optionally tweaked with the given
/// tweak.
pub fn secp256k1_schnorr_sign(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
    msg: &[u8; 32],
    tweak: Option<&[u8; 32]>,
) -> Result<[u8; 64], ()> {
    let private_key = secp256k1_get_private_key(hal, keypath)?;
    let mut keypair =
        bitcoin::secp256k1::Keypair::from_seckey_slice(SECP256K1, &private_key).map_err(|_| ())?;

    if let Some(tweak) = tweak {
        keypair = keypair
            .add_xonly_tweak(
                SECP256K1,
                &bitcoin::secp256k1::Scalar::from_be_bytes(*tweak).map_err(|_| ())?,
            )
            .map_err(|_| ())?;
    }

    let aux_rand = hal.random().random_32_bytes();
    let sig = SECP256K1.sign_schnorr_with_aux_rand(
        &bitcoin::secp256k1::Message::from_digest(*msg),
        &keypair,
        &aux_rand,
    );
    Ok(sig.serialize())
}

/// Get the seed to be used for u2f
#[cfg(feature = "app-u2f")]
pub fn get_u2f_seed(hal: &mut impl crate::hal::Hal) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let bip39_seed = copy_bip39_seed(hal)?;

    let mut engine = HmacEngine::<bitcoin::hashes::sha256::Hash>::new(&bip39_seed);
    // Null-terminator for backwards compatibility from the time when this was coded in C.
    engine.input(b"u2f\0");
    Ok(zeroize::Zeroizing::new(
        Hmac::from_engine(engine).to_byte_array().to_vec(),
    ))
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_get_u2f_seed(mut seed_out: util::bytes::BytesMut) -> bool {
    match get_u2f_seed(&mut crate::hal::BitBox02Hal::new()) {
        Ok(seed) => {
            seed_out.as_mut().copy_from_slice(&seed);
            true
        }
        Err(_) => false,
    }
}

#[cfg(feature = "testing")]
pub mod testing {
    /// This mocks an unlocked keystore with the given bip39 recovery words and bip39 passphrase.
    pub fn mock_unlocked_using_mnemonic(mnemonic: &str, passphrase: &str) {
        let mut mock_hal = crate::hal::testing::TestingHal::new();
        let seed = crate::bip39::mnemonic_to_seed(mnemonic).unwrap();
        super::retain_seed(&mut mock_hal, &seed).unwrap();
        util::bb02_async::block_on(super::unlock_bip39(
            &mut mock_hal,
            &seed,
            passphrase,
            async || {},
        ))
        .unwrap();
    }

    pub const TEST_MNEMONIC: &str = "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay";

    /// This mocks an unlocked keystore with a fixed bip39 seed based on these bip39 recovery words:
    /// `purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay`
    pub fn mock_unlocked() {
        mock_unlocked_using_mnemonic(TEST_MNEMONIC, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::{TestingHal, TestingRandom};
    use hex_lit::hex;

    use bitbox02::testing::mock_memory;
    use testing::{TEST_MNEMONIC, mock_unlocked, mock_unlocked_using_mnemonic};
    use util::bb02_async::block_on;

    use bitcoin::secp256k1;

    #[test]
    fn test_copy_seed() {
        let mut mock_hal = TestingHal::new();
        // 12 words
        mock_unlocked_using_mnemonic(
            "trust cradle viable innocent stand equal little small junior frost laundry room",
            "",
        );
        assert_eq!(
            copy_seed(&mut mock_hal).unwrap().as_slice(),
            b"\xe9\xa6\x3f\xcd\x3a\x4d\x48\x98\x20\xa6\x63\x79\x2b\xad\xf6\xdd",
        );

        // 18 words
        mock_unlocked_using_mnemonic(
            "pupil parent toe bright slam plastic spy suspect verb battle nominee loan call crystal upset razor luggage join",
            "",
        );
        assert_eq!(
            copy_seed(&mut mock_hal).unwrap().as_slice(),
            b"\xad\xf4\x07\x8e\x0e\x0c\xb1\x4c\x34\xd6\xd6\xf2\x82\x6a\x57\xc1\x82\x06\x6a\xbb\xcd\x95\x84\xcf",
        );

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );
        assert_eq!(
            copy_seed(&mut mock_hal).unwrap().as_slice(),
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );
    }

    #[test]
    fn test_encrypt_and_store_seed_invalid_size() {
        mock_memory();
        lock();
        assert!(matches!(
            encrypt_and_store_seed(&mut TestingHal::new(), &[0; 31], "foo"),
            Err(Error::SeedSize)
        ));
    }

    #[test]
    fn test_create_and_store_seed() {
        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");

        let host_entropy = hex!(
            "25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe"
        );

        let mut hal = TestingHal::new();

        // Invalid seed lengths
        for size in [8, 24, 40] {
            assert!(matches!(
                create_and_store_seed(&mut hal, "password", &host_entropy[..size]),
                Err(Error::SeedSize)
            ));
        }

        // Hack to get the random bytes that will be used.
        let seed_random = [0x34; 32];

        // Derived from mock_salt_root and "password".
        let password_salted_hashed =
            hex!("e8c70a20d9108fbb9454b1b8e2d7373e78cbaf9de025ab2d4f4d3c7a6711694c");

        // expected_seed = seed_random ^ host_entropy ^ password_salted_hashed
        let expected_seed: Vec<u8> = seed_random
            .into_iter()
            .zip(host_entropy.iter())
            .zip(password_salted_hashed)
            .map(|((a, &b), c)| a ^ b ^ c)
            .collect();

        for size in [16, 32] {
            mock_memory();
            bitbox02::random::fake_reset();
            bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();
            lock();

            let mut hal = TestingHal::new();
            hal.random.mock_next(seed_random);
            assert!(create_and_store_seed(&mut hal, "password", &host_entropy[..size]).is_ok());
            assert_eq!(
                copy_seed(&mut hal).unwrap().as_slice(),
                &expected_seed[..size]
            );
            // Check the seed has been stored encrypted with the expected encryption key.
            // Decrypt and check seed.
            let cipher = bitbox02::memory::get_encrypted_seed_and_hmac().unwrap();

            // Same as Python:
            // import hmac, hashlib; hmac.digest(b"unit-test", b"password", hashlib.sha256).hex()
            // See also: mock_securechip.c
            let expected_encryption_key =
                hex!("e56de448f5f1d29cdcc0e0099007309afe4d5a3ef2349e99dcc41840ad98409e");
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

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        let seed2 = hex!("c28135734876aff9ccf4f1d60df8d19a0a38fd02085883f65fc608eb769a635d");
        assert!(encrypt_and_store_seed(&mut TestingHal::new(), &seed, "password").is_ok());
        // Create new (different) seed.
        assert!(encrypt_and_store_seed(&mut TestingHal::new(), &seed2, "password").is_ok());
        assert_eq!(
            copy_seed(&mut TestingHal::new()).unwrap().as_slice(),
            &seed2
        );
    }

    #[test]
    fn test_lock() {
        let mut mock_hal = TestingHal::new();
        lock();
        assert!(is_locked());

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        assert!(is_locked()); // still locked, it is only unlocked after unlock_bip39.
        assert!(block_on(unlock_bip39(&mut mock_hal, &seed, "foo", async || {})).is_ok());
        assert!(!is_locked());
        lock();
        assert!(is_locked());
    }

    #[test]
    fn test_unlock() {
        mock_memory();
        lock();

        let mut mock_hal = TestingHal::new();

        assert!(matches!(
            unlock(&mut mock_hal, "password"),
            Err(Error::Unseeded)
        ));

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");

        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        lock();

        // Mock random value used for creating the unstretched seed.
        mock_hal.random.mock_next(hex_lit::hex!(
            "fe0976011452a72212e4b8bd572b5be30141a356f11337d29d35ea8ff997befc"
        ));

        // First call: unlock. The first one does a seed rentention (1 securechip event).
        mock_hal.securechip.event_counter_reset();
        assert_eq!(unlock(&mut mock_hal, "password").unwrap().as_slice(), seed);
        assert_eq!(mock_hal.securechip.get_event_counter(), 6);

        // Loop to check that unlocking works while unlocked.
        for _ in 0..2 {
            // Further calls perform a password check.The password check does not do the retention
            // so it ends up needing one secure chip operation less.
            mock_hal.securechip.event_counter_reset();
            assert_eq!(unlock(&mut mock_hal, "password").unwrap().as_slice(), seed);
            assert_eq!(mock_hal.securechip.get_event_counter(), 5);
        }

        // Also check that the retained seed was encrypted with the expected encryption key.
        let decrypted = {
            let expected_retained_seed_secret =
                hex!("b156be416530c6fc00018844161774a3546a53ac6dd4a0462608838e216008f7");
            bitbox_aes::decrypt_with_hmac(
                &expected_retained_seed_secret,
                RETAINED_SEED.read().unwrap().encrypted_seed.as_slice(),
            )
            .unwrap()
        };
        assert_eq!(decrypted.as_slice(), seed.as_slice());

        // First 9 wrong attempts.
        for i in 1..bitbox02::memory::MAX_UNLOCK_ATTEMPTS {
            assert!(matches!(
                unlock(&mut mock_hal, "invalid password"),
                Err(Error::IncorrectPassword)
            ));
            assert_eq!(
                get_remaining_unlock_attempts(),
                bitbox02::memory::MAX_UNLOCK_ATTEMPTS - i
            );
            // Still seeded.
            assert!(bitbox02::memory::is_seeded());
            // Wrong password does not lock the keystore again if already unlocked.
            assert!(copy_seed(&mut mock_hal).is_ok());
        }
        // Last attempt, triggers reset.
        assert!(matches!(
            unlock(&mut mock_hal, "invalid password"),
            Err(Error::MaxAttemptsExceeded),
        ));
        // Last wrong attempt locks & resets. There is no more seed.
        assert!(!bitbox02::memory::is_seeded());
        assert!(copy_seed(&mut mock_hal).is_err());
        assert!(matches!(
            unlock(&mut mock_hal, "password"),
            Err(Error::Unseeded)
        ));
    }

    #[test]
    fn test_unlock_lockout_while_locked() {
        mock_memory();
        lock();

        let mut mock_hal = TestingHal::new();

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        lock();
        assert!(is_locked());
        assert!(copy_seed(&mut mock_hal).is_err());

        for attempt in 1..bitbox02::memory::MAX_UNLOCK_ATTEMPTS {
            assert!(matches!(
                unlock(&mut mock_hal, "invalid password"),
                Err(Error::IncorrectPassword),
            ));

            assert_eq!(
                get_remaining_unlock_attempts(),
                bitbox02::memory::MAX_UNLOCK_ATTEMPTS - attempt
            );
            assert!(is_locked());
            assert!(copy_seed(&mut mock_hal).is_err());
            assert!(bitbox02::memory::is_seeded());
        }

        assert!(matches!(
            unlock(&mut mock_hal, "invalid password"),
            Err(Error::MaxAttemptsExceeded)
        ));
        assert!(is_locked());
        assert!(copy_seed(&mut mock_hal).is_err());
        assert!(!bitbox02::memory::is_seeded());
        assert!(matches!(
            unlock(&mut mock_hal, "password"),
            Err(Error::Unseeded)
        ));
    }

    /// Ensures that if the recorded unlock attempts already reached the maximum before calling
    /// `unlock()`, the keystore immediately returns `MaxAttemptsExceeded` without performing any
    /// secure chip operations.
    #[test]
    fn test_unlock_preexisting_lockout() {
        mock_memory();
        lock();

        let mut mock_hal = TestingHal::new();

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        lock();
        assert!(is_locked());

        bitbox02::memory::set_unlock_attempts_for_testing(bitbox02::memory::MAX_UNLOCK_ATTEMPTS);
        assert_eq!(get_remaining_unlock_attempts(), 0);
        assert_eq!(
            bitbox02::memory::smarteeprom_get_unlock_attempts(),
            bitbox02::memory::MAX_UNLOCK_ATTEMPTS
        );

        assert!(matches!(
            unlock(&mut mock_hal, "password"),
            Err(Error::MaxAttemptsExceeded)
        ));
        assert!(is_locked());
        assert!(copy_seed(&mut mock_hal).is_err());
        assert!(!bitbox02::memory::is_seeded());
    }

    /// Ensures the failed-attempt counter resets once a correct password is entered while the
    /// keystore is locked, so a later wrong attempt after relocking still sees the full allowance.
    #[test]
    fn test_unlock_failed_attempts_reset_locked() {
        mock_memory();
        lock();

        let mut mock_hal = TestingHal::new();

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        lock();

        fn wrong_attempt(hal: &mut impl crate::hal::Hal) {
            assert!(matches!(
                unlock(hal, "wrong"),
                Err(Error::IncorrectPassword)
            ));
            assert_eq!(
                get_remaining_unlock_attempts(),
                bitbox02::memory::MAX_UNLOCK_ATTEMPTS - 1
            );
        }

        wrong_attempt(&mut mock_hal);
        assert!(copy_seed(&mut mock_hal).is_err());

        assert_eq!(unlock(&mut mock_hal, "password").unwrap().as_slice(), seed);
        assert!(copy_seed(&mut mock_hal).is_ok());

        lock();
        assert!(copy_seed(&mut mock_hal).is_err());

        wrong_attempt(&mut mock_hal);
        assert!(copy_seed(&mut mock_hal).is_err());
        assert!(bitbox02::memory::is_seeded());
    }

    /// Ensures the failed-attempt counter resets when the keystore stays unlocked throughout, so
    /// interleaving wrong attempts with successful unlocks cannot exhaust the counter prematurely.
    #[test]
    fn test_unlock_failed_attempts_reset_unlocked() {
        mock_memory();
        lock();

        let mut mock_hal = TestingHal::new();

        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");
        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(encrypt_and_store_seed(&mut mock_hal, &seed, "password").is_ok());
        lock();

        assert_eq!(unlock(&mut mock_hal, "password").unwrap().as_slice(), seed);
        assert!(copy_seed(&mut mock_hal).is_ok());

        fn wrong_attempt(hal: &mut impl crate::hal::Hal) {
            assert!(matches!(
                unlock(hal, "wrong"),
                Err(Error::IncorrectPassword)
            ));
            assert_eq!(
                get_remaining_unlock_attempts(),
                bitbox02::memory::MAX_UNLOCK_ATTEMPTS - 1
            );
        }

        wrong_attempt(&mut mock_hal);
        assert!(copy_seed(&mut mock_hal).is_ok());

        assert_eq!(unlock(&mut mock_hal, "password").unwrap().as_slice(), seed);
        assert!(copy_seed(&mut mock_hal).is_ok());

        wrong_attempt(&mut mock_hal);
        assert!(copy_seed(&mut mock_hal).is_ok());
        assert!(bitbox02::memory::is_seeded());
    }

    #[test]
    fn test_unlock_bip39() {
        mock_memory();
        lock();

        let seed = hex!("1111111111111111222222222222222233333333333333334444444444444444");

        let mock_salt_root =
            hex!("3333333333333333444444444444444411111111111111112222222222222222");
        bitbox02::memory::set_salt_root(&mock_salt_root).unwrap();

        assert!(root_fingerprint().is_err());
        assert!(encrypt_and_store_seed(&mut TestingHal::new(), &seed, "password").is_ok());
        assert!(root_fingerprint().is_err());
        // Incorrect seed passed
        assert!(
            block_on(unlock_bip39(
                &mut TestingHal::new(),
                b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "foo",
                async || {}
            ))
            .is_err()
        );
        // Correct seed passed.
        let mut mock_hal = TestingHal::new();
        // Mock random value used for creating the unstretched bip39 seed encryption key.
        mock_hal.random.mock_next(hex!(
            "9b44c7048893faaf6e2d7625d13d8f1cab0765fd61f159d9713e08155d06717c"
        ));

        mock_hal.securechip.event_counter_reset();
        assert!(block_on(unlock_bip39(&mut mock_hal, &seed, "foo", async || {})).is_ok());
        assert_eq!(mock_hal.securechip.get_event_counter(), 1);
        assert_eq!(root_fingerprint(), Ok(vec![0xf1, 0xbc, 0x3c, 0x46]),);

        let expected_bip39_seed = hex!(
            "2b3c63de86f0f2b13cc6a36c1ba2314fbc1b40c77ab9cb64e96ba4d5c62fc204748ca6626a9f035e7d431bce8c9210ec0bdffc2e7db873dee56c8ac2153eee9a"
        );

        assert_eq!(
            copy_bip39_seed(&mut mock_hal).unwrap().as_slice(),
            expected_bip39_seed.as_slice()
        );

        // Check that the retained bip39 seed was encrypted with the expected encryption key.
        let decrypted = {
            let expected_retained_bip39_seed_secret =
                hex!("856d9a8c1ea42a69ae76324244ace674397ff1360a4ba4c85ffbd42cee8a7f29");
            bitbox_aes::decrypt_with_hmac(
                &expected_retained_bip39_seed_secret,
                RETAINED_BIP39_SEED
                    .read()
                    .unwrap()
                    .encrypted_seed
                    .as_slice(),
            )
            .unwrap()
        };
        assert_eq!(decrypted.as_slice(), expected_bip39_seed.as_slice());
    }

    #[test]
    fn test_secp256k1_get_private_key() {
        lock();

        let mut mock_hal = TestingHal::new();

        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key(&mut mock_hal, keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            secp256k1_get_private_key(&mut mock_hal, keypath)
                .unwrap()
                .as_slice(),
            hex!("4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"),
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 1);
    }

    #[test]
    fn test_secp256k1_get_private_key_twice() {
        lock();

        let mut mock_hal = TestingHal::new();

        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key_twice(&mut mock_hal, keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            secp256k1_get_private_key_twice(&mut mock_hal, keypath)
                .unwrap()
                .as_slice(),
            hex!("4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"),
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 2);
    }

    #[test]
    fn test_get_bip39_mnemonic() {
        lock();
        assert!(get_bip39_mnemonic(&mut TestingHal::new()).is_err());

        mock_unlocked();

        assert_eq!(
            get_bip39_mnemonic(&mut TestingHal::new()).unwrap().as_str(),
            TEST_MNEMONIC
        );
    }

    #[test]
    fn test_get_xpub_twice() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];
        // Also test with unhardened and non-zero elements.
        let keypath_5 = &[44 + HARDENED, 1 + HARDENED, 10 + HARDENED, 1, 100];

        let mut mock_hal = TestingHal::new();

        lock();
        assert!(get_xpub_twice(&mut mock_hal, keypath).is_err());

        // 24 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        mock_hal.securechip.event_counter_reset();

        assert_eq!(
            get_xpub_twice(&mut mock_hal, &[])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );

        assert_eq!(mock_hal.securechip.get_event_counter(), 2);

        assert_eq!(
            get_xpub_twice(&mut mock_hal, keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );
        assert_eq!(
            get_xpub_twice(&mut mock_hal, keypath_5)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6HHn1zdtf1RjePopiTV5nxf8jY2xwbJicTQ91jV4cUJZ5EnbvXyBGDhqWt8B9JxxBt9vExi4pdWzrbrM43qSFs747VCGmSy2DPWAhg9MkUg",
        );

        // 18 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject",
            "",
        );
        assert_eq!(
            get_xpub_twice(&mut mock_hal, keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
        );

        // 12 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
            "",
        );
        assert_eq!(
            get_xpub_twice(&mut mock_hal, keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
        )
    }

    #[test]
    fn test_get_xpubs_twice() {
        lock();

        assert!(get_xpubs_twice(&mut TestingHal::new(), &[]).is_err());

        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        // Helper to convert to strings.
        fn get(hal: &mut impl crate::hal::Hal, keypaths: &[&[u32]]) -> Vec<String> {
            get_xpubs_twice(hal, keypaths)
                .unwrap()
                .iter()
                .map(|xpub| xpub.serialize_str(bip32::XPubType::Xpub).unwrap())
                .collect()
        }

        let mut mock_hal = TestingHal::new();

        mock_hal.securechip.event_counter_reset();
        assert!(get_xpubs_twice(&mut mock_hal, &[]).unwrap().is_empty());
        assert_eq!(mock_hal.securechip.get_event_counter(), 0);

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            get(
                &mut mock_hal,
                &[
                    &[84 + HARDENED, HARDENED, HARDENED],
                    &[86 + HARDENED, HARDENED, HARDENED],
                ]
            ),
            vec![
                "xpub6CNbmcHwZDudAvCAZVE5kejUoFD63mbkRbRMA2HoF9oNWsCofni87gJKp31qZJ9FsCMQR2vK9AS51mT8dgUMGsHW6SfaAKb4eSzpqJn7zwK",
                "xpub6CGwpj8iQNuzSeeEKF4yuQt32fpLqfHj7sUfFH4uW34DoctWPksxAdjNYC9KwYgwA149B7SDdcLH1aFmucRcjBL4U6piN7HgaiFCBsToamH",
            ],
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 2);
    }

    #[test]
    fn test_root_fingerprint() {
        lock();
        assert_eq!(root_fingerprint(), Err(()));

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(root_fingerprint(), Ok(vec![0x02, 0x40, 0xe9, 0x2a]));
        // fingerprint is precomputed during bip39 unlock, so takes no securechip events.
        assert_eq!(bitbox02::securechip::fake_event_counter(), 0);

        mock_unlocked_using_mnemonic(
            "small agent wife animal marine cloth exit thank stool idea steel frame",
            "",
        );
        assert_eq!(root_fingerprint(), Ok(vec![0xf4, 0x0b, 0x46, 0x9a]));

        lock();
        assert_eq!(root_fingerprint(), Err(()));
    }

    #[test]
    fn test_stretch_retained_seed_encryption_key_success() {
        mock_memory();
        let salt_root = hex!("0000000000000000111111111111111122222222222222223333333333333333");
        bitbox02::memory::set_salt_root(&salt_root).unwrap();

        let encryption_key =
            hex!("00112233445566778899aabbccddeeff112233445566778899aabbccddeeff00");

        let stretched = stretch_retained_seed_encryption_key(
            &mut TestingHal::new(),
            &encryption_key,
            "keystore_retained_seed_access_in",
            "keystore_retained_seed_access_out",
        )
        .unwrap();

        let expected = hex!("b6b20683810aee16b5603ae95d14eaae5ae2c8d9df9b66e1b67c698e627bb208");
        assert_eq!(stretched.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_stretch_retained_seed_encryption_key_salt_error() {
        mock_memory();
        bitbox02::memory::set_salt_root(&[0xffu8; 32]).unwrap();

        let encryption_key = [0u8; 32];
        let result = stretch_retained_seed_encryption_key(
            &mut TestingHal::new(),
            &encryption_key,
            "purpose_in",
            "purpose_out",
        );
        assert!(matches!(result, Err(Error::Salt)));
    }

    #[test]
    fn test_bip85_bip39() {
        lock();
        assert!(bip85_bip39(&mut TestingHal::new(), 12, 0).is_err());

        // Test fixtures generated using:
        // `docker build -t bip85 .`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 1 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 2147483647 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 18`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 24`
        // in  https://github.com/ethankosakovsky/bip85/tree/435a0589746c1036735d0a5081167e08abfa7413.

        mock_unlocked_using_mnemonic(
            "virtual weapon code laptop defy cricket vicious target wave leopard garden give",
            "",
        );

        assert_eq!(
            bip85_bip39(&mut TestingHal::new(), 12, 0).unwrap().as_ref() as &str,
            "slender whip place siren tissue chaos ankle door only assume tent shallow",
        );
        assert_eq!(
            bip85_bip39(&mut TestingHal::new(), 12, 1).unwrap().as_ref() as &str,
            "income soft level reunion height pony crane use unfold win keen satisfy",
        );
        assert_eq!(
            bip85_bip39(&mut TestingHal::new(), 12, HARDENED - 1)
                .unwrap()
                .as_ref() as &str,
            "carry build nerve market domain energy mistake script puzzle replace mixture idea",
        );
        assert_eq!(
            bip85_bip39(&mut TestingHal::new(), 18, 0).unwrap().as_ref() as &str,
            "enact peasant tragic habit expand jar senior melody coin acid logic upper soccer later earn napkin planet stereo",
        );
        assert_eq!(
            bip85_bip39(&mut TestingHal::new(), 24, 0).unwrap().as_ref() as &str,
            "cabbage wink october add anchor mean tray surprise gasp tomorrow garbage habit beyond merge where arrive beef gentle animal office drop panel chest size",
        );

        // Invalid number of words.
        assert!(bip85_bip39(&mut TestingHal::new(), 10, 0).is_err());
        // Index too high.
        assert!(bip85_bip39(&mut TestingHal::new(), 12, HARDENED).is_err());
    }

    #[test]
    fn test_bip85_ln() {
        lock();
        assert!(bip85_ln(&mut TestingHal::new(), 0).is_err());

        mock_unlocked_using_mnemonic(
            "virtual weapon code laptop defy cricket vicious target wave leopard garden give",
            "",
        );

        assert_eq!(
            bip85_ln(&mut TestingHal::new(), 0).unwrap().as_slice(),
            hex!("3a5f3b888aab88e2a9ab991b60a03ed8"),
        );
        assert_eq!(
            bip85_ln(&mut TestingHal::new(), 1).unwrap().as_slice(),
            hex!("e7d9ce75f8cb17570e665417b47fa0be"),
        );
        assert_eq!(
            bip85_ln(&mut TestingHal::new(), HARDENED - 1)
                .unwrap()
                .as_slice(),
            hex!("1f3b75ea252749700a1e453469148ca6"),
        );

        // Index too high.
        assert!(bip85_ln(&mut TestingHal::new(), HARDENED).is_err());
    }

    #[test]
    fn test_fixtures() {
        struct Test {
            seed_len: usize,
            mnemonic_passphrase: &'static str,
            expected_mnemonic: &'static str,
            expected_xpub: &'static str,
            expected_u2f_seed: [u8; 32],
        }
        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");

        let tests = [
            Test {
                seed_len: 32,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
                expected_xpub: "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
                expected_u2f_seed: hex!(
                    "4f464a6667ad88eebcd0f02982761e474ee0dd16253160320f49d1d6681745e9"
                ),
            },
            Test {
                seed_len: 32,
                mnemonic_passphrase: "abc",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
                expected_xpub: "xpub6DXBP3HhFdhUTafatEULxfTXUUxDVuCxfa9RAiBU5r6aRgKiABbeBDyqwWWjmKPP1BZvpvVNMbVR5LeHzhQphtLcPZ8jk3MdLBgc2sACJwR",
                expected_u2f_seed: hex!(
                    "d599da991ad83baaf449c789e2dff1539dd66983b47a1dec1c00ff3f352cccbc"
                ),
            },
            Test {
                seed_len: 24,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject",
                expected_xpub: "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
                expected_u2f_seed: hex!(
                    "fb9dc3fb0a17390776df5c3d8f9261bc5fd5df9f00414cee1393e37e0efda7ef"
                ),
            },
            Test {
                seed_len: 16,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
                expected_xpub: "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
                expected_u2f_seed: hex!(
                    "20d68b206aff9667b623a460ce61fc94762de67561d6855ca9a6df7b409b2a54"
                ),
            },
        ];

        for test in tests {
            mock_memory();
            lock();
            let seed = &seed[..test.seed_len];

            let mut mock_hal = TestingHal::new();

            assert!(
                block_on(unlock_bip39(
                    &mut mock_hal,
                    seed,
                    test.mnemonic_passphrase,
                    async || {}
                ))
                .is_err()
            );

            mock_hal.securechip.event_counter_reset();
            assert!(encrypt_and_store_seed(&mut mock_hal, seed, "foo").is_ok());
            assert_eq!(mock_hal.securechip.get_event_counter(), 7);

            assert!(is_locked());

            mock_hal.securechip.event_counter_reset();
            assert!(
                block_on(unlock_bip39(
                    &mut mock_hal,
                    seed,
                    test.mnemonic_passphrase,
                    async || {}
                ))
                .is_ok()
            );
            assert_eq!(mock_hal.securechip.get_event_counter(), 1);

            assert!(!is_locked());
            assert_eq!(
                get_bip39_mnemonic(&mut mock_hal).unwrap().as_str(),
                test.expected_mnemonic,
            );
            let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];

            mock_hal.securechip.event_counter_reset();
            let xpub = get_xpub_once(&mut mock_hal, keypath).unwrap();
            assert_eq!(mock_hal.securechip.get_event_counter(), 1);

            assert_eq!(
                xpub.serialize_str(crate::bip32::XPubType::Xpub).unwrap(),
                test.expected_xpub,
            );
            assert_eq!(
                get_u2f_seed(&mut mock_hal).unwrap().as_slice(),
                test.expected_u2f_seed
            );
        }
    }

    #[test]
    fn test_secp256k1_sign() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_nonce = [0x56u8; 32];

        let sign_result = secp256k1_sign(&private_key, &msg, &host_nonce).unwrap();

        // Verify signature against expected pubkey.

        let expected_pubkey = {
            let pubkey = hex!("023ffb4a4e41444d40e4e1e4c6cc329bcba2be50d0ef380aea19d490c373be58fb");
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

        let recovered_pubkey = SECP256K1.recover_ecdsa(&msg, &recoverable_sig).unwrap();
        assert_eq!(recovered_pubkey, expected_pubkey);

        // Verify signature.
        assert!(
            SECP256K1
                .verify_ecdsa(&msg, &recoverable_sig.to_standard(), &expected_pubkey)
                .is_ok()
        );
    }

    #[test]
    fn test_secp256k1_nonce_commit() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_commitment = [0xabu8; 32];

        let client_commitment =
            secp256k1_nonce_commit(&private_key, &msg, &host_commitment).unwrap();
        assert_eq!(
            hex::encode(client_commitment),
            "0381e4136251c87f2947b735159c6dd644a7b58d35b437e20c878e5129f1320e5e",
        );
    }

    #[test]
    fn test_secp256k1_antiklepto_protocol() {
        mock_unlocked();

        let mut keypath = [84 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0];
        let mut msg = [0x23u8; 32];
        let mut host_nonce = [0x55u8; 32];

        for index in 0..3 {
            keypath[4] = index;
            msg[0] = index as u8;
            host_nonce[0] = index as u8;

            // Protocol steps are described in secp256k1/include/secp256k1_ecdsa_s2c.h under
            // "ECDSA Anti-Klepto Protocol".

            // Protocol step 1.
            let host_commitment_vec =
                bitbox02::secp256k1::ecdsa_anti_exfil_host_commit(SECP256K1, &host_nonce).unwrap();
            let host_commitment: [u8; 32] = host_commitment_vec.try_into().unwrap();

            // Get pubkey at keypath.
            let private_key = secp256k1_get_private_key(&mut TestingHal::new(), &keypath).unwrap();
            let private_key_bytes: [u8; 32] = private_key.as_slice().try_into().unwrap();
            let secret_key = secp256k1::SecretKey::from_slice(&private_key_bytes).unwrap();
            let public_key = secret_key.public_key(SECP256K1);

            // Commit - protocol step 2.
            let signer_commitment =
                secp256k1_nonce_commit(&private_key_bytes, &msg, &host_commitment).unwrap();
            // Protocol step 3: host_nonce sent from host to signer to be used in step 4.
            // Sign - protocol step 4.
            let sign_result = secp256k1_sign(&private_key_bytes, &msg, &host_nonce).unwrap();

            let signature =
                secp256k1::ecdsa::Signature::from_compact(&sign_result.signature).unwrap();
            // Protocol step 5: host verification.
            bitbox02::secp256k1::anti_exfil_host_verify(
                SECP256K1,
                &signature,
                &msg,
                &public_key,
                &host_nonce,
                &signer_commitment,
            )
            .unwrap();

            let message = secp256k1::Message::from_digest_slice(&msg).unwrap();
            let recoverable_sig = secp256k1::ecdsa::RecoverableSignature::from_compact(
                &sign_result.signature,
                secp256k1::ecdsa::RecoveryId::from_i32(sign_result.recid as i32).unwrap(),
            )
            .unwrap();
            assert!(
                SECP256K1
                    .verify_ecdsa(&message, &recoverable_sig.to_standard(), &public_key)
                    .is_ok()
            );
        }
    }

    #[test]
    fn test_secp256k1_schnorr_sign() {
        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );
        let keypath = [86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        let msg = [0x88u8; 32];

        let expected_pubkey = {
            let pubkey = hex!("cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115");
            secp256k1::XOnlyPublicKey::from_slice(&pubkey).unwrap()
        };

        // Test without tweak

        let mut mock_hal = TestingHal::new();
        mock_hal.securechip.event_counter_reset();
        let sig = secp256k1_schnorr_sign(&mut mock_hal, &keypath, &msg, None).unwrap();
        assert_eq!(mock_hal.securechip.get_event_counter(), 1);

        assert!(
            SECP256K1
                .verify_schnorr(
                    &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                    &secp256k1::Message::from_digest_slice(&msg).unwrap(),
                    &expected_pubkey
                )
                .is_ok()
        );

        // Test with tweak
        let tweak = secp256k1::Scalar::from_be_bytes(hex!(
            "a39fb163dbd9b5e0840af3cc1ee41d5b31245c5dd8d6bdc3d026d09b8964997c"
        ))
        .unwrap();
        let (tweaked_pubkey, _) = expected_pubkey.add_tweak(SECP256K1, &tweak).unwrap();
        let mut mock_hal = TestingHal::new();
        let sig = secp256k1_schnorr_sign(&mut mock_hal, &keypath, &msg, Some(&tweak.to_be_bytes()))
            .unwrap();
        assert!(
            SECP256K1
                .verify_schnorr(
                    &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                    &secp256k1::Message::from_digest(msg),
                    &tweaked_pubkey
                )
                .is_ok()
        );
    }

    // Functional test to store seeds, lock/unlock, retrieve seed.
    #[test]
    fn test_seeds() {
        let seed = hex!("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044");

        for seed_size in [16, 24, 32] {
            mock_memory();
            lock();

            let mut mock_hal = TestingHal::new();

            // Can repeat until initialized - initialized means backup has been created.
            for _ in 0..2 {
                assert!(encrypt_and_store_seed(&mut mock_hal, &seed[..seed_size], "foo").is_ok());
            }
            // Also unlocks, so we can get the retained seed.
            assert_eq!(
                copy_seed(&mut mock_hal).unwrap().as_slice(),
                &seed[..seed_size]
            );

            lock();
            // Can't get seed before unlock.
            assert!(copy_seed(&mut mock_hal).is_err());

            // Wrong password.
            assert!(matches!(
                unlock(&mut mock_hal, "bar"),
                Err(Error::IncorrectPassword)
            ));
            assert_eq!(get_remaining_unlock_attempts(), 9);

            // Correct password. First time: unlock. After unlock, it becomes a password check.
            for _ in 0..3 {
                assert_eq!(
                    unlock(&mut mock_hal, "foo").unwrap().as_slice(),
                    &seed[..seed_size]
                );
            }
            assert_eq!(
                copy_seed(&mut mock_hal).unwrap().as_slice(),
                &seed[..seed_size]
            );

            // Can't store new seed once initialized.
            bitbox02::memory::set_initialized().unwrap();
            assert!(matches!(
                encrypt_and_store_seed(&mut mock_hal, &seed[..seed_size], "foo"),
                Err(Error::Memory)
            ));
        }
    }
}
