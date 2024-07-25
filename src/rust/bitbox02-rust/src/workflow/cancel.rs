// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::bb02_async::option_no_screensaver;
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
    title: &str,
    component: &mut bitbox02::ui::Component<'_>,
    result_cell: &ResultCell<R>,
) -> Result<R, Error> {
    component.screen_stack_push();
    loop {
        let result = option_no_screensaver(result_cell).await;
        if let Err(Error::Cancelled) = result {
            let params = confirm::Params {
                title,
                body: "Do you really\nwant to cancel?",
                ..Default::default()
            };

            if let Err(confirm::UserAbort) = confirm::confirm(&params).await {
                continue;
            }
        }
        return result;
    }
}
