// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::System;
use core::time::Duration;

pub struct BitBox02System;

impl System for BitBox02System {
    async fn startup() {
        let upside_down = crate::ui::choose_orientation().await;
        if upside_down {
            crate::screen_rotate();
        }

        // During this delay the bb02 logotype is shown.
        crate::delay::delay_for(Duration::from_millis(1300)).await;

        // Switch to lockscreen that shows "See the bitbox app" and device name.
        crate::ui::screen_process_waiting_switch_to_lockscreen();
    }

    #[allow(clippy::empty_loop)]
    fn reboot(&mut self) -> ! {
        unsafe { bitbox02_sys::reboot() }
        loop {}
    }

    fn reboot_to_bootloader(&mut self) -> ! {
        crate::reboot_to_bootloader()
    }

    fn reset_ble(&mut self) {
        crate::reset_ble()
    }

    fn smarteeprom_disable(&mut self) {
        unsafe { bitbox02_sys::smarteeprom_disable() }
    }
}
