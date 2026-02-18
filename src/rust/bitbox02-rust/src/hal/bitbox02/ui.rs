// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

use crate::hal::Ui;
use crate::hal::ui::{ConfirmParams, EnterStringParams, Font, UserAbort};
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
    ) -> bitbox02::ui::TrinaryChoice {
        bitbox02::ui::trinary_choice(message, label_left, label_middle, label_right).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bitbox02_font() {
        let cases = [
            (Font::Default, bitbox02::ui::Font::Default),
            (Font::Password11X12, bitbox02::ui::Font::Password11X12),
            (Font::Monogram5X9, bitbox02::ui::Font::Monogram5X9),
        ];
        for (input, expected) in cases {
            assert_eq!(to_bitbox02_font(input) as i32, expected as i32);
        }
    }

    #[test]
    fn test_to_bitbox02_confirm_params() {
        let fonts = [
            (Font::Default, bitbox02::ui::Font::Default),
            (Font::Password11X12, bitbox02::ui::Font::Password11X12),
            (Font::Monogram5X9, bitbox02::ui::Font::Monogram5X9),
        ];
        for (font, expected_font) in fonts {
            let input = ConfirmParams {
                title: "title",
                title_autowrap: true,
                body: "body",
                font,
                scrollable: true,
                longtouch: true,
                accept_only: true,
                accept_is_nextarrow: true,
                display_size: 42,
            };
            let output = to_bitbox02_confirm_params(&input);
            assert_eq!(output.title, "title");
            assert!(output.title_autowrap);
            assert_eq!(output.body, "body");
            assert_eq!(output.font as i32, expected_font as i32);
            assert!(output.scrollable);
            assert!(output.longtouch);
            assert!(output.accept_only);
            assert!(output.accept_is_nextarrow);
            assert_eq!(output.display_size, 42);
        }
    }

    #[test]
    fn test_to_bitbox02_trinary_input_string_params() {
        let input_without_wordlist = EnterStringParams {
            title: "Enter",
            wordlist: None,
            number_input: true,
            hide: true,
            special_chars: true,
            longtouch: true,
            cancel_is_backbutton: true,
            default_to_digits: true,
        };
        let output_without_wordlist =
            to_bitbox02_trinary_input_string_params(&input_without_wordlist);
        assert_eq!(output_without_wordlist.title, "Enter");
        assert!(output_without_wordlist.wordlist.is_none());
        assert!(output_without_wordlist.number_input);
        assert!(output_without_wordlist.hide);
        assert!(output_without_wordlist.special_chars);
        assert!(output_without_wordlist.longtouch);
        assert!(output_without_wordlist.cancel_is_backbutton);
        assert!(output_without_wordlist.default_to_digits);

        let wordlist = [1u16, 2, 3];
        let input_with_wordlist = EnterStringParams {
            title: "Seed",
            wordlist: Some(&wordlist),
            number_input: false,
            hide: false,
            special_chars: false,
            longtouch: false,
            cancel_is_backbutton: false,
            default_to_digits: false,
        };
        let output_with_wordlist = to_bitbox02_trinary_input_string_params(&input_with_wordlist);
        assert_eq!(output_with_wordlist.title, "Seed");
        assert_eq!(output_with_wordlist.wordlist.unwrap(), wordlist.as_slice());
        assert!(!output_with_wordlist.number_input);
        assert!(!output_with_wordlist.hide);
        assert!(!output_with_wordlist.special_chars);
        assert!(!output_with_wordlist.longtouch);
        assert!(!output_with_wordlist.cancel_is_backbutton);
        assert!(!output_with_wordlist.default_to_digits);
    }
}
