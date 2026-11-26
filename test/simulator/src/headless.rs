// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::Ui;
use bitbox_hal::ui::{
    CanCancel, ConfirmParams, EnterStringParams, Font, MnemonicWorkflow, TrinaryChoice, UserAbort,
    WordlistEntryAbort,
};
use bitbox02::ui::log;
use std::time::Duration;
use zeroize::Zeroizing;

use crate::core::{MNEMONIC, SimulatorCore, TICK_INTERVAL};

type HeadlessHal = crate::hal::SimulatorHal<HeadlessUi, false>;

#[derive(Default)]
pub struct HeadlessUi {
    mnemonic: Option<MnemonicWorkflow>,
    words: Vec<Zeroizing<String>>,
    word_index: usize,
}

impl HeadlessUi {
    fn restore_word(&self) -> &'static str {
        assert!(self.mnemonic == Some(MnemonicWorkflow::Restore));
        MNEMONIC.split(' ').nth(self.word_index).unwrap()
    }
}

pub struct Progress;
impl bitbox_hal::ui::Progress for Progress {
    fn set_fraction(&mut self, numerator: u32, denominator: u32) {
        assert!(denominator != 0 && numerator <= denominator);
    }
}

pub struct Empty;
impl bitbox_hal::ui::Empty for Empty {}

impl Ui for HeadlessUi {
    type Progress = Progress;
    type Empty = Empty;
    type UnlockAnimation = ();

    fn mnemonic_workflow_begin(&mut self, workflow: MnemonicWorkflow) {
        self.words.clear();
        self.word_index = 0;
        self.mnemonic = Some(workflow);
        if workflow == MnemonicWorkflow::Restore {
            println!("Restored from recovery words below:");
            println!("{MNEMONIC}");
        }
    }

    fn mnemonic_workflow_end(&mut self) {
        if self.mnemonic == Some(MnemonicWorkflow::Confirm)
            && !self.words.is_empty()
            && self.word_index == self.words.len()
        {
            println!("Words confirmed");
        }
        self.reset();
    }

    async fn confirm(&mut self, params: &ConfirmParams<'_>) -> Result<(), UserAbort> {
        if self.mnemonic.is_none() {
            log::confirm(params.title, params.body);
        }
        Ok(())
    }

    fn has_glyph(&self, font: Font, c: char) -> bool {
        bitbox02::hal::ui::BitBox02Ui::<bitbox_platform_host::timer::HostTimer>::new()
            .has_glyph(font, c)
    }

    async fn confirm_swap(&mut self, title: &str, from: &str, to: &str) -> Result<(), UserAbort> {
        if self.mnemonic.is_none() {
            log::swap(title, from, to);
        }
        Ok(())
    }

    async fn verify_recipient(&mut self, recipient: &str, amount: &str) -> Result<(), UserAbort> {
        if self.mnemonic.is_none() {
            log::transaction_address(amount, recipient);
        }
        Ok(())
    }

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        _longtouch: bool,
    ) -> Result<(), UserAbort> {
        if self.mnemonic.is_none() {
            log::transaction_fee(total, fee);
        }
        Ok(())
    }

    fn unlock_animation_create(&mut self) {}
    async fn unlock_animation_play(&mut self, _animation: ()) {}

    async fn status(&mut self, title: &str, _status_success: bool) {
        if self.mnemonic.is_none() {
            log::status(title);
        }
    }

    async fn waiting(&mut self, _message: &str) {
        std::future::pending::<()>().await;
    }

    fn print_screen(&mut self, _duration: Duration, _msg: &str) {}
    fn switch_to_logo(&mut self) {}

    fn reset(&mut self) {
        self.words.clear();
        self.word_index = 0;
        self.mnemonic = None;
    }

    fn progress_create(&mut self, _title: &str) -> Progress {
        Progress
    }

    fn empty_create(&mut self) -> Empty {
        Empty
    }

    async fn enter_string(
        &mut self,
        params: &EnterStringParams<'_>,
        _can_cancel: CanCancel,
        _preset: &str,
    ) -> Result<Zeroizing<String>, UserAbort> {
        if self.mnemonic.is_none() {
            log::enter_string(params.title);
        }
        Ok(Zeroizing::new(String::new()))
    }

    async fn enter_wordlist_word(
        &mut self,
        _title: &str,
        wordlist: &[u16],
        _preset: &str,
    ) -> Result<Zeroizing<String>, WordlistEntryAbort> {
        let word = self.restore_word();
        assert!(
            wordlist
                .iter()
                .any(|&index| { bitbox02_rust::bip39::get_word(index).unwrap().as_str() == word })
        );
        self.word_index += 1;
        Ok(Zeroizing::new(word.into()))
    }

    async fn insert_sdcard(&mut self) -> Result<(), UserAbort> {
        Ok(())
    }

    async fn menu(&mut self, words: &[&str], _title: Option<&str>) -> Result<u8, UserAbort> {
        let word = self.restore_word();
        let index = words
            .iter()
            .position(|candidate| *candidate == word)
            .unwrap();
        self.word_index += 1;
        Ok(index.try_into().unwrap())
    }

    async fn trinary_choice(
        &mut self,
        _message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> TrinaryChoice {
        assert!(self.mnemonic == Some(MnemonicWorkflow::Restore));
        assert_eq!(
            (label_left, label_middle, label_right),
            (Some("12"), None, Some("24"))
        );
        TrinaryChoice::Right
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), UserAbort> {
        self.words = words
            .iter()
            .map(|word| Zeroizing::new((*word).into()))
            .collect();
        self.word_index = 0;
        for word in words {
            println!("{word}");
        }
        Ok(())
    }

    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        _title: &str,
    ) -> Result<u8, UserAbort> {
        let word = self.words[self.word_index].as_str();
        let index = choices
            .iter()
            .position(|candidate| *candidate == word)
            .unwrap();
        self.word_index += 1;
        Ok(index.try_into().unwrap())
    }
}

pub fn run(args: &crate::Args) -> std::io::Result<()> {
    let mut core = SimulatorCore::<HeadlessHal>::new(args)?;
    while core.tick()? {
        std::thread::park_timeout(TICK_INTERVAL);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_hal::memory::Language;

    struct Random(u32);

    impl bitbox_hal::Random for Random {
        fn factory_randomness(&mut self) -> &'static [u8; 32] {
            &[0; 32]
        }

        fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
            for byte in out {
                self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
                *byte = (self.0 >> 24) as u8;
            }
        }
    }

    #[async_test::test]
    async fn test_get_with_language() {
        let mut ui = HeadlessUi::default();
        for language in [Language::English, Language::German, Language::English] {
            let mnemonic = bitbox02_rust::workflow::mnemonic::get_with_language(&mut ui, language)
                .await
                .map_err(|_| ())
                .unwrap();
            assert_eq!(mnemonic.as_str(), MNEMONIC);
            assert!(ui.mnemonic.is_none());
            assert!(ui.words.is_empty());
            assert_eq!(ui.word_index, 0);
        }
    }

    #[async_test::test]
    async fn test_show_and_confirm_mnemonic() {
        let mut ui = HeadlessUi::default();
        let mut random = Random(1);
        let words: Vec<_> = MNEMONIC.split(' ').collect();
        for count in [12, 18, 24, 12] {
            bitbox02_rust::workflow::mnemonic::show_and_confirm_mnemonic(
                &mut ui,
                &mut random,
                &words[..count],
                Language::English,
            )
            .await
            .map_err(|_| ())
            .unwrap();
            assert!(ui.mnemonic.is_none());
            assert!(ui.words.is_empty());
        }
    }

    #[async_test::test]
    async fn test_waiting() {
        let mut ui = HeadlessUi::default();
        let mut task: util::bb02_async::Task<'_, ()> = Box::pin(ui.waiting("Please wait"));
        assert!(util::bb02_async::spin(&mut task).is_pending());
        drop(task);
        ui.confirm(&ConfirmParams::default())
            .await
            .map_err(|_| ())
            .unwrap();
    }
}
