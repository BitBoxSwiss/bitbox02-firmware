// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;
use crate::hal::Ui;
use core::cell::RefCell;

use super::confirm;

#[derive(Debug)]
pub enum Error {
    Cancelled,
}

pub type ResultCell<R> = RefCell<Option<Result<R, Error>>>;

/// Resolves the `with_cancel` future as cancelled.
pub fn cancel<R>(result_cell: &ResultCell<R>) {
    *result_cell.borrow_mut() = Some(Err(Error::Cancelled));
}

/// Resolves the `with_cancel` future with the given result.
pub fn set_result<R>(result_cell: &ResultCell<R>, result: R) {
    *result_cell.borrow_mut() = Some(Ok(result));
}

/// Blocks on showing/running a component until `cancel` or `result` is
/// called on the same `result_cell`.
/// In the former, a prompt with the given title to confirm cancellation is shown.
///
/// * `title` - title to show in the cancel confirm prompt.
/// * `component` - component to process
/// * `result_cell` - result var to synchronize the result on. Pass the same to `cancel` and
///   `set_result`.
pub async fn with_cancel<R>(
    hal_ui: &mut impl Ui,
    title: &str,
    component: &mut bitbox02::ui::Component<'_>,
    result_cell: &ResultCell<R>,
) -> Result<R, Error> {
    component.screen_stack_push();
    loop {
        let result = option_no_screensaver(hal_ui, result_cell).await;
        if let Err(Error::Cancelled) = result {
            let params = confirm::Params {
                title,
                body: "Do you really\nwant to cancel?",
                ..Default::default()
            };

            if let Err(confirm::UserAbort) = confirm::confirm(hal_ui, &params).await {
                continue;
            }
        }
        return result;
    }
}
