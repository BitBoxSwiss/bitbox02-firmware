// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

use bitbox_lvgl::timer::LvTimer;

pub async fn delay_for(duration: Duration) {
    let (responder, result) = util::futures::completion::completion();
    let timer = LvTimer::new(duration.as_millis() as u32, move || responder.resolve(()))
        .expect("failed to create delay timer");
    timer.set_repeat_count(1);
    let _timer = timer;
    result.await;
}
