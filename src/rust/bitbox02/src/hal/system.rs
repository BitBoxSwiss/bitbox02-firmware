// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::System;

pub struct BitBox02System;

impl System for BitBox02System {
    fn reboot_to_bootloader(&mut self) -> ! {
        crate::reboot_to_bootloader()
    }
}
