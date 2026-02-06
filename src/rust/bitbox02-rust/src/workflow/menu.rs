// SPDX-License-Identifier: Apache-2.0

pub use super::cancel::Error as CancelError;

/// Returns the index of the word chosen by the user.
pub async fn pick(words: &[&str], title: Option<&str>) -> Result<u8, CancelError> {
    match bitbox02::ui::menu(bitbox02::ui::MenuParams {
        words,
        title,
        select_word: true,
        continue_on_last: false,
        cancel_confirm_title: None,
    })
    .await
    {
        bitbox02::ui::MenuResponse::SelectWord(choice_idx) => Ok(choice_idx),
        bitbox02::ui::MenuResponse::ContinueOnLast => panic!("unexpected continue-on-last"),
        bitbox02::ui::MenuResponse::Cancel => Err(CancelError::Cancelled),
    }
}
