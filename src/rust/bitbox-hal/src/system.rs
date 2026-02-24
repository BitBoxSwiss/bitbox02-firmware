// SPDX-License-Identifier: Apache-2.0

#[allow(async_fn_in_trait)]
pub trait System {
    /// Runs device-specific startup UI/initialization before regular operation.
    ///
    /// Startup may briefly show the logo, but it must finish on the lockscreen.
    /// Here, "lockscreen" means the waiting screen that shows "See the BitBoxApp"
    /// and the (possibly empty) device name.
    async fn startup();

    fn reboot(&mut self) -> !;
    fn reboot_to_bootloader(&mut self) -> !;
    fn reset_ble(&mut self);
    fn smarteeprom_disable(&mut self);
}
