// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use bitbox_hal as hal;
use bitbox_lvgl::LvDisplay;
use bitbox_platform_host::{
    eeprom::FakeEeprom, memory::FakeMemory, sd::FakeSd, securechip::FakeSecureChip,
};
use bitbox03::ui;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use std::sync::Once;

mod random;
mod system;

struct BitBox03State {
    ui: ui::BitBox03Ui,
    random: random::BitBox03Random,
    sd: FakeSd,
    securechip: FakeSecureChip,
    memory: FakeMemory,
    eeprom: FakeEeprom,
    system: system::BitBox03System,
}

impl BitBox03State {
    fn new() -> Self {
        let mut sd = FakeSd::new();
        sd.inserted = Some(true);

        let mut memory = FakeMemory::new();
        memory.set_platform(bitbox_hal::memory::Platform::BitBox03);

        Self {
            ui: ui::BitBox03Ui::new(),
            random: random::BitBox03Random {},
            sd,
            securechip: FakeSecureChip::new(),
            memory,
            eeprom: FakeEeprom::new(),
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

unsafe impl<T> Sync for Singleton<T> {}

static BITBOX03_INIT: Once = Once::new();
static BITBOX03: Singleton<MaybeUninit<BitBox03State>> = Singleton::new(MaybeUninit::uninit());

fn state() -> &'static mut BitBox03State {
    BITBOX03_INIT.call_once(|| unsafe {
        (*BITBOX03.get()).write(BitBox03State::new());
    });
    unsafe { (&mut *BITBOX03.get()).assume_init_mut() }
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
    type Sd = FakeSd;
    type SecureChip = FakeSecureChip;
    type Memory = FakeMemory;
    type Eeprom = FakeEeprom;
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
