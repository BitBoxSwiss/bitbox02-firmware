// SPDX-License-Identifier: Apache-2.0

pub use bitbox02::ui::TrinaryChoice;
use bitbox02::ui::trinary_choice;

pub async fn choose(
    message: &str,
    label_left: Option<&str>,
    label_middle: Option<&str>,
    label_right: Option<&str>,
) -> TrinaryChoice {
    trinary_choice(message, label_left, label_middle, label_right).await
}
