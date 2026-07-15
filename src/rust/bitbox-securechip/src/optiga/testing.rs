// SPDX-License-Identifier: Apache-2.0

//! Test controls for the Optiga raw backend and production async engine.

use super::{AsyncOpState, STATE, WAKER, raw};

pub(in crate::optiga) fn lock() -> std::sync::MutexGuard<'static, ()> {
    raw::test_lock()
}

pub(in crate::optiga) fn reset() {
    STATE.write(AsyncOpState::Idle);
    WAKER.clear();
    raw::test_reset();
}

pub(in crate::optiga) fn block_next_operation() {
    raw::test_block_next_operation();
}

pub(in crate::optiga) fn complete_blocked_operation() {
    raw::test_complete_blocked_operation();
}

pub(in crate::optiga) fn fail_next_start(error: u16) {
    raw::test_fail_next_start(error);
}

pub(in crate::optiga) fn fail_next_completion(error: u16) {
    raw::test_fail_next_completion(error);
}

pub(in crate::optiga) fn set_next_read_len(len: u16) {
    raw::test_set_next_read_len(len);
}

pub(in crate::optiga) fn started_operations() -> usize {
    raw::test_started_operations()
}

pub(in crate::optiga) fn completed_pointer_accesses() -> usize {
    raw::test_completed_pointer_accesses()
}

pub(in crate::optiga) fn seed_oid_password(password_hash: &[u8; super::super::KDF_LEN]) {
    raw::test_seed_oid_password(password_hash);
}

pub(in crate::optiga) fn set_counter(oid: u16, counter: u32, threshold: u32) {
    raw::test_set_counter(oid, counter, threshold);
}

pub(in crate::optiga) fn counter(oid: u16) -> u32 {
    raw::test_get_counter(oid)
}

pub(in crate::optiga) fn threshold(oid: u16) -> u32 {
    raw::test_get_threshold(oid)
}
