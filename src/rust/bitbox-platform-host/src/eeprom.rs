// SPDX-License-Identifier: Apache-2.0

pub struct FakeEeprom {
    pub enabled: bool,
    unlock_attempts: u8,
}

impl FakeEeprom {
    pub fn new() -> Self {
        Self {
            enabled: true,
            unlock_attempts: 0,
        }
    }

    pub fn set_unlock_attempts_for_testing(&mut self, attempts: u8) {
        self.unlock_attempts = attempts;
    }
}

impl bitbox_hal::Eeprom for FakeEeprom {
    fn setup(&mut self) {
        self.enabled = true;
        self.unlock_attempts = 0;
    }

    fn init(&mut self) {}

    fn is_enabled(&mut self) -> bool {
        self.enabled
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        self.unlock_attempts
    }

    fn increment_unlock_attempts(&mut self) {
        self.unlock_attempts += 1;
    }

    fn reset_unlock_attempts(&mut self) {
        self.unlock_attempts = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox_hal::Eeprom;

    #[test]
    fn test_disable() {
        let mut eeprom = FakeEeprom::new();
        assert!(eeprom.enabled);
        eeprom.disable();
        assert!(!eeprom.enabled);
        eeprom.setup();
        assert!(eeprom.enabled);
    }

    #[test]
    fn test_unlock_attempts() {
        let mut eeprom = FakeEeprom::new();
        assert_eq!(eeprom.get_unlock_attempts(), 0);
        eeprom.increment_unlock_attempts();
        eeprom.increment_unlock_attempts();
        assert_eq!(eeprom.get_unlock_attempts(), 2);
        eeprom.reset_unlock_attempts();
        assert_eq!(eeprom.get_unlock_attempts(), 0);
    }

    #[test]
    fn test_is_enabled() {
        let mut eeprom = FakeEeprom::new();
        assert!(eeprom.is_enabled());
        eeprom.disable();
        assert!(!eeprom.is_enabled());
    }
}
