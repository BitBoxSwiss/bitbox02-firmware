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
pub use pb::btc_script_config::multisig::ScriptType as MultisigScriptType;
pub use pb::btc_script_config::SimpleType;

const ALL_MULTISCRIPT_SCRIPT_TYPES: [MultisigScriptType; 2] =
    [MultisigScriptType::P2wsh, MultisigScriptType::P2wshP2sh];

const ALL_SIMPLE_SCRIPT_TYPES: [SimpleType; 3] =
    [SimpleType::P2wpkhP2sh, SimpleType::P2wpkh, SimpleType::P2tr];

use util::bip32::HARDENED;

const BIP44_ACCOUNT_MIN: u32 = HARDENED;
const BIP44_ACCOUNT_MAX: u32 = HARDENED + 99; // 100 accounts
const BIP44_ADDRESS_MAX: u32 = 9999; // 10k addresses

const PURPOSE_P2WPKH_P2SH: u32 = 49 + HARDENED;
const PURPOSE_P2WPKH: u32 = 84 + HARDENED;
const PURPOSE_P2TR: u32 = 86 + HARDENED;
const PURPOSE_MULTISIG: u32 = 48 + HARDENED;
const MULTISIG_SCRIPT_TYPE_P2WSH: u32 = 2 + HARDENED;
const MULTISIG_SCRIPT_TYPE_P2WSH_P2SH: u32 = 1 + HARDENED;

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
            && (BIP44_ACCOUNT_MIN..=BIP44_ACCOUNT_MAX).contains(&account)
        {
            return Ok(());
        }
    }
    Err(())
}

/// Validates a multisig keypath.
/// Supported:
/// - Electrum-style: m/48'/coin'/account'/script_type', where script_type is 1 for p2wsh-p2sh and 2
///   for p2wsh.
/// - Nunchuk-style: m/48'/coin'/account', independent of the script type.
fn validate_account_multisig(
    keypath: &[u32],
    expected_coin: u32,
    script_type: MultisigScriptType,
) -> Result<(), ()> {
    match keypath.len() {
        4 => {
            validate_account(&keypath[..3], PURPOSE_MULTISIG, expected_coin)?;
            let expected_bip44_script_type = match script_type {
                MultisigScriptType::P2wsh => MULTISIG_SCRIPT_TYPE_P2WSH,
                MultisigScriptType::P2wshP2sh => MULTISIG_SCRIPT_TYPE_P2WSH_P2SH,
            };
            if keypath[3] != expected_bip44_script_type {
                return Err(());
            }
            Ok(())
        }
        3 => validate_account(keypath, PURPOSE_MULTISIG, expected_coin),
        _ => Err(()),
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ReceiveSpend {
    Receive,
    Spend,
}

/// Validates that change is 0 or 1 and address is less than 10000.
fn validate_change_address(change: u32, address: u32, mode: ReceiveSpend) -> Result<(), ()> {
    if change <= 1 && (mode == ReceiveSpend::Spend || address <= BIP44_ADDRESS_MAX) {
        Ok(())
    } else {
        Err(())
    }
}

/// Validates that the address index is valid and that the account keypath prefix is not empty.
///
/// Note that the change index is not verified as in policies, it can be different to 0 and 1
/// (e.g. with a key `@0/<10;11>` it can be 10 or 11). This is verified by the user during policy
/// registration.
///
/// The account-level keypath is also not validated (except that it is not empty), as there is no
/// standard for policy keypaths, and the keypaths pointing to our own account-level xpubs are
/// verified by the user during policy registration.
pub fn validate_address_policy(keypath: &[u32], mode: ReceiveSpend) -> Result<(), ()> {
    if keypath.len() >= 2 {
        let (keypath_account, keypath_rest) = keypath.split_at(keypath.len() - 2);
        if !keypath_account.is_empty()
            && (mode == ReceiveSpend::Spend || keypath_rest[1] <= BIP44_ADDRESS_MAX)
        {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

/// Validates a singlesig keypath.
/// Supported:
/// - P2WPKH-P2SH: m/49'/coin'/account'
/// - P2WPKH: m/84'/coin'/account'
/// - P2TR: m/86'/coin'/account' (only if `taproot_support` is true)
pub fn validate_account_simple(
    keypath: &[u32],
    expected_coin: u32,
    script_type: SimpleType,
    taproot_support: bool,
) -> Result<(), ()> {
    if !taproot_support && script_type == SimpleType::P2tr {
        return Err(());
    }
    let bip44_purpose = match script_type {
        SimpleType::P2wpkhP2sh => PURPOSE_P2WPKH_P2SH,
        SimpleType::P2wpkh => PURPOSE_P2WPKH,
        SimpleType::P2tr => PURPOSE_P2TR,
    };
    validate_account(keypath, bip44_purpose, expected_coin)
}

/// Validates that the prefix (all but last two elements) of the keypath is a valid singlesig
/// account keypath and the last two elements are a valid change and receive element.
pub fn validate_address_simple(
    keypath: &[u32],
    expected_coin: u32,
    script_type: SimpleType,
    taproot_support: bool,
    mode: ReceiveSpend,
) -> Result<(), ()> {
    if keypath.len() >= 2 {
        let (keypath_account, keypath_rest) = keypath.split_at(keypath.len() - 2);
        validate_account_simple(keypath_account, expected_coin, script_type, taproot_support)?;
        validate_change_address(keypath_rest[0], keypath_rest[1], mode)
    } else {
        Err(())
    }
}

/// Checks if the the xpub at this keypath can be exported without warning the user of that it is an
/// unusual keypath.
pub fn validate_xpub(keypath: &[u32], expected_coin: u32, taproot_support: bool) -> Result<(), ()> {
    for &script_type in ALL_MULTISCRIPT_SCRIPT_TYPES.iter() {
        if validate_account_multisig(keypath, expected_coin, script_type).is_ok() {
            return Ok(());
        }
    }
    for &script_type in ALL_SIMPLE_SCRIPT_TYPES.iter() {
        if validate_account_simple(keypath, expected_coin, script_type, taproot_support).is_ok() {
            return Ok(());
        }
    }
    // m/45', used/exported by Unchained.
    if keypath == [45 + HARDENED] {
        return Ok(());
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

    #[test]
    fn test_validate_account_multisig() {
        let coin = 1 + HARDENED;

        // Valid p2wsh-p2sh.
        assert!(validate_account_multisig(
            &[48 + HARDENED, coin, 0 + HARDENED, 1 + HARDENED],
            coin,
            MultisigScriptType::P2wshP2sh
        )
        .is_ok());

        // Valid p2wsh.
        assert!(validate_account_multisig(
            &[48 + HARDENED, coin, 0 + HARDENED, 2 + HARDENED],
            coin,
            MultisigScriptType::P2wsh
        )
        .is_ok());

        // Valid Nunchuk-style.
        assert!(validate_account_multisig(
            &[48 + HARDENED, coin, 0 + HARDENED],
            coin,
            MultisigScriptType::P2wsh
        )
        .is_ok());
        assert!(validate_account_multisig(
            &[48 + HARDENED, coin, 0 + HARDENED],
            coin,
            MultisigScriptType::P2wshP2sh
        )
        .is_ok());

        // Valid script (last element).
        assert!(validate_account_multisig(
            &[48 + HARDENED, coin, 0 + HARDENED, 1 + HARDENED],
            coin,
            MultisigScriptType::P2wsh
        )
        .is_err());

        // Wrong purpose.
        assert!(validate_account_multisig(
            &[49 + HARDENED, coin, 0 + HARDENED, 2 + HARDENED],
            coin,
            MultisigScriptType::P2wsh
        )
        .is_err());
    }

    #[test]
    fn test_validate_address_simple() {
        let bip44_account = 99 + HARDENED;
        let bip44_coin = 1 + HARDENED;
        let taproot_support = true;
        for mode in [ReceiveSpend::Receive, ReceiveSpend::Spend] {
            // valid p2wpkh-p2sh; receive
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_ok());

            // valid p2wpkh-p2sh; receive on high address
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0, 9999],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_ok());

            // invalid p2wpkh-p2sh; receive on too high address - only allowed when spending
            assert_eq!(
                validate_address_simple(
                    &[49 + HARDENED, bip44_coin, bip44_account, 0, 10000],
                    bip44_coin,
                    SimpleType::P2wpkhP2sh,
                    taproot_support,
                    mode,
                )
                .is_ok(),
                mode == ReceiveSpend::Spend
            );

            // valid p2wpkh-p2sh; change
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 1, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_ok());

            // valid p2wpkh-p2sh; invalid bip44 change values
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 2, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0 + HARDENED, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 1 + HARDENED, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());

            // invalid p2wpkh-p2sh; wrong purpose
            assert!(validate_address_simple(
                &[84 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());

            // invalid p2wpkh-p2sh; account too high
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, 100 + HARDENED, 0, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());

            // invalid p2wpkh-p2sh; account too low
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, HARDENED - 1, 0, 0],
                bip44_coin,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());

            // invalid p2wpkh-p2sh; expected coin mismatch
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin + 1,
                SimpleType::P2wpkhP2sh,
                taproot_support,
                mode,
            )
            .is_err());

            // valid p2wpkh
            assert!(validate_address_simple(
                &[84 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2wpkh,
                taproot_support,
                mode,
            )
            .is_ok());

            // invalid p2wpkh; wrong purpose
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2wpkh,
                taproot_support,
                mode,
            )
            .is_err());

            // valid p2tr
            assert!(validate_address_simple(
                &[86 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2tr,
                taproot_support,
                mode,
            )
            .is_ok());

            // invalid p2tr, taproot not supported
            assert!(validate_address_simple(
                &[86 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2tr,
                false,
                mode,
            )
            .is_err());

            // invalid p2tr; wrong purpose
            assert!(validate_address_simple(
                &[49 + HARDENED, bip44_coin, bip44_account, 0, 0],
                bip44_coin,
                SimpleType::P2tr,
                taproot_support,
                mode,
            )
            .is_err());
        }
    }

    #[test]
    fn test_validate_address_policy() {
        for mode in [ReceiveSpend::Receive, ReceiveSpend::Spend] {
            assert!(validate_address_policy(&[523, 2342, 0], mode).is_ok());
            assert!(validate_address_policy(&[523, 2342, 9999], mode).is_ok());
            // No account-level part.
            assert!(validate_address_policy(&[2342, 0], mode).is_err());
        }

        // Address too high when receiving.
        assert!(validate_address_policy(&[523, 2342, 10000], ReceiveSpend::Receive).is_err());
        // Ok when spending.
        assert!(validate_address_policy(&[523, 2342, 10000], ReceiveSpend::Spend).is_ok());
    }

    #[test]
    fn test_validate_xpub() {
        let bip44_coin = 1 + HARDENED;
        let taproot_support = true;
        // Valid singlesig xpubs.
        assert!(validate_xpub(
            &[49 + HARDENED, bip44_coin, 0 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());
        assert!(validate_xpub(
            &[84 + HARDENED, bip44_coin, 0 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());
        assert!(validate_xpub(
            &[86 + HARDENED, bip44_coin, 0 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());

        // Valid multisig xpubs.
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, 0 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, 0 + HARDENED, 1 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_ok());

        // No taproot.
        assert!(validate_xpub(
            &[86 + HARDENED, bip44_coin, 0 + HARDENED],
            bip44_coin,
            false,
        )
        .is_err());

        // Invalid multisig script type.
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, 0 + HARDENED, 3 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_err());

        // Coin mismatch.
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED],
            bip44_coin + 1,
            taproot_support
        )
        .is_err());

        // Invalid account.
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, HARDENED - 1, 2 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_err());
        assert!(validate_xpub(
            &[48 + HARDENED, bip44_coin, HARDENED + 100, 2 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_err());

        // Invalid purpose.
        assert!(validate_xpub(
            &[44 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_err());
        assert!(validate_xpub(
            &[100 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED],
            bip44_coin,
            taproot_support
        )
        .is_err());
    }
}
