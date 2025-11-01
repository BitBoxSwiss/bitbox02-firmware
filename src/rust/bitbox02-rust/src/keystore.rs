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
pub use bitbox02::keystore::Error;
pub use bitbox02::keystore::SignResult;
use bitbox02::{keystore, securechip};

use util::bip32::HARDENED;
use util::cell::SyncCell;

use crate::secp256k1::SECP256K1;

use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha256, sha512};

/// Length of a compressed secp256k1 pubkey.
const EC_PUBLIC_KEY_LEN: usize = 33;

static ROOT_FINGERPRINT: SyncCell<Option<[u8; 4]>> = SyncCell::new(None);

/// Locks the keystore (resets to state before `unlock()`).
pub fn lock() {
    keystore::_lock();
    ROOT_FINGERPRINT.write(None)
}

/// Returns false if the keystore is unlocked (unlock() followed by unlock_bip39()), true otherwise.
pub fn is_locked() -> bool {
    keystore::_is_locked()
}

pub fn unlock(password: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    keystore::_unlock(password)
}

/// Unlocks the bip39 seed. The input seed must be the keystore seed (i.e. must match the output
/// of `keystore_copy_seed()`).
/// `mnemonic_passphrase` is the bip39 passphrase used in the derivation. Use the empty string if no
/// passphrase is needed or provided.
pub async fn unlock_bip39(
    seed: &[u8],
    mnemonic_passphrase: &str,
    yield_now: impl AsyncFn(),
) -> Result<(), Error> {
    keystore::unlock_bip39_check(seed)?;

    let (bip39_seed, root_fingerprint) =
        crate::bip39::derive_seed(seed, mnemonic_passphrase, &yield_now).await;

    let (bip39_seed_2, root_fingerprint_2) =
        crate::bip39::derive_seed(seed, mnemonic_passphrase, &yield_now).await;

    if bip39_seed != bip39_seed_2 || root_fingerprint != root_fingerprint_2 {
        return Err(Error::Memory);
    }

    keystore::unlock_bip39_finalize(bip39_seed.as_slice().try_into().unwrap())?;

    // Store root fingerprint.
    ROOT_FINGERPRINT.write(Some(root_fingerprint));
    Ok(())
}

/// Returns a copy of the retained seed. Errors if the keystore is locked.
pub fn copy_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    keystore::_copy_seed()
}

/// Returns a copy of the retained bip39 seed. Errors if the keystore is locked.
pub fn copy_bip39_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    keystore::_copy_bip39_seed()
}

/// Restores a seed. This also unlocks the keystore with this seed.
/// `password` is the password with which we encrypt the seed.
pub fn encrypt_and_store_seed(seed: &[u8], password: &str) -> Result<(), Error> {
    keystore::_encrypt_and_store_seed(seed, password)
}

/// Generates the seed, mixes it with host_entropy, and stores it encrypted with the
/// password. The size of the host entropy determines the size of the seed. Can be either 16 or 32
/// bytes, resulting in 12 or 24 BIP39 recovery words.
/// This also unlocks the keystore with the new seed.
pub fn create_and_store_seed(password: &str, host_entropy: &[u8]) -> Result<(), Error> {
    let seed_len = host_entropy.len();
    if !matches!(seed_len, 16 | 32) {
        return Err(Error::SeedSize);
    }

    let mut seed_vec = bitbox02::random::random_32_bytes();
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

    encrypt_and_store_seed(seed, password)
}

/// Returns the keystore's seed encoded as a BIP-39 mnemonic.
pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    crate::bip39::mnemonic_from_seed(&copy_seed()?)
}

fn get_xprv(keypath: &[u32]) -> Result<bip32::Xprv, ()> {
    let bip39_seed = copy_bip39_seed()?;
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
pub fn secp256k1_get_private_key(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let xprv = get_xprv(keypath)?;
    Ok(zeroize::Zeroizing::new(
        xprv.xprv.private_key.secret_bytes().to_vec(),
    ))
}

/// Get the private key at the keypath, computed twice to mitigate the risk of bitflips.
pub fn secp256k1_get_private_key_twice(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let privkey = secp256k1_get_private_key(keypath)?;
    if privkey == secp256k1_get_private_key(keypath)? {
        Ok(privkey)
    } else {
        Err(())
    }
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_once(keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let xpriv = get_xprv(keypath)?;
    let xpub = bitcoin::bip32::Xpub::from_priv(SECP256K1, &xpriv.xprv);
    Ok(bip32::Xpub::from(xpub))
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_twice(keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let res1 = get_xpub_once(keypath)?;
    let res2 = get_xpub_once(keypath)?;
    if res1 != res2 {
        return Err(());
    }
    Ok(res1)
}

/// Gets multiple xpubs at once. This is better than multiple calls to `get_xpub_twice()` as it only
/// uses two secure chip operations in total, instead of two per xpub.
pub fn get_xpubs_twice(keypaths: &[&[u32]]) -> Result<Vec<bip32::Xpub>, ()> {
    if is_locked() {
        return Err(());
    }
    if keypaths.is_empty() {
        return Ok(vec![]);
    }
    // We get the root xprv as a starting point (twice to mitigate bitflips), afterwards we don't
    // need the securechip anymore.
    let xprv = get_xprv(&[])?;
    let xprv2 = get_xprv(&[])?;

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
    encryption_key: &[u8; 32],
    purpose_in: &str,
    purpose_out: &str,
) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
    let salted_in = crate::salt::hash_data(encryption_key, purpose_in).map_err(|_| Error::Salt)?;

    let kdf = securechip::kdf(salted_in.as_slice()).map_err(|err| match err {
        securechip::Error::SecureChip(sc_err) => Error::SecureChip(sc_err as i32),
        securechip::Error::Status(status) => Error::SecureChip(status),
    })?;

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

/// # Safety
///
/// `encryption_key` must refer to a 32-byte buffer and `out` must have space for 32 bytes.
/// `purpose_in` and `purpose_out` must be null-terminated C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_keystore_stretch_retained_seed_encryption_key(
    encryption_key: util::bytes::Bytes,
    purpose_in: *const core::ffi::c_char,
    purpose_out: *const core::ffi::c_char,
    mut out: util::bytes::BytesMut,
) -> bool {
    let encryption_key: [u8; 32] = match encryption_key.as_ref().try_into() {
        Ok(key) => key,
        Err(_) => return false,
    };
    let purpose_in = unsafe { bitbox02::util::str_from_null_terminated_ptr(purpose_in) };
    let purpose_out = unsafe { bitbox02::util::str_from_null_terminated_ptr(purpose_out) };
    let (purpose_in, purpose_out) = match (purpose_in, purpose_out) {
        (Ok(purpose_in), Ok(purpose_out)) => (purpose_in, purpose_out),
        _ => return false,
    };

    match stretch_retained_seed_encryption_key(&encryption_key, purpose_in, purpose_out) {
        Ok(stretched) => {
            out.as_mut().copy_from_slice(stretched.as_slice());
            true
        }
        Err(_) => false,
    }
}

fn bip85_entropy(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let priv_key = secp256k1_get_private_key_twice(keypath)?;

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
pub fn bip85_bip39(words: u32, index: u32) -> Result<zeroize::Zeroizing<String>, ()> {
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

    let entropy = bip85_entropy(&keypath)?;
    crate::bip39::mnemonic_from_seed(&entropy[..seed_size])
}

/// Computes a 16 byte deterministic seed specifically for Lightning hot wallets according to BIP-85.
/// It is the same as BIP-85 with app number 39', but instead using app number 19534' (= 0x4c4e =
/// 'LN'). https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
/// Restricted to 16 byte output entropy.
/// `index` must be smaller than `bip32::HARDENED`.
pub fn bip85_ln(index: u32) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
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

    let mut entropy = bip85_entropy(&keypath)?;
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
    keypath: &[u32],
    msg: &[u8; 32],
    tweak: Option<&[u8; 32]>,
) -> Result<[u8; 64], ()> {
    let private_key = secp256k1_get_private_key(keypath)?;
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

    let sig = SECP256K1.sign_schnorr_with_aux_rand(
        &bitcoin::secp256k1::Message::from_digest(*msg),
        &keypair,
        &bitbox02::random::random_32_bytes(),
    );
    Ok(sig.serialize())
}

/// Get the seed to be used for u2f
#[cfg(feature = "app-u2f")]
pub fn get_u2f_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let bip39_seed = copy_bip39_seed()?;

    let mut engine = HmacEngine::<bitcoin::hashes::sha256::Hash>::new(&bip39_seed);
    // Null-terminator for backwards compatibility from the time when this was coded in C.
    engine.input(b"u2f\0");
    Ok(zeroize::Zeroizing::new(
        Hmac::from_engine(engine).to_byte_array().to_vec(),
    ))
}

/// # Safety
///
/// keypath pointer has point to a buffer of length `keypath_len` uint32 elements.
#[cfg(feature = "c-unit-testing")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_secp256k1_get_private_key(
    keypath: *const u32,
    keypath_len: usize,
    mut out: util::bytes::BytesMut,
) -> bool {
    match unsafe { secp256k1_get_private_key(core::slice::from_raw_parts(keypath, keypath_len)) } {
        Ok(private_key) => {
            out.as_mut().copy_from_slice(&private_key);
            true
        }
        Err(()) => false,
    }
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_get_u2f_seed(mut seed_out: util::bytes::BytesMut) -> bool {
    match get_u2f_seed() {
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
        let seed = crate::bip39::mnemonic_to_seed(mnemonic).unwrap();
        bitbox02::keystore::mock_unlocked(&seed);
        util::bb02_async::block_on(super::unlock_bip39(&seed, passphrase, async || {})).unwrap();
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

    use hex_lit::hex;

    use bitbox02::testing::mock_memory;
    use testing::{TEST_MNEMONIC, mock_unlocked, mock_unlocked_using_mnemonic};
    use util::bb02_async::block_on;

    use bitcoin::secp256k1;

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
            bitbox02::random::fake_reset();
            bitbox02::random::random_32_bytes()
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
            bitbox02::random::fake_reset();
            bitbox02::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();
            lock();

            assert!(create_and_store_seed("password", &host_entropy[..size]).is_ok());
            assert_eq!(copy_seed().unwrap().as_slice(), &expected_seed[..size]);
            // Check the seed has been stored encrypted with the expected encryption key.
            // Decrypt and check seed.
            let cipher = bitbox02::memory::get_encrypted_seed_and_hmac().unwrap();

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
        // Create new (different) seed.
        assert!(encrypt_and_store_seed(&seed2, "password").is_ok());
        assert_eq!(copy_seed().unwrap().as_slice(), &seed2);
    }

    #[test]
    fn test_lock() {
        lock();
        assert!(is_locked());

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(is_locked()); // still locked, it is only unlocked after unlock_bip39.
        assert!(block_on(unlock_bip39(&seed, "foo", async || {})).is_ok());
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
        bitbox02::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        lock();

        // First call: unlock. The first one does a seed rentention (1 securechip event).
        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(unlock("password").unwrap().as_slice(), seed);
        assert_eq!(bitbox02::securechip::fake_event_counter(), 6);

        // Loop to check that unlocking works while unlocked.
        for _ in 0..2 {
            // Further calls perform a password check.The password check does not do the retention
            // so it ends up needing one secure chip operation less.
            bitbox02::securechip::fake_event_counter_reset();
            assert_eq!(unlock("password").unwrap().as_slice(), seed);
            assert_eq!(bitbox02::securechip::fake_event_counter(), 5);
        }

        // Also check that the retained seed was encrypted with the expected encryption key.
        let decrypted = {
            let retained_seed_encrypted: &[u8] = keystore::test_get_retained_seed_encrypted();
            let expected_retained_seed_secret =
                hex::decode("b156be416530c6fc00018844161774a3546a53ac6dd4a0462608838e216008f7")
                    .unwrap();
            bitbox_aes::decrypt_with_hmac(&expected_retained_seed_secret, retained_seed_encrypted)
                .unwrap()
        };
        assert_eq!(decrypted.as_slice(), seed.as_slice());

        // First 9 wrong attempts.
        for i in 1..bitbox02::memory::MAX_UNLOCK_ATTEMPTS {
            assert!(matches!(
                unlock("invalid password"),
                Err(Error::IncorrectPassword { remaining_attempts }) if remaining_attempts
                    == bitbox02::memory::MAX_UNLOCK_ATTEMPTS  - i
            ));
            // Still seeded.
            assert!(bitbox02::memory::is_seeded());
            // Wrong password does not lock the keystore again if already unlocked.
            assert!(copy_seed().is_ok());
        }
        // Last attempt, triggers reset.
        assert!(matches!(
            unlock("invalid password"),
            Err(Error::MaxAttemptsExceeded),
        ));
        // Last wrong attempt locks & resets. There is no more seed.
        assert!(!bitbox02::memory::is_seeded());
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
        bitbox02::memory::set_salt_root(mock_salt_root.as_slice().try_into().unwrap()).unwrap();

        assert!(root_fingerprint().is_err());
        assert!(encrypt_and_store_seed(&seed, "password").is_ok());
        assert!(root_fingerprint().is_err());
        // Incorrect seed passed
        assert!(
            block_on(unlock_bip39(
                b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "foo",
                async || {}
            ))
            .is_err()
        );
        // Correct seed passed.
        bitbox02::securechip::fake_event_counter_reset();
        assert!(block_on(unlock_bip39(&seed, "foo", async || {})).is_ok());
        assert_eq!(bitbox02::securechip::fake_event_counter(), 1);
        assert_eq!(root_fingerprint(), Ok(vec![0xf1, 0xbc, 0x3c, 0x46]),);

        let expected_bip39_seed = hex::decode("2b3c63de86f0f2b13cc6a36c1ba2314fbc1b40c77ab9cb64e96ba4d5c62fc204748ca6626a9f035e7d431bce8c9210ec0bdffc2e7db873dee56c8ac2153eee9a").unwrap();

        assert_eq!(
            copy_bip39_seed().unwrap().as_slice(),
            expected_bip39_seed.as_slice()
        );

        // Check that the retained bip39 seed was encrypted with the expected encryption key.
        let decrypted = {
            let retained_bip39_seed_encrypted: &[u8] =
                keystore::test_get_retained_bip39_seed_encrypted();
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
    fn test_secp256k1_get_private_key() {
        lock();
        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key(keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            secp256k1_get_private_key(keypath).unwrap().as_slice(),
            hex!("4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"),
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 1);
    }

    #[test]
    fn test_secp256k1_get_private_key_twice() {
        lock();
        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key_twice(keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            secp256k1_get_private_key_twice(keypath).unwrap().as_slice(),
            hex!("4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"),
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);
    }

    #[test]
    fn test_get_bip39_mnemonic() {
        lock();
        assert!(get_bip39_mnemonic().is_err());

        mock_unlocked();

        assert_eq!(get_bip39_mnemonic().unwrap().as_str(), TEST_MNEMONIC);
    }

    #[test]
    fn test_get_xpub_twice() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];
        // Also test with unhardened and non-zero elements.
        let keypath_5 = &[44 + HARDENED, 1 + HARDENED, 10 + HARDENED, 1, 100];

        lock();
        assert!(get_xpub_twice(keypath).is_err());

        // 24 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();

        assert_eq!(
            get_xpub_twice(&[])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );

        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);

        assert_eq!(
            get_xpub_twice(keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );
        assert_eq!(
            get_xpub_twice(keypath_5)
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
            get_xpub_twice(keypath)
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
            get_xpub_twice(keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
        )
    }

    #[test]
    fn test_get_xpubs_twice() {
        lock();
        assert!(get_xpubs_twice(&[]).is_err());

        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        // Helper to convert to strings.
        let get = |keypaths| -> Vec<String> {
            get_xpubs_twice(keypaths)
                .unwrap()
                .iter()
                .map(|xpub| xpub.serialize_str(bip32::XPubType::Xpub).unwrap())
                .collect()
        };

        bitbox02::securechip::fake_event_counter_reset();
        assert!(get_xpubs_twice(&[]).unwrap().is_empty());
        assert_eq!(bitbox02::securechip::fake_event_counter(), 0);

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            get(&[
                &[84 + HARDENED, HARDENED, HARDENED],
                &[86 + HARDENED, HARDENED, HARDENED],
            ]),
            vec![
                "xpub6CNbmcHwZDudAvCAZVE5kejUoFD63mbkRbRMA2HoF9oNWsCofni87gJKp31qZJ9FsCMQR2vK9AS51mT8dgUMGsHW6SfaAKb4eSzpqJn7zwK",
                "xpub6CGwpj8iQNuzSeeEKF4yuQt32fpLqfHj7sUfFH4uW34DoctWPksxAdjNYC9KwYgwA149B7SDdcLH1aFmucRcjBL4U6piN7HgaiFCBsToamH",
            ],
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);
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
            &encryption_key,
            "keystore_retained_seed_access_in",
            "keystore_retained_seed_access_out",
        )
        .unwrap();

        let expected = hex!("b6b20683810aee16b5603ae95d14eaae5ae2c8d9df9b66e1b67c698e627bb208");
        assert_eq!(stretched.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_rust_keystore_stretch_retained_seed_encryption_key_success() {
        mock_memory();
        let salt_root =
            hex::decode("0000000000000000111111111111111122222222222222223333333333333333")
                .unwrap();
        bitbox02::memory::set_salt_root(salt_root.as_slice().try_into().unwrap()).unwrap();

        let encryption_key_vec =
            hex::decode("00112233445566778899aabbccddeeff112233445566778899aabbccddeeff00")
                .unwrap();

        let mut out = [0u8; 32];
        let purpose_in = c"keystore_retained_seed_access_in";
        let purpose_out = c"keystore_retained_seed_access_out";

        let success = unsafe {
            rust_keystore_stretch_retained_seed_encryption_key(
                util::bytes::rust_util_bytes(encryption_key_vec.as_ptr(), encryption_key_vec.len()),
                purpose_in.as_ptr(),
                purpose_out.as_ptr(),
                util::bytes::rust_util_bytes_mut(out.as_mut_ptr(), out.len()),
            )
        };
        assert!(success);
        let expected =
            hex::decode("b6b20683810aee16b5603ae95d14eaae5ae2c8d9df9b66e1b67c698e627bb208")
                .unwrap();
        assert_eq!(out, expected.as_slice());
    }

    #[test]
    fn test_stretch_retained_seed_encryption_key_salt_error() {
        mock_memory();
        bitbox02::memory::set_salt_root(&[0xffu8; 32]).unwrap();

        let encryption_key = [0u8; 32];
        let result =
            stretch_retained_seed_encryption_key(&encryption_key, "purpose_in", "purpose_out");
        assert!(matches!(result, Err(Error::Salt)));
    }

    #[test]
    fn test_bip85_bip39() {
        lock();
        assert!(bip85_bip39(12, 0).is_err());

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
            bip85_bip39(12, 0).unwrap().as_ref() as &str,
            "slender whip place siren tissue chaos ankle door only assume tent shallow",
        );
        assert_eq!(
            bip85_bip39(12, 1).unwrap().as_ref() as &str,
            "income soft level reunion height pony crane use unfold win keen satisfy",
        );
        assert_eq!(
            bip85_bip39(12, HARDENED - 1).unwrap().as_ref() as &str,
            "carry build nerve market domain energy mistake script puzzle replace mixture idea",
        );
        assert_eq!(
            bip85_bip39(18, 0).unwrap().as_ref() as &str,
            "enact peasant tragic habit expand jar senior melody coin acid logic upper soccer later earn napkin planet stereo",
        );
        assert_eq!(
            bip85_bip39(24, 0).unwrap().as_ref() as &str,
            "cabbage wink october add anchor mean tray surprise gasp tomorrow garbage habit beyond merge where arrive beef gentle animal office drop panel chest size",
        );

        // Invalid number of words.
        assert!(bip85_bip39(10, 0).is_err());
        // Index too high.
        assert!(bip85_bip39(12, HARDENED).is_err());
    }

    #[test]
    fn test_bip85_ln() {
        lock();
        assert!(bip85_ln(0).is_err());

        mock_unlocked_using_mnemonic(
            "virtual weapon code laptop defy cricket vicious target wave leopard garden give",
            "",
        );

        assert_eq!(
            bip85_ln(0).unwrap().as_slice(),
            hex!("3a5f3b888aab88e2a9ab991b60a03ed8"),
        );
        assert_eq!(
            bip85_ln(1).unwrap().as_slice(),
            hex!("e7d9ce75f8cb17570e665417b47fa0be"),
        );
        assert_eq!(
            bip85_ln(HARDENED - 1).unwrap().as_slice(),
            hex!("1f3b75ea252749700a1e453469148ca6"),
        );

        // Index too high.
        assert!(bip85_ln(HARDENED).is_err());
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

            assert!(block_on(unlock_bip39(seed, test.mnemonic_passphrase, async || {})).is_err());

            bitbox02::securechip::fake_event_counter_reset();
            assert!(encrypt_and_store_seed(seed, "foo").is_ok());
            assert_eq!(bitbox02::securechip::fake_event_counter(), 7);

            assert!(is_locked());

            bitbox02::securechip::fake_event_counter_reset();
            assert!(block_on(unlock_bip39(seed, test.mnemonic_passphrase, async || {})).is_ok());
            assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

            assert!(!is_locked());
            assert_eq!(
                get_bip39_mnemonic().unwrap().as_str(),
                test.expected_mnemonic,
            );
            let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];

            bitbox02::securechip::fake_event_counter_reset();
            let xpub = get_xpub_once(keypath).unwrap();
            assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

            assert_eq!(
                xpub.serialize_str(crate::bip32::XPubType::Xpub).unwrap(),
                test.expected_xpub,
            );
            assert_eq!(get_u2f_seed().unwrap().as_slice(), test.expected_u2f_seed);
        }
    }

    #[test]
    fn test_secp256k1_sign() {
        let private_key =
            hex::decode("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c")
                .unwrap();
        let msg = [0x88u8; 32];
        let host_nonce = [0x56u8; 32];

        let sign_result =
            secp256k1_sign(&private_key.try_into().unwrap(), &msg, &host_nonce).unwrap();

        // Verify signature against expected pubkey.

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
        let private_key =
            hex::decode("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c")
                .unwrap();
        let msg = [0x88u8; 32];
        let host_commitment = [0xabu8; 32];

        let client_commitment =
            secp256k1_nonce_commit(&private_key.try_into().unwrap(), &msg, &host_commitment)
                .unwrap();
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
            let private_key = secp256k1_get_private_key(&keypath).unwrap();
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
        bitbox02::random::fake_reset();

        bitbox02::securechip::fake_event_counter_reset();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, None).unwrap();
        assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

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
        bitbox02::random::fake_reset();
        let tweak = secp256k1::Scalar::from_be_bytes(hex!(
            "a39fb163dbd9b5e0840af3cc1ee41d5b31245c5dd8d6bdc3d026d09b8964997c"
        ))
        .unwrap();
        let (tweaked_pubkey, _) = expected_pubkey.add_tweak(SECP256K1, &tweak).unwrap();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, Some(&tweak.to_be_bytes())).unwrap();
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
        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        for seed_size in [16, 24, 32] {
            mock_memory();
            lock();

            // Can repeat until initialized - initialized means backup has been created.
            for _ in 0..2 {
                assert!(encrypt_and_store_seed(&seed[..seed_size], "foo").is_ok());
            }
            // Also unlocks, so we can get the retained seed.
            assert_eq!(copy_seed().unwrap().as_slice(), &seed[..seed_size]);

            lock();
            // Can't get seed before unlock.
            assert!(copy_seed().is_err());

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
            assert_eq!(copy_seed().unwrap().as_slice(), &seed[..seed_size]);

            // Can't store new seed once initialized.
            bitbox02::memory::set_initialized().unwrap();
            assert!(matches!(
                encrypt_and_store_seed(&seed[..seed_size], "foo"),
                Err(Error::Memory)
            ));
        }
    }
}
