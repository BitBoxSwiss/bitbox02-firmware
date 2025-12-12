// SPDX-License-Identifier: Apache-2.0

pub use super::cancel::Error as CancelError;

use crate::bb02_async::option_no_screensaver;

use alloc::boxed::Box;
use core::cell::RefCell;

/// Returns the index of the word chosen by the user.
pub async fn pick(words: &[&str], title: Option<&str>) -> Result<u8, CancelError> {
    let result = RefCell::new(None as Option<Result<u8, CancelError>>);
    let mut component = bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words,
        title,
        select_word_cb: Some(Box::new(|choice_idx| {
            *result.borrow_mut() = Some(Ok(choice_idx));
        })),
        continue_on_last_cb: None,
        cancel_cb: Some(Box::new(|| {
            *result.borrow_mut() = Some(Err(CancelError::Cancelled));
        })),
    });
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
