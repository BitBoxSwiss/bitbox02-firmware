use bitbox_hal::Eeprom;

pub struct BitBox03Eeprom {
    enabled: bool,
    unlock_attempts: u8,
}

impl BitBox03Eeprom {
    pub const fn new() -> Self {
        Self {
            enabled: true,
            unlock_attempts: 0,
        }
    }
}

impl Eeprom for BitBox03Eeprom {
    fn setup(&mut self) {
        self.enabled = true;
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
        self.unlock_attempts = self.unlock_attempts.saturating_add(1);
    }

    fn reset_unlock_attempts(&mut self) {
        self.unlock_attempts = 0;
    }
}
