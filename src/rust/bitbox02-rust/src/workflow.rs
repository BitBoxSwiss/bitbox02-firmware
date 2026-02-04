// SPDX-License-Identifier: Apache-2.0

pub mod cancel;
pub mod confirm;
pub mod menu;
#[cfg_attr(
    all(feature = "c-unit-testing", not(feature = "testing")),
    path = "workflow/mnemonic_c_unit_tests.rs"
)]
pub mod mnemonic;
pub mod orientation_screen;
pub mod pairing;
pub mod password;
pub mod sdcard;
pub mod status;
#[cfg(feature = "testing")]
pub mod testing;
pub mod transaction;
pub mod trinary_choice;
pub mod trinary_input_string;
#[cfg(feature = "app-u2f")]
pub mod u2f_c_api;
pub mod unlock;
pub mod unlock_animation;
pub mod verify_message;

use alloc::string::String;

#[allow(async_fn_in_trait)]
pub trait Workflows {
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort>;

    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), transaction::UserAbort>;

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort>;

    async fn status(&mut self, title: &str, status_success: bool);

    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error>;

    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort>;

    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, menu::CancelError>;

    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> trinary_choice::TrinaryChoice;

    /// Display the BIP39 mnemonic to the user.
    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error>;

    /// Display these BIP39 mnemonic word choices to the user as part of the quiz to confirm the
    /// user backuped up the mnemonic correctly.
    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        title: &str,
    ) -> Result<u8, cancel::Error>;

    /// Display the mnemonic words and have the user confirm them in a multiple-choice quiz.
    ///
    /// The default implementation is implemented in terms of `self.show_mnemonic()`,
    /// `self.quiz_mnemonic_word()`, etc.
    ///
    /// This function is defined in the HAL so unit tests can easily mock it. Real implementations
    /// should leave the default implementation.
    async fn show_and_confirm_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error>
    where
        Self: Sized,
    {
        mnemonic::show_and_confirm_mnemonic(self, words).await
    }

    /// Retrieve a BIP39 mnemonic sentence of 12 or 24 words from the user.
    ///
    /// This function is defined in the HAL so unit tests can easily mock it. Real implementations
    /// should leave the default implementation.
    async fn get_mnemonic(&mut self) -> Result<zeroize::Zeroizing<String>, cancel::Error>
    where
        Self: Sized,
    {
        mnemonic::get(self).await
    }
}

pub struct RealWorkflows;

impl Workflows for RealWorkflows {
    #[inline(always)]
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort> {
        confirm::confirm(params).await
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
