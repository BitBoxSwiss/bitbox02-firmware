// SPDX-License-Identifier: Apache-2.0

use util::bip32::HARDENED;

const ACCOUNT_MIN: u32 = HARDENED;
const ACCOUNT_MAX: u32 = HARDENED + 99; // 100 accounts

const PURPOSE_BIP44: u32 = 44 + HARDENED;
const COIN_SOLANA: u32 = 501 + HARDENED;
const PHANTOM_CHANGE: u32 = HARDENED;

pub struct Error;

pub fn validate(keypath: &[u32]) -> Result<(), Error> {
    if let &[PURPOSE_BIP44, COIN_SOLANA, account, PHANTOM_CHANGE] = keypath {
        if (ACCOUNT_MIN..=ACCOUNT_MAX).contains(&account) {
            return Ok(());
        }
    }
    Err(Error)
}

pub fn is_valid_keypath(keypath: &[u32]) -> bool {
    validate(keypath).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED]).is_ok());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, 99 + HARDENED, HARDENED]).is_ok());

        assert!(validate(&[]).is_err());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, HARDENED]).is_err());
        assert!(validate(&[44 + HARDENED, 500 + HARDENED, HARDENED, HARDENED]).is_err());
        assert!(validate(&[45 + HARDENED, 501 + HARDENED, HARDENED, HARDENED]).is_err());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, HARDENED - 1, HARDENED]).is_err());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, 100 + HARDENED, HARDENED]).is_err());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, HARDENED, 0]).is_err());
        assert!(validate(&[44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED, 0]).is_err());
    }
}
