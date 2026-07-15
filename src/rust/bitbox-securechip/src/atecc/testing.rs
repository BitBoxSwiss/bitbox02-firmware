// SPDX-License-Identifier: Apache-2.0

//! Test controls for the ATECC raw backend and production async engine.

use super::{AsyncOpState, STATE, raw};

pub(in crate::atecc) fn lock() -> std::sync::MutexGuard<'static, ()> {
    raw::test_lock()
}

pub(in crate::atecc) fn reset() {
    STATE.write(AsyncOpState::Idle);
    raw::test_reset();
}

pub(in crate::atecc) fn block_next_chip_command() {
    raw::test_block_next_chip_command();
}

pub(in crate::atecc) fn unblock_chip_command() {
    raw::test_unblock_chip_command();
}

pub(in crate::atecc) fn fail_next_start(error: i32) {
    raw::test_fail_next_start(error);
}

pub(in crate::atecc) fn fail_next_completion(error: i32) {
    raw::test_fail_next_completion(error);
}

pub(in crate::atecc) fn fail_next_response(error: i32) {
    raw::test_fail_next_response(error);
}

pub(in crate::atecc) fn started_commands() -> usize {
    raw::test_started_commands()
}

pub(in crate::atecc) fn derivekey_rollkey_calls() -> usize {
    raw::test_get_derivekey_rollkey_calls()
}

pub(in crate::atecc) fn rollkey_kdf_calls() -> usize {
    raw::test_get_rollkey_kdf_calls()
}

pub(in crate::atecc) fn kdf_calls() -> usize {
    raw::test_get_kdf_calls()
}

pub(in crate::atecc) fn kdf_key_write_calls() -> usize {
    raw::test_get_kdf_key_write_calls()
}

pub(in crate::atecc) fn io_temp_key_active() -> bool {
    raw::test_io_temp_key_active()
}
