use util::bip32::HARDENED;

static ACCOUNT_MAX: u32 = 99; // 100 accounts

/// Does limit checks the keypath, whitelisting bip44 purpose, account and change.
/// Returns true if the keypath is valid, false if it is invalid.
pub fn is_valid_keypath_address(keypath: &[u32], expected_coin: u32) -> bool {
    if keypath.len() != 5 {
        return false;
    }
    if &keypath[..4] != &[44 + HARDENED, expected_coin, 0 + HARDENED, 0] {
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
    fn test_is_valid_keypath_address() {
        assert!(!is_valid_keypath_address(&[1, 2, 3], 4));
    }
}
