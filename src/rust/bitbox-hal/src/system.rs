// SPDX-License-Identifier: Apache-2.0

use core::sync::atomic::{AtomicU32, Ordering};

const COMMUNICATION_TIMEOUT_MS: i64 = 500;
const COMMUNICATION_TIMEOUT_TICK_MS: i64 = 100;
const NO_PENDING_COMMUNICATION_TIMEOUT_RESET_MS: u32 = u32::MAX;
static PENDING_COMMUNICATION_TIMEOUT_RESET_MS: AtomicU32 =
    AtomicU32::new(NO_PENDING_COMMUNICATION_TIMEOUT_RESET_MS);

#[allow(async_fn_in_trait)]
pub trait System {
    /// Runs device-specific startup UI/initialization before regular operation.
    ///
    /// Startup may briefly show the logo, but it must finish on the lockscreen.
    /// Here, "lockscreen" means the waiting screen that shows "See the BitBoxApp"
    /// and the (possibly empty) device name.
    async fn startup();

    /// Reset the communication timeout watchdog for outstanding operations.
    ///
    /// This watchdog tracks the amount of time to wait before an outstanding
    /// operation times out (for example, if the client closes).
    /// Use this for long running operations that are expected to take longer
    /// than about 300ms (a bit less than the 500ms timeout before a task is
    /// cancelled).
    fn communication_timeout_reset(&mut self, value: i16);

    fn is_btconly(&mut self) -> bool;
    fn reboot(&mut self) -> !;
    fn reboot_to_bootloader(&mut self) -> !;
    fn reset_ble(&mut self);
}

pub fn communication_timeout_reset_ms(value: i16) -> u32 {
    // BB02 stores this timeout as a watchdog counter that increments every 100 ms and times out
    // once it grows past the nominal 500 ms limit. Callers reset that counter directly, so a
    // negative value means "pretend less time has elapsed so far" and therefore extend the
    // deadline. The newer transports track a plain timeout duration in milliseconds instead, so we
    // convert the BB02-style counter reset into the equivalent timeout window here. Example: `-70`
    // means "move the counter back by 70 ticks", i.e. extend the timeout window by 7000 ms beyond
    // the normal 500 ms outstanding-operation timeout.
    let timeout_ms = COMMUNICATION_TIMEOUT_MS - i64::from(value) * COMMUNICATION_TIMEOUT_TICK_MS;
    timeout_ms.max(0) as u32
}

pub fn request_communication_timeout_reset_ms(timeout_ms: u32) {
    PENDING_COMMUNICATION_TIMEOUT_RESET_MS.store(timeout_ms, Ordering::Relaxed);
}

pub fn take_pending_communication_timeout_reset_ms() -> Option<u32> {
    match PENDING_COMMUNICATION_TIMEOUT_RESET_MS
        .swap(NO_PENDING_COMMUNICATION_TIMEOUT_RESET_MS, Ordering::Relaxed)
    {
        NO_PENDING_COMMUNICATION_TIMEOUT_RESET_MS => None,
        timeout_ms => Some(timeout_ms),
    }
}
