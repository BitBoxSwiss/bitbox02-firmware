// SPDX-License-Identifier: Apache-2.0

pub struct TestingSystem {
    ble_reset_count: u32,
    btconly: bool,
}

impl TestingSystem {
    pub fn new() -> Self {
        Self {
            ble_reset_count: 0,
            btconly: false,
        }
    }

    pub fn ble_reset_count(&self) -> u32 {
        self.ble_reset_count
    }

    pub fn set_btconly(&mut self, btconly: bool) {
        self.btconly = btconly;
    }
}

impl crate::hal::System for TestingSystem {
    async fn startup() {}

    fn communication_timeout_reset(&mut self, _value: i16) {}

    fn is_btconly(&mut self) -> bool {
        self.btconly
    }

    fn reboot(&mut self) -> ! {
        panic!("reboot called")
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        panic!("reboot_to_bootloader called")
    }

    fn reset_ble(&mut self) {
        self.ble_reset_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::System;

    #[test]
    fn test_reset_ble() {
        let mut system = TestingSystem::new();
        assert_eq!(system.ble_reset_count(), 0);
        system.reset_ble();
        system.reset_ble();
        assert_eq!(system.ble_reset_count(), 2);
    }

    #[test]
    fn test_is_btconly() {
        let mut system = TestingSystem::new();
        assert!(!system.is_btconly());
        system.set_btconly(true);
        assert!(system.is_btconly());
    }
}
