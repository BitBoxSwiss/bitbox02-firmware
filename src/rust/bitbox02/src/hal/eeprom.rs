// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::Eeprom;

pub struct BitBox02Eeprom;

impl Eeprom for BitBox02Eeprom {
    fn setup(&mut self) {
        unsafe { bitbox02_sys::smarteeprom_bb02_config() };
    }

    fn init(&mut self) {
        unsafe { bitbox02_sys::bitbox02_smarteeprom_init() };
    }

    fn is_enabled(&mut self) -> bool {
        unsafe { bitbox02_sys::smarteeprom_is_enabled() }
    }

    fn disable(&mut self) {
        unsafe { bitbox02_sys::smarteeprom_disable() };
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        unsafe { bitbox02_sys::bitbox02_smarteeprom_get_unlock_attempts() }
    }

    fn increment_unlock_attempts(&mut self) {
        unsafe {
            bitbox02_sys::bitbox02_smarteeprom_increment_unlock_attempts();
        }
    }

    fn reset_unlock_attempts(&mut self) {
        unsafe {
            bitbox02_sys::bitbox02_smarteeprom_reset_unlock_attempts();
        }
    }
}
