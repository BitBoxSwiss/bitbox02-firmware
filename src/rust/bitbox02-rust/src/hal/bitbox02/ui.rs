// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

use crate::hal::Ui;
use crate::workflow::{cancel, confirm, sdcard, transaction, trinary_input_string};

pub struct BitBox02Ui;

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
        match bitbox02::ui::confirm_transaction_address(amount, recipient).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(transaction::UserAbort),
        }
    }

    #[inline(always)]
    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort> {
        match bitbox02::ui::confirm_transaction_fee(total, fee, longtouch).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(transaction::UserAbort),
        }
    }

    #[inline(always)]
    async fn status(&mut self, title: &str, status_success: bool) {
        bitbox02::ui::status(title, status_success).await
    }

    #[inline(always)]
    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error> {
        let can_cancel = match can_cancel {
            trinary_input_string::CanCancel::Yes => true,
            trinary_input_string::CanCancel::No => false,
        };
        bitbox02::ui::trinary_input_string(params, can_cancel, preset)
            .await
            .or(Err(trinary_input_string::Error::Cancelled))
    }

    #[inline(always)]
    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort> {
        match bitbox02::ui::sdcard().await {
            bitbox02::ui::SdcardResponse::Inserted => Ok(()),
            bitbox02::ui::SdcardResponse::Cancelled => Err(sdcard::UserAbort),
        }
    }

    #[inline(always)]
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, cancel::Error> {
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
            bitbox02::ui::MenuResponse::Cancel => Err(cancel::Error::Cancelled),
        }
    }

    #[inline(always)]
    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> bitbox02::ui::TrinaryChoice {
        bitbox02::ui::trinary_choice(message, label_left, label_middle, label_right).await
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error> {
        match bitbox02::ui::menu(bitbox02::ui::MenuParams {
            words,
            title: None,
            select_word: false,
            continue_on_last: true,
            cancel_confirm_title: Some("Recovery\nwords"),
        })
        .await
        {
            bitbox02::ui::MenuResponse::ContinueOnLast => Ok(()),
            bitbox02::ui::MenuResponse::SelectWord(_) => panic!("unexpected select-word"),
            bitbox02::ui::MenuResponse::Cancel => Err(cancel::Error::Cancelled),
        }
    }

    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        title: &str,
    ) -> Result<u8, cancel::Error> {
        match bitbox02::ui::menu(bitbox02::ui::MenuParams {
            words: choices,
            title: Some(title),
            select_word: true,
            continue_on_last: false,
            cancel_confirm_title: Some("Recovery\nwords"),
        })
        .await
        {
            bitbox02::ui::MenuResponse::SelectWord(choice_idx) => Ok(choice_idx),
            bitbox02::ui::MenuResponse::ContinueOnLast => panic!("unexpected continue-on-last"),
            bitbox02::ui::MenuResponse::Cancel => Err(cancel::Error::Cancelled),
        }
    }
}
