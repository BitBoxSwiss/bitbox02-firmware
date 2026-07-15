// SPDX-License-Identifier: Apache-2.0

use super::{ATECC_OPS_STATUS_BUSY, BLOCK_SIZE, NONCE_NUMIN_SIZE, SIGNATURE_SIZE, Slot};
use std::sync::{LazyLock, Mutex, MutexGuard};
use util::sha2::hmac_sha256;
use zeroize::Zeroizing;

const SLOT_ROLLKEY: Slot = Slot::ATECC_SLOT_ROLLKEY;
const SLOT_KDF: Slot = Slot::ATECC_SLOT_KDF;
const SLOT_DATA0: Slot = Slot::ATECC_SLOT_DATA0;

enum Command {
    NonceRand,
    Checkmac,
    Random,
    CounterRead,
    InfoRevision,
    Kdf(Slot, [u8; 32]),
    DerivekeyRollkey,
    NonceLoadMsgdigest,
    SignAttestation,
    GendigEncryptionKey,
    ReadBlock(Slot, u8),
    WriteEncryptedBlock(Slot, u8, [u8; BLOCK_SIZE]),
}

/// Models the raw ATECC command context, responses, and test-controlled completion behavior used
/// to exercise the production async engine without hardware.
struct FakeState {
    status: i32,
    pending: Option<Command>,
    block_next_command: bool,
    blocked_command_active: bool,
    next_start_error: Option<i32>,
    next_completion_error: Option<i32>,
    next_response_error: Option<i32>,
    started_commands: usize,
    data0_block0: [u8; BLOCK_SIZE],
    random_response: [u8; 32],
    counter_response: u32,
    info_response: [u8; 4],
    kdf_response: [u8; 32],
    kdf_nonce_response: [u8; 32],
    read_block_response: [u8; BLOCK_SIZE],
    derivekey_rollkey_calls: usize,
    rollkey_kdf_calls: usize,
    kdf_calls: usize,
    kdf_key_write_calls: usize,
    io_temp_key_active: bool,
}

impl Default for FakeState {
    fn default() -> Self {
        Self {
            status: 0,
            pending: None,
            block_next_command: false,
            blocked_command_active: false,
            next_start_error: None,
            next_completion_error: None,
            next_response_error: None,
            started_commands: 0,
            data0_block0: [0; BLOCK_SIZE],
            random_response: [0; 32],
            counter_response: 2_097_150,
            info_response: [0, 0, 0, 0x03],
            kdf_response: [0; 32],
            kdf_nonce_response: [0; 32],
            read_block_response: [0; BLOCK_SIZE],
            derivekey_rollkey_calls: 0,
            rollkey_kdf_calls: 0,
            kdf_calls: 0,
            kdf_key_write_calls: 0,
            io_temp_key_active: false,
        }
    }
}

static TEST_LOCK: Mutex<()> = Mutex::new(());
static STATE: LazyLock<Mutex<FakeState>> = LazyLock::new(|| Mutex::new(FakeState::default()));

fn lock_state() -> MutexGuard<'static, FakeState> {
    STATE.lock().unwrap()
}

fn start(command: Command) -> i32 {
    let mut state = lock_state();
    state.started_commands += 1;
    if let Some(error) = state.next_start_error.take() {
        state.status = error;
        return error;
    }
    assert!(state.pending.is_none());
    state.status = ATECC_OPS_STATUS_BUSY;
    state.blocked_command_active = state.block_next_command;
    state.block_next_command = false;
    state.pending = Some(command);
    0
}

fn complete(state: &mut FakeState, command: Command) {
    match command {
        Command::Kdf(slot, msg) => {
            let key = if slot == SLOT_ROLLKEY {
                state.rollkey_kdf_calls += 1;
                b"atecc-rollkey".as_slice()
            } else {
                state.kdf_calls += 1;
                b"atecc-kdf".as_slice()
            };
            hmac_sha256(key, &msg, &mut state.kdf_response);
            state.kdf_nonce_response.fill(0);
        }
        Command::DerivekeyRollkey => state.derivekey_rollkey_calls += 1,
        Command::ReadBlock(slot, block) => {
            state.read_block_response = if slot == SLOT_DATA0 && block == 0 {
                state.data0_block0
            } else {
                [0; BLOCK_SIZE]
            };
        }
        Command::WriteEncryptedBlock(slot, block, value) => {
            if slot == SLOT_DATA0 && block == 0 {
                state.data0_block0 = value;
            }
        }
        Command::NonceRand
        | Command::Checkmac
        | Command::Random
        | Command::CounterRead
        | Command::InfoRevision
        | Command::NonceLoadMsgdigest
        | Command::SignAttestation
        | Command::GendigEncryptionKey => {}
    }
}

fn response_status(state: &mut FakeState) -> i32 {
    state.next_response_error.take().unwrap_or(0)
}

pub(super) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

pub(super) fn test_reset() {
    *lock_state() = FakeState::default();
}

pub(super) fn test_block_next_chip_command() {
    lock_state().block_next_command = true;
}

pub(super) fn test_unblock_chip_command() {
    lock_state().blocked_command_active = false;
}

pub(super) fn test_fail_next_start(error: i32) {
    lock_state().next_start_error = Some(error);
}

pub(super) fn test_fail_next_completion(error: i32) {
    lock_state().next_completion_error = Some(error);
}

pub(super) fn test_fail_next_response(error: i32) {
    lock_state().next_response_error = Some(error);
}

pub(super) fn test_started_commands() -> usize {
    lock_state().started_commands
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

pub(super) fn test_io_temp_key_active() -> bool {
    lock_state().io_temp_key_active
}

pub(super) fn status() -> i32 {
    lock_state().status
}

pub(super) fn poll_delay_ms() -> u32 {
    1
}

pub(super) fn poll() {
    let mut state = lock_state();
    if state.status != ATECC_OPS_STATUS_BUSY || state.blocked_command_active {
        return;
    }
    let command = state.pending.take().unwrap();
    if let Some(error) = state.next_completion_error.take() {
        state.status = error;
        return;
    }
    complete(&mut state, command);
    state.status = 0;
}

pub(super) fn clear_io_temp_key() {
    lock_state().io_temp_key_active = false;
}

pub(super) fn start_nonce_rand(_num_in: &[u8; NONCE_NUMIN_SIZE]) -> i32 {
    start(Command::NonceRand)
}

pub(super) fn start_checkmac(_response: &Zeroizing<[u8; 32]>) -> i32 {
    start(Command::Checkmac)
}

pub(super) fn start_random() -> i32 {
    start(Command::Random)
}

pub(super) fn start_counter_read() -> i32 {
    start(Command::CounterRead)
}

pub(super) fn start_info_revision() -> i32 {
    start(Command::InfoRevision)
}

pub(super) fn start_kdf(slot: Slot, msg: &[u8; 32]) -> i32 {
    start(Command::Kdf(slot, *msg))
}

pub(super) fn start_derivekey_rollkey() -> i32 {
    start(Command::DerivekeyRollkey)
}

pub(super) fn start_nonce_load_msgdigest(_msg: &[u8; 32]) -> i32 {
    start(Command::NonceLoadMsgdigest)
}

pub(super) fn start_sign_attestation() -> i32 {
    start(Command::SignAttestation)
}

pub(super) fn start_gendig_encryption_key() -> i32 {
    start(Command::GendigEncryptionKey)
}

pub(super) fn start_read_block(slot: Slot, block: u8) -> i32 {
    start(Command::ReadBlock(slot, block))
}

pub(super) fn start_write_encrypted_block(
    slot: Slot,
    block: u8,
    value: &Zeroizing<[u8; BLOCK_SIZE]>,
    _mac: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> i32 {
    start(Command::WriteEncryptedBlock(slot, block, **value))
}

pub(super) fn read_random_response(out: &mut Zeroizing<[u8; 32]>) -> i32 {
    let mut state = lock_state();
    let status = response_status(&mut state);
    if status == 0 {
        **out = state.random_response;
    }
    status
}

pub(super) fn read_counter_response(out: &mut u32) -> i32 {
    let mut state = lock_state();
    let status = response_status(&mut state);
    if status == 0 {
        *out = state.counter_response;
    }
    status
}

pub(super) fn read_info_response(out: &mut [u8; 4]) -> i32 {
    let mut state = lock_state();
    let status = response_status(&mut state);
    if status == 0 {
        *out = state.info_response;
    }
    status
}

pub(super) fn read_kdf_response(
    data: &mut Zeroizing<[u8; 32]>,
    nonce: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    let mut state = lock_state();
    let status = response_status(&mut state);
    if status == 0 {
        **data = state.kdf_response;
        **nonce = state.kdf_nonce_response;
    }
    status
}

pub(super) fn read_sign_response(_out: &mut [u8; SIGNATURE_SIZE]) -> i32 {
    let mut state = lock_state();
    state.next_response_error.take().unwrap_or(-1)
}

pub(super) fn read_block_response(out: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> i32 {
    let mut state = lock_state();
    let status = response_status(&mut state);
    if status == 0 {
        **out = state.read_block_response;
    }
    status
}

pub(super) fn auth_compute_response(
    _num_in: &[u8; NONCE_NUMIN_SIZE],
    _rand_out: &Zeroizing<[u8; 32]>,
    _auth_key: &[u8; 32],
    response: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    response.fill(0);
    0
}

pub(super) fn kdf_decrypt(
    _io_protection_key: &[u8; 32],
    _nonce_out: &Zeroizing<[u8; 32]>,
    _data: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    0
}

pub(super) fn io_prepare_tempkey(
    _num_in: &[u8; NONCE_NUMIN_SIZE],
    _rand_out: &Zeroizing<[u8; 32]>,
) -> i32 {
    lock_state().io_temp_key_active = true;
    0
}

pub(super) fn io_apply_gendig(_encryption_key: &[u8; 32]) -> i32 {
    0
}

pub(super) fn io_prepare_encrypted_write(
    slot: Slot,
    _block: u8,
    input: &Zeroizing<[u8; BLOCK_SIZE]>,
    encrypted: &mut Zeroizing<[u8; BLOCK_SIZE]>,
    mac: &mut Zeroizing<[u8; BLOCK_SIZE]>,
) -> i32 {
    if slot == SLOT_KDF {
        lock_state().kdf_key_write_calls += 1;
    }
    **encrypted = **input;
    mac.fill(0);
    0
}

pub(super) fn io_decrypt_block(_data: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> i32 {
    0
}
