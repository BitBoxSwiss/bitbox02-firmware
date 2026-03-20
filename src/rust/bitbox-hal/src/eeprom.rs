// SPDX-License-Identifier: Apache-2.0

pub trait Eeprom {
    fn setup(&mut self);
    fn init(&mut self);
    fn is_enabled(&mut self) -> bool;
    fn disable(&mut self);
    fn get_unlock_attempts(&mut self) -> u8;
    fn increment_unlock_attempts(&mut self);
    fn reset_unlock_attempts(&mut self);
}
