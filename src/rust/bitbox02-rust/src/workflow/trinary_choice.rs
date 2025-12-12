// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;
use core::cell::RefCell;

use alloc::boxed::Box;

pub use bitbox02::ui::TrinaryChoice;
use bitbox02::ui::trinary_choice_create;

pub async fn choose(
    message: &str,
    label_left: Option<&str>,
    label_middle: Option<&str>,
    label_right: Option<&str>,
) -> TrinaryChoice {
    let result = RefCell::new(None as Option<TrinaryChoice>);

    let mut component = trinary_choice_create(
        message,
        label_left,
        label_middle,
        label_right,
        Box::new(|choice| {
            *result.borrow_mut() = Some(choice);
        }),
    );
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
