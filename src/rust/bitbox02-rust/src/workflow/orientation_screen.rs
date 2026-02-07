// SPDX-License-Identifier: Apache-2.0

use bitbox02::delay::delay_for;
use bitbox02::ui::choose_orientation;
use core::time::Duration;

pub async fn orientation_screen() -> bool {
    let upside_down = choose_orientation().await;
    if upside_down {
        bitbox02::screen_rotate()
    }

    // During this delay the bb02 logotype is shown
    delay_for(Duration::from_millis(1300)).await;

    // Switch to lockscreen that shows "See the bitbox app" and device name
    bitbox02::ui::screen_process_waiting_switch_to_lockscreen();

    upside_down
}
