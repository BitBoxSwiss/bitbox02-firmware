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
    /// than the timeout window, which depends on the active transport.
    ///
    /// `value` counts in 100ms units: `0` restarts the normal window, a
    /// negative value extends it by `abs(value)` units.
    fn communication_timeout_reset(&mut self, value: i16);

    fn is_btconly(&mut self) -> bool;
    fn reboot(&mut self) -> !;
    fn reboot_to_bootloader(&mut self) -> !;
    fn reset_ble(&mut self);
}
