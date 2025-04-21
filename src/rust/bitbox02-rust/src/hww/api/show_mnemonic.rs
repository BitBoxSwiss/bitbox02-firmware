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

use alloc::vec::Vec;

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::workflow::{confirm, mnemonic, unlock};

use bitbox02::keystore;

/// Handle the ShowMnemonic API call. This shows the seed encoded as
/// 12/18/24 BIP39 English words. Afterwards, for each word, the user
/// is asked to pick the right word among 5 words, to check if they
/// wrote it down correctly.
pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    if bitbox02::memory::is_initialized() {
        unlock::unlock_keystore(hal, "Unlock device", unlock::CanCancel::Yes).await?;
    }

    let mnemonic_sentence = keystore::get_bip39_mnemonic()?;

    hal.ui()
        .confirm(&confirm::Params {
            title: "Warning",
            body: "DO NOT share your\nrecovery words with\nanyone!",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    hal.ui()
        .confirm(&confirm::Params {
            title: "Recovery\nwords",
            body: "Please write down\nthe following words",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let words: Vec<&str> = mnemonic_sentence.split(' ').collect();

    mnemonic::show_and_confirm_mnemonic(hal, &words).await?;

    bitbox02::memory::set_initialized().or(Err(Error::Memory))?;

    hal.ui().status("Backup created", true).await;
    Ok(Response::Success(pb::Success {}))
}
