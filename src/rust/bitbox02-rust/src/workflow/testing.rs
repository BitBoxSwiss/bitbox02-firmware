// SPDX-License-Identifier: Apache-2.0

use super::{
    Workflows, cancel, confirm, menu, sdcard, transaction, trinary_choice, trinary_input_string,
};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Screen {
    Confirm {
        title: String,
        body: String,
        longtouch: bool,
    },
    TotalFee {
        total: String,
        fee: String,
        longtouch: bool,
    },
    Recipient {
        recipient: String,
        amount: String,
    },
    Status {
        title: String,
        success: bool,
    },
    ShowAndConfirmMnemonic {
        mnemonic: String,
    },
    More,
}

type EnterStringCb<'a> = Box<
    dyn FnMut(&trinary_input_string::Params<'_>) -> Result<String, trinary_input_string::Error>
        + 'a,
>;

/// An Workflows implementation for unit tests. Collects all screens and provides helper functions
/// to verify them.
pub struct TestingWorkflows<'a> {
    _abort_nth: Option<usize>,
    pub screens: Vec<Screen>,
    _enter_string: Option<EnterStringCb<'a>>,
}

impl Workflows for TestingWorkflows<'_> {
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort> {
        self.screens.push(Screen::Confirm {
            title: params.title.into(),
            body: params.body.into(),
            longtouch: params.longtouch,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(confirm::UserAbort);
        }
        Ok(())
    }

    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), transaction::UserAbort> {
        self.screens.push(Screen::Recipient {
            recipient: recipient.into(),
            amount: amount.into(),
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(transaction::UserAbort);
        }
        Ok(())
    }

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort> {
        self.screens.push(Screen::TotalFee {
            total: total.into(),
            fee: fee.into(),
            longtouch,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(transaction::UserAbort);
        }
        Ok(())
    }

    async fn status(&mut self, title: &str, status_success: bool) {
        self.screens.push(Screen::Status {
            title: title.into(),
            success: status_success,
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            panic!("canot abort status screen");
        }
    }

    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        _can_cancel: trinary_input_string::CanCancel,
        _preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error> {
        self._enter_string.as_mut().unwrap()(params).map(zeroize::Zeroizing::new)
    }

    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort> {
        Ok(())
    }

    async fn menu(
        &mut self,
        _words: &[&str],
        _title: Option<&str>,
    ) -> Result<u8, menu::CancelError> {
        todo!("not used in unit tests yet");
    }

    async fn trinary_choice(
        &mut self,
        _message: &str,
        _label_left: Option<&str>,
        _label_middle: Option<&str>,
        _label_right: Option<&str>,
    ) -> trinary_choice::TrinaryChoice {
        todo!("not used in unit tests yet");
    }

    async fn show_mnemonic(&mut self, _words: &[&str]) -> Result<(), cancel::Error> {
        todo!("not used in unit tests yet");
    }

    async fn quiz_mnemonic_word(
        &mut self,
        _choices: &[&str],
        _title: &str,
    ) -> Result<u8, cancel::Error> {
        todo!("not used in unit tests yet");
    }

    async fn show_and_confirm_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error> {
        self.screens.push(Screen::ShowAndConfirmMnemonic {
            mnemonic: words.join(" "),
        });
        Ok(())
    }

    async fn get_mnemonic(&mut self) -> Result<zeroize::Zeroizing<String>, cancel::Error>
    where
        Self: Sized,
    {
        let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";

        Ok(zeroize::Zeroizing::new(words.into()))
    }

    fn screen_saver_disable(&mut self) {
        // no-op or record in self.screens if you care
    }

    fn screen_saver_enable(&mut self) {
        // no-op
    }
}

impl<'a> TestingWorkflows<'a> {
    pub fn new() -> Self {
        Self {
            screens: vec![],
            _abort_nth: None,
            _enter_string: None,
        }
    }

    /// Make the `n`-th workflow (0-indexed) fail with a user abort. If that workflow cannot be
    /// aborted, there will be panic.
    pub fn abort_nth(&mut self, n: usize) {
        self._abort_nth = Some(n);
    }

    pub fn contains_confirm(&self, confirm_title: &str, confirm_body: &str) -> bool {
        self.screens.iter().any(|screen| match screen {
            Screen::Confirm { title, body, .. } => title == confirm_title && body == confirm_body,
            _ => false,
        })
    }

    pub fn set_enter_string(&mut self, cb: EnterStringCb<'a>) {
        self._enter_string = Some(cb);
    }

    pub fn remove_enter_string(&mut self) {
        self._enter_string = None;
    }
}
