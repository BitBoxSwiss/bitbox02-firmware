// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;
use core::cell::UnsafeCell;
mod eeprom;
pub mod io;
mod memory;
mod random;
mod sd;
mod securechip;
mod system;
mod ui;

use bitbox_hal as hal;
use bitbox_lvgl::LvDisplay;

struct BitBox03State {
    ui: ui::BitBox03Ui,
    random: random::BitBox03Random,
    sd: sd::BitBox03Sd,
    securechip: securechip::BitBox03SecureChip,
    memory: memory::BitBox03Memory,
    eeprom: eeprom::BitBox03Eeprom,
    system: system::BitBox03System,
}

impl BitBox03State {
    const fn new() -> Self {
        Self {
            ui: ui::BitBox03Ui::new(),
            random: random::BitBox03Random {},
            sd: sd::BitBox03Sd {},
            securechip: securechip::BitBox03SecureChip {},
            memory: memory::BitBox03Memory {},
            eeprom: eeprom::BitBox03Eeprom::new(),
            system: system::BitBox03System {},
        }
    }
}

struct Singleton<T>(UnsafeCell<T>);

impl<T> Singleton<T> {
    const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    fn get(&self) -> *mut T {
        self.0.get()
    }
}

// The BitBox03 HAL is intentionally shared across all handles.
// This keeps all constructions pointed at the same state.
unsafe impl<T> Sync for Singleton<T> {}

static BITBOX03: Singleton<BitBox03State> = Singleton::new(BitBox03State::new());

fn state() -> &'static mut BitBox03State {
    unsafe { &mut *BITBOX03.get() }
}

#[derive(Copy, Clone, Default)]
pub struct BitBox03;

impl BitBox03 {
    pub const fn new() -> BitBox03 {
        BitBox03
    }

    pub fn init(&mut self, display: LvDisplay) {
        state().ui.init(display);
    }
}

impl hal::Hal for BitBox03 {
    type Ui = ui::BitBox03Ui;

    type Random = random::BitBox03Random;

    type Sd = sd::BitBox03Sd;

    type SecureChip = securechip::BitBox03SecureChip;

    type Memory = memory::BitBox03Memory;

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
        let state = state();
        hal::HalSubsystems {
            ui: &mut state.ui,
            random: &mut state.random,
            sd: &mut state.sd,
            securechip: &mut state.securechip,
            memory: &mut state.memory,
            eeprom: &mut state.eeprom,
            system: &mut state.system,
        }
    }
}
