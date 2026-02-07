// SPDX-License-Identifier: Apache-2.0

pub use super::cancel::Error as CancelError;

/// Returns the index of the word chosen by the user.
pub async fn pick(words: &[&str], title: Option<&str>) -> Result<u8, CancelError> {
    bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words,
        title,
        select_word: true,
        continue_on_last: false,
        cancel: true,
    })
    .await
    .or(Err(CancelError::Cancelled))
}
