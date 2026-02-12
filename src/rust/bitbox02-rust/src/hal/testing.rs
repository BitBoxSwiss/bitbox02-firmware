// SPDX-License-Identifier: Apache-2.0

pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

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
            system: TestingSystem::new(),
        }
    }
}

impl crate::hal::Hal for TestingHal<'_> {
    fn ui(&mut self) -> &mut impl crate::hal::Ui {
        &mut self.ui
    }

    fn sd(&mut self) -> &mut impl crate::hal::Sd {
        &mut self.sd
    }

    fn random(&mut self) -> &mut impl crate::hal::Random {
        &mut self.random
    }

    fn securechip(&mut self) -> &mut impl crate::hal::SecureChip {
        &mut self.securechip
    }

    fn memory(&mut self) -> &mut impl crate::hal::Memory {
        &mut self.memory
    }

    fn system(&mut self) -> &mut impl crate::hal::System {
        &mut self.system
    }
}
