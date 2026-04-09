// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use alloc::{boxed::Box, vec::Vec};
use hex_lit::hex;
use std::sync::{LazyLock, Mutex, MutexGuard};
use zeroize::Zeroizing;

//------------------------------------------------------------------------------
// Fixed test vectors / keys (deterministic fakes).

const KDF_CMAC_KEY_FIXED: [u8; super::KDF_LEN] = [0xA0; super::KDF_LEN];
const KDF_HMAC_KEY_FIXED: [u8; super::KDF_LEN] = [0xB0; super::KDF_LEN];
const KDF_HMAC_WRITEPROTECTED_KEY_FIXED: [u8; super::KDF_LEN] = [0xC0; super::KDF_LEN];
const PASSWORD_SECRET_FIXED: [u8; super::KDF_LEN] = [0x99; super::KDF_LEN];
const AUTH_CODE_RANDOM_FIXED: [u8; super::KDF_LEN] = [0x77; super::KDF_LEN];
const OPTIGA_CRYPT_ERROR: i32 = bitbox_securechip_sys::OPTIGA_CRYPT_ERROR as i32;
const OPTIGA_CRYPT_ERROR_INVALID_INPUT: i32 =
    bitbox_securechip_sys::OPTIGA_CRYPT_ERROR_INVALID_INPUT as i32;
const OPTIGA_UTIL_ERROR: i32 = bitbox_securechip_sys::OPTIGA_UTIL_ERROR as i32;
const OPTIGA_UTIL_ERROR_INVALID_INPUT: i32 =
    bitbox_securechip_sys::OPTIGA_UTIL_ERROR_INVALID_INPUT as i32;
const OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT: i32 =
    bitbox_securechip_sys::OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT as i32;

#[derive(Clone)]
struct FakeState {
    oid_password: [u8; super::KDF_LEN],
    oid_password_set: bool,
    oid_arbitrary_data: [u8; super::ARBITRARY_DATA_LEN],
    oid_counter_password_buf: [u8; 8],
    oid_counter_hmac_writeprotected_buf: [u8; 8],
    authorized_password: bool,
    authorized_password_secret: bool,
    random_ctr: usize,
}

impl Default for FakeState {
    fn default() -> Self {
        Self {
            oid_password: [0; super::KDF_LEN],
            oid_password_set: false,
            oid_arbitrary_data: [0; super::ARBITRARY_DATA_LEN],
            oid_counter_password_buf: [0; 8],
            oid_counter_hmac_writeprotected_buf: [0; 8],
            authorized_password: false,
            authorized_password_secret: false,
            random_ctr: 0,
        }
    }
}

static TEST_LOCK: Mutex<()> = Mutex::new(());
static STATE: LazyLock<Mutex<FakeState>> = LazyLock::new(|| Mutex::new(FakeState::default()));

fn lock_state() -> MutexGuard<'static, FakeState> {
    STATE.lock().unwrap()
}

fn counter_buf(state: &FakeState, oid: u16) -> &[u8; 8] {
    match oid {
        super::OID_COUNTER_PASSWORD => &state.oid_counter_password_buf,
        super::OID_COUNTER_HMAC_WRITEPROTECTED => &state.oid_counter_hmac_writeprotected_buf,
        _ => panic!("unexpected counter oid"),
    }
}

fn counter_buf_mut(state: &mut FakeState, oid: u16) -> &mut [u8; 8] {
    match oid {
        super::OID_COUNTER_PASSWORD => &mut state.oid_counter_password_buf,
        super::OID_COUNTER_HMAC_WRITEPROTECTED => &mut state.oid_counter_hmac_writeprotected_buf,
        _ => panic!("unexpected counter oid"),
    }
}

fn get_counter_from_buf(buf: &[u8; 8]) -> u32 {
    u32::from_be_bytes(buf[..4].try_into().unwrap())
}

fn get_threshold_from_buf(buf: &[u8; 8]) -> u32 {
    u32::from_be_bytes(buf[4..].try_into().unwrap())
}

fn set_counter_in_buf(buf: &mut [u8; 8], counter: u32) {
    buf[..4].copy_from_slice(&counter.to_be_bytes());
}

fn set_threshold_in_buf(buf: &mut [u8; 8], threshold: u32) {
    buf[4..].copy_from_slice(&threshold.to_be_bytes());
}

fn compute_hmac(key: &[u8], data: &[u8]) -> [u8; super::KDF_LEN] {
    let mut out = [0u8; super::KDF_LEN];
    super::hmac_sha256(key, data, &mut out);
    out
}

pub(super) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

pub(super) fn test_reset() {
    *lock_state() = FakeState::default();
}

pub(super) fn test_seed_oid_password(password_hash: &[u8; super::KDF_LEN]) {
    let mut state = lock_state();
    state.oid_password = *password_hash;
    state.oid_password_set = true;
}

pub(super) fn test_set_counter(oid: u16, counter: u32, threshold: u32) {
    let mut state = lock_state();
    let buf = counter_buf_mut(&mut state, oid);
    set_counter_in_buf(buf, counter);
    set_threshold_in_buf(buf, threshold);
}

pub(super) fn test_get_counter(oid: u16) -> u32 {
    let state = lock_state();
    get_counter_from_buf(counter_buf(&state, oid))
}

pub(super) fn test_get_threshold(oid: u16) -> u32 {
    let state = lock_state();
    get_threshold_from_buf(counter_buf(&state, oid))
}

//------------------------------------------------------------------------------
// Fake optiga_ops API surface (unit-test seam).

pub(super) async fn util_read_data(oid: u16, offset: u16, out: &mut [u8]) -> Result<(), Error> {
    if offset != 0 {
        return Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT));
    }

    let state = lock_state();
    match oid {
        super::OID_PASSWORD_SECRET => {
            if !state.authorized_password {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR));
            }
            if out.len() != super::KDF_LEN {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT));
            }
            out.copy_from_slice(&PASSWORD_SECRET_FIXED);
            Ok(())
        }
        super::OID_COUNTER => {
            if out.len() != 4 {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT));
            }
            out.fill(0);
            Ok(())
        }
        super::OID_ARBITRARY_DATA => {
            if out.len() != super::ARBITRARY_DATA_LEN {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT));
            }
            out.copy_from_slice(&state.oid_arbitrary_data);
            Ok(())
        }
        _ => Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT)),
    }
}

pub(super) async fn crypt_hmac(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    msg: &[u8; super::KDF_LEN],
    mac_out: &mut [u8; super::KDF_LEN],
) -> Result<(), Error> {
    crypt_hmac_sync(hmac_type, secret, msg, mac_out)
}

pub(super) async fn util_write_data(
    oid: u16,
    write_type: u8,
    offset: u16,
    buffer: &[u8],
) -> Result<(), Error> {
    util_write_data_sync(oid, write_type, offset, buffer)
}

pub(super) async fn crypt_symmetric_encrypt(
    encryption_mode: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t,
    symmetric_key_oid: bitbox_securechip_sys::optiga_key_id_t,
    plain_data: &[u8; super::KDF_LEN],
    encrypted_data: &mut [u8; 16],
) -> Result<(), Error> {
    crypt_symmetric_encrypt_sync(
        encryption_mode,
        symmetric_key_oid,
        plain_data,
        encrypted_data,
    )
}

pub(super) async fn crypt_generate_auth_code(
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    random_data: &mut [u8; 32],
) -> Result<(), Error> {
    crypt_generate_auth_code_sync(rng_type, random_data)
}

pub(super) async fn crypt_ecdsa_sign(
    _digest: &[u8; super::KDF_LEN],
    _private_key: bitbox_securechip_sys::optiga_key_id_t,
) -> Result<Vec<u8>, Error> {
    const SIG_DER: [u8; 9] = hex!("02021234020300abcd");
    Ok(SIG_DER.to_vec())
}

pub(super) async fn crypt_hmac_verify(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input_data: &[u8; super::KDF_LEN],
    hmac: &[u8; super::KDF_LEN],
) -> Result<(), Error> {
    crypt_hmac_verify_sync(hmac_type, secret, input_data, hmac)
}

pub(super) async fn crypt_symmetric_generate_key(
    key_type: bitbox_securechip_sys::optiga_symmetric_key_type_t,
    key_usage: bitbox_securechip_sys::optiga_key_usage_t,
) -> Result<(), Error> {
    crypt_symmetric_generate_key_sync(key_type, key_usage)
}

pub(super) async fn crypt_clear_auto_state(secret: u16) -> Result<(), Error> {
    crypt_clear_auto_state_sync(secret)
}

pub(super) async fn crypt_random(
    _rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    out: &mut [u8; 32],
) -> Result<(), Error> {
    *out = [0u8; 32];
    Ok(())
}

pub(super) fn random_32_bytes(
    _random: &mut impl bitbox_hal::Random,
    _mixin: &[u8; super::KDF_LEN],
) -> Result<Box<Zeroizing<[u8; super::KDF_LEN]>>, Error> {
    let mut state = lock_state();
    let src = match state.random_ctr {
        0 => KDF_HMAC_KEY_FIXED,
        1 => PASSWORD_SECRET_FIXED,
        2 => KDF_HMAC_WRITEPROTECTED_KEY_FIXED,
        _ => unreachable!(),
    };
    state.random_ctr = (state.random_ctr + 1) % 3;
    Ok(Box::new(Zeroizing::new(src)))
}

pub(super) fn util_write_data_sync(
    oid: u16,
    _write_type: u8,
    offset: u16,
    buffer: &[u8],
) -> Result<(), Error> {
    if offset != 0 {
        return Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT));
    }

    let mut state = lock_state();
    match oid {
        super::OID_PASSWORD => {
            if !state.authorized_password_secret || buffer.len() != super::KDF_LEN {
                return Err(Error::from_status(if !state.authorized_password_secret {
                    OPTIGA_UTIL_ERROR
                } else {
                    OPTIGA_UTIL_ERROR_INVALID_INPUT
                }));
            }
            state.oid_password.copy_from_slice(buffer);
            state.oid_password_set = true;
            Ok(())
        }
        super::OID_COUNTER_PASSWORD | super::OID_COUNTER_HMAC_WRITEPROTECTED => {
            if oid == super::OID_COUNTER_HMAC_WRITEPROTECTED && !state.authorized_password {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR));
            }
            if buffer.len() != 8 {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT));
            }
            counter_buf_mut(&mut state, oid).copy_from_slice(buffer);
            Ok(())
        }
        super::OID_HMAC => {
            if buffer.len() != super::KDF_LEN {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT));
            }
            assert_eq!(buffer, KDF_HMAC_KEY_FIXED.as_slice());
            Ok(())
        }
        super::OID_HMAC_WRITEPROTECTED => {
            if !state.authorized_password || buffer.len() != super::KDF_LEN {
                return Err(Error::from_status(if !state.authorized_password {
                    OPTIGA_UTIL_ERROR
                } else {
                    OPTIGA_UTIL_ERROR_INVALID_INPUT
                }));
            }
            assert_eq!(buffer, KDF_HMAC_WRITEPROTECTED_KEY_FIXED.as_slice());
            Ok(())
        }
        super::OID_PASSWORD_SECRET => {
            assert_eq!(buffer, PASSWORD_SECRET_FIXED.as_slice());
            Ok(())
        }
        super::OID_ARBITRARY_DATA => {
            if buffer.len() != super::ARBITRARY_DATA_LEN {
                return Err(Error::from_status(OPTIGA_UTIL_ERROR_INVALID_INPUT));
            }
            state.oid_arbitrary_data.copy_from_slice(buffer);
            Ok(())
        }
        // Accept other writes without emulating full semantics (counter reset, hmac key, etc.).
        _ => Ok(()),
    }
}

pub(super) fn crypt_hmac_sync(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input_data: &[u8; super::KDF_LEN],
    mac_out: &mut [u8; super::KDF_LEN],
) -> Result<(), Error> {
    // Use hmac_sha256 with a different fixed key and msg as the value.
    if hmac_type != super::OPTIGA_HMAC_SHA_256 {
        return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT));
    }

    let mut state = lock_state();
    let key = match secret {
        super::OID_HMAC => &KDF_HMAC_KEY_FIXED,
        super::OID_HMAC_WRITEPROTECTED => {
            // Emulate the small monotonic counter that is attached to using the
            // hmac_writeprotected slot. Stored as {counter_be_u32, threshold_be_u32}.
            let buf = counter_buf_mut(&mut state, super::OID_COUNTER_HMAC_WRITEPROTECTED);
            let counter = get_counter_from_buf(buf);
            let threshold = get_threshold_from_buf(buf);
            if counter >= threshold {
                return Err(Error::from_status(super::OPTIGA_HMAC_VERIFY_FAIL));
            }
            set_counter_in_buf(buf, counter + 1);
            &KDF_HMAC_WRITEPROTECTED_KEY_FIXED
        }
        _ => return Err(Error::from_status(OPTIGA_CRYPT_ERROR)),
    };

    mac_out.copy_from_slice(&compute_hmac(key, input_data));
    Ok(())
}

pub(super) fn crypt_symmetric_encrypt_sync(
    encryption_mode: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t,
    symmetric_key_oid: bitbox_securechip_sys::optiga_key_id_t,
    plain_data: &[u8; super::KDF_LEN],
    encrypted_data: &mut [u8; 16],
) -> Result<(), Error> {
    // Use hmac_sha256 with a fixed key and msg as the value, truncated to 16 bytes.
    if encryption_mode != super::OPTIGA_SYMMETRIC_CMAC
        || symmetric_key_oid != super::key_id_from_oid(super::OID_AES_SYMKEY)
    {
        return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT));
    }
    let out = compute_hmac(&KDF_CMAC_KEY_FIXED, plain_data);
    encrypted_data.copy_from_slice(&out[..16]);
    Ok(())
}

pub(super) fn crypt_symmetric_generate_key_sync(
    key_type: bitbox_securechip_sys::optiga_symmetric_key_type_t,
    key_usage: bitbox_securechip_sys::optiga_key_usage_t,
) -> Result<(), Error> {
    if key_type != super::OPTIGA_SYMMETRIC_AES_256
        || key_usage != super::OPTIGA_KEY_USAGE_ENCRYPTION
    {
        return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT));
    }
    // We keep using the fixed cmac key in the tests.
    Ok(())
}

pub(super) fn crypt_generate_auth_code_sync(
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    random_data: &mut [u8; 32],
) -> Result<(), Error> {
    if rng_type != super::OPTIGA_RNG_TYPE_TRNG {
        return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT));
    }
    random_data.copy_from_slice(&AUTH_CODE_RANDOM_FIXED);
    Ok(())
}

pub(super) fn crypt_hmac_verify_sync(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input_data: &[u8; super::KDF_LEN],
    hmac: &[u8; super::KDF_LEN],
) -> Result<(), Error> {
    if hmac_type != super::OPTIGA_HMAC_SHA_256 {
        return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT));
    }

    let mut state = lock_state();
    let key = match secret {
        super::OID_PASSWORD_SECRET => &PASSWORD_SECRET_FIXED,
        super::OID_PASSWORD => {
            // Emulate the small monotonic counter that is attached to password authorization.
            // Stored as {counter_be_u32, threshold_be_u32}.
            let buf = counter_buf_mut(&mut state, super::OID_COUNTER_PASSWORD);
            let counter = get_counter_from_buf(buf);
            let threshold = get_threshold_from_buf(buf);
            if counter >= threshold {
                return Err(Error::from_status(super::OPTIGA_HMAC_VERIFY_FAIL));
            }
            set_counter_in_buf(buf, counter + 1);

            if !state.oid_password_set {
                return Err(Error::from_status(OPTIGA_CRYPT_ERROR));
            }
            &state.oid_password
        }
        _ => return Err(Error::from_status(OPTIGA_CRYPT_ERROR_INVALID_INPUT)),
    };

    let computed = compute_hmac(key, input_data);
    if computed.as_slice() != hmac.as_slice() {
        return Err(Error::from_status(super::OPTIGA_HMAC_VERIFY_FAIL));
    }

    match secret {
        super::OID_PASSWORD => state.authorized_password = true,
        super::OID_PASSWORD_SECRET => state.authorized_password_secret = true,
        _ => {}
    }
    Ok(())
}

pub(super) fn crypt_clear_auto_state_sync(secret: u16) -> Result<(), Error> {
    let mut state = lock_state();
    match secret {
        super::OID_PASSWORD => state.authorized_password = false,
        super::OID_PASSWORD_SECRET => state.authorized_password_secret = false,
        _ => {}
    }
    Ok(())
}
