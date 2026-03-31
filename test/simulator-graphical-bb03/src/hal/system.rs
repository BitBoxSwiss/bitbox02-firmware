// SPDX-License-Identifier: Apache-2.0

use bitbox_hal as hal;

pub struct BitBox03System;

impl hal::system::System for BitBox03System {
    async fn startup() {}

    fn is_btconly(&mut self) -> bool {
        false
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

    fn communication_timeout_reset(&mut self, _value: i16) {
        todo!()
    }
}
