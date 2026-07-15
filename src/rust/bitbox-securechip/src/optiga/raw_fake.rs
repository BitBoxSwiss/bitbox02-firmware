// SPDX-License-Identifier: Apache-2.0

use hex_lit::hex;
use std::sync::{LazyLock, Mutex, MutexGuard};

const KDF_LEN: usize = super::super::KDF_LEN;
const KDF_CMAC_KEY_FIXED: [u8; KDF_LEN] = [0xA0; KDF_LEN];
// Outputs of the production entropy mixer for TestingRandom's first three values and the fake
// chip's all-zero random output.
const KDF_HMAC_KEY_FIXED: [u8; KDF_LEN] =
    hex!("39dfec3e1c0088b4dadc06ee8f5e0187fb2b93a957b0fc9fa7b80e303ab2f3c5");
const KDF_HMAC_WRITEPROTECTED_KEY_FIXED: [u8; KDF_LEN] =
    hex!("cb47020ccd1aaa6d7fc64ab812f83ff1996be6987c83d39cbdb9720f3501ce99");
const PASSWORD_SECRET_FIXED: [u8; KDF_LEN] =
    hex!("0abf2413d2f222b1c1b3ff60ff8392684bb5a33a1e3f7e94f45291172602b25b");
const AUTH_CODE_RANDOM_FIXED: [u8; KDF_LEN] = [0x77; KDF_LEN];
const OPTIGA_CRYPT_ERROR: u16 = bitbox_securechip_sys::OPTIGA_CRYPT_ERROR as u16;
const OPTIGA_CRYPT_ERROR_INVALID_INPUT: u16 =
    bitbox_securechip_sys::OPTIGA_CRYPT_ERROR_INVALID_INPUT as u16;
const OPTIGA_UTIL_ERROR: u16 = bitbox_securechip_sys::OPTIGA_UTIL_ERROR as u16;
const OPTIGA_UTIL_ERROR_INVALID_INPUT: u16 =
    bitbox_securechip_sys::OPTIGA_UTIL_ERROR_INVALID_INPUT as u16;
const OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT: u16 =
    bitbox_securechip_sys::OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT as u16;
const OPTIGA_LIB_SUCCESS: u16 = bitbox_securechip_sys::OPTIGA_LIB_SUCCESS as u16;
const OPTIGA_LIB_BUSY: u16 = bitbox_securechip_sys::OPTIGA_LIB_BUSY as u16;

struct RetainedMutPtr(*mut u8);

// The production wrapper gives the raw API pointers to static buffers and serializes access to
// them. This test-only wrapper retains such a pointer behind a mutex until fake completion.
unsafe impl Send for RetainedMutPtr {}

enum BlockedOperation {
    Completed(u16),
    CryptRandom {
        out: RetainedMutPtr,
        out_len: u16,
        completion_status: u16,
    },
}

/// Models Optiga object state and the raw asynchronous status/callback behavior needed to exercise
/// the production engine without a device.
struct FakeState {
    status: u16,
    block_next_operation: bool,
    blocked_operation: Option<BlockedOperation>,
    next_start_error: Option<u16>,
    next_completion_error: Option<u16>,
    next_read_len: Option<u16>,
    started_operations: usize,
    completed_pointer_accesses: usize,
    oid_password: [u8; KDF_LEN],
    oid_password_set: bool,
    #[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
    oid_arbitrary_data: [u8; super::super::ARBITRARY_DATA_LEN],
    oid_counter_password_buf: [u8; 8],
    oid_counter_hmac_writeprotected_buf: [u8; 8],
    authorized_password: bool,
    authorized_password_secret: bool,
}

impl Default for FakeState {
    fn default() -> Self {
        Self {
            status: OPTIGA_LIB_SUCCESS,
            block_next_operation: false,
            blocked_operation: None,
            next_start_error: None,
            next_completion_error: None,
            next_read_len: None,
            started_operations: 0,
            completed_pointer_accesses: 0,
            oid_password: [0; KDF_LEN],
            oid_password_set: false,
            #[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
            oid_arbitrary_data: [0; super::super::ARBITRARY_DATA_LEN],
            oid_counter_password_buf: [0; 8],
            oid_counter_hmac_writeprotected_buf: [0; 8],
            authorized_password: false,
            authorized_password_secret: false,
        }
    }
}

static TEST_LOCK: Mutex<()> = Mutex::new(());
static STATE: LazyLock<Mutex<FakeState>> = LazyLock::new(|| Mutex::new(FakeState::default()));

fn lock_state() -> MutexGuard<'static, FakeState> {
    STATE.lock().unwrap()
}

fn start(operation: impl FnOnce(&mut FakeState) -> u16) -> u16 {
    let mut state = lock_state();
    state.started_operations += 1;
    if let Some(error) = state.next_start_error.take() {
        return error;
    }

    let operation_status = operation(&mut state);
    let completion_status = state
        .next_completion_error
        .take()
        .unwrap_or(operation_status);
    if state.block_next_operation {
        state.block_next_operation = false;
        state.blocked_operation = Some(BlockedOperation::Completed(completion_status));
        state.status = OPTIGA_LIB_BUSY;
    } else {
        state.status = completion_status;
    }
    OPTIGA_LIB_SUCCESS
}

fn counter_buf(state: &FakeState, oid: u16) -> &[u8; 8] {
    match oid {
        super::super::OID_COUNTER_PASSWORD => &state.oid_counter_password_buf,
        super::super::OID_COUNTER_HMAC_WRITEPROTECTED => &state.oid_counter_hmac_writeprotected_buf,
        _ => panic!("unexpected counter oid"),
    }
}

fn counter_buf_mut(state: &mut FakeState, oid: u16) -> &mut [u8; 8] {
    match oid {
        super::super::OID_COUNTER_PASSWORD => &mut state.oid_counter_password_buf,
        super::super::OID_COUNTER_HMAC_WRITEPROTECTED => {
            &mut state.oid_counter_hmac_writeprotected_buf
        }
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

fn compute_hmac(key: &[u8], data: &[u8]) -> [u8; KDF_LEN] {
    let mut out = [0u8; KDF_LEN];
    super::super::hmac_sha256(key, data, &mut out);
    out
}

pub(super) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

pub(super) fn test_reset() {
    *lock_state() = FakeState::default();
}

pub(super) fn test_block_next_operation() {
    lock_state().block_next_operation = true;
}

pub(super) fn test_complete_blocked_operation() {
    let mut state = lock_state();
    let blocked_operation = state.blocked_operation.take().unwrap();
    state.status = match blocked_operation {
        BlockedOperation::Completed(status) => status,
        BlockedOperation::CryptRandom {
            out,
            out_len,
            completion_status,
        } => {
            if completion_status == OPTIGA_LIB_SUCCESS {
                unsafe {
                    core::slice::from_raw_parts_mut(out.0, out_len as usize).fill(0);
                }
                state.completed_pointer_accesses += 1;
            }
            completion_status
        }
    };
    drop(state);
    super::rust_optiga_callback_wake();
}

pub(super) fn test_fail_next_start(error: u16) {
    lock_state().next_start_error = Some(error);
}

pub(super) fn test_fail_next_completion(error: u16) {
    lock_state().next_completion_error = Some(error);
}

pub(super) fn test_set_next_read_len(len: u16) {
    lock_state().next_read_len = Some(len);
}

pub(super) fn test_started_operations() -> usize {
    lock_state().started_operations
}

pub(super) fn test_completed_pointer_accesses() -> usize {
    lock_state().completed_pointer_accesses
}

pub(super) fn test_seed_oid_password(password_hash: &[u8; KDF_LEN]) {
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

pub(super) fn status() -> bitbox_securechip_sys::optiga_lib_status_t {
    lock_state().status
}

pub(super) fn set_status_busy() {
    lock_state().status = OPTIGA_LIB_BUSY;
}

pub(super) fn util_instance() -> *mut bitbox_securechip_sys::optiga_util_t {
    core::ptr::null_mut()
}

pub(super) fn crypt_instance() -> *mut bitbox_securechip_sys::optiga_crypt_t {
    core::ptr::null_mut()
}

pub(super) unsafe fn util_read_data(
    _util: *mut bitbox_securechip_sys::optiga_util_t,
    oid: u16,
    offset: u16,
    out: *mut u8,
    out_len: *mut u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|state| {
        if offset != 0 {
            return OPTIGA_UTIL_ERROR_INVALID_INPUT;
        }
        let requested_len = unsafe { out_len.read() } as usize;
        let out = unsafe { core::slice::from_raw_parts_mut(out, requested_len) };
        let result = match oid {
            super::super::OID_PASSWORD_SECRET => {
                if !state.authorized_password {
                    OPTIGA_UTIL_ERROR
                } else if out.len() != KDF_LEN {
                    OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT
                } else {
                    out.copy_from_slice(&PASSWORD_SECRET_FIXED);
                    OPTIGA_LIB_SUCCESS
                }
            }
            super::super::OID_COUNTER => {
                if out.len() != 4 {
                    OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT
                } else {
                    out.fill(0);
                    OPTIGA_LIB_SUCCESS
                }
            }
            #[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
            super::super::OID_ARBITRARY_DATA => {
                if out.len() != super::super::ARBITRARY_DATA_LEN {
                    OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT
                } else {
                    out.copy_from_slice(&state.oid_arbitrary_data);
                    OPTIGA_LIB_SUCCESS
                }
            }
            _ => OPTIGA_UTIL_ERROR_INVALID_INPUT,
        };
        if let Some(len) = state.next_read_len.take() {
            unsafe {
                out_len.write(len);
            }
        }
        result
    })
}

pub(super) unsafe fn crypt_hmac(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input: *const u8,
    input_len: u32,
    mac: *mut u8,
    mac_len: *mut u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|state| {
        if hmac_type != super::super::OPTIGA_HMAC_SHA_256
            || input_len as usize != KDF_LEN
            || unsafe { mac_len.read() } as usize != KDF_LEN
        {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        let input = unsafe { core::slice::from_raw_parts(input, KDF_LEN) };
        let key = match secret {
            super::super::OID_HMAC => &KDF_HMAC_KEY_FIXED,
            super::super::OID_HMAC_WRITEPROTECTED => {
                // Emulate the small monotonic counter that is attached to using the
                // hmac_writeprotected slot. Stored as {counter_be_u32, threshold_be_u32}.
                let buf = counter_buf_mut(state, super::super::OID_COUNTER_HMAC_WRITEPROTECTED);
                let counter = get_counter_from_buf(buf);
                let threshold = get_threshold_from_buf(buf);
                if counter >= threshold {
                    return super::super::OPTIGA_HMAC_VERIFY_FAIL as u16;
                }
                set_counter_in_buf(buf, counter + 1);
                &KDF_HMAC_WRITEPROTECTED_KEY_FIXED
            }
            _ => return OPTIGA_CRYPT_ERROR,
        };
        unsafe {
            core::ptr::copy_nonoverlapping(compute_hmac(key, input).as_ptr(), mac, KDF_LEN);
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn util_write_data(
    _util: *mut bitbox_securechip_sys::optiga_util_t,
    oid: u16,
    _write_type: u8,
    offset: u16,
    input: *const u8,
    input_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|state| {
        if offset != 0 {
            return OPTIGA_UTIL_ERROR_INVALID_INPUT;
        }
        let input = unsafe { core::slice::from_raw_parts(input, input_len as usize) };
        match oid {
            super::super::OID_PASSWORD => {
                if !state.authorized_password_secret || input.len() != KDF_LEN {
                    return if !state.authorized_password_secret {
                        OPTIGA_UTIL_ERROR
                    } else {
                        OPTIGA_UTIL_ERROR_INVALID_INPUT
                    };
                }
                state.oid_password.copy_from_slice(input);
                state.oid_password_set = true;
                OPTIGA_LIB_SUCCESS
            }
            super::super::OID_COUNTER_PASSWORD | super::super::OID_COUNTER_HMAC_WRITEPROTECTED => {
                if oid == super::super::OID_COUNTER_HMAC_WRITEPROTECTED
                    && !state.authorized_password
                {
                    return OPTIGA_UTIL_ERROR;
                }
                if input.len() != 8 {
                    return OPTIGA_UTIL_ERROR_INVALID_INPUT;
                }
                counter_buf_mut(state, oid).copy_from_slice(input);
                OPTIGA_LIB_SUCCESS
            }
            super::super::OID_HMAC => {
                if input.len() != KDF_LEN {
                    return OPTIGA_UTIL_ERROR_INVALID_INPUT;
                }
                assert_eq!(input, KDF_HMAC_KEY_FIXED.as_slice());
                OPTIGA_LIB_SUCCESS
            }
            super::super::OID_HMAC_WRITEPROTECTED => {
                if !state.authorized_password || input.len() != KDF_LEN {
                    return if !state.authorized_password {
                        OPTIGA_UTIL_ERROR
                    } else {
                        OPTIGA_UTIL_ERROR_INVALID_INPUT
                    };
                }
                assert_eq!(input, KDF_HMAC_WRITEPROTECTED_KEY_FIXED.as_slice());
                OPTIGA_LIB_SUCCESS
            }
            super::super::OID_PASSWORD_SECRET => {
                assert_eq!(input, PASSWORD_SECRET_FIXED.as_slice());
                OPTIGA_LIB_SUCCESS
            }
            #[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
            super::super::OID_ARBITRARY_DATA => {
                if input.len() != super::super::ARBITRARY_DATA_LEN {
                    return OPTIGA_UTIL_ERROR_INVALID_INPUT;
                }
                state.oid_arbitrary_data.copy_from_slice(input);
                OPTIGA_LIB_SUCCESS
            }
            // Accept other writes without emulating full semantics (counter reset, hmac key, etc.).
            _ => OPTIGA_LIB_SUCCESS,
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn crypt_symmetric_encrypt(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    encryption_mode: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t,
    symmetric_key_oid: bitbox_securechip_sys::optiga_key_id_t,
    plain_data: *const u8,
    plain_data_len: u32,
    _iv: *const u8,
    _iv_len: u16,
    _associated_data: *const u8,
    _associated_data_len: u16,
    encrypted_data: *mut u8,
    encrypted_data_len: *mut u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|_| {
        if encryption_mode != super::super::OPTIGA_SYMMETRIC_CMAC
            || symmetric_key_oid != super::super::key_id_from_oid(super::super::OID_AES_SYMKEY)
            || plain_data_len as usize != KDF_LEN
            || unsafe { encrypted_data_len.read() } != 16
        {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        let input = unsafe { core::slice::from_raw_parts(plain_data, KDF_LEN) };
        let out = compute_hmac(&KDF_CMAC_KEY_FIXED, input);
        unsafe {
            core::ptr::copy_nonoverlapping(out.as_ptr(), encrypted_data, 16);
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_generate_auth_code(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    _optional_data: *const u8,
    _optional_data_len: u16,
    random_data: *mut u8,
    random_data_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|_| {
        if rng_type != super::super::OPTIGA_RNG_TYPE_TRNG || random_data_len as usize != KDF_LEN {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(AUTH_CODE_RANDOM_FIXED.as_ptr(), random_data, KDF_LEN);
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_ecdsa_sign(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    _digest: *const u8,
    digest_len: u8,
    _private_key: bitbox_securechip_sys::optiga_key_id_t,
    signature: *mut u8,
    signature_len: *mut u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    const SIG_DER: [u8; 9] = hex!("02021234020300abcd");
    start(|_| {
        if digest_len as usize != KDF_LEN || unsafe { signature_len.read() } < SIG_DER.len() as u16
        {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(SIG_DER.as_ptr(), signature, SIG_DER.len());
            signature_len.write(SIG_DER.len() as u16);
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_hmac_verify(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input: *const u8,
    input_len: u32,
    hmac: *const u8,
    hmac_len: u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|state| {
        if hmac_type != super::super::OPTIGA_HMAC_SHA_256
            || input_len as usize != KDF_LEN
            || hmac_len as usize != KDF_LEN
        {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        let input = unsafe { core::slice::from_raw_parts(input, KDF_LEN) };
        let hmac = unsafe { core::slice::from_raw_parts(hmac, KDF_LEN) };
        let key = match secret {
            super::super::OID_PASSWORD_SECRET => &PASSWORD_SECRET_FIXED,
            super::super::OID_PASSWORD => {
                // Emulate the small monotonic counter that is attached to password authorization.
                // Stored as {counter_be_u32, threshold_be_u32}.
                let buf = counter_buf_mut(state, super::super::OID_COUNTER_PASSWORD);
                let counter = get_counter_from_buf(buf);
                let threshold = get_threshold_from_buf(buf);
                if counter >= threshold {
                    return super::super::OPTIGA_HMAC_VERIFY_FAIL as u16;
                }
                set_counter_in_buf(buf, counter + 1);

                if !state.oid_password_set {
                    return OPTIGA_CRYPT_ERROR;
                }
                &state.oid_password
            }
            _ => return OPTIGA_CRYPT_ERROR_INVALID_INPUT,
        };

        if compute_hmac(key, input).as_slice() != hmac {
            return super::super::OPTIGA_HMAC_VERIFY_FAIL as u16;
        }
        match secret {
            super::super::OID_PASSWORD => state.authorized_password = true,
            super::super::OID_PASSWORD_SECRET => state.authorized_password_secret = true,
            _ => {}
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_symmetric_generate_key(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    key_type: bitbox_securechip_sys::optiga_symmetric_key_type_t,
    key_usage: u8,
    _export_symmetric_key: u8,
    symmetric_key: *mut core::ffi::c_void,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|_| {
        if key_type != super::super::OPTIGA_SYMMETRIC_AES_256
            || key_usage != super::super::OPTIGA_KEY_USAGE_ENCRYPTION as u8
            || unsafe {
                symmetric_key
                    .cast::<bitbox_securechip_sys::optiga_key_id_t>()
                    .read()
            } != super::super::key_id_from_oid(super::super::OID_AES_SYMKEY)
        {
            return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
        }
        // We keep using the fixed cmac key in the tests.
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_clear_auto_state(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    secret: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    start(|state| {
        match secret {
            super::super::OID_PASSWORD => state.authorized_password = false,
            super::super::OID_PASSWORD_SECRET => state.authorized_password_secret = false,
            _ => {}
        }
        OPTIGA_LIB_SUCCESS
    })
}

pub(super) unsafe fn crypt_random(
    _crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    _rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    out: *mut u8,
    out_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    let mut state = lock_state();
    state.started_operations += 1;
    if let Some(error) = state.next_start_error.take() {
        return error;
    }

    let completion_status = state
        .next_completion_error
        .take()
        .unwrap_or(OPTIGA_LIB_SUCCESS);
    if state.block_next_operation {
        state.block_next_operation = false;
        state.blocked_operation = Some(BlockedOperation::CryptRandom {
            out: RetainedMutPtr(out),
            out_len,
            completion_status,
        });
        state.status = OPTIGA_LIB_BUSY;
    } else {
        if completion_status == OPTIGA_LIB_SUCCESS {
            unsafe {
                core::slice::from_raw_parts_mut(out, out_len as usize).fill(0);
            }
        }
        state.status = completion_status;
    }
    OPTIGA_LIB_SUCCESS
}
