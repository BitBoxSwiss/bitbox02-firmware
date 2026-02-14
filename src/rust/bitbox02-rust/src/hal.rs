// SPDX-License-Identifier: Apache-2.0

pub mod bitbox02;
pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

#[cfg(feature = "testing")]
pub mod testing;

pub use bitbox02::BitBox02Hal;
pub use memory::Memory;
pub use random::Random;
pub use sd::Sd;
pub use securechip::SecureChip;
pub use system::System;
pub use ui::Ui;

pub struct HalSubsystems<
    'a,
    Ui: ui::Ui,
    Random: random::Random,
    Sd: sd::Sd,
    SecureChip: securechip::SecureChip,
    Memory: memory::Memory,
    System: system::System,
> {
    pub ui: &'a mut Ui,
    pub random: &'a mut Random,
    pub sd: &'a mut Sd,
    pub securechip: &'a mut SecureChip,
    pub memory: &'a mut Memory,
    pub system: &'a mut System,
}

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    type Ui: ui::Ui;
    type Random: random::Random;
    type Sd: sd::Sd;
    type SecureChip: securechip::SecureChip;
    type Memory: memory::Memory;
    type System: system::System;

    fn subsystems(
        &mut self,
    ) -> HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::System,
    >;

    fn ui(&mut self) -> &mut Self::Ui {
        self.subsystems().ui
    }

    fn random(&mut self) -> &mut Self::Random {
        self.subsystems().random
    }

    fn sd(&mut self) -> &mut Self::Sd {
        self.subsystems().sd
    }

    fn securechip(&mut self) -> &mut Self::SecureChip {
        self.subsystems().securechip
    }

    fn memory(&mut self) -> &mut Self::Memory {
        self.subsystems().memory
    }

    fn system(&mut self) -> &mut Self::System {
        self.subsystems().system
    }
}
