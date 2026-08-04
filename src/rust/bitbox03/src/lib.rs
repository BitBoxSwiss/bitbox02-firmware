// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

use littlefs2::driver::Storage;

mod eeprom;
pub mod io;
mod memory;
mod random;
mod sd;
mod securechip;
mod system;
pub mod timer;
pub mod ui;

pub use memory::BitBox03Memory;

use bitbox_hal as hal;
use bitbox_lvgl::LvDisplay;

struct BitBox03State<UserStorage, VendorStorage> {
    ui: ui::BitBox03Ui,
    random: random::BitBox03Random,
    sd: sd::BitBox03Sd,
    securechip: securechip::BitBox03SecureChip,
    memory: memory::BitBox03Memory<UserStorage, VendorStorage>,
    eeprom: eeprom::BitBox03Eeprom,
    system: system::BitBox03System,
}

impl<UserStorage, VendorStorage> BitBox03State<UserStorage, VendorStorage> {
    const fn new() -> Self {
        Self {
            ui: ui::BitBox03Ui::new(),
            random: random::BitBox03Random {},
            sd: sd::BitBox03Sd {},
            securechip: securechip::BitBox03SecureChip {},
            memory: memory::BitBox03Memory::new(),
            eeprom: eeprom::BitBox03Eeprom::new(),
            system: system::BitBox03System {},
        }
    }
}

pub struct BitBox03<UserStorage, VendorStorage> {
    state: BitBox03State<UserStorage, VendorStorage>,
}

impl<UserStorage, VendorStorage> BitBox03<UserStorage, VendorStorage> {
    pub const fn new() -> Self {
        Self {
            state: BitBox03State::new(),
        }
    }

    pub fn init(&mut self, display: LvDisplay) {
        self.state.ui.init(display);
    }
}

impl<UserStorage, VendorStorage> Default for BitBox03<UserStorage, VendorStorage> {
    fn default() -> Self {
        Self::new()
    }
}

impl<UserStorage, VendorStorage> hal::Hal for BitBox03<UserStorage, VendorStorage>
where
    UserStorage: Storage + Default,
    VendorStorage: Storage + Default,
{
    type Ui = ui::BitBox03Ui;
    type Random = random::BitBox03Random;
    type Sd = sd::BitBox03Sd;
    type SecureChip = securechip::BitBox03SecureChip;
    type Memory = memory::BitBox03Memory<UserStorage, VendorStorage>;
    type Eeprom = eeprom::BitBox03Eeprom;
    type System = system::BitBox03System;

    fn as_mut(
        &mut self,
    ) -> hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::Eeprom,
        Self::System,
    > {
        hal::HalSubsystems {
            ui: &mut self.state.ui,
            random: &mut self.state.random,
            sd: &mut self.state.sd,
            securechip: &mut self.state.securechip,
            memory: &mut self.state.memory,
            eeprom: &mut self.state.eeprom,
            system: &mut self.state.system,
        }
    }
}
