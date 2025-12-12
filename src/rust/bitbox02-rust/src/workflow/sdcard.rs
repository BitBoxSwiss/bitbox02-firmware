// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;
use core::cell::RefCell;

pub struct UserAbort;

pub async fn sdcard() -> Result<(), UserAbort> {
    let result = RefCell::new(None as Option<Result<(), UserAbort>>);
    let mut component = bitbox02::ui::sdcard_create(|sd_done| {
        *result.borrow_mut() = if sd_done {
            Some(Ok(()))
        } else {
            Some(Err(UserAbort))
        };
    });
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
