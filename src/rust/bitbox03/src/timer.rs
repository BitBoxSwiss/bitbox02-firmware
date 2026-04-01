// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

pub struct BitBox03Timer;

impl bitbox_hal::timer::Timer for BitBox03Timer {
    async fn delay_for(duration: Duration) {
        let _ = duration;
        todo!()
    }
}
