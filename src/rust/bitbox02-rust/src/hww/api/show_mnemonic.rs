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

extern crate alloc;
use alloc::vec::Vec;

use super::pb;
use super::Error;

use pb::response::Response;

use crate::workflow::{mnemonic, status, unlock};
use bitbox02::keystore;

const NUM_RANDOM_WORDS: u8 = 5;

fn create_random_unique_words(word: &str, length: u8) -> (u8, Vec<&str>) {
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
                return word;
            }

            // A random word everywhere else.
            // Loop until we get a unique word, we don't want repeated words in the list.
            loop {
                let idx = rand16() % keystore::BIP39_WORDLIST_LEN;
                if picked_indices.contains(&idx) {
                    continue;
                };
                let random_word = keystore::get_bip39_word(idx).unwrap();
                if random_word == word {
                    continue;
                }
                picked_indices.push(idx);
                return random_word;
            }
        })
        .collect();

    (index_word, result)
}

pub async fn process() -> Result<Response, Error> {
    unlock::unlock_keystore("Unlock device", unlock::CanCancel::Yes).await?;

    let mnemonic_sentence = keystore::get_bip39_mnemonic()?;

    let words: Vec<&str> = mnemonic_sentence.split(' ').collect();

    // Part 1) Scroll through words
    mnemonic::show_mnemonic(&words).await?;

    // Part 2) Confirm words
    for (word_idx, word) in words.iter().enumerate() {
        let title = format!("{:02}", word_idx + 1);
        let (correct_idx, mut choices) = create_random_unique_words(word, NUM_RANDOM_WORDS);
        choices.push("Back to\nrecovery words");
        let back_idx = (choices.len() - 1) as u8;
        loop {
            match mnemonic::confirm_word(&choices, &title).await? {
                selected_idx if selected_idx == correct_idx => break,
                selected_idx if selected_idx == back_idx => mnemonic::show_mnemonic(&words).await?,
                _ => status::status("Incorrect word\nTry again", false).await,
            }
        }
    }

    status::status("Success", true).await;
    Ok(Response::Success(pb::Success {}))
}
