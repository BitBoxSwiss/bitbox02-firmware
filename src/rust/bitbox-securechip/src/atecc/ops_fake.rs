// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::extra_unused_type_parameters)]

use bitbox_hal::timer::Timer;
use bitbox_securechip_sys::atecc_slot_t as Slot;
use core::future::poll_fn;
use core::task::Poll;
use std::sync::{LazyLock, Mutex, MutexGuard};
use util::sha2::hmac_sha256 as util_hmac_sha256;
use zeroize::Zeroizing;

pub(super) const NONCE_NUMIN_SIZE: usize = bitbox_securechip_sys::NONCE_NUMIN_SIZE as usize;
const BLOCK_SIZE: usize = 32;
const SLOT_ROLLKEY: Slot = Slot::ATECC_SLOT_ROLLKEY;
const SLOT_KDF: Slot = Slot::ATECC_SLOT_KDF;
const SLOT_DATA0: Slot = Slot::ATECC_SLOT_DATA0;

#[derive(Clone)]
struct FakeState {
    data0_block0: [u8; BLOCK_SIZE],
    block_next_chip_command: bool,
    blocked_chip_command_active: bool,
    derivekey_rollkey_calls: usize,
    rollkey_kdf_calls: usize,
    kdf_calls: usize,
    kdf_key_write_calls: usize,
}

impl Default for FakeState {
    fn default() -> Self {
        Self {
            data0_block0: [0; BLOCK_SIZE],
            block_next_chip_command: false,
            blocked_chip_command_active: false,
            derivekey_rollkey_calls: 0,
            rollkey_kdf_calls: 0,
            kdf_calls: 0,
            kdf_key_write_calls: 0,
        }
    }
}

static TEST_LOCK: Mutex<()> = Mutex::new(());
static STATE: LazyLock<Mutex<FakeState>> = LazyLock::new(|| Mutex::new(FakeState::default()));

fn lock_state() -> MutexGuard<'static, FakeState> {
    STATE.lock().unwrap()
}

fn fake_hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    util_hmac_sha256(key, data, &mut result);
    result
}

async fn maybe_block_next_chip_command() {
    let should_block = {
        let mut state = lock_state();
        if state.block_next_chip_command {
            state.block_next_chip_command = false;
            state.blocked_chip_command_active = true;
            true
        } else {
            false
        }
    };

    if !should_block {
        return;
    }

    poll_fn(|_| {
        if lock_state().blocked_chip_command_active {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
    .await;
}

pub(super) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

pub(super) fn test_reset() {
    *lock_state() = FakeState::default();
}

pub(super) fn test_block_next_chip_command() {
    let mut state = lock_state();
    state.block_next_chip_command = true;
}

pub(super) fn test_unblock_chip_command() {
    let mut state = lock_state();
    state.blocked_chip_command_active = false;
}

pub(super) fn test_get_derivekey_rollkey_calls() -> usize {
    lock_state().derivekey_rollkey_calls
}

pub(super) fn test_get_rollkey_kdf_calls() -> usize {
    lock_state().rollkey_kdf_calls
}

pub(super) fn test_get_kdf_calls() -> usize {
    lock_state().kdf_calls
}

pub(super) fn test_get_kdf_key_write_calls() -> usize {
    lock_state().kdf_key_write_calls
}

pub(super) async fn chip_nonce_rand<T: Timer>(
    _num_in: &[u8; NONCE_NUMIN_SIZE],
) -> Result<Zeroizing<[u8; 32]>, i32> {
    maybe_block_next_chip_command().await;
    Ok(Zeroizing::new([0u8; 32]))
}

pub(super) async fn chip_checkmac<T: Timer>(_response: &Zeroizing<[u8; 32]>) -> Result<(), i32> {
    maybe_block_next_chip_command().await;
    Ok(())
}

pub(super) async fn chip_random<T: Timer>() -> Result<Zeroizing<[u8; 32]>, i32> {
    maybe_block_next_chip_command().await;
    Ok(Zeroizing::new([0u8; 32]))
}

pub(super) async fn chip_counter_read<T: Timer>() -> Result<u32, i32> {
    maybe_block_next_chip_command().await;
    Ok(2_097_150)
}

pub(super) async fn chip_info_revision<T: Timer>() -> Result<[u8; 4], i32> {
    maybe_block_next_chip_command().await;
    Ok([0, 0, 0, 0x03])
}

pub(super) async fn chip_kdf<T: Timer>(
    slot: Slot,
    msg: &[u8; 32],
) -> Result<(Zeroizing<[u8; 32]>, Zeroizing<[u8; 32]>), i32> {
    maybe_block_next_chip_command().await;

    let key = if slot == SLOT_ROLLKEY {
        let mut state = lock_state();
        state.rollkey_kdf_calls += 1;
        b"atecc-rollkey".as_slice()
    } else {
        let mut state = lock_state();
        state.kdf_calls += 1;
        b"atecc-kdf".as_slice()
    };
    Ok((
        Zeroizing::new(fake_hmac_sha256(key, msg)),
        Zeroizing::new([0u8; 32]),
    ))
}

pub(super) async fn chip_derivekey_rollkey<T: Timer>() -> Result<(), i32> {
    maybe_block_next_chip_command().await;
    let mut state = lock_state();
    state.derivekey_rollkey_calls += 1;
    Ok(())
}

pub(super) async fn chip_nonce_load_msgdigest<T: Timer>(_msg: &[u8; 32]) -> Result<(), i32> {
    maybe_block_next_chip_command().await;
    Ok(())
}

pub(super) async fn chip_sign_attestation<T: Timer>() -> Result<[u8; 64], i32> {
    maybe_block_next_chip_command().await;
    Err(-1)
}

pub(super) async fn chip_gendig_encryption_key<T: Timer>() -> Result<(), i32> {
    maybe_block_next_chip_command().await;
    Ok(())
}

pub(super) async fn chip_read_block<T: Timer>(
    slot: Slot,
    block: u8,
) -> Result<Zeroizing<[u8; BLOCK_SIZE]>, i32> {
    maybe_block_next_chip_command().await;
    if slot == SLOT_DATA0 && block == 0 {
        Ok(Zeroizing::new(lock_state().data0_block0))
    } else {
        Ok(Zeroizing::new([0u8; BLOCK_SIZE]))
    }
}

pub(super) async fn chip_write_encrypted_block<T: Timer>(
    slot: Slot,
    block: u8,
    value: &Zeroizing<[u8; BLOCK_SIZE]>,
    _mac: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> Result<(), i32> {
    maybe_block_next_chip_command().await;
    if slot == SLOT_DATA0 && block == 0 {
        let mut state = lock_state();
        state.data0_block0 = **value;
    }
    Ok(())
}

pub(super) async fn host_check_mac(
    _num_in: &[u8; NONCE_NUMIN_SIZE],
    _rand_out: &Zeroizing<[u8; 32]>,
    _auth_key: &[u8; 32],
) -> Result<Zeroizing<[u8; 32]>, i32> {
    Ok(Zeroizing::new([0u8; 32]))
}

pub(super) async fn host_kdf_decrypt(
    _io_protection_key: &[u8; 32],
    _nonce_out: &Zeroizing<[u8; 32]>,
    _data: &mut Zeroizing<[u8; 32]>,
) -> Result<(), i32> {
    Ok(())
}

pub(super) async fn host_nonce(
    _num_in: &[u8; NONCE_NUMIN_SIZE],
    _rand_out: &Zeroizing<[u8; 32]>,
) -> Result<(), i32> {
    Ok(())
}

pub(super) async fn host_gendig(_encryption_key: &[u8; 32]) -> Result<(), i32> {
    Ok(())
}

pub(super) async fn host_write_auth_mac(
    slot: Slot,
    _block: u8,
    input_data: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> Result<(Zeroizing<[u8; BLOCK_SIZE]>, Zeroizing<[u8; BLOCK_SIZE]>), i32> {
    if slot == SLOT_KDF {
        let mut state = lock_state();
        state.kdf_key_write_calls += 1;
    }
    Ok((
        Zeroizing::new(**input_data),
        Zeroizing::new([0u8; BLOCK_SIZE]),
    ))
}

pub(super) async fn host_io_decrypt(_data: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> Result<(), i32> {
    Ok(())
}
