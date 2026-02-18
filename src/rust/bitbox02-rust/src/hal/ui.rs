// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

pub struct UserAbort;

#[derive(Copy, Clone, Default)]
pub enum Font {
    #[default]
    Default,
    Password11X12,
    Monogram5X9,
}

#[derive(Default)]
pub struct ConfirmParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    pub title_autowrap: bool,
    /// The confirmation body of the screen. Max 200 chars, otherwise **panic**.
    pub body: &'a str,
    pub font: Font,
    /// If true, the body is horizontally scrollable.
    pub scrollable: bool,
    /// If true, require the hold gesture to confirm instead of tap.
    pub longtouch: bool,
    /// If true, the user can only confirm, not reject.
    pub accept_only: bool,
    /// if true, the accept icon is a right arrow instead of a checkmark (indicating going to the
    /// "next" screen).
    pub accept_is_nextarrow: bool,
    /// Print the value of this variable in the corner. Will not print when 0
    pub display_size: usize,
}

#[derive(Default)]
pub struct EnterStringParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    /// Currently specialized to the BIP39 wordlist: a list of BIP39 word indices. Can be extended if needed.
    pub wordlist: Option<&'a [u16]>,
    pub number_input: bool,
    pub hide: bool,
    pub special_chars: bool,
    pub longtouch: bool,
    pub cancel_is_backbutton: bool,
    pub default_to_digits: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TrinaryChoice {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone)]
pub enum CanCancel {
    No,
    Yes,
}

#[allow(async_fn_in_trait)]
pub trait Ui {
    /// Returns `Ok(())` if the user accepts, `Err(UserAbort)` if the user rejects.
    async fn confirm(&mut self, params: &ConfirmParams<'_>) -> Result<(), UserAbort>;

    async fn verify_recipient(&mut self, recipient: &str, amount: &str) -> Result<(), UserAbort>;

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), UserAbort>;

    async fn status(&mut self, title: &str, status_success: bool);

    /// If `can_cancel` is `Yes`, the workflow can be cancelled.
    /// If it is `No`, the result is always `Ok(())`.
    /// If `preset` is not empty, it must be part of `params.wordlist` and will be pre-entered.
    async fn enter_string(
        &mut self,
        params: &EnterStringParams<'_>,
        can_cancel: CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, UserAbort>;

    async fn insert_sdcard(&mut self) -> Result<(), UserAbort>;

    /// Returns the index of the word chosen by the user.
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, UserAbort>;

    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> TrinaryChoice;

    /// Display the BIP39 mnemonic to the user.
    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), UserAbort>;

    /// Display these BIP39 mnemonic word choices to the user as part of the quiz to confirm the
    /// user backuped up the mnemonic correctly.
    async fn quiz_mnemonic_word(&mut self, choices: &[&str], title: &str) -> Result<u8, UserAbort>;
}
