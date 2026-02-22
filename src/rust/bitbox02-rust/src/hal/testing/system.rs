// SPDX-License-Identifier: Apache-2.0

pub struct TestingSystem {
    pub(crate) smarteeprom_enabled: bool,
    ble_reset_count: u32,
}

impl TestingSystem {
    pub fn new() -> Self {
        Self {
            smarteeprom_enabled: true,
            ble_reset_count: 0,
        }
    }

    pub fn ble_reset_count(&self) -> u32 {
        self.ble_reset_count
    }
}

impl crate::hal::System for TestingSystem {
    async fn startup() {}

    fn reboot(&mut self) -> ! {
        panic!("reboot called")
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        panic!("reboot_to_bootloader called")
    }

    fn reset_ble(&mut self) {
        self.ble_reset_count += 1;
    }

    fn smarteeprom_disable(&mut self) {
        self.smarteeprom_enabled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::System;

    #[test]
    fn test_smarteeprom_disable() {
        let mut system = TestingSystem::new();
        assert!(system.smarteeprom_enabled);
        system.smarteeprom_disable();
        assert!(!system.smarteeprom_enabled);
    }

    #[test]
    fn test_reset_ble() {
        let mut system = TestingSystem::new();
        assert_eq!(system.ble_reset_count(), 0);
        system.reset_ble();
        system.reset_ble();
        assert_eq!(system.ble_reset_count(), 2);
    }
}
