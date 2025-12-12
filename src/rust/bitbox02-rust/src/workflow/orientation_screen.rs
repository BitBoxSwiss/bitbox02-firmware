// SPDX-License-Identifier: Apache-2.0

use bitbox02::delay::delay_for;
use core::time::Duration;
use util::bb02_async::option;

pub async fn choose_orientation() -> bool {
    let result = core::cell::RefCell::new(None as Option<bool>);
    let mut orientation_arrows = bitbox02::ui::orientation_arrows(|upside_down| {
        *result.borrow_mut() = Some(upside_down);
    });
    orientation_arrows.screen_stack_push();
    // Wait until orientation has been chosen
    option(&result).await
}

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
