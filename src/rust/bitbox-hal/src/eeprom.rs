// SPDX-License-Identifier: Apache-2.0

/// Flash-backed EEPROM emulation required by the BitBox HAL.
///
/// Implementations provide two pieces of behavior:
/// - device-specific EEPROM-emulation bring-up/teardown
/// - persistent storage for the unlock-attempt counter
///
/// The expected lifecycle is to call [`Self::setup`] during startup to ensure the backend is
/// configured, then [`Self::init`] to initialize or migrate the BitBox-specific data stored in
/// the emulated EEPROM before accessing the counter.
pub trait Eeprom {
    /// Ensures the flash-backed EEPROM backend is configured for BitBox use.
    ///
    /// Hardware implementations may update the MCU's EEPROM-emulation configuration and can
    /// require a reboot before the new configuration takes effect.
    fn setup(&mut self);

    /// Initializes or migrates the BitBox-specific contents stored in the emulated EEPROM.
    ///
    /// This should be safe to call on every boot after [`Self::setup`].
    fn init(&mut self);

    /// Returns whether the flash-backed EEPROM emulation is currently enabled.
    fn is_enabled(&mut self) -> bool;

    /// Disables the flash-backed EEPROM emulation.
    ///
    /// On hardware, the change may only take effect after a reboot.
    fn disable(&mut self);

    /// Returns the persisted number of recorded unlock attempts.
    ///
    /// This counter is used to enforce the maximum number of allowed unlock attempts. Callers
    /// increment it before an unlock attempt and reset it after a successful unlock.
    fn get_unlock_attempts(&mut self) -> u8;

    /// Increments the persisted unlock-attempt counter by one.
    ///
    /// Implementations are expected to reject invalid state or increments past the supported
    /// maximum instead of silently wrapping.
    fn increment_unlock_attempts(&mut self);

    /// Resets the persisted unlock-attempt counter to zero.
    fn reset_unlock_attempts(&mut self);
}
