// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::general::abort;
use crate::hal::{SecureChip, Ui};

/// Resets the device:
/// - Updates secure chip KDF keys.
/// - Resets the securechip EEPROM (U2F counter).
/// - Resets MCU flash app memory.
/// - Disables SmartEEPROM memory (will be erased/setup on next boot).
/// - Shows a "Device reset" status message.
///
/// `status` selects whether the status message indicates success or failure (user invoked vs forced).
pub(crate) async fn reset(hal: &mut impl crate::hal::Hal, status: bool) {
    crate::keystore::lock();
    // Resetting takes longer than the default 500 ms watchdog. Bump the watchdog timeout to roughly
    // 7 seconds (longer than needed) so we don't assume communication was lost and this task gets
    // dropped at an await point.
    const LONG_TIMEOUT: i16 = -70;
    bitbox02::usb_processing::timeout_reset(LONG_TIMEOUT);

    // Reset secure chip keys and U2F counter with retries. We retry in case there are transient
    // errors.
    let mut reset_ok = false;
    for _ in 0..5 {
        if hal.securechip().reset_keys().is_ok() {
            reset_ok = true;
            break;
        }
    }
    if !reset_ok {
        abort("Could not reset secure chip.");
    }

    #[cfg(feature = "app-u2f")]
    {
        let mut u2f_ok = false;
        for _ in 0..5 {
            if hal.securechip().u2f_counter_set(0).is_ok() {
                u2f_ok = true;
                break;
            }
        }
        if !u2f_ok {
            abort("Could not initialize U2F counter.");
        }
    }

    if bitbox02::memory::reset_hww().is_err() {
        abort("Could not reset memory.");
    }

    // Disable SmartEEPROM so it will be erased on next reboot.
    bitbox02::smarteeprom::disable();

    // Show "Device reset" status using the UI workflow.
    hal.ui().status("Device reset", status).await;

    // The ble chip needs to be restarted to load the new secrets.
    if matches!(
        bitbox02::memory::get_platform(),
        Ok(bitbox02::memory::Platform::BitBox02Plus)
    ) {
        bitbox02::reset_ble();
    }

    #[cfg(not(feature = "testing"))]
    bitbox02::reboot();
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::SecureChip;
    use crate::hal::testing::TestingHal;
    use crate::keystore;
    use crate::keystore::testing::mock_unlocked;
    use crate::workflow::testing::Screen;
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    #[test]
    fn test_reset_success() {
        mock_memory();

        keystore::lock();
        mock_unlocked();
        bitbox02::memory::set_device_name("Custom name").unwrap();
        assert!(!keystore::is_locked());
        assert!(bitbox02::smarteeprom::is_enabled());

        let mut hal = TestingHal::new();
        // Make the reset keys call fail once, to test that it is retried.
        hal.securechip.mock_reset_keys_fails();

        // Simulate a non-zero U2F counter before reset.
        SecureChip::u2f_counter_set(&mut hal.securechip, 42).unwrap();

        hal.securechip.event_counter_reset();
        block_on(reset(&mut hal, true));
        // Secure chip operations happened as expected: reset_keys() was retried once, but only the
        // successful call increments the event counter.
        assert_eq!(hal.securechip.get_event_counter(), 1);

        // Keystore is locked again.
        assert!(keystore::is_locked());

        // Memory has been reset to factory defaults.
        assert_eq!(bitbox02::memory::get_device_name().as_str(), "My BitBox");

        // SmartEEPROM was disabled as part of the reset.
        assert!(!bitbox02::smarteeprom::is_enabled());

        assert_eq!(hal.securechip.get_u2f_counter(), 0);

        assert_eq!(
            hal.ui.screens,
            vec![Screen::Status {
                title: "Device reset".into(),
                success: true,
            }],
        );
    }

    #[test]
    fn test_reset_status_failure() {
        mock_memory();

        let mut hal = TestingHal::new();
        block_on(reset(&mut hal, false));

        assert_eq!(
            hal.ui.screens,
            vec![Screen::Status {
                title: "Device reset".into(),
                success: false,
            }],
        );
    }
}
