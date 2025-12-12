// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "simulator-graphical")]
pub fn bb02_config() {
    unsafe { bitbox02_sys::smarteeprom_bb02_config() };
}

#[cfg(feature = "simulator-graphical")]
pub fn init() {
    unsafe { bitbox02_sys::bitbox02_smarteeprom_init() };
}

pub fn disable() {
    unsafe { bitbox02_sys::smarteeprom_disable() };
}

#[cfg(feature = "testing")]
pub fn is_enabled() -> bool {
    unsafe { bitbox02_sys::smarteeprom_is_enabled() }
}
