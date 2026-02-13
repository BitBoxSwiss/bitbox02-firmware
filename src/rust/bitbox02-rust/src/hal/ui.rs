// SPDX-License-Identifier: Apache-2.0

use crate::workflow::{cancel, confirm, mnemonic, sdcard, transaction, trinary_input_string};

use alloc::string::String;

pub struct UserAbort;

#[allow(async_fn_in_trait)]
pub trait Ui {
    /// Returns `Ok(())` if the user accepts, `Err(UserAbort)` if the user rejects.
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), UserAbort>;

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

    /// If `can_cancel` is `Yes`, the workflow can be cancelled.
    /// If it is `No`, the result is always `Ok(())`.
    /// If `preset` is not empty, it must be part of `params.wordlist` and will be pre-entered.
    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error>;

    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort>;

    /// Returns the index of the word chosen by the user.
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, cancel::Error>;

    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> bitbox02::ui::TrinaryChoice;

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
    async fn show_and_confirm_mnemonic(
        &mut self,
        random: &mut impl crate::hal::Random,
        words: &[&str],
    ) -> Result<(), cancel::Error>
    where
        Self: Sized,
    {
        mnemonic::show_and_confirm_mnemonic(self, random, words).await
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
