// SPDX-License-Identifier: Apache-2.0

pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

use bitbox_hal::Hal;
use core::sync::atomic::{AtomicBool, Ordering};

static BITBOX02_HAL_TAKEN: AtomicBool = AtomicBool::new(false);

pub struct BitBox02Hal {
    ui: ui::BitBox02Ui,
    sd: sd::BitBox02Sd,
    random: random::BitBox02Random,
    securechip: securechip::BitBox02SecureChip,
    memory: memory::BitBox02Memory,
    system: system::BitBox02System,
}

impl BitBox02Hal {
    const fn new() -> Self {
        Self {
            ui: ui::BitBox02Ui,
            sd: sd::BitBox02Sd,
            random: random::BitBox02Random,
            securechip: securechip::BitBox02SecureChip,
            memory: memory::BitBox02Memory,
            system: system::BitBox02System,
        }
    }

    /// Returns the single HAL instance.
    pub fn take() -> Option<Self> {
        BITBOX02_HAL_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .ok()
            .map(|_| Self::new())
    }

    #[cfg(any(
        test,
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ))]
    pub fn reset_for_testing() {
        BITBOX02_HAL_TAKEN.store(false, Ordering::Release);
    }
}

impl Hal for BitBox02Hal {
    type Ui = ui::BitBox02Ui;
    type Random = random::BitBox02Random;
    type Sd = sd::BitBox02Sd;
    type SecureChip = securechip::BitBox02SecureChip;
    type Memory = memory::BitBox02Memory;
    type System = system::BitBox02System;

    fn as_mut(
        &mut self,
    ) -> bitbox_hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::System,
    > {
        bitbox_hal::HalSubsystems {
            ui: &mut self.ui,
            random: &mut self.random,
            sd: &mut self.sd,
            securechip: &mut self.securechip,
            memory: &mut self.memory,
            system: &mut self.system,
        }
    }
}
