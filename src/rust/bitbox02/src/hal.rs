// SPDX-License-Identifier: Apache-2.0

pub mod eeprom;
pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod timer;
pub mod ui;

use bitbox_hal::Hal;

pub struct BitBox02Hal<Timer = timer::BitBox02Timer> {
    ui: ui::BitBox02Ui<Timer>,
    sd: sd::BitBox02Sd,
    random: random::BitBox02Random,
    securechip: securechip::BitBox02SecureChip<Timer>,
    memory: memory::BitBox02Memory,
    eeprom: eeprom::BitBox02Eeprom,
    system: system::BitBox02System<Timer>,
}

impl<Timer: bitbox_hal::timer::Timer> grounded::const_init::ConstInit for BitBox02Hal<Timer> {
    const VAL: Self = Self::new();
}

impl<Timer> BitBox02Hal<Timer> {
    pub const fn new() -> Self {
        Self {
            ui: ui::BitBox02Ui::new(),
            sd: sd::BitBox02Sd,
            random: random::BitBox02Random,
            securechip: securechip::BitBox02SecureChip::new(),
            memory: memory::BitBox02Memory,
            eeprom: eeprom::BitBox02Eeprom,
            system: system::BitBox02System::new(),
        }
    }
}

impl<Timer> Default for BitBox02Hal<Timer> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Timer: bitbox_hal::timer::Timer> Hal for BitBox02Hal<Timer> {
    type Ui = ui::BitBox02Ui<Timer>;
    type Random = random::BitBox02Random;
    type Sd = sd::BitBox02Sd;
    type SecureChip = securechip::BitBox02SecureChip<Timer>;
    type Memory = memory::BitBox02Memory;
    type Eeprom = eeprom::BitBox02Eeprom;
    type System = system::BitBox02System<Timer>;

    fn as_mut(
        &mut self,
    ) -> bitbox_hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::Eeprom,
        Self::System,
    > {
        bitbox_hal::HalSubsystems {
            ui: &mut self.ui,
            random: &mut self.random,
            sd: &mut self.sd,
            securechip: &mut self.securechip,
            memory: &mut self.memory,
            eeprom: &mut self.eeprom,
            system: &mut self.system,
        }
    }
}
