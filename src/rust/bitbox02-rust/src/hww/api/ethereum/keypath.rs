// Copyright 2021 Shift Crypto AG
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

use super::Error;
use crate::workflow::confirm;
use bitbox02::app_eth::Params;

/// If the second element of `keypath` does not match the expected bip44 coin value for the given
/// coin, we warn the user about an unusual keypath.
///
/// The keypath is already assumed to be validated and that the second element is one of the
/// Ethereum bip44 values, e.g. `60'` or `1'`.
///
/// A warning suffices so the user does not accidentally send e.g. mainnet coins to a testnet path
/// (m/44'/1'/...). It is safe to make a transaction on the 'wrong' keypath as the chain id is
/// unique and part of the transaction sighash.
pub async fn warn_unusual_keypath(
    params: &Params,
    title: &str,
    keypath: &[u32],
) -> Result<(), Error> {
    if keypath.len() < 2 {
        return Err(Error::InvalidInput);
    }
    if keypath[1] != params.bip44_coin {
        let body = format!(
            "Unusual keypath warning: {}. Proceed only if you know what you are doing.",
            util::bip32::to_string(keypath)
        );
        let params = confirm::Params {
            title,
            body: &body,
            scrollable: true,
            ..Default::default()
        };
        return Ok(confirm::confirm(&params).await?);
    }
    Ok(())
}
