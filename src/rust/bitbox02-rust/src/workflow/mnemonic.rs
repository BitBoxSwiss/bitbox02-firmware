// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::{CanCancel, ConfirmParams, TrinaryChoice, UserAbort, WordlistEntryAbort};

use alloc::string::String;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

const NUM_RANDOM_WORDS: u8 = 5;

/// Number of words in the BIP-39 wordlist.
const BIP39_WORDLIST_LEN: u16 = 2048;

fn as_str_vec(v: &[zeroize::Zeroizing<String>]) -> Vec<&str> {
    v.iter().map(|s| s.as_str()).collect()
}

/// Return 5 words from the BIP39 wordlist, 4 of which are random, and
/// one of them is provided `word`. Returns the position of `word` in
/// the list of words, and the lis of words.  This is used to test if
/// the user wrote down the seed words properly.
fn create_random_unique_words(
    hal_random: &mut impl crate::hal::Random,
    word: &str,
    length: u8,
) -> (u8, Vec<zeroize::Zeroizing<String>>) {
    fn rand16(hal_random: &mut impl crate::hal::Random) -> u16 {
        let mut rand = [0u8; 32];
        hal_random.mcu_32_bytes(&mut rand);
        ((rand[0] as u16) << 8) | (rand[1] as u16)
    }

    let index_word = (rand16(hal_random) as u8) % length;
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
                let idx = rand16(hal_random) % BIP39_WORDLIST_LEN;
                if picked_indices.contains(&idx) {
                    continue;
                };
                let random_word = crate::bip39::get_word(idx).unwrap();
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

pub async fn show_and_confirm_mnemonic(
    hal_ui: &mut impl crate::hal::Ui,
    hal_random: &mut impl crate::hal::Random,
    words: &[&str],
) -> Result<(), UserAbort> {
    hal_ui
        .confirm(&ConfirmParams {
            title: "",
            body: &format!("{} words follow", words.len()),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await
        .map_err(|_| UserAbort)?;

    // Part 1) Scroll through words
    hal_ui.show_mnemonic(words).await?;

    // Can only succeed due to `accept_only`.
    let _ = hal_ui
        .confirm(&ConfirmParams {
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
        let (correct_idx, choices) = create_random_unique_words(hal_random, word, NUM_RANDOM_WORDS);
        let mut choices: Vec<&str> = choices.iter().map(|c| c.as_ref()).collect();
        choices.push("Back to\nrecovery words");
        let back_idx = (choices.len() - 1) as u8;
        loop {
            match hal_ui.quiz_mnemonic_word(&choices, &title).await? {
                selected_idx if selected_idx == correct_idx => break,
                selected_idx if selected_idx == back_idx => hal_ui.show_mnemonic(words).await?,
                _ => hal_ui.status("Incorrect word\nTry again", false).await,
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
                crate::bip39::get_word(i).unwrap().as_str(),
            ));
            if let Ok(seed) = crate::bip39::mnemonic_to_seed(&mnemonic) {
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
        .map(|word_idx| crate::bip39::get_word(word_idx).unwrap())
        .collect()
}

/// Select the 24th word from a list of 8 valid candidate words presented as a menu.
/// Returns `Ok(None)` if the user chooses "None of them".
/// Returns `Ok(Some(word))` if the user chooses a word.
/// Returns `Err(UserAbort)` if the user cancels.
async fn get_24th_word(
    hal_ui: &mut impl crate::hal::Ui,
    title: &str,
    entered_words: &[&str],
) -> Result<Option<zeroize::Zeroizing<String>>, UserAbort> {
    let mut choices = lastword_choices_strings(entered_words);
    // Add one more menu entry.
    let none_of_them_idx = {
        choices.push(zeroize::Zeroizing::new("None of them".into()));
        choices.len() - 1
    };
    loop {
        match hal_ui.menu(&as_str_vec(&choices), Some(title)).await {
            Err(UserAbort) => return Err(UserAbort),
            Ok(choice_idx) if choice_idx as usize == none_of_them_idx => {
                let params = ConfirmParams {
                    title: "",
                    body: "Invalid. Check\nrecovery words.\nRestart?",
                    ..Default::default()
                };
                if let Ok(()) = hal_ui.confirm(&params).await {
                    return Ok(None);
                }
            }
            Ok(choice_idx) => {
                // Confirm word picked from menu again, as a typo here would be extremely annoying.
                // Double checking is also safer, as the user might not even realize they made a typo.
                let word = choices[choice_idx as usize].clone();
                if let Ok(()) = hal_ui
                    .confirm(&ConfirmParams {
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
/// Returns `Err(UserAbort)` if the user cancels.
async fn get_12th_18th_word(
    hal_ui: &mut impl crate::hal::Ui,
    title: &str,
    entered_words: &[&str],
) -> Result<zeroize::Zeroizing<String>, WordlistEntryAbort> {
    // With 12/18 words there are 128/32 candidates, so we limit the keyboard to allow entering only
    // these.
    loop {
        let choices = lastword_choices(entered_words);
        let word = enter_word_from_wordlist(hal_ui, title, &choices, "").await?;

        // Confirm word picked again, as a typo here would be extremely annoying.  Double checking
        // is also safer, as the user might not even realize they made a typo.
        if let Ok(()) = hal_ui
            .confirm(&ConfirmParams {
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

fn wordlist_contains(wordlist: &[u16], word: &str) -> bool {
    wordlist
        .iter()
        .any(|word_idx| match crate::bip39::get_word(*word_idx) {
            Ok(candidate) => candidate.as_str() == word,
            Err(()) => false,
        })
}

async fn enter_word_from_wordlist(
    hal_ui: &mut impl crate::hal::Ui,
    title: &str,
    wordlist: &[u16],
    preset: &str,
) -> Result<zeroize::Zeroizing<String>, WordlistEntryAbort> {
    loop {
        let word = hal_ui
            .enter_wordlist_word(
                &crate::hal::ui::EnterStringParams {
                    title,
                    wordlist: Some(wordlist),
                    ..Default::default()
                },
                CanCancel::Yes,
                preset,
            )
            .await?;

        if wordlist_contains(wordlist, &word) {
            return Ok(word);
        }
        hal_ui.status("Invalid word\nTry again", false).await;
    }
}

/// Retrieve a BIP39 mnemonic sentence of 12 or 24 words from the user.
pub async fn get(
    hal_ui: &mut impl crate::hal::Ui,
) -> Result<zeroize::Zeroizing<String>, UserAbort> {
    let num_words: usize = match hal_ui
        .trinary_choice("How many words?", Some("12"), None, Some("24"))
        .await
    {
        TrinaryChoice::Left => 12,
        TrinaryChoice::Middle => unreachable!(),
        TrinaryChoice::Right => 24,
    };

    hal_ui
        .status(&format!("Enter {} words", num_words), true)
        .await;

    // Provide all bip39 words to restrict the keyboard entry.
    let bip39_wordlist: Vec<u16> = (0..BIP39_WORDLIST_LEN).collect();

    let mut word_idx: usize = 0;
    let mut entered_words = vec![zeroize::Zeroizing::new(String::new()); num_words];
    while word_idx < num_words {
        let title = format!("{} of {}", word_idx + 1, num_words);

        // The already entered word will already be filled out (if not empty, i.e. not entered
        // before). This happens when one goes back to edit previous words, and also when the user
        // goes forward again.
        let preset = entered_words[word_idx].as_str();

        let user_entry: Result<zeroize::Zeroizing<String>, WordlistEntryAbort> = if word_idx
            == num_words - 1
        {
            // For the last word, we can restrict to a subset of bip39 words that fulfil the
            // checksum requirement. This special case exists so that users can generate a seed
            // using only the device and no external software, allowing seed generation via dice
            // throws, for example.
            if num_words == 24 {
                // With 24 words there are only 8 valid candidates. We presnet them as a menu.
                match get_24th_word(hal_ui, &title, &as_str_vec(&entered_words[..word_idx])).await {
                    Ok(None) => return Err(UserAbort),
                    Ok(Some(r)) => Ok(r),
                    // The menu has a single abort control, so ask what the user meant.
                    Err(UserAbort) => Err(WordlistEntryAbort::Unspecified),
                }
            } else {
                get_12th_18th_word(hal_ui, &title, &as_str_vec(&entered_words[..word_idx])).await
            }
        } else {
            enter_word_from_wordlist(hal_ui, &title, &bip39_wordlist, preset).await
        };

        match user_entry {
            Err(abort) => {
                // User left the word entry without entering a word. There are two choices:
                enum GetWordError {
                    Cancel,
                    EditPrevious,
                }

                let cancel_choice = match abort {
                    // A dedicated back control (BitBox03) goes straight back to the previous
                    // word; in the first word there is no previous word, so it acts as cancel.
                    WordlistEntryAbort::Back if word_idx > 0 => GetWordError::EditPrevious,
                    WordlistEntryAbort::Back | WordlistEntryAbort::Cancel => GetWordError::Cancel,
                    // A single abort control (BitBox02): ask whether the user meant to edit the
                    // previous word or to cancel — except in the first word, where there is no
                    // previous word and we go straight to the cancel action.
                    WordlistEntryAbort::Unspecified if word_idx == 0 => GetWordError::Cancel,
                    WordlistEntryAbort::Unspecified => {
                        match hal_ui
                            .menu(&["Edit previous word", "Cancel restore"], Some("Choose"))
                            .await
                        {
                            Err(UserAbort) => {
                                // Cancel cancelled.
                                continue;
                            }
                            Ok(0) => GetWordError::EditPrevious,
                            Ok(1) => GetWordError::Cancel,
                            _ => panic!("only two choices"),
                        }
                    }
                };

                match cancel_choice {
                    GetWordError::EditPrevious => word_idx -= 1,
                    GetWordError::Cancel => {
                        let params = ConfirmParams {
                            title: "Restore",
                            body: "Cancel restore?",
                            ..Default::default()
                        };

                        if let Err(UserAbort) = hal_ui.confirm(&params).await {
                            // Cancel cancelled.
                            continue;
                        }
                        return Err(UserAbort);
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

    use crate::hal::testing::{TestingRandom, TestingUi};
    use alloc::boxed::Box;
    use alloc::collections::VecDeque;
    use alloc::string::String;

    fn bruteforce_lastword(mnemonic: &[&str]) -> Vec<zeroize::Zeroizing<String>> {
        let mut result = Vec::new();
        for i in 0..BIP39_WORDLIST_LEN {
            let word = crate::bip39::get_word(i).unwrap();
            let mut m = mnemonic.to_vec();
            m.push(&word);
            if crate::bip39::mnemonic_to_seed(&m.join(" ")).is_ok() {
                result.push(word);
            }
        }
        result
    }

    #[test]
    fn test_create_random_unique_words() {
        let mut random = TestingRandom::new();
        // Place the target at index 2 in a 5-entry list.
        TestingUi::prepare_mnemonic_quiz_word_random(&mut random);
        let (correct_idx, choices) =
            create_random_unique_words(&mut random, "zoo", NUM_RANDOM_WORDS);
        assert_eq!(correct_idx, 2);
        assert_eq!(
            as_str_vec(&choices),
            vec!["abandon", "ability", "zoo", "able", "about"]
        );

        let mut unique = as_str_vec(&choices);
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), choices.len());
    }

    #[async_test::test]
    async fn test_show_and_confirm_mnemonic() {
        let words: Vec<&str> = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide"
            .split(' ')
            .collect();
        let mut ui = TestingUi::new();
        let mut random = TestingRandom::new();
        ui.prepare_show_and_confirm_mnemonic(&mut random, words.len());

        let result = show_and_confirm_mnemonic(&mut ui, &mut random, &words).await;
        assert!(result.is_ok());
        TestingUi::assert_show_and_confirm_mnemonic_screens(&ui.screens, &words);
    }

    #[async_test::test]
    async fn test_get() {
        let words: Vec<&str> = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide"
            .split(' ')
            .collect();
        let mut ui = TestingUi::new();
        ui.prepare_get_mnemonic_24_words(&words);

        let result = get(&mut ui).await;
        assert!(result.is_ok());
        let mnemonic = match result {
            Ok(mnemonic) => mnemonic,
            Err(_) => panic!("unexpected user abort"),
        };
        assert_eq!(mnemonic.as_str(), words.join(" "));
    }

    #[async_test::test]
    async fn test_get_retries_invalid_word() {
        let words: Vec<&str> = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide"
            .split(' ')
            .collect();
        let mut entries: VecDeque<String> = ["notaword"]
            .into_iter()
            .chain(words[..23].iter().copied())
            .map(String::from)
            .collect();
        let last_word = words[23];
        let mut ui = TestingUi::new();

        ui.set_trinary_choice(Box::new(
            |message, label_left, label_middle, label_right| {
                assert_eq!(message, "How many words?");
                assert_eq!(label_left, Some("12"));
                assert_eq!(label_middle, None);
                assert_eq!(label_right, Some("24"));
                TrinaryChoice::Right
            },
        ));
        ui.set_menu(Box::new(move |menu_words, title| {
            assert_eq!(title, Some("24 of 24"));
            Ok(menu_words
                .iter()
                .position(|word| *word == last_word)
                .unwrap()
                .try_into()
                .unwrap())
        }));
        ui.set_enter_string(Box::new(move |params| {
            assert!(params.wordlist.is_some());
            assert!(params.title.ends_with(" of 24"));
            Ok(entries.pop_front().unwrap())
        }));

        let result = get(&mut ui).await;
        assert!(result.is_ok());
        let mnemonic = match result {
            Ok(mnemonic) => mnemonic,
            Err(_) => panic!("unexpected user abort"),
        };
        assert_eq!(mnemonic.as_str(), words.join(" "));
        assert!(ui.screens.iter().any(
            |screen| matches!(screen, crate::hal::testing::Screen::Status {
                    title,
                    success: false,
                } if title == "Invalid word\nTry again")
        ));
    }

    /// Scripts `enter_wordlist_word` with a fixed (expected title, response) sequence, as a UI
    /// with dedicated back/cancel controls (BitBox03) produces it.
    fn script_word_entries(
        ui: &mut TestingUi<'_>,
        script: Vec<(String, Result<String, WordlistEntryAbort>)>,
    ) {
        let mut script: VecDeque<(String, Result<String, WordlistEntryAbort>)> =
            script.into_iter().collect();
        ui.set_enter_wordlist_word(Box::new(move |params| {
            let (expected_title, response) = script.pop_front().expect("unexpected word entry");
            assert_eq!(params.title, expected_title);
            assert!(params.wordlist.is_some());
            response
        }));
    }

    fn word(s: &str) -> Result<String, WordlistEntryAbort> {
        Ok(String::from(s))
    }

    #[async_test::test]
    async fn test_get_back_goes_straight_to_previous_word() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        // No menu is configured: if the workflow showed its "Choose" menu, the test would panic.

        let first_eleven = [
            "boring", "portion", "dish", "oyster", "truth", "pigeon", "viable", "emerge", "sort",
            "crash", "wire",
        ];
        let last_word = crate::bip39::get_word(lastword_choices(&first_eleven)[0]).unwrap();
        let mut script = vec![
            (String::from("1 of 12"), word("boring")),
            (String::from("2 of 12"), word("mistake")),
            // A dedicated back control goes straight back to the previous word, which can then
            // be replaced.
            (String::from("3 of 12"), Err(WordlistEntryAbort::Back)),
            (String::from("2 of 12"), word("portion")),
        ];
        for (i, w) in first_eleven.iter().enumerate().skip(2) {
            script.push((format!("{} of 12", i + 1), word(w)));
        }
        script.push((String::from("12 of 12"), word(&last_word)));
        script_word_entries(&mut ui, script);

        let result = get(&mut ui).await;
        let Ok(mnemonic) = result else {
            panic!("unexpected user abort");
        };
        let mut expected: Vec<&str> = first_eleven.to_vec();
        expected.push(&last_word);
        assert_eq!(mnemonic.as_str(), expected.join(" "));
        // Going back never asks for cancel confirmation.
        assert!(!ui.contains_confirm("Restore", "Cancel restore?"));
    }

    #[async_test::test]
    async fn test_get_cancel_asks_to_confirm() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words

        // A dedicated cancel control goes straight to the "Cancel restore?" confirmation (the
        // testing UI accepts confirms by default), without the "Choose" menu.
        script_word_entries(
            &mut ui,
            vec![
                (String::from("1 of 12"), word("boring")),
                (String::from("2 of 12"), Err(WordlistEntryAbort::Cancel)),
            ],
        );

        let result = get(&mut ui).await;
        assert!(result.is_err(), "confirmed cancel must abort the restore");
        assert!(ui.contains_confirm("Restore", "Cancel restore?"));
    }

    #[async_test::test]
    async fn test_get_cancel_rejected_continues() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        // Screens: 0 = "Enter 12 words" status, 1 = the "Cancel restore?" confirm — reject it.
        ui.abort_nth(1);

        let first_eleven = [
            "boring", "mistake", "dish", "oyster", "truth", "pigeon", "viable", "emerge", "sort",
            "crash", "wire",
        ];
        let last_word = crate::bip39::get_word(lastword_choices(&first_eleven)[0]).unwrap();
        let mut script = vec![
            (String::from("1 of 12"), word("boring")),
            (String::from("2 of 12"), Err(WordlistEntryAbort::Cancel)),
            // Rejecting the cancel confirmation returns to the same word.
            (String::from("2 of 12"), word("mistake")),
        ];
        for (i, w) in first_eleven.iter().enumerate().skip(2) {
            script.push((format!("{} of 12", i + 1), word(w)));
        }
        script.push((String::from("12 of 12"), word(&last_word)));
        script_word_entries(&mut ui, script);

        let result = get(&mut ui).await;
        let Ok(mnemonic) = result else {
            panic!("unexpected user abort");
        };
        let mut expected: Vec<&str> = first_eleven.to_vec();
        expected.push(&last_word);
        assert_eq!(mnemonic.as_str(), expected.join(" "));
    }

    #[async_test::test]
    async fn test_get_back_on_first_word_asks_to_confirm_cancel() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words

        // In the first word there is no previous word: back acts as a cancel request.
        script_word_entries(
            &mut ui,
            vec![(String::from("1 of 12"), Err(WordlistEntryAbort::Back))],
        );

        let result = get(&mut ui).await;
        assert!(result.is_err(), "confirmed cancel must abort the restore");
        assert!(ui.contains_confirm("Restore", "Cancel restore?"));
    }

    /// Scripts `enter_string` with a fixed (expected title, response) sequence, as a UI with a
    /// single abort control (BitBox02) produces it — reaching the workflow through the
    /// `enter_wordlist_word` fallback that maps the abort to `Unspecified`.
    fn script_single_abort_word_entries(
        ui: &mut TestingUi<'_>,
        script: Vec<(String, Result<String, UserAbort>)>,
    ) {
        let mut script: VecDeque<(String, Result<String, UserAbort>)> =
            script.into_iter().collect();
        ui.set_enter_string(Box::new(move |params| {
            let (expected_title, response) = script.pop_front().expect("unexpected word entry");
            assert_eq!(params.title, expected_title);
            assert!(params.wordlist.is_some());
            response
        }));
    }

    /// Scripts the "Choose" menu with fixed responses, asserting its exact contents.
    fn script_choose_menu(ui: &mut TestingUi<'_>, responses: Vec<Result<u8, UserAbort>>) {
        let mut responses: VecDeque<Result<u8, UserAbort>> = responses.into_iter().collect();
        ui.set_menu(Box::new(move |menu_words, title| {
            assert_eq!(menu_words, ["Edit previous word", "Cancel restore"]);
            assert_eq!(title, Some("Choose"));
            responses.pop_front().expect("unexpected menu")
        }));
    }

    /// BitBox02 parity: a single-control abort at a word > 1 shows the "Choose" menu; "Edit
    /// previous word" goes back one word, and cancelling the menu returns to the same word.
    #[async_test::test]
    async fn test_get_unspecified_abort_shows_choose_menu() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        script_choose_menu(
            &mut ui,
            vec![
                Ok(0),          // first abort: edit the previous word
                Err(UserAbort), // second abort: cancel the menu -> same word again
            ],
        );

        let first_eleven = [
            "portion", "mistake", "dish", "oyster", "truth", "pigeon", "viable", "emerge", "sort",
            "crash", "wire",
        ];
        let last_word = crate::bip39::get_word(lastword_choices(&first_eleven)[0]).unwrap();
        let mut script = vec![
            (String::from("1 of 12"), Ok(String::from("boring"))),
            (String::from("2 of 12"), Err(UserAbort)), // menu -> edit previous word
            (String::from("1 of 12"), Ok(String::from("portion"))),
            (String::from("2 of 12"), Err(UserAbort)), // menu -> cancelled -> same word
            (String::from("2 of 12"), Ok(String::from("mistake"))),
        ];
        for (i, w) in first_eleven.iter().enumerate().skip(2) {
            script.push((format!("{} of 12", i + 1), Ok(String::from(*w))));
        }
        script.push((String::from("12 of 12"), Ok(String::from(&*last_word))));
        script_single_abort_word_entries(&mut ui, script);

        let result = get(&mut ui).await;
        let Ok(mnemonic) = result else {
            panic!("unexpected user abort");
        };
        let mut expected: Vec<&str> = first_eleven.to_vec();
        expected.push(&last_word);
        assert_eq!(mnemonic.as_str(), expected.join(" "));
        assert!(!ui.contains_confirm("Restore", "Cancel restore?"));
    }

    /// BitBox02 parity: picking "Cancel restore" in the "Choose" menu asks for confirmation and
    /// aborts.
    #[async_test::test]
    async fn test_get_unspecified_abort_menu_cancel_confirms() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        script_choose_menu(&mut ui, vec![Ok(1)]);
        script_single_abort_word_entries(
            &mut ui,
            vec![
                (String::from("1 of 12"), Ok(String::from("boring"))),
                (String::from("2 of 12"), Err(UserAbort)),
            ],
        );

        let result = get(&mut ui).await;
        assert!(result.is_err(), "confirmed cancel must abort the restore");
        assert!(ui.contains_confirm("Restore", "Cancel restore?"));
    }

    /// BitBox02 parity: a single-control abort in the first word skips the menu (there is no
    /// previous word) and goes straight to the cancel confirmation.
    #[async_test::test]
    async fn test_get_unspecified_abort_first_word_goes_to_cancel() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        // No menu is configured: showing it would panic the test.
        script_single_abort_word_entries(&mut ui, vec![(String::from("1 of 12"), Err(UserAbort))]);

        let result = get(&mut ui).await;
        assert!(result.is_err(), "confirmed cancel must abort the restore");
        assert!(ui.contains_confirm("Restore", "Cancel restore?"));
    }

    /// Back from the restricted last-word candidate screen goes straight to the previous word,
    /// like from any other word.
    #[async_test::test]
    async fn test_get_back_at_last_word_goes_to_previous_word() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words
        // No menu is configured: showing it would panic the test.

        let first_eleven = [
            "boring", "mistake", "dish", "oyster", "truth", "pigeon", "viable", "emerge", "sort",
            "crash", "wire",
        ];
        let last_word = crate::bip39::get_word(lastword_choices(&first_eleven)[0]).unwrap();
        let mut script: Vec<(String, Result<String, WordlistEntryAbort>)> = first_eleven
            .iter()
            .enumerate()
            .map(|(i, w)| (format!("{} of 12", i + 1), word(w)))
            .collect();
        script.push((String::from("12 of 12"), Err(WordlistEntryAbort::Back)));
        script.push((String::from("11 of 12"), word("wire")));
        script.push((String::from("12 of 12"), word(&last_word)));
        script_word_entries(&mut ui, script);

        let result = get(&mut ui).await;
        let Ok(mnemonic) = result else {
            panic!("unexpected user abort");
        };
        let mut expected: Vec<&str> = first_eleven.to_vec();
        expected.push(&last_word);
        assert_eq!(mnemonic.as_str(), expected.join(" "));
        assert!(!ui.contains_confirm("Restore", "Cancel restore?"));
    }

    /// Cancel from the restricted last-word candidate screen asks "Cancel restore?" directly.
    #[async_test::test]
    async fn test_get_cancel_at_last_word_confirms() {
        let mut ui = TestingUi::new();
        ui.set_trinary_choice(Box::new(|_, _, _, _| TrinaryChoice::Left)); // 12 words

        let first_eleven = [
            "boring", "mistake", "dish", "oyster", "truth", "pigeon", "viable", "emerge", "sort",
            "crash", "wire",
        ];
        let mut script: Vec<(String, Result<String, WordlistEntryAbort>)> = first_eleven
            .iter()
            .enumerate()
            .map(|(i, w)| (format!("{} of 12", i + 1), word(w)))
            .collect();
        script.push((String::from("12 of 12"), Err(WordlistEntryAbort::Cancel)));
        script_word_entries(&mut ui, script);

        let result = get(&mut ui).await;
        assert!(result.is_err(), "confirmed cancel must abort the restore");
        assert!(ui.contains_confirm("Restore", "Cancel restore?"));
    }

    #[test]
    fn test_lastword_choices() {
        // 23 words

        assert_eq!(
            &as_str_vec(&bruteforce_lastword(&["violin"; 23])),
            &[
                "boss", "coyote", "dry", "habit", "panel", "regular", "speed", "winter"
            ]
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
