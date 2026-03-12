// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;
pub mod io;
mod memory;
mod random;
mod sd;
mod securechip;
mod system;
mod ui;

use bitbox_hal as hal;
use bitbox_lvgl::LvDisplay;

pub struct BitBox03 {
    ui: ui::BitBox03Ui,
    random: random::BitBox03Random,
    sd: sd::BitBox03Sd,
    securechip: securechip::BitBox03SecureChip,
    memory: memory::BitBox03Memory,
    system: system::BitBox03System,
}

impl BitBox03 {
    pub fn new() -> BitBox03 {
        BitBox03 {
            ui: ui::BitBox03Ui::new(),
            random: random::BitBox03Random {},
            sd: sd::BitBox03Sd {},
            securechip: securechip::BitBox03SecureChip {},
            memory: memory::BitBox03Memory {},
            system: system::BitBox03System {},
        }
    }

    pub fn init(&mut self, display: LvDisplay) {
        self.ui.init(display);
    }
}

impl hal::Hal for BitBox03 {
    type Ui = ui::BitBox03Ui;

    type Random = random::BitBox03Random;

    type Sd = sd::BitBox03Sd;

    type SecureChip = securechip::BitBox03SecureChip;

    type Memory = memory::BitBox03Memory;

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
        Self::System,
    > {
        hal::HalSubsystems {
            ui: &mut self.ui,
            random: &mut self.random,
            sd: &mut self.sd,
            securechip: &mut self.securechip,
            memory: &mut self.memory,
            system: &mut self.system,
        }
    }
}
