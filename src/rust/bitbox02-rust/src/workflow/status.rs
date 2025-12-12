// SPDX-License-Identifier: Apache-2.0

use bitbox02::delay::delay_for;
use core::time::Duration;

pub async fn status(title: &str, status_success: bool) {
    let mut component = bitbox02::ui::status_create(title, status_success);
    component.screen_stack_push();
    delay_for(Duration::from_millis(2000)).await;
}
