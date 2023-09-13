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
