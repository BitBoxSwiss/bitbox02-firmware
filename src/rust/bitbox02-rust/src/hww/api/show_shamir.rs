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
use crate::workflow::confirm;

use pb::response::Response;

use crate::workflow::{mnemonic, status, unlock};
use bitbox02::keystore;
use sharks::{ Sharks, Share };
use rand_chacha::rand_core::SeedableRng;

/// Handle the ShowShamir API call. This shows the seed shards encoded as
/// 12/18/24 BIP39 English words. Afterwards, for each word, the user
/// is asked to pick the right word among 5 words, to check if they
/// wrote it down correctly.
pub async fn process() -> Result<Response, Error> {
    if bitbox02::memory::is_initialized() {
        unlock::unlock_keystore("Unlock device", unlock::CanCancel::Yes).await?;
    }
    // Set a minimum threshold of 10 shares
    let sharks = Sharks(3);

    // // Obtain an iterator over the shares for secret [1, 2, 3, 4]
    // // TODO: use RNG from SE?
    let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);

    let seed = bitbox02::keystore::copy_seed()?;
    let dealer = sharks.dealer_rng(&seed, &mut rng);
    // let dealer = sharks.dealer_rng(&[1,2,3,4], &mut rng);
    // Get 3 shares
    let mut shares: Vec<Share> = dealer.take(3).collect();
    for s in shares {

    // shares.remove(1);
    // shares.remove(0);
    // Recover the original secret!
    // bitbox02::print_stdout("Recovering...\n");
    // let secret = sharks.recover(shares.as_slice());
    // match secret {
    //     Err(e) => bitbox02::print_stdout(&format!("Error {}\n", e)),
    //     Ok(_) => bitbox02::print_stdout("***test ok\n"),
    // }
    // assert_eq!(*secret.unwrap(), *seed);
    let mnemonic_sentence = keystore::get_bip39_mnemonic_from_bytes(Vec::from(&s).as_ptr(), seed.len())?;
    
    // let mnemonic_sentence = keystore::get_bip39_mnemonic()?;

    confirm::confirm(&confirm::Params {
        title: "Warning",
        body: "DO NOT share your\nrecovery words with\nanyone!",
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title: "Recovery\nwords",
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
