// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;
use core::cell::RefCell;

pub async fn status(title: &str, status_success: bool) {
    let result = RefCell::new(None);
    let mut component = bitbox02::ui::status_create(title, status_success, || {
        *result.borrow_mut() = Some(());
    });
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
