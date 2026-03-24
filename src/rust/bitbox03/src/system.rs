use bitbox_hal as hal;

pub struct BitBox03System;

impl hal::system::System for BitBox03System {
    async fn startup() {
        todo!()
    }

    fn reboot(&mut self) -> ! {
        todo!()
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        todo!()
    }

    fn reset_ble(&mut self) {
        todo!()
    }

    fn smarteeprom_disable(&mut self) {
        todo!()
    }

    fn communication_timeout_reset(&mut self, _value: i16) {
        todo!()
    }
}
