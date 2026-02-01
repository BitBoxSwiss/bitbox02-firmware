// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;

pub use bitbox02::ui::{ConfirmParams as Params, Font};

pub struct UserAbort;

use crate::hal::Ui;

/// Returns true if the user accepts, false if the user rejects.
pub async fn confirm(ui: &mut impl Ui, params: &Params<'_>) -> Result<(), UserAbort> {
    let result = core::cell::RefCell::new(None as Option<Result<(), UserAbort>>);

    // The component will set the result when the user accepted/rejected.
    let mut component = bitbox02::ui::confirm_create(params, |accepted| {
        *result.borrow_mut() = if accepted {
            Some(Ok(()))
        } else {
            Some(Err(UserAbort))
        };
    });
    component.screen_stack_push();
    option_no_screensaver(ui, &result).await
}
