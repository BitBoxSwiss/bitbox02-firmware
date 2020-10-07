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
use super::cancel::{cancel, set_result, with_cancel};

extern crate alloc;
use alloc::boxed::Box;
use core::cell::RefCell;

/// Displays all mnemonic words in a scroll-through screen.
pub async fn show_mnemonic(words: &[&str]) -> Result<(), CancelError> {
    let result = RefCell::new(None);
    let mut component = bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words,
        title: None,
        select_word_cb: None,
        continue_on_last_cb: Some(Box::new(|| {
            set_result(&result, ());
        })),
        cancel_cb: Some(Box::new(|| {
            cancel(&result);
        })),
    });
    with_cancel("Recovery\nwords", &mut component, &result).await
}

/// Displays the `choices` to the user, returning the index of the selected choice.
pub async fn confirm_word(choices: &[&str], title: &str) -> Result<u8, CancelError> {
    let result = RefCell::new(None);
    let mut component = bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words: choices,
        title: Some(title),
        select_word_cb: Some(Box::new(|idx| {
            set_result(&result, idx);
        })),
        continue_on_last_cb: None,
        cancel_cb: Some(Box::new(|| {
            cancel(&result);
        })),
    });
    with_cancel("Recovery\nwords", &mut component, &result).await
}
