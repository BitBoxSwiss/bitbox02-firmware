// SPDX-License-Identifier: Apache-2.0

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
