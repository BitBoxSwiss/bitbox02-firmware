// Copyright 2020 Shift Cryptosecurity AG
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

pub use bitbox02::ui::{ConfirmParams as Params, Font};

pub struct UserAbort;

/// Returns true if the user accepts, false if the user rejects.
pub async fn confirm(params: &Params<'_>) -> Result<(), UserAbort> {
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
    option_no_screensaver(&result).await
}
