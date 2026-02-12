// SPDX-License-Identifier: Apache-2.0

pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

use crate::hal::{Hal, Memory, Random, Sd, SecureChip, System, Ui};

pub struct BitBox02Hal {
    ui: ui::BitBox02Ui,
    sd: sd::BitBox02Sd,
    random: random::BitBox02Random,
    securechip: securechip::BitBox02SecureChip,
    memory: memory::BitBox02Memory,
    system: system::BitBox02System,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: ui::BitBox02Ui,
            sd: sd::BitBox02Sd,
            random: random::BitBox02Random,
            securechip: securechip::BitBox02SecureChip,
            memory: memory::BitBox02Memory,
            system: system::BitBox02System,
        }
    }
}

impl Hal for BitBox02Hal {
    fn ui(&mut self) -> &mut impl Ui {
        &mut self.ui
    }

    fn sd(&mut self) -> &mut impl Sd {
        &mut self.sd
    }

    fn random(&mut self) -> &mut impl Random {
        &mut self.random
    }

    fn securechip(&mut self) -> &mut impl SecureChip {
        &mut self.securechip
    }

    fn memory(&mut self) -> &mut impl Memory {
        &mut self.memory
    }

    fn system(&mut self) -> &mut impl System {
        &mut self.system
    }
}
