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
use super::confirm;
use super::menu;
use super::status::status;
use super::trinary_choice::{choose, TrinaryChoice};
use super::trinary_input_string;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use sha2::{Digest, Sha256};

fn as_str_vec(v: &[zeroize::Zeroizing<String>]) -> Vec<&str> {
    v.iter().map(|s| s.as_str()).collect()
}

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

/// Given 23 initial words, this function returns list of candidate words for the last word, such
/// that the resulting bip39 phrase has a valid checksum. There are always exactly 8 such words.
/// `entered_words` must contain 23 words from the BIP39 wordlist.
fn lastword_choices(entered_words: &[&str]) -> Vec<zeroize::Zeroizing<String>> {
    if entered_words.len() != 23 {
        panic!("must have entered 23 words");
    }

    // A 24 word seedphrase encodes 24*11 bits (33 bytes). The last byte is the checksum (hash over
    // the first 32 bytes). The last word, 11 bits, is the last 3 bits of the seed plus 8 bits of
    // the checksum. We first need the first 23 words converted to bytes so we can enumerate the 8
    // choices for the last word. libwally only lets us convert 24 words if the checksum
    // matches. Instead of rolling our own decoding function, we quickly find one valid word by
    // brute-force. We need to check at most 256 words for that, as there is exactly one valid word
    // for each 256 words block.
    let mut seed: zeroize::Zeroizing<Vec<u8>> = {
        let mut i = 0;
        loop {
            let mnemonic = zeroize::Zeroizing::new(format!(
                "{} {}",
                entered_words.join(" "),
                bitbox02::keystore::get_bip39_word(i).unwrap().as_str(),
            ));
            if let Ok(seed) = bitbox02::keystore::bip39_mnemonic_to_seed(&mnemonic) {
                break seed;
            }
            i += 1;
            if i >= 256 {
                // There must be a valid word in the first 256 bip39 words. Something went wrong.
                panic!("Could not find a valid word");
            }
        }
    };

    // Generate all 8 words matching the bip39 checksum.
    (0..8)
        .map(|i| {
            // Set last three bits of the seed to `i`.
            seed[31] &= 0b11111000;
            seed[31] |= i;
            // Compute checksum.
            let hash = Sha256::digest(&seed);
            // Last word is 11 bits: <last 3 bits of the seed || 8 bits checksum>.
            let word_idx: u16 = ((i as u16) << 8) | (hash[0] as u16);
            bitbox02::keystore::get_bip39_word(word_idx).unwrap()
        })
        .collect()
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

        let user_entry = if word_idx == 23 {
            // For the last word, we can restrict to a subset of bip39 words that fulfil the
            // checksum requirement. We do this only when entering 24 words, which results in a
            // small list of 8 valid candidates.  This special case exists so that users can
            // generate a seed using only the device and no external software, allowing seed
            // generation via dice throws, for example.

            let mut choices = lastword_choices(&as_str_vec(&entered_words[..word_idx]));
            // Add one more menu entry.
            let none_of_them_idx = {
                choices.push(zeroize::Zeroizing::new("None of them".into()));
                choices.len() - 1
            };
            match super::menu::pick(&as_str_vec(&choices), Some(&title)).await {
                Err(super::menu::CancelError::Cancelled) => {
                    Err(trinary_input_string::Error::Cancelled)
                }
                Ok(choice_idx) if choice_idx as usize == none_of_them_idx => {
                    let params = confirm::Params {
                        title: "",
                        body: "Recovery words\ninvalid.\nRestart?",
                        ..Default::default()
                    };
                    if let Ok(()) = confirm::confirm(&params).await {
                        return Err(());
                    }
                    continue;
                }
                Ok(choice_idx) => {
                    // Confirm word picked from menu again, as a typo here would be extremely annoying.
                    // Double checking is also safer, as the user might not even realize they made a typo.
                    let word = choices[choice_idx as usize].clone();
                    match confirm::confirm(&confirm::Params {
                        title: &title,
                        body: &word,
                        ..Default::default()
                    })
                    .await
                    {
                        Err(confirm::UserAbort) => continue,
                        Ok(()) => Ok(word),
                    }
                }
            }
        } else {
            trinary_input_string::enter(
                &trinary_input_string::Params {
                    title: &title,
                    wordlist: Some(&bip39_wordlist),
                    ..Default::default()
                },
                trinary_input_string::CanCancel::Yes,
                preset,
            )
            .await
            .map(|s| s.as_string())
        };

        match user_entry {
            Err(CancelError::Cancelled) => {
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
                        let params = confirm::Params {
                            title: "Restore",
                            body: "Do you really\nwant to cancel?",
                            ..Default::default()
                        };

                        if let Err(confirm::UserAbort) = confirm::confirm(&params).await {
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
        as_str_vec(&entered_words[..num_words]).join(" "),
    ))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;

    fn bruteforce_lastword(mnemonic: &[&str]) -> Vec<zeroize::Zeroizing<String>> {
        let mut result = Vec::new();
        for i in 0..bitbox02::keystore::BIP39_WORDLIST_LEN {
            let word = bitbox02::keystore::get_bip39_word(i).unwrap();
            let mut m = mnemonic.to_vec();
            m.push(&word);
            if bitbox02::keystore::bip39_mnemonic_to_seed(&m.join(" ")).is_ok() {
                result.push(word);
            }
        }
        result
    }

    #[test]
    fn test_lastword_choices() {
        let _guard = MUTEX.lock().unwrap();

        assert_eq!(
            &as_str_vec(&bruteforce_lastword(&["violin"; 23])),
            &["boss", "coyote", "dry", "habit", "panel", "regular", "speed", "winter"]
        );

        assert_eq!(
            &lastword_choices(&["violin"; 23]),
            &bruteforce_lastword(&["violin"; 23]),
        );

        let mnemonic = "side stuff card razor rescue enhance risk exchange ozone render large describe gas juice offer permit vendor custom forget lecture divide junior narrow".split(' ').collect::<Vec<&str>>();
        assert_eq!(
            &lastword_choices(&mnemonic),
            &bruteforce_lastword(&mnemonic)
        );
    }
}
