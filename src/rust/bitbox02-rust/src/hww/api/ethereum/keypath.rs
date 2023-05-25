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

use super::params::Params;
use super::Error;
use crate::workflow::confirm;

use util::bip32::HARDENED;

const PURPOSE: u32 = 44 + HARDENED;
const COIN_MAINNET: u32 = 60 + HARDENED;
const COIN_TESTNET: u32 = 1 + HARDENED;

const ACCOUNT_MAX: u32 = 99; // 100 accounts
const ACCOUNT_MIN_H: u32 = 0 + HARDENED;
const ACCOUNT_MAX_H: u32 = ACCOUNT_MAX + HARDENED;

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
        return Ok(confirm::confirm(&confirm::Params {
            title,
            body: &body,
            title_autowrap: true,
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?);
    }
    Ok(())
}

/// Does limit checks the keypath, whitelisting bip44 purpose, account and change.
/// Allows the following xpubs:
/// For BitBoxApp, MyEtherWalelt: m'/44'/60'/0'/0 and m'/44'/1'/0'/0.
/// For Ledger Live compatibility: m/44'/60'/account' and m/44'/1'/account'
/// @return true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_xpub(keypath: &[u32]) -> bool {
    match keypath {
        // BitBoxApp, MyEtherWallet
        [PURPOSE, COIN_MAINNET | COIN_TESTNET, HARDENED, 0] => true,
        // Ledger Live
        [PURPOSE, COIN_MAINNET | COIN_TESTNET, ACCOUNT_MIN_H..=ACCOUNT_MAX_H] => true,
        _ => false,
    }
}

/// Does limit checks the keypath.
/// Returns true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_address(keypath: &[u32]) -> bool {
    match keypath {
        // BitBoxApp, MyEtherWallet
        [PURPOSE, COIN_MAINNET | COIN_TESTNET, HARDENED, 0, 0..=ACCOUNT_MAX] => true,
        // Ledger Live
        [PURPOSE, COIN_MAINNET | COIN_TESTNET, ACCOUNT_MIN_H..=ACCOUNT_MAX_H, 0, 0] => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_keypath_xpub() {
        assert!(is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0
        ]));
        assert!(is_valid_keypath_xpub(&[
            44 + HARDENED,
            1 + HARDENED,
            0 + HARDENED,
            0
        ]));
        // wrong coin.
        assert!(!is_valid_keypath_xpub(&[
            44 + HARDENED,
            0 + HARDENED,
            0 + HARDENED,
            0
        ]));
        // too short
        assert!(!is_valid_keypath_xpub(&[44 + HARDENED, 60 + HARDENED,]));
        // too long
        assert!(!is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0,
            0
        ]));

        // Ledger Live
        assert!(is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
        ]));
        assert!(is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            99 + HARDENED,
        ]));
        // account too high
        assert!(!is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            100 + HARDENED,
        ]));
    }

    #[test]
    fn test_is_valid_keypath_address() {
        // 100 good paths.
        for account in 0..100 {
            assert!(is_valid_keypath_address(&[
                44 + HARDENED,
                60 + HARDENED,
                0 + HARDENED,
                0,
                account
            ]));
            assert!(is_valid_keypath_address(&[
                44 + HARDENED,
                1 + HARDENED,
                0 + HARDENED,
                0,
                account
            ]));
            // wrong coin
            assert!(!is_valid_keypath_address(&[
                44 + HARDENED,
                0 + HARDENED,
                0 + HARDENED,
                0,
                account
            ]));
        }
        // account too high
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0,
            100
        ]));

        // too short
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0
        ]));
        // too long
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0,
            0,
            0
        ]));
        // tweak keypath elements
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED + 1,
            60 + HARDENED,
            0 + HARDENED,
            0,
            0
        ]));
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED + 1,
            0 + HARDENED,
            0,
            0
        ]));
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0 + 1,
            0
        ]));

        // Ledger Live

        // 100 good paths.
        for account in 0..100 {
            assert!(is_valid_keypath_address(&[
                44 + HARDENED,
                60 + HARDENED,
                account + HARDENED,
                0,
                0
            ]));
        }
        // account too high
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            100 + HARDENED,
            0,
            0
        ]));
        // tweak keypath elements
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED + 1,
            60 + HARDENED,
            1 + HARDENED,
            0,
            0
        ]));
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED + 1,
            1 + HARDENED,
            0,
            0
        ]));
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            1 + HARDENED,
            0 + 1,
            0
        ]));
        assert!(!is_valid_keypath_address(&[
            44 + HARDENED,
            60 + HARDENED,
            1 + HARDENED,
            0,
            0 + 1,
        ]));
    }
}
