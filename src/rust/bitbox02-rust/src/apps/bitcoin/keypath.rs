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

use crate::pb;

use util::bip32::HARDENED;

const BIP44_ACCOUNT_MIN: u32 = HARDENED;
const BIP44_ACCOUNT_MAX: u32 = HARDENED + 99; // 100 accounts

/// Validates a keypath to be
/// m/expected_purpose/expected_coin/account, where account between 0' and 99'.
pub fn validate_account(
    keypath: &[u32],
    expected_purpose: u32,
    expected_coin: u32,
) -> Result<(), ()> {
    if let [purpose, coin, account] = *keypath {
        if purpose == expected_purpose
            && coin == expected_coin
            && account >= BIP44_ACCOUNT_MIN
            && account <= BIP44_ACCOUNT_MAX
        {
            return Ok(());
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_account() {
        assert!(validate_account(&[], 0, 0).is_err());
        assert!(validate_account(&[0], 0, 0).is_err());
        assert!(validate_account(&[0, 0], 0, 0).is_err());
        assert!(validate_account(&[0, 0, 0], 0, 0).is_err());
        assert!(validate_account(&[0, 0, 0, 0], 0, 0).is_err());

        for account in 0..100 {
            assert!(validate_account(&[0, 0, account + HARDENED], 0, 0).is_ok());
        }
        assert!(validate_account(&[0, 0, 100 + HARDENED], 0, 0).is_err());

        assert!(validate_account(
            &[84 + HARDENED, 1 + HARDENED, 1 + HARDENED],
            84 + HARDENED,
            1 + HARDENED,
        )
        .is_ok());

        // Too many elements.
        assert!(validate_account(
            &[84 + HARDENED, 1 + HARDENED, 1 + HARDENED, 1 + HARDENED],
            84 + HARDENED,
            1 + HARDENED,
        )
        .is_err());
    }
}
