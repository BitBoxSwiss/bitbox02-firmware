// SPDX-License-Identifier: Apache-2.0

pub mod eeprom;
pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

pub use eeprom::TestingEeprom;
pub use memory::TestingMemory;
pub use random::TestingRandom;
pub use sd::TestingSd;
pub use securechip::TestingSecureChip;
pub use system::TestingSystem;
pub use ui::{Screen, TestingUi};

pub struct TestingHal<'a> {
    pub ui: TestingUi<'a>,
    pub sd: TestingSd,
    pub random: TestingRandom,
    pub securechip: TestingSecureChip,
    pub memory: TestingMemory,
    pub eeprom: TestingEeprom,
    pub system: TestingSystem,
}

impl TestingHal<'_> {
    pub fn new() -> Self {
        Self {
            ui: TestingUi::new(),
            sd: TestingSd::new(),
            random: TestingRandom::new(),
            securechip: TestingSecureChip::new(),
            memory: TestingMemory::new(),
            eeprom: TestingEeprom::new(),
            system: TestingSystem::new(),
        }
    }
}

impl Default for TestingHal<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> crate::hal::Hal for TestingHal<'a> {
    type Ui = TestingUi<'a>;
    type Random = TestingRandom;
    type Sd = TestingSd;
    type SecureChip = TestingSecureChip;
    type Memory = TestingMemory;
    type Eeprom = TestingEeprom;
    type System = TestingSystem;

    fn as_mut(
        &mut self,
    ) -> crate::hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::Eeprom,
        Self::System,
    > {
        crate::hal::HalSubsystems {
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
