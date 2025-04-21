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
use super::trinary_choice::TrinaryChoice;
use super::trinary_input_string;
use super::Workflows;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use sha2::{Digest, Sha256};

const NUM_RANDOM_WORDS: u8 = 5;

fn as_str_vec(v: &[zeroize::Zeroizing<String>]) -> Vec<&str> {
    v.iter().map(|s| s.as_str()).collect()
}

/// Return 5 words from the BIP39 wordlist, 4 of which are random, and
/// one of them is provided `word`. Returns the position of `word` in
/// the list of words, and the lis of words.  This is used to test if
/// the user wrote down the seed words properly.
fn create_random_unique_words(word: &str, length: u8) -> (u8, Vec<zeroize::Zeroizing<String>>) {
    fn rand16() -> u16 {
        let mut rand = [0u8; 32];
        bitbox02::random::mcu_32_bytes(&mut rand);
        ((rand[0] as u16) << 8) | (rand[1] as u16)
    }

    let index_word = (rand16() as u8) % length;
    let mut picked_indices = Vec::new();
    let result = (0..length)
        .map(|i| {
            // The correct word at the right index.
            if i == index_word {
                return zeroize::Zeroizing::new(word.into());
            }

            // A random word everywhere else.
            // Loop until we get a unique word, we don't want repeated words in the list.
            loop {
                let idx = rand16() % bitbox02::keystore::BIP39_WORDLIST_LEN;
                if picked_indices.contains(&idx) {
                    continue;
                };
                let random_word = bitbox02::keystore::get_bip39_word(idx).unwrap();
                if random_word.as_str() == word {
                    continue;
                }
                picked_indices.push(idx);
                return random_word;
            }
        })
        .collect();

    (index_word, result)
}

/// Displays all mnemonic words in a scroll-through screen.
async fn show_mnemonic(words: &[&str]) -> Result<(), CancelError> {
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
async fn confirm_word(choices: &[&str], title: &str) -> Result<u8, CancelError> {
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

pub async fn show_and_confirm_mnemonic(
    hal: &mut impl crate::hal::Hal,
    words: &[&str],
) -> Result<(), CancelError> {
    // Part 1) Scroll through words
    show_mnemonic(words).await?;

    // Can only succeed due to `accept_only`.
    let _ = hal
        .ui()
        .confirm(&confirm::Params {
            title: "",
            body: "Please confirm\neach word",
            accept_only: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await;

    // Part 2) Confirm words
    for (word_idx, word) in words.iter().enumerate() {
        let title = format!("{:02}", word_idx + 1);
        let (correct_idx, choices) = create_random_unique_words(word, NUM_RANDOM_WORDS);
        let mut choices: Vec<&str> = choices.iter().map(|c| c.as_ref()).collect();
        choices.push("Back to\nrecovery words");
        let back_idx = (choices.len() - 1) as u8;
        loop {
            match confirm_word(&choices, &title).await? {
                selected_idx if selected_idx == correct_idx => break,
                selected_idx if selected_idx == back_idx => show_mnemonic(words).await?,
                _ => hal.ui().status("Incorrect word\nTry again", false).await,
            }
        }
    }

    Ok(())
}

/// Given 11/17/23 initial words, this function returns a list of candidate words for the last word,
/// such that the resulting bip39 phrase has a valid checksum. There are always exactly 8 such words
/// for 24 word mnemonics, 32 words for 18 word mnemonics and 128 words for 12 word mnemonics.
/// `entered_words` must contain 11/17/23 words from the BIP39 wordlist.
/// The result is the list of indices of the words in the BIP39 wordlist.
fn lastword_choices(entered_words: &[&str]) -> Vec<u16> {
    let (seed_len_bits, checksum_len_bits, bitmask_seed) = match entered_words.len() {
        11 => (128, 4, 0b10000000),
        17 => (192, 6, 0b11100000),
        23 => (256, 8, 0b11111000),
        _ => panic!("invalid number of entered words"),
    };
    let num_candidates = 1 << (seed_len_bits % 11);

    // A seedphrase for 12/18/24 words encodes 128/192/256 bits. The last 4/6/8 bits is the checksum
    // (hash over the first 32 bytes). The last word, 11 bits, is the last 7/5/3 bits of the seed
    // plus 4/6/8 bits of the checksum. We first need the first 11/17/23 words converted to bytes so
    // we can enumerate the 8/32/128 choices for the last word. libwally only lets us convert a
    // seedphrase if the checksum matches. Instead of rolling our own decoding function, we quickly
    // find one valid word by brute-force. We need to check at most 16/64/256 words for that, as
    // there is exactly one valid word for each 256 words block.
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
            if i >= 2048 / num_candidates {
                // There must be a valid word in the first 16/64/256 bip39 words. Something went
                // wrong.
                panic!("Could not find a valid word");
            }
        }
    };
    let seed_len = seed.len();
    // Generate all words matching the bip39 checksum.
    (0..num_candidates)
        .map(|i: u16| {
            // Set last 7/5/3 bits of the seed to `i`.
            seed[seed_len - 1] &= bitmask_seed;
            seed[seed_len - 1] |= i as u8;
            // Compute checksum.
            let hash = Sha256::digest(&seed);
            // Last word is 11 bits: <last 7/5/3 bits of the seed || 4/6/8 bits checksum>.
            let word_idx: u16 =
                (i << checksum_len_bits) | (hash[0] >> (8 - checksum_len_bits)) as u16;
            word_idx
        })
        .collect()
}

fn lastword_choices_strings(entered_words: &[&str]) -> Vec<zeroize::Zeroizing<String>> {
    lastword_choices(entered_words)
        .into_iter()
        .map(|word_idx| bitbox02::keystore::get_bip39_word(word_idx).unwrap())
        .collect()
}

/// Select the 24th word from a list of 8 valid candidate words presented as a menu.
/// Returns `Ok(None)` if the user chooses "None of them".
/// Returns `Ok(Some(word))` if the user chooses a word.
/// Returns `Err(CancelError::Cancelled)` if the user cancels.
async fn get_24th_word(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    entered_words: &[&str],
) -> Result<Option<zeroize::Zeroizing<String>>, CancelError> {
    let mut choices = lastword_choices_strings(entered_words);
    // Add one more menu entry.
    let none_of_them_idx = {
        choices.push(zeroize::Zeroizing::new("None of them".into()));
        choices.len() - 1
    };
    loop {
        match hal.ui().menu(&as_str_vec(&choices), Some(title)).await {
            Err(menu::CancelError::Cancelled) => return Err(CancelError::Cancelled),
            Ok(choice_idx) if choice_idx as usize == none_of_them_idx => {
                let params = confirm::Params {
                    title: "",
                    body: "Invalid. Check\nrecovery words.\nRestart?",
                    ..Default::default()
                };
                if let Ok(()) = hal.ui().confirm(&params).await {
                    return Ok(None);
                }
            }
            Ok(choice_idx) => {
                // Confirm word picked from menu again, as a typo here would be extremely annoying.
                // Double checking is also safer, as the user might not even realize they made a typo.
                let word = choices[choice_idx as usize].clone();
                if let Ok(()) = hal
                    .ui()
                    .confirm(&confirm::Params {
                        title,
                        body: &word,
                        ..Default::default()
                    })
                    .await
                {
                    return Ok(Some(word));
                }
            }
        }
    }
}

/// Select the last word of a 12 or 18 word mnemonic from a list of valid candidate words. The input
/// is the trinary input keyboard with the wordlist restricted to these candidates.
///
/// Returns `Ok(word)` if the user chooses a word.
/// Returns `Err(CancelError::Cancelled)` if the user cancels.
async fn get_12th_18th_word(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    entered_words: &[&str],
) -> Result<zeroize::Zeroizing<String>, CancelError> {
    // With 12/18 words there are 128/32 candidates, so we limit the keyboard to allow entering only
    // these.
    loop {
        let choices = lastword_choices(entered_words);
        let candidates = bitbox02::keystore::get_bip39_wordlist(Some(&choices));
        let word = hal
            .ui()
            .enter_string(
                &trinary_input_string::Params {
                    title,
                    wordlist: Some(&candidates),
                    ..Default::default()
                },
                trinary_input_string::CanCancel::Yes,
                "",
            )
            .await?;

        // Confirm word picked again, as a typo here would be extremely annoying.  Double checking
        // is also safer, as the user might not even realize they made a typo.
        if let Ok(()) = hal
            .ui()
            .confirm(&confirm::Params {
                title,
                body: &word,
                ..Default::default()
            })
            .await
        {
            return Ok(word);
        }
    }
}

/// Retrieve a BIP39 mnemonic sentence of 12, 18 or 24 words from the user.
pub async fn get(
    hal: &mut impl crate::hal::Hal,
) -> Result<zeroize::Zeroizing<String>, CancelError> {
    let num_words: usize = match hal
        .ui()
        .trinary_choice("How many words?", "12", "18", "24")
        .await
    {
        TrinaryChoice::TRINARY_CHOICE_LEFT => 12,
        TrinaryChoice::TRINARY_CHOICE_MIDDLE => 18,
        TrinaryChoice::TRINARY_CHOICE_RIGHT => 24,
    };

    hal.ui()
        .status(&format!("Enter {} words", num_words), true)
        .await;

    // Provide all bip39 words to restrict the keyboard entry.
    let bip39_wordlist = bitbox02::keystore::get_bip39_wordlist(None);

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

        let user_entry: Result<zeroize::Zeroizing<String>, CancelError> = if word_idx
            == num_words - 1
        {
            // For the last word, we can restrict to a subset of bip39 words that fulfil the
            // checksum requirement. This special case exists so that users can generate a seed
            // using only the device and no external software, allowing seed generation via dice
            // throws, for example.
            if num_words == 24 {
                // With 24 words there are only 8 valid candidates. We presnet them as a menu.
                match get_24th_word(hal, &title, &as_str_vec(&entered_words[..word_idx])).await {
                    Ok(None) => return Err(CancelError::Cancelled),
                    Ok(Some(r)) => Ok(r),
                    Err(e) => Err(e),
                }
            } else {
                get_12th_18th_word(hal, &title, &as_str_vec(&entered_words[..word_idx])).await
            }
        } else {
            hal.ui()
                .enter_string(
                    &trinary_input_string::Params {
                        title: &title,
                        wordlist: Some(&bip39_wordlist),
                        ..Default::default()
                    },
                    trinary_input_string::CanCancel::Yes,
                    preset,
                )
                .await
                .into()
        };

        match user_entry {
            Err(CancelError::Cancelled) => {
                // User clicked the cancel button. There are two choices:
                enum GetWordError {
                    Cancel,
                    EditPrevious,
                }

                let cancel_choice = if word_idx == 0 {
                    // In the first word, there is no previous word, so we go straight to the cancel
                    // action.
                    GetWordError::Cancel
                } else {
                    // In all other words, we give the choice between editing the previous word and
                    // cancelling.
                    match hal
                        .ui()
                        .menu(&["Edit previous word", "Cancel restore"], Some("Choose"))
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

                        if let Err(confirm::UserAbort) = hal.ui().confirm(&params).await {
                            // Cancel cancelled.
                            continue;
                        }
                        return Err(CancelError::Cancelled);
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
    use super::*;

    use alloc::boxed::Box;
    use bitbox02::testing::{mock, Data};

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
        // 23 words

        assert_eq!(
            &as_str_vec(&bruteforce_lastword(&["violin"; 23])),
            &["boss", "coyote", "dry", "habit", "panel", "regular", "speed", "winter"]
        );

        assert_eq!(
            &lastword_choices_strings(&["violin"; 23]),
            &bruteforce_lastword(&["violin"; 23]),
        );

        let mnemonic = "side stuff card razor rescue enhance risk exchange ozone render large describe gas juice offer permit vendor custom forget lecture divide junior narrow".split(' ').collect::<Vec<&str>>();
        assert_eq!(
            &lastword_choices_strings(&mnemonic),
            &bruteforce_lastword(&mnemonic)
        );

        // 17 words

        assert_eq!(
            &as_str_vec(&bruteforce_lastword(&["violin"; 17])),
            &[
                "all", "appear", "bike", "book", "cash", "click", "cycle", "disagree", "donate",
                "essence", "fence", "gadget", "ghost", "hotel", "industry", "lab", "lizard",
                "modify", "much", "oblige", "pond", "pull", "raccoon", "reunion", "side", "smoke",
                "steak", "taxi", "tongue", "used", "wall", "wonder"
            ]
        );

        assert_eq!(
            &lastword_choices_strings(&["violin"; 17]),
            &bruteforce_lastword(&["violin"; 17]),
        );

        let mnemonic = "alpha write diary chicken cable spoil dirt hair bike fiction system bright mimic garage giggle involve leisure".split(' ').collect::<Vec<&str>>();
        assert_eq!(
            &lastword_choices_strings(&mnemonic),
            &bruteforce_lastword(&mnemonic)
        );

        // 11 words

        assert_eq!(
            &as_str_vec(&bruteforce_lastword(&["violin"; 11])),
            &[
                "achieve", "actress", "affair", "all", "amount", "arm", "arrest", "attend",
                "bacon", "bar", "best", "bitter", "body", "box", "brush", "bulk", "cage", "carry",
                "chalk", "chicken", "city", "climb", "color", "convince", "cotton", "crawl",
                "cruel", "dawn", "degree", "desk", "diet", "disease", "double", "dumb", "duty",
                "elder", "enemy", "engage", "essay", "evoke", "faint", "family", "feel", "finger",
                "flush", "foil", "frame", "garage", "giant", "glue", "gorilla", "green", "habit",
                "health", "horse", "hover", "illness", "inherit", "intact", "island", "keen",
                "know", "ladder", "lawsuit", "lesson", "lobster", "love", "main", "matter",
                "mention", "milk", "monitor", "mother", "myself", "nest", "nose", "offer", "open",
                "outer", "paddle", "peanut", "pear", "piece", "polar", "post", "print", "pulse",
                "purpose", "rally", "rebuild", "regret", "report", "rifle", "rocket", "royal",
                "salon", "sea", "segment", "shallow", "ship", "similar", "slice", "snake", "soft",
                "source", "spray", "steel", "style", "super", "swim", "talk", "tent", "they",
                "tiny", "tone", "treat", "trim", "turtle", "unaware", "upper", "van", "viable",
                "vivid", "walnut", "weird", "window", "worth", "zero"
            ]
        );

        assert_eq!(
            &lastword_choices_strings(&["violin"; 11]),
            &bruteforce_lastword(&["violin"; 11]),
        );

        let mnemonic = "outer elite desert faint cliff useless teach screen combine exercise below"
            .split(' ')
            .collect::<Vec<&str>>();
        assert_eq!(
            &lastword_choices_strings(&mnemonic),
            &bruteforce_lastword(&mnemonic)
        );
    }
}
