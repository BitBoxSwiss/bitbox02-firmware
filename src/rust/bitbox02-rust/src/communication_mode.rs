// SPDX-License-Identifier: Apache-2.0

//! The BitBox02 Nova has two communication modes: USB and Bluetooth.
//! Bluetooth is active until the first USB request is seen, at which point USB takes priority.

use crate::hal::Memory;
use util::cell::SyncCell;

static USB_HWW_REQUEST_SEEN: SyncCell<bool> = SyncCell::new(false);
static HAS_BLE: SyncCell<Option<bool>> = SyncCell::new(None);

/// Call this when the first USB request is seen. After this, `ble_enabled()` returns false even on
/// Bluetooth-enabled devices (USB takes priority).
pub fn ble_disable() {
    USB_HWW_REQUEST_SEEN.write(true);
}

/// Returns true if this device is Bluetooth-enabled and we have not seen a USB request yet, which
/// means we are communicating via Bluetooth.
pub fn ble_enabled(hal: &mut impl crate::hal::Hal) -> bool {
    !USB_HWW_REQUEST_SEEN.read() && has_ble(hal)
}

fn has_ble(hal: &mut impl crate::hal::Hal) -> bool {
    if let Some(has_ble) = HAS_BLE.read() {
        return has_ble;
    }

    let has_ble = matches!(
        hal.memory().get_platform(),
        Ok(bitbox02::memory::Platform::BitBox02Plus),
    );
    HAS_BLE.write(Some(has_ble));
    has_ble
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_disable() {
    ble_disable();
}

/// C interface.
#[unsafe(no_mangle)]
pub extern "C" fn rust_communication_mode_ble_enabled() -> bool {
    ble_enabled(&mut crate::hal::BitBox02Hal::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;

    fn reset_for_testing() {
        USB_HWW_REQUEST_SEEN.write(false);
        HAS_BLE.write(None);
    }

    #[test]
    fn test_ble_disabled_on_non_plus() {
        reset_for_testing();
        let mut hal = TestingHal::new();
        hal.memory
            .set_platform(bitbox02::memory::Platform::BitBox02);

        assert!(!ble_enabled(&mut hal));

        ble_disable();
        assert!(!ble_enabled(&mut hal));
    }

    #[test]
    fn test_ble_enabled_until_usb_request_seen() {
        reset_for_testing();
        let mut hal = TestingHal::new();
        hal.memory
            .set_platform(bitbox02::memory::Platform::BitBox02Plus);

        assert!(ble_enabled(&mut hal));

        ble_disable();
        assert!(!ble_enabled(&mut hal));
    }
}
