// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

use crate::hal::Ui;
use crate::hal::ui::{ConfirmParams, EnterStringParams, Font, TrinaryChoice, UserAbort};
use crate::workflow::trinary_input_string;

pub struct BitBox02Ui;

fn to_bitbox02_font(font: Font) -> bitbox02::ui::Font {
    match font {
        Font::Default => bitbox02::ui::Font::Default,
        Font::Password11X12 => bitbox02::ui::Font::Password11X12,
        Font::Monogram5X9 => bitbox02::ui::Font::Monogram5X9,
    }
}

fn to_bitbox02_confirm_params<'a>(
    params: &'a ConfirmParams<'a>,
) -> bitbox02::ui::ConfirmParams<'a> {
    bitbox02::ui::ConfirmParams {
        title: params.title,
        title_autowrap: params.title_autowrap,
        body: params.body,
        font: to_bitbox02_font(params.font),
        scrollable: params.scrollable,
        longtouch: params.longtouch,
        accept_only: params.accept_only,
        accept_is_nextarrow: params.accept_is_nextarrow,
        display_size: params.display_size,
    }
}

fn to_bitbox02_trinary_input_string_params<'a>(
    params: &'a EnterStringParams<'a>,
) -> bitbox02::ui::TrinaryInputStringParams<'a> {
    bitbox02::ui::TrinaryInputStringParams {
        title: params.title,
        wordlist: params.wordlist,
        number_input: params.number_input,
        hide: params.hide,
        special_chars: params.special_chars,
        longtouch: params.longtouch,
        cancel_is_backbutton: params.cancel_is_backbutton,
        default_to_digits: params.default_to_digits,
    }
}

fn to_hal_trinary_choice(choice: bitbox02::ui::TrinaryChoice) -> TrinaryChoice {
    match choice {
        bitbox02::ui::TrinaryChoice::TRINARY_CHOICE_LEFT => TrinaryChoice::Left,
        bitbox02::ui::TrinaryChoice::TRINARY_CHOICE_MIDDLE => TrinaryChoice::Middle,
        bitbox02::ui::TrinaryChoice::TRINARY_CHOICE_RIGHT => TrinaryChoice::Right,
    }
}

impl Ui for BitBox02Ui {
    #[inline(always)]
    async fn confirm(&mut self, params: &ConfirmParams<'_>) -> Result<(), UserAbort> {
        let params = to_bitbox02_confirm_params(params);
        match bitbox02::ui::confirm(&params).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(UserAbort),
        }
    }

    #[inline(always)]
    async fn verify_recipient(&mut self, recipient: &str, amount: &str) -> Result<(), UserAbort> {
        match bitbox02::ui::confirm_transaction_address(amount, recipient).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(UserAbort),
        }
    }

    #[inline(always)]
    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), UserAbort> {
        match bitbox02::ui::confirm_transaction_fee(total, fee, longtouch).await {
            bitbox02::ui::ConfirmResponse::Approved => Ok(()),
            bitbox02::ui::ConfirmResponse::Cancelled => Err(UserAbort),
        }
    }

    #[inline(always)]
    async fn status(&mut self, title: &str, status_success: bool) {
        bitbox02::ui::status(title, status_success).await
    }

    #[inline(always)]
    async fn enter_string(
        &mut self,
        params: &EnterStringParams<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, UserAbort> {
        let params = to_bitbox02_trinary_input_string_params(params);
        let can_cancel = match can_cancel {
            trinary_input_string::CanCancel::Yes => true,
            trinary_input_string::CanCancel::No => false,
        };
        bitbox02::ui::trinary_input_string(&params, can_cancel, preset)
            .await
            .map_err(|_| UserAbort)
    }

    #[inline(always)]
    async fn insert_sdcard(&mut self) -> Result<(), UserAbort> {
        match bitbox02::ui::sdcard().await {
            bitbox02::ui::SdcardResponse::Inserted => Ok(()),
            bitbox02::ui::SdcardResponse::Cancelled => Err(UserAbort),
        }
    }

    #[inline(always)]
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, UserAbort> {
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
            bitbox02::ui::MenuResponse::Cancel => Err(UserAbort),
        }
    }

    #[inline(always)]
    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> TrinaryChoice {
        to_hal_trinary_choice(
            bitbox02::ui::trinary_choice(message, label_left, label_middle, label_right).await,
        )
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), UserAbort> {
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
            bitbox02::ui::MenuResponse::Cancel => Err(UserAbort),
        }
    }

    async fn quiz_mnemonic_word(&mut self, choices: &[&str], title: &str) -> Result<u8, UserAbort> {
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
            bitbox02::ui::MenuResponse::Cancel => Err(UserAbort),
        }
    }
}
