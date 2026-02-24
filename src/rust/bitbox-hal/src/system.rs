// SPDX-License-Identifier: Apache-2.0

#[allow(async_fn_in_trait)]
pub trait System {
    /// Runs device-specific startup UI/initialization before regular operation.
    async fn startup();

    fn reboot_to_bootloader(&mut self) -> !;
}
