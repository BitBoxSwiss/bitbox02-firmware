// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

use crate::hal::Ui;
use crate::workflow::{
    cancel, confirm, menu, mnemonic, sdcard, status, transaction, trinary_choice,
    trinary_input_string,
};

pub(crate) struct BitBox02Ui;

impl Ui for BitBox02Ui {
    #[inline(always)]
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort> {
        match bitbox02::ui::confirm(params).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(confirm::UserAbort),
        }
    }

    #[inline(always)]
    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), transaction::UserAbort> {
        transaction::verify_recipient(recipient, amount).await
    }

    #[inline(always)]
    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort> {
        transaction::verify_total_fee(total, fee, longtouch).await
    }

    #[inline(always)]
    async fn status(&mut self, title: &str, status_success: bool) {
        status::status(title, status_success).await
    }

    #[inline(always)]
    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error> {
        trinary_input_string::enter(params, can_cancel, preset).await
    }

    #[inline(always)]
    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort> {
        sdcard::sdcard().await
    }

    #[inline(always)]
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, menu::CancelError> {
        menu::pick(words, title).await
    }

    #[inline(always)]
    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> trinary_choice::TrinaryChoice {
        trinary_choice::choose(message, label_left, label_middle, label_right).await
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error> {
        mnemonic::show_mnemonic(words).await
    }

    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        title: &str,
    ) -> Result<u8, cancel::Error> {
        mnemonic::confirm_word(choices, title).await
    }
}
