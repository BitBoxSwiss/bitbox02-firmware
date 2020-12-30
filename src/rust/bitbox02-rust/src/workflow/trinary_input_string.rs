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

pub use super::cancel::Error;
pub use bitbox02::ui::TrinaryInputStringParams as Params;

use crate::bb02_async::option;
use bitbox02::input::SafeInputString;
use core::cell::RefCell;

extern crate alloc;
use alloc::boxed::Box;

pub enum CanCancel {
    No,
    Yes,
}

/// If `can_cancel` is `Yes`, the workflow can be cancelled.
/// If it is no, the result is always `Ok(())`.
/// If `preset` is not empty, it must be part of `params.wordlist` and will be pre-entered.
/// ```
pub async fn enter(
    params: &Params<'_>,
    can_cancel: CanCancel,
    preset: &str,
) -> Result<SafeInputString, Error> {
    let result = RefCell::new(None as Option<Result<SafeInputString, ()>>); // Err means cancelled.
    let mut component = bitbox02::ui::trinary_input_string_create(
        &params,
        |string| *result.borrow_mut() = Some(Ok(string)),
        match can_cancel {
            CanCancel::Yes => Some(Box::new(|| *result.borrow_mut() = Some(Err(())))),
            CanCancel::No => None,
        },
    );
    if !preset.is_empty() {
        bitbox02::ui::trinary_input_string_set_input(&mut component, preset);
    }
    component.screen_stack_push();
    option(&result)
        .await
        .or(Err(super::cancel::Error::Cancelled))
}
