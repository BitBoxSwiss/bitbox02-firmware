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

use crate::bb02_async::screensaver_without;
use bitbox02::ui::{MenuParams, menu_create};

use alloc::boxed::Box;
use core::cell::RefCell;

/// Returns the index of the word chosen by the user.
pub async fn pick(words: &[&str], title: Option<&str>) -> Result<u8, CancelError> {
    screensaver_without(menu_create(MenuParams {
        words,
        title,
        select_word: true,
        continue_on_last: false,
        cancel: true,
    }))
    .await
    .or(Err(CancelError::Cancelled))
}
