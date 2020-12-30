// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use super::cancel::Error as CancelError;
use super::cancel::{cancel, set_result, with_cancel};
use super::menu;
use super::status::status;
use super::trinary_choice::{choose, TrinaryChoice};
use super::trinary_input_string;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

/// Displays all mnemonic words in a scroll-through screen.
pub async fn show_mnemonic(words: &[&str]) -> Result<(), CancelError> {
    let result = RefCell::new(None);
    let mut component = bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words,
        title: None,
        select_word_cb: None,
        continue_on_last_cb: Some(Box::new(|| {
            set_result(&result, ());
        })),
        cancel_cb: Some(Box::new(|| {
            cancel(&result);
        })),
    });
    with_cancel("Recovery\nwords", &mut component, &result).await
}

/// Displays the `choices` to the user, returning the index of the selected choice.
pub async fn confirm_word(choices: &[&str], title: &str) -> Result<u8, CancelError> {
    let result = RefCell::new(None);
    let mut component = bitbox02::ui::menu_create(bitbox02::ui::MenuParams {
        words: choices,
        title: Some(title),
        select_word_cb: Some(Box::new(|idx| {
            set_result(&result, idx);
        })),
        continue_on_last_cb: None,
        cancel_cb: Some(Box::new(|| {
            cancel(&result);
        })),
    });
    with_cancel("Recovery\nwords", &mut component, &result).await
}

/// Retrieve a BIP39 mnemonic sentence of 12, 18 or 24 words from the user.
pub async fn get() -> Result<zeroize::Zeroizing<String>, ()> {
    let num_words: usize = match choose("How many words?", "12", "18", "24").await {
        TrinaryChoice::TRINARY_CHOICE_LEFT => 12,
        TrinaryChoice::TRINARY_CHOICE_MIDDLE => 18,
        TrinaryChoice::TRINARY_CHOICE_RIGHT => 24,
    };

    status(&format!("Enter {} words", num_words), true).await;

    // Provide all bip39 words to restrict the keyboard entry.
    let bip39_wordlist = bitbox02::keystore::get_bip39_wordlist()?;

    let mut word_idx: usize = 0;
    let mut entered_words = vec![zeroize::Zeroizing::new(String::new()); num_words];
    while word_idx < num_words {
        let title = match word_idx + 1 {
            n @ 1 | n @ 21 => format!("{}st word", n),
            n @ 2 | n @ 22 => format!("{}nd word", n),
            n @ 3 | n @ 23 => format!("{}rd word", n),
            n => format!("{}th word", n),
        };

        // The already entered word will already be filled out (if not empty, i.e. not entered
        // before). This happens when one goes back to edit previous words, and also when the user
        // goes forward again.
        let preset = entered_words[word_idx].as_str();

        let user_entry = trinary_input_string::enter(
            &trinary_input_string::Params {
                title: &title,
                wordlist: Some(&bip39_wordlist),
                ..Default::default()
            },
            trinary_input_string::CanCancel::Yes,
            preset,
        )
        .await
        .map(|s| s.as_string());

        match user_entry {
            Err(trinary_input_string::Error::Cancelled) => {
                // User clicked the cancel button. There are two choices:
                enum GetWordError {
                    Cancel,
                    EditPrevious,
                };

                let cancel_choice = if word_idx == 0 {
                    // In the first word, there is no previous word, so we go straight to the cancel
                    // action.
                    GetWordError::Cancel
                } else {
                    // In all other words, we give the choice between editing the previous word and
                    // cancelling.
                    match menu::pick(&["Edit previous word", "Cancel restore"], Some("Choose"))
                        .await
                    {
                        Err(menu::CancelError::Cancelled) => {
                            // Cancel cancelled.
                            continue;
                        }
                        Ok(0) => GetWordError::EditPrevious,
                        Ok(1) => GetWordError::Cancel,
                        _ => panic!("only two choices"),
                    }
                };

                match cancel_choice {
                    GetWordError::EditPrevious => word_idx -= 1,
                    GetWordError::Cancel => {
                        let params = super::confirm::Params {
                            title: "Restore",
                            body: "Do you really\nwant to cancel?",
                            ..Default::default()
                        };

                        if !super::confirm::confirm(&params).await {
                            // Cancel cancelled.
                            continue;
                        }
                        return Err(());
                    }
                }
            }
            Ok(word) => {
                entered_words[word_idx] = word;
                word_idx += 1;
            }
        }
    }
    Ok(zeroize::Zeroizing::new(
        entered_words[..num_words]
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" "),
    ))
}
