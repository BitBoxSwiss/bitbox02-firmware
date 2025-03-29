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

use util::bip32::HARDENED;

const BIP44_ACCOUNT_MIN: u32 = HARDENED;
const BIP44_ACCOUNT_MAX: u32 = HARDENED + 99; // 100 accounts
const BIP44_ADDRESS_MAX: u32 = 9999; // 10k addresses

const BIP44_PURPOSE_SHELLEY: u32 = 1852 + HARDENED;
const BIP44_COIN: u32 = 1815 + HARDENED;
const BIP44_STAKE_ROLE: u32 = 2;
const BIP44_STAKE_ADDRESS: u32 = 0;

pub struct Error;

fn check_account(account: u32) -> Result<(), Error> {
    if (BIP44_ACCOUNT_MIN..=BIP44_ACCOUNT_MAX).contains(&account) {
        Ok(())
    } else {
        Err(Error)
    }
}

fn check_address(address: u32) -> Result<(), Error> {
    if address <= BIP44_ADDRESS_MAX {
        Ok(())
    } else {
        Err(Error)
    }
}

/// Validates a keypath to be
/// m/1852'/1815'/account, where account between 0' and 99'.
///
/// See: https://cips.cardano.org/cips/cip1852/
pub fn validate_account_shelley(keypath: &[u32]) -> Result<(), Error> {
    if let &[BIP44_PURPOSE_SHELLEY, BIP44_COIN, account] = keypath {
        check_account(account)?;
        return Ok(());
    }
    Err(Error)
}

/// Validates that the prefix (all but last two elements) of the keypath is a valid shelley account
/// payment keypath (m/1852'/1815'/account/role/address, where role is 0 or 1 (receive vs change) and address is less than 10000.
///
/// See: https://cips.cardano.org/cips/cip1852/
pub fn validate_address_shelley_payment(
    keypath: &[u32],
    bip44_account: Option<u32>,
) -> Result<(), Error> {
    if let &[BIP44_PURPOSE_SHELLEY, BIP44_COIN, account, role, address] = keypath {
        if bip44_account.is_some_and(|a| a != account) {
            return Err(Error);
        }
        check_account(account)?;
        check_address(address)?;
        if role <= 1 {
            return Ok(());
        }
    }
    Err(Error)
}

/// Validates that the prefix (all but last two elements) of the keypath is a valid shelley account
/// payment keypath (m/1852'/1815'/account/role/0, where role is 2 (stake). The last element, the
/// address index, is fixed to zero to avoid ransom attacks (address search space is
/// `BIP44_ADDRESS_MAX` for the payment key, would be squared in combination with the staking key).
///
/// See: https://github.com/cardano-foundation/CIPs/blob/master/CIP-1852/CIP-1852.md and especially
/// https://github.com/cardano-foundation/CIPs/blob/master/CIP-0011/CIP-0011.md
pub fn validate_address_shelley_stake(
    keypath: &[u32],
    bip44_account: Option<u32>,
) -> Result<(), Error> {
    if let &[BIP44_PURPOSE_SHELLEY, BIP44_COIN, account, BIP44_STAKE_ROLE, BIP44_STAKE_ADDRESS] =
        keypath
    {
        if bip44_account.is_some_and(|a| a != account) {
            return Err(Error);
        }
        check_account(account)?;
        return Ok(());
    }
    Err(Error)
}

/// Validate both keypaths and also check that they have the same first three elements, to ensure
/// they are from the same account.
pub fn validate_address_shelley(
    keypath_payment: &[u32],
    keypath_stake: &[u32],
    bip44_account: Option<u32>,
) -> Result<(), Error> {
    validate_address_shelley_payment(keypath_payment, bip44_account)?;
    validate_address_shelley_stake(keypath_stake, bip44_account)?;
    if keypath_payment[..3] != keypath_stake[..3] {
        return Err(Error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_account_shelley() {
        let purpose = 1852 + HARDENED;
        let coin = 1815 + HARDENED;
        assert!(validate_account_shelley(&[]).is_err());
        assert!(validate_account_shelley(&[purpose]).is_err());
        assert!(validate_account_shelley(&[purpose, coin]).is_err());
        assert!(validate_account_shelley(&[purpose, coin, HARDENED, HARDENED]).is_err());

        for account in 0..100 {
            assert!(validate_account_shelley(&[purpose, coin, account + HARDENED]).is_ok());
        }
        // account too high
        assert!(validate_account_shelley(&[purpose, coin, 100 + HARDENED]).is_err());
        // invalid coin
        assert!(validate_account_shelley(&[purpose, 1852 + HARDENED, 0 + HARDENED]).is_err());
    }

    #[test]
    fn test_validate_address_shelley_payment() {
        let purpose = 1852 + HARDENED;
        let coin = 1815 + HARDENED;
        let account = 99 + HARDENED;

        assert!(validate_address_shelley_payment(&[purpose, coin, account, 0, 0], None).is_ok());

        // force account
        assert!(
            validate_address_shelley_payment(&[purpose, coin, account, 0, 0], Some(account))
                .is_ok()
        );

        // force account, mismatch
        assert!(validate_address_shelley_payment(
            &[purpose, coin, account, 0, 0],
            Some(50 + HARDENED)
        )
        .is_err());

        // high address
        assert!(validate_address_shelley_payment(&[purpose, coin, account, 0, 9999], None).is_ok());

        // invalid, too high address
        assert!(
            validate_address_shelley_payment(&[purpose, coin, account, 0, 10000], None).is_err()
        );

        // valid change
        assert!(validate_address_shelley_payment(&[purpose, coin, account, 1, 0], None).is_ok());

        // invalid change
        assert!(validate_address_shelley_payment(&[purpose, coin, account, 2, 0], None).is_err());

        // wrong purpose
        assert!(
            validate_address_shelley_payment(&[1853 + HARDENED, coin, account, 0, 0], None)
                .is_err()
        );

        // wrong coin
        assert!(
            validate_address_shelley_payment(&[purpose, coin + 1, account, 0, 0], None).is_err()
        );

        // account too high
        assert!(
            validate_address_shelley_payment(&[purpose, coin, 100 + HARDENED, 0, 0], None).is_err()
        );

        // account too low
        assert!(
            validate_address_shelley_payment(&[purpose, coin, HARDENED - 1, 0, 0], None).is_err()
        );
    }

    #[test]
    fn test_validate_address_shelley_stake() {
        let purpose = 1852 + HARDENED;
        let coin = 1815 + HARDENED;
        let account = 99 + HARDENED;

        assert!(validate_address_shelley_stake(&[purpose, coin, account, 2, 0], None).is_ok());

        // force account
        assert!(
            validate_address_shelley_stake(&[purpose, coin, account, 2, 0], Some(account)).is_ok()
        );

        // force account, mismatch
        assert!(validate_address_shelley_stake(
            &[purpose, coin, account, 2, 0],
            Some(50 + HARDENED)
        )
        .is_err());

        // invalid address
        assert!(validate_address_shelley_stake(&[purpose, coin, account, 2, 1], None).is_err());

        // invalid roles
        assert!(validate_address_shelley_stake(&[purpose, coin, account, 0, 0], None).is_err());
        assert!(validate_address_shelley_stake(&[purpose, coin, account, 1, 0], None).is_err());

        // wrong purpose
        assert!(
            validate_address_shelley_stake(&[1853 + HARDENED, coin, account, 2, 0], None).is_err()
        );

        // wrong coin
        assert!(validate_address_shelley_stake(&[purpose, coin + 1, account, 2, 0], None).is_err());

        // account too high
        assert!(
            validate_address_shelley_stake(&[purpose, coin, 100 + HARDENED, 2, 0], None).is_err()
        );

        // account too low
        assert!(
            validate_address_shelley_stake(&[purpose, coin, HARDENED - 1, 2, 0], None).is_err()
        );
    }

    #[test]
    fn test_validate_address_shelley() {
        let purpose = 1852 + HARDENED;
        let coin = 1815 + HARDENED;
        let account = 99 + HARDENED;

        assert!(validate_address_shelley(
            &[purpose, coin, account, 0, 0],
            &[purpose, coin, account, 2, 0],
            None
        )
        .is_ok());

        assert!(validate_address_shelley(
            &[purpose, coin, account, 0, 100],
            &[purpose, coin, account, 2, 0],
            None
        )
        .is_ok());

        // payment key is a change key
        assert!(validate_address_shelley(
            &[purpose, coin, account, 1, 100],
            &[purpose, coin, account, 2, 0],
            None
        )
        .is_ok());

        // force account
        assert!(validate_address_shelley(
            &[purpose, coin, account, 0, 0],
            &[purpose, coin, account, 2, 0],
            Some(account),
        )
        .is_ok());

        // force account, mismatch
        assert!(validate_address_shelley(
            &[purpose, coin, account, 0, 0],
            &[purpose, coin, account, 2, 0],
            Some(50 + HARDENED),
        )
        .is_err());

        // different accounts
        assert!(validate_address_shelley(
            &[purpose, coin, 98 + HARDENED, 0, 100],
            &[purpose, coin, 99 + HARDENED, 2, 0],
            None
        )
        .is_err());

        // stake address index is not 0
        assert!(validate_address_shelley(
            &[purpose, coin, account, 0, 100],
            &[purpose, coin, account, 2, 1],
            None
        )
        .is_err());
    }
}
