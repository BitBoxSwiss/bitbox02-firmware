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
use crate::hal::Ui;
use crate::workflow::confirm;
use util::bip32::HARDENED;

const ACCOUNT_MAX: u32 = 99; // 100 accounts

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
    hal: &mut impl crate::hal::Hal,
    params: &Params,
    title: &str,
    keypath: &[u32],
) -> Result<(), Error> {
    if keypath.len() < 2 {
        return Err(Error::InvalidInput);
    }
    if keypath[1] != params.bip44_coin {
        let body = format!(
            "Warning: unusual keypath {}. Proceed only if you know what you are doing.",
            util::bip32::to_string(keypath)
        );
        return Ok(hal
            .ui()
            .confirm(&confirm::Params {
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
/// Only allows the well-known xpubs of m'/44'/60'/0'/0 and m'/44'/1'/0'/0 for now.
/// Since ethereum doesn't use the "change" path part it is always 0 and have become part of the
/// xpub keypath.
/// @return true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_xpub(keypath: &[u32]) -> bool {
    keypath.len() == 4
        && (keypath[..4] == [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0]
            || keypath[..4] == [44 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0])
}

/// Does limit checks the keypath, whitelisting bip44 purpose, account and change.
/// Returns true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_address(keypath: &[u32]) -> bool {
    if keypath.len() != 5 {
        return false;
    }
    if !is_valid_keypath_xpub(&keypath[..4]) {
        return false;
    }
    if keypath[4] > ACCOUNT_MAX {
        return false;
    }
    true
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
        assert!(!is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED
        ]));
        // too long
        assert!(!is_valid_keypath_xpub(&[
            44 + HARDENED,
            60 + HARDENED,
            0 + HARDENED,
            0,
            0
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
        for i in 0..4 {
            let mut keypath = [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
            keypath[i] += 1;
            assert!(!is_valid_keypath_address(&keypath));
        }
    }
}
