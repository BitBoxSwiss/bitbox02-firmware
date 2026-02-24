// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "simulator-graphical")]
pub fn bb02_config() {
    unsafe { bitbox02_sys::smarteeprom_bb02_config() };
}

#[cfg(feature = "simulator-graphical")]
pub fn init() {
    unsafe { bitbox02_sys::bitbox02_smarteeprom_init() };
}
