// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use crate::hal::ui::{CanCancel, ConfirmParams, EnterStringParams, TrinaryChoice, UserAbort};

use alloc::boxed::Box;
use alloc::collections::VecDeque;
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
    ShowMnemonic {
        words: Vec<String>,
    },
    QuizMnemonicWord {
        title: String,
        choices: Vec<String>,
        selected: u8,
    },
    More,
}

type EnterStringCb<'a> = Box<dyn FnMut(&EnterStringParams<'_>) -> Result<String, UserAbort> + 'a>;

/// A Ui implementation for unit tests. Collects all screens and provides helper functions
/// to verify them.
pub struct TestingUi<'a> {
    _abort_nth: Option<usize>,
    pub screens: Vec<Screen>,
    _enter_string: Option<EnterStringCb<'a>>,
    _quiz_choices: VecDeque<u8>,
}

impl Ui for TestingUi<'_> {
    async fn confirm(&mut self, params: &ConfirmParams<'_>) -> Result<(), UserAbort> {
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
            return Err(UserAbort);
        }
        Ok(())
    }

    async fn verify_recipient(&mut self, recipient: &str, amount: &str) -> Result<(), UserAbort> {
        self.screens.push(Screen::Recipient {
            recipient: recipient.into(),
            amount: amount.into(),
        });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(UserAbort);
        }
        Ok(())
    }

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), UserAbort> {
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
            return Err(UserAbort);
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
        params: &EnterStringParams<'_>,
        _can_cancel: CanCancel,
        _preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, UserAbort> {
        self._enter_string.as_mut().unwrap()(params).map(zeroize::Zeroizing::new)
    }

    async fn insert_sdcard(&mut self) -> Result<(), UserAbort> {
        Ok(())
    }

    async fn menu(&mut self, _words: &[&str], _title: Option<&str>) -> Result<u8, UserAbort> {
        todo!("not used in unit tests yet");
    }

    async fn trinary_choice(
        &mut self,
        _message: &str,
        _label_left: Option<&str>,
        _label_middle: Option<&str>,
        _label_right: Option<&str>,
    ) -> TrinaryChoice {
        todo!("not used in unit tests yet");
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), UserAbort> {
        let words: Vec<String> = words.iter().map(|word| (*word).into()).collect();
        self.screens.push(Screen::ShowMnemonic { words });
        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(UserAbort);
        }
        Ok(())
    }

    async fn quiz_mnemonic_word(&mut self, choices: &[&str], title: &str) -> Result<u8, UserAbort> {
        let selected = self._quiz_choices.pop_front().unwrap_or_else(|| {
            panic!("quiz_mnemonic_word called without queued choice; use push_quiz_choice")
        });

        self.screens.push(Screen::QuizMnemonicWord {
            title: title.into(),
            choices: choices.iter().map(|choice| (*choice).into()).collect(),
            selected,
        });

        if self
            ._abort_nth
            .as_ref()
            .is_some_and(|&n| self.screens.len() - 1 == n)
        {
            return Err(UserAbort);
        }

        if selected as usize >= choices.len() {
            panic!(
                "quiz choice {} out of bounds for {} choices",
                selected,
                choices.len()
            );
        }

        Ok(selected)
    }
}

impl<'a> TestingUi<'a> {
    pub fn new() -> Self {
        Self {
            screens: vec![],
            _abort_nth: None,
            _enter_string: None,
            _quiz_choices: VecDeque::new(),
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

    pub fn push_quiz_choice(&mut self, selected: u8) {
        self._quiz_choices.push_back(selected);
    }

    pub fn push_quiz_choices(&mut self, selected: &[u8]) {
        for choice in selected {
            self.push_quiz_choice(*choice);
        }
    }

    fn u16_to_rand(value: u16) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[0] = (value >> 8) as u8;
        out[1] = value as u8;
        out
    }

    /// Push one mocked 16-bit random value in the format consumed by
    /// `workflow::mnemonic::create_random_unique_words()` (its local `rand16`
    /// helper reads the first two bytes big-endian).
    pub fn mock_next_u16(random: &mut super::random::TestingRandom, value: u16) {
        random.mock_next(Self::u16_to_rand(value));
    }

    /// Configure random values for one `create_random_unique_words()` call so that
    /// the correct answer is placed at choice index 2 in a 5-entry list.
    pub fn prepare_mnemonic_quiz_word_random(random: &mut super::random::TestingRandom) {
        for value in [2u16, 0, 1, 2, 3] {
            Self::mock_next_u16(random, value);
        }
    }

    /// Configure deterministic random inputs and quiz responses for
    /// `workflow::mnemonic::show_and_confirm_mnemonic`.
    /// This prepares the quiz so the correct answer is always at choice index 2.
    pub fn prepare_show_and_confirm_mnemonic(
        &mut self,
        random: &mut super::random::TestingRandom,
        num_words: usize,
    ) {
        for _ in 0..num_words {
            Self::prepare_mnemonic_quiz_word_random(random);
            self.push_quiz_choice(2);
        }
    }

    /// Assert screens emitted by `workflow::mnemonic::show_and_confirm_mnemonic()`.
    pub fn assert_show_and_confirm_mnemonic_screens(screens: &[Screen], words: &[&str]) {
        assert_eq!(
            screens[0],
            Screen::Confirm {
                title: "".into(),
                body: format!("{} words follow", words.len()),
                longtouch: false
            }
        );
        assert_eq!(
            screens[1],
            Screen::ShowMnemonic {
                words: words.iter().map(|word| (*word).into()).collect()
            }
        );
        assert_eq!(
            screens[2],
            Screen::Confirm {
                title: "".into(),
                body: "Please confirm\neach word".into(),
                longtouch: false
            }
        );

        for (word_idx, expected_word) in words.iter().enumerate() {
            match &screens[3 + word_idx] {
                Screen::QuizMnemonicWord {
                    title,
                    choices,
                    selected,
                } => {
                    assert_eq!(*selected, 2);
                    assert_eq!(title, &format!("{:02}", word_idx + 1));
                    assert_eq!(choices[*selected as usize], *expected_word);
                }
                _ => panic!("unexpected screen"),
            }
        }
        assert_eq!(screens.len(), words.len() + 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::Ui;

    use util::bb02_async::block_on;

    #[test]
    fn test_quiz_choices_queue() {
        let mut ui = TestingUi::new();
        ui.push_quiz_choice(1);
        assert!(matches!(
            block_on(ui.quiz_mnemonic_word(&["a", "b", "c"], "01")),
            Ok(1)
        ));
    }

    #[test]
    fn test_quiz_choice_records_screen() {
        let mut ui = TestingUi::new();
        ui.push_quiz_choice(2);
        assert!(matches!(
            block_on(ui.quiz_mnemonic_word(&["x", "bar", "y"], "02")),
            Ok(2)
        ));
        assert_eq!(
            ui.screens,
            vec![Screen::QuizMnemonicWord {
                title: "02".into(),
                choices: vec!["x".into(), "bar".into(), "y".into()],
                selected: 2,
            }]
        );
    }

    #[test]
    #[should_panic(expected = "quiz choice 9 out of bounds for 1 choices")]
    fn test_quiz_choice_out_of_bounds_panics() {
        let mut ui = TestingUi::new();
        ui.push_quiz_choice(9);
        let _ = block_on(ui.quiz_mnemonic_word(&["a"], "01"));
    }

    #[test]
    #[should_panic(expected = "quiz_mnemonic_word called without queued choice")]
    fn test_quiz_choice_without_state_panics() {
        let mut ui = TestingUi::new();
        let _ = block_on(ui.quiz_mnemonic_word(&["a"], "01"));
    }
}
