// Copyright 2024 Shift Crypto AG
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
use crate::workflow::confirm;

use pb::response::Response;

use crate::workflow::{mnemonic, status, unlock};
use bitbox02::keystore;
use rand_chacha::rand_core::SeedableRng;
use sharks::{Share, Sharks};

/// Handle the ShowShamir API call. This shows the seed shards encoded as
/// 15/27 BIP39 English words. Afterwards, for each word, the user
/// is asked to pick the right word among 5 words, to check if they
/// wrote it down correctly.
pub async fn process() -> Result<Response, Error> {
    if bitbox02::memory::is_initialized() {
        unlock::unlock_keystore("Unlock device", unlock::CanCancel::Yes).await?;
    }
    // Set a minimum threshold of 2 shares
    const SHARES_THRESHOLD: u8 = 2;
    const SHARES_MAX: usize = 3;
    let sharks = Sharks(SHARES_THRESHOLD);

    let mut seed = [0u8; 32];
    // FIXME: this makes rand each shard generation. Should we use factory rand instead?
    bitbox02::random::mcu_32_bytes(&mut seed);
    let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    let seed = bitbox02::keystore::copy_seed()?;
    let dealer = sharks.dealer_rng(&seed, &mut rng);
    // bitbox02::print_stdout(&format!("seed: {}, len: {}\n", hex::encode(seed.clone()), seed.len()));
    confirm::confirm(&confirm::Params {
        title: "Warning",
        body: "DO NOT share your\nrecovery words with\nanyone!",
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    // Get 3 shares
    let shares: Vec<Share> = dealer.take(SHARES_MAX).collect();
    for (i, s) in shares.iter().enumerate() {
        let share_slice = Vec::from(s);
        // Sharks add a single byte to enumerate the shard. We add three bytes in front of it to
        // get an additional 4 bytes to the seed and be compliant with BIP39.
        let mut share_extended = vec![0, 0, 0];
        share_extended.extend_from_slice(&share_slice);
        // bitbox02::print_stdout(&format!("Share: {}, len: {}\n", hex::encode(share_extended.clone()), share_extended.len()));
        let mnemonic_sentence = keystore::get_bip39_mnemonic_from_bytes(share_extended)?;

        confirm::confirm(&confirm::Params {
            title: &format!("Recovery\nwords {}/{}", i + 1, SHARES_MAX),
            body: "Please write down\nthe following words",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

        let words: Vec<&str> = mnemonic_sentence.split(' ').collect();
        mnemonic::show_and_confirm_mnemonic(&words).await?;
    }

    bitbox02::memory::set_initialized().or(Err(Error::Memory))?;

    status::status("Backup created", true).await;

    Ok(Response::Success(pb::Success {}))
}
