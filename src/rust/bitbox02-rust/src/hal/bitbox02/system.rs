// SPDX-License-Identifier: Apache-2.0

use crate::hal::System;

pub(crate) struct BitBox02System;

impl System for BitBox02System {
    fn reboot_to_bootloader(&mut self) -> ! {
        bitbox02::reboot_to_bootloader()
    }
}
