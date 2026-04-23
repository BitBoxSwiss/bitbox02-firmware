// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Model, PasswordStretchAlgo, SecureChipError};
use alloc::boxed::Box;
use bitbox_hal::{Memory, Random};
use util::sha2::{hmac_sha256, hmac_sha256_overwrite, sha256};
use zeroize::Zeroizing;

mod der;

#[cfg(not(test))]
#[path = "optiga/ops.rs"]
mod ops;
#[cfg(test)]
#[path = "optiga/ops_fake.rs"]
mod ops;

const OID_AES_SYMKEY: u16 = bitbox_securechip_sys::OID_AES_SYMKEY as u16;
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
const OID_ARBITRARY_DATA: u16 = bitbox_securechip_sys::OID_ARBITRARY_DATA as u16;
const OID_COUNTER: u16 = bitbox_securechip_sys::OID_COUNTER as u16;
const OID_COUNTER_HMAC_WRITEPROTECTED: u16 =
    bitbox_securechip_sys::OID_COUNTER_HMAC_WRITEPROTECTED as u16;
const OID_COUNTER_PASSWORD: u16 = bitbox_securechip_sys::OID_COUNTER_PASSWORD as u16;
const OID_HMAC: u16 = bitbox_securechip_sys::OID_HMAC as u16;
const OID_HMAC_WRITEPROTECTED: u16 = bitbox_securechip_sys::OID_HMAC_WRITEPROTECTED as u16;
const OID_PASSWORD: u16 = bitbox_securechip_sys::OID_PASSWORD as u16;
const OID_PASSWORD_SECRET: u16 = bitbox_securechip_sys::OID_PASSWORD_SECRET as u16;
const MONOTONIC_COUNTER_MAX_USE: u32 = bitbox_securechip_sys::MONOTONIC_COUNTER_MAX_USE;
const SMALL_MONOTONIC_COUNTER_MAX_USE: u32 = bitbox_securechip_sys::SMALL_MONOTONIC_COUNTER_MAX_USE;
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
const ARBITRARY_DATA_LEN: usize =
    bitbox_securechip_sys::ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE as usize;
const KDF_LEN: usize = 32;
const OPTIGA_HMAC_SHA_256: bitbox_securechip_sys::optiga_hmac_type_t =
    bitbox_securechip_sys::optiga_hmac_type::OPTIGA_HMAC_SHA_256;
// This number of KDF iterations on the external kdf slot when stretching the device
// password using the V0 algorithm.
const KDF_NUM_ITERATIONS_V0: usize = 2;
const OPTIGA_HMAC_VERIFY_FAIL: i32 = 0x802F;
const OPTIGA_KEY_USAGE_ENCRYPTION: bitbox_securechip_sys::optiga_key_usage_t =
    bitbox_securechip_sys::optiga_key_usage::OPTIGA_KEY_USAGE_ENCRYPTION;
const OPTIGA_RNG_TYPE_TRNG: bitbox_securechip_sys::optiga_rng_type_t =
    bitbox_securechip_sys::optiga_rng_type::OPTIGA_RNG_TYPE_TRNG;
const OPTIGA_SYMMETRIC_AES_256: bitbox_securechip_sys::optiga_symmetric_key_type_t =
    bitbox_securechip_sys::optiga_symmetric_key_type::OPTIGA_SYMMETRIC_AES_256;
const OPTIGA_SYMMETRIC_CMAC: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t =
    bitbox_securechip_sys::optiga_symmetric_encryption_mode::OPTIGA_SYMMETRIC_CMAC;

fn zeroed_secret<const N: usize>() -> Box<Zeroizing<[u8; N]>> {
    Box::new(Zeroizing::new([0; N]))
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
struct ArbitraryData {
    bytes: [u8; ARBITRARY_DATA_LEN],
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
impl ArbitraryData {
    fn new() -> Self {
        Self {
            bytes: [0; ARBITRARY_DATA_LEN],
        }
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter(&self) -> u32 {
        u32::from_le_bytes(self.bytes[..4].try_into().unwrap())
    }

    fn set_u2f_counter(&mut self, counter: u32) {
        self.bytes[..4].copy_from_slice(&counter.to_le_bytes());
    }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
async fn read_arbitrary_data() -> Result<ArbitraryData, Error> {
    let mut data = ArbitraryData::new();
    ops::util_read_data(OID_ARBITRARY_DATA, 0, &mut data.bytes).await?;
    Ok(data)
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
async fn write_arbitary_data(data: &ArbitraryData) -> Result<(), Error> {
    ops::util_write_data(
        OID_ARBITRARY_DATA,
        bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
        0,
        &data.bytes,
    )
    .await
}

fn key_id_from_oid(oid: u16) -> bitbox_securechip_sys::optiga_key_id_t {
    match oid {
        OID_AES_SYMKEY => bitbox_securechip_sys::optiga_key_id::OPTIGA_KEY_ID_SECRET_BASED,
        _ => panic!("unexpected optiga key oid"),
    }
}

async fn authorize(oid_auth: u16, auth_secret: &[u8; KDF_LEN]) -> Result<(), Error> {
    let mut random_data = zeroed_secret::<KDF_LEN>();
    ops::crypt_generate_auth_code(OPTIGA_RNG_TYPE_TRNG, &mut random_data).await?;

    let mut hmac = zeroed_secret::<KDF_LEN>();
    hmac_sha256(auth_secret, random_data.as_slice(), &mut hmac);
    ops::crypt_hmac_verify(OPTIGA_HMAC_SHA_256, oid_auth, &random_data, &hmac).await
}

async fn reset_counter(oid: u16, limit: u32) -> Result<(), Error> {
    let mut counter_buf = [0u8; 8];
    counter_buf[4..8].copy_from_slice(&limit.to_be_bytes());
    ops::util_write_data(
        oid,
        bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
        0,
        &counter_buf,
    )
    .await
}

async fn kdf_hmac(
    optiga_oid: u16,
    msg: &[u8; KDF_LEN],
    mac_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    // Equivalent to Python: `hmac.new(key, msg, hashlib.sha256).digest()`.
    ops::crypt_hmac(OPTIGA_HMAC_SHA_256, optiga_oid, msg, mac_out).await
}

async fn kdf_internal(msg: &[u8; KDF_LEN], kdf_out: &mut [u8; KDF_LEN]) -> Result<(), Error> {
    let mut mac_out = zeroed_secret::<16>();
    ops::crypt_symmetric_encrypt(
        OPTIGA_SYMMETRIC_CMAC,
        key_id_from_oid(OID_AES_SYMKEY),
        msg,
        &mut mac_out,
    )
    .await?;

    sha256(mac_out.as_slice(), kdf_out);
    Ok(())
}

async fn set_password(
    memory: &mut impl Memory,
    password_secret: &[u8; KDF_LEN],
    auth_password: &[u8; KDF_LEN],
) -> Result<(), Error> {
    let result = async {
        authorize(OID_PASSWORD_SECRET, password_secret).await?;
        let auth_password_salted_hashed =
            bitbox_core_utils::salt::hash_data(memory, auth_password, "optiga_password")
                .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;
        ops::util_write_data(
            OID_PASSWORD,
            bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
            0,
            auth_password_salted_hashed.as_slice(),
        )
        .await?;
        // Add one extra to the counter threshold, as afterwards writing the
        // write-protected HMAC slot increments the counter.
        reset_counter(OID_COUNTER_PASSWORD, SMALL_MONOTONIC_COUNTER_MAX_USE + 1).await
    }
    .await;
    let cleanup_result = ops::crypt_clear_auto_state(OID_PASSWORD_SECRET).await;
    result?;
    cleanup_result
}

async fn set_hmac_writeprotected(
    memory: &mut impl Memory,
    hmac_key: &[u8; KDF_LEN],
    auth_password: &[u8; KDF_LEN],
) -> Result<(), Error> {
    let result = async {
        let auth_password_salted_hashed =
            bitbox_core_utils::salt::hash_data(memory, auth_password, "optiga_password")
                .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;
        authorize(OID_PASSWORD, &auth_password_salted_hashed).await?;
        ops::util_write_data(
            OID_HMAC_WRITEPROTECTED,
            bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
            0,
            hmac_key,
        )
        .await?;
        reset_counter(
            OID_COUNTER_HMAC_WRITEPROTECTED,
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        )
        .await
    }
    .await;
    let cleanup_result = ops::crypt_clear_auto_state(OID_PASSWORD).await;
    result?;
    cleanup_result
}

async fn v1_get_auth_password(
    memory: &mut impl Memory,
    password: &str,
    hmac_key: Option<&[u8; KDF_LEN]>,
    stretched_password_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    let password_salted_hashed = bitbox_core_utils::salt::hash_data(
        memory,
        password.as_bytes(),
        "optiga_password_stretch_in",
    )
    .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    let mut kdf_in = zeroed_secret::<KDF_LEN>();
    kdf_in.copy_from_slice(password_salted_hashed.as_slice());

    // First KDF on the internal key increments the large monotonic counter. Call only once!
    kdf_internal(&kdf_in, stretched_password_out).await?;

    // Second KDF increments the small monotonic counter in `OID_HMAC_WRITEPROTECTED`. Call only
    // once!
    kdf_in.copy_from_slice(stretched_password_out);
    if let Some(hmac_key) = hmac_key {
        hmac_sha256(hmac_key, kdf_in.as_slice(), stretched_password_out);
    } else {
        match kdf_hmac(OID_HMAC_WRITEPROTECTED, &kdf_in, stretched_password_out).await {
            Ok(()) => {}
            Err(Error::Status(OPTIGA_HMAC_VERIFY_FAIL)) => {
                return Err(Error::SecureChip(
                    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                ));
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

fn v1_combine(
    memory: &mut impl Memory,
    password: &str,
    auth_password: &[u8; KDF_LEN],
    password_secret: &[u8; KDF_LEN],
    stretched_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    hmac_sha256(password_secret, auth_password, stretched_out);

    let password_salted_hashed = bitbox_core_utils::salt::hash_data(
        memory,
        password.as_bytes(),
        "optiga_password_stretch_out",
    )
    .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    hmac_sha256_overwrite(password_salted_hashed.as_slice(), stretched_out);
    Ok(())
}

async fn optiga_verify_password_v0(
    memory: &mut impl Memory,
    password: &str,
    password_secret_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    let password_salted_hashed =
        bitbox_core_utils::salt::hash_data(memory, password.as_bytes(), "optiga_password")
            .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    let result = async {
        authorize(OID_PASSWORD, &password_salted_hashed).await?;
        ops::util_read_data(OID_PASSWORD_SECRET, 0, password_secret_out).await?;
        authorize(OID_PASSWORD_SECRET, password_secret_out).await?;
        reset_counter(OID_COUNTER_PASSWORD, SMALL_MONOTONIC_COUNTER_MAX_USE).await
    }
    .await;
    let res_clear1 = ops::crypt_clear_auto_state(OID_PASSWORD).await;
    let res_clear2 = ops::crypt_clear_auto_state(OID_PASSWORD_SECRET).await;
    result?;
    res_clear1?;
    res_clear2
}

async fn optiga_verify_password_v1(
    memory: &mut impl Memory,
    auth_password: &[u8; KDF_LEN],
    password_secret_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    let auth_password_salted_hashed =
        bitbox_core_utils::salt::hash_data(memory, auth_password, "optiga_password")
            .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    let result = async {
        authorize(OID_PASSWORD, &auth_password_salted_hashed).await?;
        ops::util_read_data(OID_PASSWORD_SECRET, 0, password_secret_out).await?;
        authorize(OID_PASSWORD_SECRET, password_secret_out).await?;
        reset_counter(OID_COUNTER_PASSWORD, SMALL_MONOTONIC_COUNTER_MAX_USE).await?;
        reset_counter(
            OID_COUNTER_HMAC_WRITEPROTECTED,
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        )
        .await
    }
    .await;
    let res_clear1 = ops::crypt_clear_auto_state(OID_PASSWORD).await;
    let res_clear2 = ops::crypt_clear_auto_state(OID_PASSWORD_SECRET).await;
    result?;
    res_clear1?;
    res_clear2
}

async fn stretch_password_v0(
    memory: &mut impl Memory,
    password: &str,
    stretched_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    let password_salted_hashed = bitbox_core_utils::salt::hash_data(
        memory,
        password.as_bytes(),
        "optiga_password_stretch_in",
    )
    .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    let mut kdf_in = zeroed_secret::<KDF_LEN>();
    kdf_in.copy_from_slice(password_salted_hashed.as_slice());

    // First KDF on the internal key increments the large monotonic counter. Call only once!
    kdf_internal(&kdf_in, stretched_out).await?;
    // Second KDF does not use any counters and we call it multiple times.
    for _ in 0..KDF_NUM_ITERATIONS_V0 {
        kdf_in.copy_from_slice(stretched_out);
        kdf_hmac(OID_HMAC, &kdf_in, stretched_out).await?;
    }

    // Verify password, incrementing the small monotonic counter.
    // Do this after the above KDF stretch so the big monotonic counter is also incremented.
    let mut password_secret = zeroed_secret::<KDF_LEN>();
    match optiga_verify_password_v0(memory, password, &mut password_secret).await {
        Ok(()) => {}
        Err(Error::Status(OPTIGA_HMAC_VERIFY_FAIL)) => {
            return Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            ));
        }
        Err(err) => return Err(err),
    }

    hmac_sha256_overwrite(password_secret.as_slice(), stretched_out);

    let password_salted_hashed = bitbox_core_utils::salt::hash_data(
        memory,
        password.as_bytes(),
        "optiga_password_stretch_out",
    )
    .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;
    hmac_sha256_overwrite(password_salted_hashed.as_slice(), stretched_out);
    Ok(())
}

async fn stretch_password_v1(
    memory: &mut impl Memory,
    password: &str,
    stretched_out: &mut [u8; KDF_LEN],
) -> Result<(), Error> {
    let mut auth_password = zeroed_secret::<KDF_LEN>();
    // Get auth password. This increments the small monotonic counter in
    // `OID_COUNTER_HMAC_WRITEPROTECTED` and the large monotonic counter.
    v1_get_auth_password(memory, password, None, &mut auth_password).await?;

    let mut password_secret = zeroed_secret::<KDF_LEN>();
    // Verify password, incrementing the small monotonic counter in `OID_COUNTER_PASSWORD`.
    match optiga_verify_password_v1(memory, &auth_password, &mut password_secret).await {
        Ok(()) => {}
        Err(Error::Status(OPTIGA_HMAC_VERIFY_FAIL)) => {
            return Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            ));
        }
        Err(err) => return Err(err),
    }

    v1_combine(
        memory,
        password,
        &auth_password,
        &password_secret,
        stretched_out,
    )
}

pub async fn attestation_sign(challenge: &[u8; 32], signature: &mut [u8; 64]) -> Result<(), ()> {
    let sig_der = ops::crypt_ecdsa_sign(
        challenge,
        bitbox_securechip_sys::optiga_key_id::OPTIGA_KEY_ID_E0F1,
    )
    .await
    .map_err(|_| ())?;
    der::parse_optiga_signature(sig_der.as_slice(), signature)
}

pub async fn random() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = zeroed_secret::<32>();
    ops::crypt_random(OPTIGA_RNG_TYPE_TRNG, &mut result).await?;
    Ok(result)
}

pub async fn monotonic_increments_remaining() -> Result<u32, ()> {
    let mut counter_buf = [0; 4];
    ops::util_read_data(OID_COUNTER, 0, &mut counter_buf)
        .await
        .map_err(|_| ())?;
    let counter = u32::from_be_bytes(counter_buf);
    if counter > MONOTONIC_COUNTER_MAX_USE {
        panic!("optiga monotonic counter larger than max");
    }
    Ok(MONOTONIC_COUNTER_MAX_USE - counter)
}

pub async fn reset_keys(random: &mut impl Random, memory: &mut impl Memory) -> Result<(), ()> {
    // This resets `OID_AES_SYMKEY` and the `OID_HMAC`/`OID_HMAC_WRITEPROTECTED` keys, as well as
    // the `OID_PASSWORD_SECRET` and `OID_PASSWORD` keys. A password is needed because updating the
    // `OID_PASSWORD` key requires auth using the `OID_PASSWORD_SECRET` key, but any password is
    // fine for the purpose of resetting the keys.
    //
    // We reset using V1, the latest algorithm. It covers resetting everything from V0 as well.
    init_new_password(
        random,
        memory,
        "",
        PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
    )
    .await
    .map(|_| ())
    .map_err(|_| ())
}

pub async fn init_new_password(
    random: &mut impl Random,
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    if password_stretch_algo != PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1 {
        // New passwords must use the latest algo.
        return Err(Error::SecureChip(
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
        ));
    }

    let mut stretched = zeroed_secret::<KDF_LEN>();
    // Set new HMAC key.
    let securechip_random = self::random().await?;
    let new_hmac_key = ops::random_32_bytes(random, &securechip_random)?;
    ops::util_write_data(
        OID_HMAC,
        bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
        0,
        new_hmac_key.as_slice(),
    )
    .await?;
    // Set new symmetric key.
    ops::crypt_symmetric_generate_key(OPTIGA_SYMMETRIC_AES_256, OPTIGA_KEY_USAGE_ENCRYPTION)
        .await?;

    let securechip_random = self::random().await?;
    let password_secret = ops::random_32_bytes(random, &securechip_random)?;
    ops::util_write_data(
        OID_PASSWORD_SECRET,
        bitbox_securechip_sys::OPTIGA_UTIL_ERASE_AND_WRITE as u8,
        0,
        password_secret.as_slice(),
    )
    .await?;

    let securechip_random = self::random().await?;
    let new_hmac_writeprotected_key = ops::random_32_bytes(random, &securechip_random)?;

    let mut auth_password = zeroed_secret::<KDF_LEN>();
    v1_get_auth_password(
        memory,
        password,
        Some(&new_hmac_writeprotected_key),
        &mut auth_password,
    )
    .await?;
    set_password(memory, &password_secret, &auth_password).await?;
    set_hmac_writeprotected(memory, &new_hmac_writeprotected_key, &auth_password).await?;
    v1_combine(
        memory,
        password,
        &auth_password,
        &password_secret,
        stretched.as_mut(),
    )?;

    Ok(stretched)
}

pub async fn stretch_password(
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut stretched = zeroed_secret::<KDF_LEN>();
    match password_stretch_algo {
        PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0 => {
            stretch_password_v0(memory, password, stretched.as_mut()).await?
        }
        PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1 => {
            stretch_password_v1(memory, password, stretched.as_mut()).await?
        }
    }
    Ok(stretched)
}

pub async fn kdf(msg: &[u8; KDF_LEN]) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut result = zeroed_secret::<KDF_LEN>();
    ops::crypt_hmac(OPTIGA_HMAC_SHA_256, OID_HMAC, msg, result.as_mut()).await?;
    Ok(result)
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub async fn u2f_counter_set(counter: u32) -> Result<(), ()> {
    let mut data = read_arbitrary_data().await.map_err(|_| ())?;
    data.set_u2f_counter(counter);
    write_arbitary_data(&data).await.map_err(|_| ())
}

#[cfg(feature = "app-u2f")]
pub async fn u2f_counter_inc() -> Result<u32, ()> {
    let mut data = read_arbitrary_data().await.map_err(|_| ())?;
    let counter = data.u2f_counter().wrapping_add(1);
    data.set_u2f_counter(counter);
    write_arbitary_data(&data).await.map_err(|_| ())?;
    Ok(counter)
}

pub fn model() -> Result<Model, ()> {
    Ok(Model::OPTIGA_TRUST_M_V3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_platform_host::memory::FakeMemory;
    use hex_lit::hex;

    //------------------------------------------------------------------------------
    // Fixed test vectors / keys (deterministic fakes).

    const SALT_ROOT_FIXED: [u8; 32] = [0x42; 32];

    struct TestRandom;

    // Unused in these unit tests. The fake `ops::random_32_bytes()` implementation ignores the
    // HAL RNG and returns fixed test vectors instead.
    impl bitbox_hal::Random for TestRandom {
        fn factory_randomness(&mut self) -> &'static [u8; 32] {
            unreachable!("unused in optiga unit tests")
        }

        fn mcu_32_bytes(&mut self, _out: &mut [u8; 32]) {
            unreachable!("unused in optiga unit tests")
        }
    }

    fn setup_test() -> (std::sync::MutexGuard<'static, ()>, FakeMemory) {
        let guard = ops::test_lock();
        ops::test_reset();
        let mut memory = FakeMemory::new();
        // Provides a fixed salt root for deterministic hash_data() results.
        memory.set_salt_root(&SALT_ROOT_FIXED);
        (guard, memory)
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_attestation_sign() {
        let (_guard, _memory) = setup_test();
        let mut signature = [0u8; 64];
        attestation_sign(&[0x42; 32], &mut signature).await.unwrap();
        assert_eq!(
            signature,
            hex!(
                "0000000000000000000000000000000000000000000000000000000000001234\
                 000000000000000000000000000000000000000000000000000000000000abcd"
            ),
        );
    }

    #[cfg(feature = "app-u2f")]
    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_u2f_counter_set() {
        let (_guard, _memory) = setup_test();
        u2f_counter_set(42).await.unwrap();
        assert_eq!(read_arbitrary_data().await.unwrap().u2f_counter(), 42);
    }

    #[cfg(feature = "app-u2f")]
    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_u2f_counter_inc() {
        let (_guard, _memory) = setup_test();
        u2f_counter_set(42).await.unwrap();
        assert_eq!(u2f_counter_inc().await.unwrap(), 43);
        assert_eq!(read_arbitrary_data().await.unwrap().u2f_counter(), 43);
    }

    // Expected stretched_out for password "pw" for the V0 algorithm given the deterministic fake
    // constants in ops_fake.rs.
    //
    // Repro script (mirrors stretch_password() with the unit test fakes):
    // ```python
    // import hashlib, hmac
    //
    // def sha256(b: bytes) -> bytes:
    //     return hashlib.sha256(b).digest()
    //
    // def hmac_sha256(key: bytes, msg: bytes) -> bytes:
    //     return hmac.new(key, msg, hashlib.sha256).digest()
    //
    // def salt_hash_data(data: bytes, purpose: bytes, salt_root: bytes) -> bytes:
    //     return sha256(salt_root + purpose + data)
    //
    // def kdf_internal(msg: bytes, cmac_key: bytes) -> bytes:
    //     # crypt_symmetric_encrypt_sync fake: HMAC-SHA256(cmac_key, msg)[:16]
    //     return sha256(hmac_sha256(cmac_key, msg)[:16])
    //
    // def kdf_hmac(msg: bytes, hmac_key: bytes) -> bytes:
    //     # crypt_hmac_sync fake: HMAC-SHA256(hmac_key, msg)
    //     return hmac_sha256(hmac_key, msg)
    //
    // salt_root = bytes([0x42]) * 32
    // cmac_key = bytes([0xA0]) * 32
    // hmac_key = bytes([0xB0]) * 32
    // password_secret = bytes([0x99]) * 32
    // password = b"pw"
    //
    // kdf_in = salt_hash_data(password, b"optiga_password_stretch_in", salt_root)
    // stretched = kdf_internal(kdf_in, cmac_key)
    // for _ in range(2):
    //     stretched = kdf_hmac(stretched, hmac_key)
    // stretched = hmac_sha256(password_secret, stretched)
    // out_salt = salt_hash_data(password, b"optiga_password_stretch_out", salt_root)
    // stretched = hmac_sha256(out_salt, stretched)
    // print(stretched.hex())
    // ```
    const EXPECTED_STRETCHED_OUT_V0: [u8; 32] =
        hex!("c41f87b7c9f3169c14f3f26287093c311819067776f6163b8a0fdf3dfb8b8ebb");

    // Expected stretched_out for password "pw" for the V1 algorithm given the deterministic fake
    // constants in ops_fake.rs.
    //
    // Repro script (mirrors stretch_password() with the unit test fakes):
    // ```python
    // import hashlib, hmac
    //
    // def sha256(b: bytes) -> bytes:
    //     return hashlib.sha256(b).digest()
    //
    // def hmac_sha256(key: bytes, msg: bytes) -> bytes:
    //     return hmac.new(key, msg, hashlib.sha256).digest()
    //
    // def salt_hash_data(data: bytes, purpose: bytes, salt_root: bytes) -> bytes:
    //     return sha256(salt_root + purpose + data)
    //
    // def kdf_internal(msg: bytes, cmac_key: bytes) -> bytes:
    //     # crypt_symmetric_encrypt_sync fake: HMAC-SHA256(cmac_key, msg)[:16]
    //     return sha256(hmac_sha256(cmac_key, msg)[:16])
    //
    // def kdf_hmac(msg: bytes, hmac_key: bytes) -> bytes:
    //     # crypt_hmac_sync fake: HMAC-SHA256(hmac_key, msg)
    //     return hmac_sha256(hmac_key, msg)
    //
    // salt_root = bytes([0x42]) * 32
    // cmac_key = bytes([0xA0]) * 32
    // hmac_writeprotected_key = bytes([0xC0]) * 32
    // password_secret = bytes([0x99]) * 32
    // password = b"pw"
    //
    // kdf_in = salt_hash_data(password, b"optiga_password_stretch_in", salt_root)
    // stretched = kdf_internal(kdf_in, cmac_key)
    // stretched = kdf_hmac(stretched, hmac_writeprotected_key)
    // stretched = hmac_sha256(password_secret, stretched)
    // out_salt = salt_hash_data(password, b"optiga_password_stretch_out", salt_root)
    // stretched = hmac_sha256(out_salt, stretched)
    // print(stretched.hex())
    // ```
    const EXPECTED_STRETCHED_OUT_V1: [u8; 32] =
        hex!("c59ec3c3b1c45f7e7639a629f5b34d1e4dc508f3b5b9577dd9dd57eecf496751");

    fn seed_v0_password(memory: &mut FakeMemory, password: &str) {
        // Seed the OID_PASSWORD and OID_PASSWORD_COUNTER objects as if they were
        // provisioned earlier.
        let oid_password =
            bitbox_core_utils::salt::hash_data(memory, password.as_bytes(), "optiga_password")
                .unwrap();
        ops::test_seed_oid_password(&oid_password);
        ops::test_set_counter(OID_COUNTER_PASSWORD, 0, SMALL_MONOTONIC_COUNTER_MAX_USE);
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_optiga_stretch_password_v0_success() {
        let (_guard, mut memory) = setup_test();
        seed_v0_password(&mut memory, "pw");

        let stretched_out = stretch_password(
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
        )
        .await
        .unwrap();

        assert_eq!(
            stretched_out.as_slice(),
            EXPECTED_STRETCHED_OUT_V0.as_slice()
        );
        // Successful password verification resets the small monotonic counter/threshold.
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_optiga_stretch_password_v0_attempt_counter() {
        let (_guard, mut memory) = setup_test();
        seed_v0_password(&mut memory, "pw");

        assert_eq!(
            stretch_password(
                &mut memory,
                "wrong",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
            )
            .await,
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            )),
        );
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 1);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        assert_eq!(
            stretch_password(
                &mut memory,
                "wrong",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
            )
            .await,
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            )),
        );
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 2);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        stretch_password(
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
        )
        .await
        .unwrap();
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        for _ in 0..SMALL_MONOTONIC_COUNTER_MAX_USE {
            assert_eq!(
                stretch_password(
                    &mut memory,
                    "wrong",
                    PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
                )
                .await,
                Err(Error::SecureChip(
                    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                )),
            );
        }
        assert_eq!(
            ops::test_get_counter(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        // After exhausting all allowed attempts, a correct password fails as well.
        assert_eq!(
            stretch_password(
                &mut memory,
                "pw",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
            )
            .await,
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            )),
        );
        assert_eq!(
            ops::test_get_counter(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    // Test that after initializing a new password, exhausting all allowed attempts locks means a
    // correct password fails as well.
    // Attempts after init are special because the PASSWORD_COUNTER init/threshold are offset by 1.
    async fn test_optiga_password_v1_stretch_exhaust_fails_after_init() {
        let (_guard, mut memory) = setup_test();
        let mut random = TestRandom;

        let stretched = init_new_password(
            &mut random,
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
        )
        .await
        .unwrap();
        assert_eq!(stretched.as_slice(), EXPECTED_STRETCHED_OUT_V1.as_slice());

        // Counter & threshold of password counter. After init, it is at 1, but the threshold is
        // increased by 1, so the number of attempts is still 10.
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 1);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE + 1,
        );
        // Counter & threshold of hmac_writeprotected counter.
        assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        // Exhaust all attempts.
        for i in 1..=SMALL_MONOTONIC_COUNTER_MAX_USE {
            assert_eq!(
                stretch_password(
                    &mut memory,
                    "wrong",
                    PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
                )
                .await,
                Err(Error::SecureChip(
                    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                )),
            );

            // Counter & threshold of password counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 1 + i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_PASSWORD),
                SMALL_MONOTONIC_COUNTER_MAX_USE + 1,
            );
            // Counter & threshold of hmac_writeprotected counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
                SMALL_MONOTONIC_COUNTER_MAX_USE,
            );
        }

        // Even a correct password doesn't work.
        assert_eq!(
            stretch_password(
                &mut memory,
                "pw",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
            )
            .await,
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            )),
        );
        let stretched = [0u8; KDF_LEN];
        assert_eq!(stretched.as_slice(), [0u8; KDF_LEN].as_slice());
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    // Test that after initializing a new password, one can make a few failed stretch attempts, and
    // that doing a correct attempt resets the counters.
    async fn test_optiga_password_v1() {
        let (_guard, mut memory) = setup_test();
        let mut random = TestRandom;

        let stretched = init_new_password(
            &mut random,
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
        )
        .await
        .unwrap();
        assert_eq!(stretched.as_slice(), EXPECTED_STRETCHED_OUT_V1.as_slice());

        // Counter & threshold of password counter. After init, it is at 1, but the threshold is
        // increased by 1, so the number of attempts is still 10.
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 1);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE + 1,
        );
        // Counter & threshold of hmac_writeprotected counter.
        assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        // A few failed attempts:
        for i in 1..=2 {
            assert_eq!(
                stretch_password(
                    &mut memory,
                    "wrong",
                    PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
                )
                .await,
                Err(Error::SecureChip(
                    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                )),
            );

            // Counter & threshold of password counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 1 + i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_PASSWORD),
                SMALL_MONOTONIC_COUNTER_MAX_USE + 1,
            );
            // Counter & threshold of hmac_writeprotected counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
                SMALL_MONOTONIC_COUNTER_MAX_USE,
            );
        }

        // Correct attempt gets the right stretched value and resets counters.
        let stretched = stretch_password(
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
        )
        .await
        .unwrap();
        assert_eq!(stretched.as_slice(), EXPECTED_STRETCHED_OUT_V1.as_slice());
        // Counter & threshold of password counter.
        assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_PASSWORD),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );
        // Counter & threshold of hmac_writeprotected counter.
        assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), 0);
        assert_eq!(
            ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
            SMALL_MONOTONIC_COUNTER_MAX_USE,
        );

        // Exhaust all attempts. The PASSWORD_COUNTER during this is different to above as the
        // counter/threshold was reset to 0/MAX after the correct stretch attempt.
        for i in 1..=SMALL_MONOTONIC_COUNTER_MAX_USE {
            assert_eq!(
                stretch_password(
                    &mut memory,
                    "wrong",
                    PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
                )
                .await,
                Err(Error::SecureChip(
                    SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                )),
            );

            // Counter & threshold of password counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_PASSWORD), i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_PASSWORD),
                SMALL_MONOTONIC_COUNTER_MAX_USE,
            );
            // Counter & threshold of hmac_writeprotected counter.
            assert_eq!(ops::test_get_counter(OID_COUNTER_HMAC_WRITEPROTECTED), i);
            assert_eq!(
                ops::test_get_threshold(OID_COUNTER_HMAC_WRITEPROTECTED),
                SMALL_MONOTONIC_COUNTER_MAX_USE,
            );
        }

        // Even a correct password doesn't work anymore.
        assert_eq!(
            stretch_password(
                &mut memory,
                "pw",
                PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
            )
            .await,
            Err(Error::SecureChip(
                SecureChipError::SC_ERR_INCORRECT_PASSWORD,
            )),
        );
        let stretched = [0u8; KDF_LEN];
        assert_eq!(stretched.as_slice(), [0u8; KDF_LEN].as_slice());
    }
}
