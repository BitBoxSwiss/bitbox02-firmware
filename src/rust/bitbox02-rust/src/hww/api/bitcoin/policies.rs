// Copyright 2023 Shift Crypto AG
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
use super::pb;
use super::Error;
use pb::BtcCoin;

use pb::btc_script_config::Policy;

use alloc::string::String;
use alloc::vec::Vec;

use core::str::FromStr;

use util::bip32::HARDENED;

use miniscript::TranslatePk;

use crate::bip32;
use crate::workflow::confirm;

use sha2::{Digest, Sha256};

// Arbitrary limit of keys that can be present in a policy.
const MAX_KEYS: usize = 20;

// We only support Bitcoin testnet for now.
fn check_enabled(coin: BtcCoin) -> Result<(), Error> {
    if !matches!(coin, BtcCoin::Tbtc) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

/// Checks if the key is our key by comparing the root fingerprints
/// and deriving and comparing the xpub at the keypath.
fn is_our_key(key: &pb::KeyOriginInfo) -> Result<bool, ()> {
    let our_root_fingerprint = crate::keystore::root_fingerprint()?;
    match key {
        pb::KeyOriginInfo {
            root_fingerprint,
            keypath,
            xpub: Some(xpub),
            ..
        } if root_fingerprint.as_slice() == our_root_fingerprint.as_slice() => {
            let our_xpub = crate::keystore::get_xpub(keypath)?.serialize(None)?;
            let maybe_our_xpub = bip32::Xpub::from(xpub).serialize(None)?;
            Ok(our_xpub == maybe_our_xpub)
        }
        _ => Ok(false),
    }
}

/// Parses a wallet policy of the form `@NUM1/<NUM2;NUM3>/*` into `(NUM1,NUM2,NUM3)`.
/// `@NUM/**` is equivalent o `@NUM/<0;1>/*`.
/// The derivation numbers are checked to be distinct and unhardened.
fn parse_wallet_policy_pk(pk: &str) -> Result<(usize, u32, u32), ()> {
    fn validate_no_leading_zero(num: &str) -> Result<(), ()> {
        if num.len() > 1 && num.starts_with('0') {
            Err(())
        } else {
            Ok(())
        }
    }
    let (left, right) = pk.strip_prefix('@').ok_or(())?.split_once('/').ok_or(())?;
    validate_no_leading_zero(left)?;
    let (receive_index, change_index): (u32, u32) = match right {
        "**" => (0, 1),
        right => {
            let (left_number_str, right_number_str) = right
                .strip_prefix('<')
                .ok_or(())?
                .strip_suffix(">/*")
                .ok_or(())?
                .split_once(';')
                .ok_or(())?;
            validate_no_leading_zero(left_number_str)?;
            validate_no_leading_zero(right_number_str)?;
            (
                left_number_str.parse().or(Err(()))?,
                right_number_str.parse().or(Err(()))?,
            )
        }
    };
    if receive_index == change_index || receive_index >= HARDENED || change_index >= HARDENED {
        return Err(());
    }
    Ok((left.parse().or(Err(()))?, receive_index, change_index))
}

/// Given policy pubkeys like `@0/<left;right>/*` and the keys list, determine if the given keypath
/// is valid and whether it points to a receive or change address.
///
/// Example: pubkeys "@0/<10;11>/*" and "@1/<20;21>/*", with our key [fp/48'/1'/0'/3']xpub...],
/// derived using keypath m/48'/1'/0'/3'/11/5 means that this is the address index 5 at the change
/// path.
fn get_change_and_address_index<R: core::convert::AsRef<str>, T: core::iter::Iterator<Item = R>>(
    pubkeys: T,
    keys: &[pb::KeyOriginInfo],
    keypath: &[u32],
) -> Result<(bool, u32), Error> {
    for pk in pubkeys {
        let (key_index, multipath_index_left, multipath_index_right) =
            parse_wallet_policy_pk(pk.as_ref()).or(Err(Error::InvalidInput))?;

        match keys.get(key_index) {
            Some(pb::KeyOriginInfo {
                keypath: keypath_account,
                ..
            }) if keypath.starts_with(&keypath_account)
                && keypath.len() == keypath_account.len() + 2 =>
            {
                let keypath_change = keypath[keypath.len() - 2];
                let is_change = if keypath_change == multipath_index_left {
                    false
                } else if keypath_change == multipath_index_right {
                    true
                } else {
                    continue;
                };
                return Ok((is_change, keypath[keypath.len() - 1]));
            }
            _ => continue,
        }
    }
    Err(Error::InvalidInput)
}

struct WalletPolicyPkTranslator<'a> {
    keys: &'a [pb::KeyOriginInfo],
    is_change: bool,
    address_index: u32,
}

impl<'a> miniscript::Translator<String, bitcoin::PublicKey, Error>
    for WalletPolicyPkTranslator<'a>
{
    fn pk(&mut self, pk: &String) -> Result<bitcoin::PublicKey, Error> {
        let (key_index, multipath_index_left, multipath_index_right) =
            parse_wallet_policy_pk(&pk).or(Err(Error::InvalidInput))?;

        match self.keys.get(key_index) {
            Some(pb::KeyOriginInfo {
                xpub: Some(xpub), ..
            }) => {
                let multipath_index = if self.is_change {
                    multipath_index_right
                } else {
                    multipath_index_left
                };
                let derived_xpub =
                    bip32::Xpub::from(xpub).derive(&[multipath_index, self.address_index])?;
                Ok(bitcoin::PublicKey::from_slice(derived_xpub.public_key())
                    .or(Err(Error::Generic))?)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    // Miniscript hash fragments not supported.
    miniscript::translate_hash_fail!(String, bitcoin::PublicKey, Error);
}

/// See `ParsedPolicy`.
#[derive(Debug)]
pub struct Wsh<'a> {
    policy: &'a Policy,
    miniscript_expr: miniscript::Miniscript<String, miniscript::Segwitv0>,
}

/// Result of `parse()`.
#[derive(Debug)]
pub enum ParsedPolicy<'a> {
    // `wsh(...)` policies
    Wsh(Wsh<'a>),
    // `tr(...)` Taproot etc. in the future.
}

impl<'a> ParsedPolicy<'a> {
    fn get_policy(&self) -> &Policy {
        match self {
            Self::Wsh(Wsh { ref policy, .. }) => policy,
        }
    }

    /// Check that it is impossible to create a derivation with duplicate pubkeys, assuming all the
    /// keys in the key vector are distinct.
    ///
    /// Even though the rust-miniscript library checks for duplicate keys, it does so on the raw
    /// miniscript, which would not catch e.g. that `wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;1>/*)))` has
    /// a duplicate change derivation if we derive at the receive path.
    ///
    /// Also checks that each key is used, e.g. if there are 3 keys in the key vector, @0, @1 and @2
    /// must be present.
    fn validate_keys(&self) -> Result<(), Error> {
        match self {
            Self::Wsh(Wsh {
                policy,
                miniscript_expr,
            }) => {
                // in "@key_index/<left;right>", keeps track of (key_index,left) and
                // (key_index,right) to check for duplicates.
                let mut derivations_seen: Vec<(usize, u32)> = Vec::new();

                let mut keys_seen: Vec<bool> = vec![false; policy.keys.len()];

                for pk in miniscript_expr.iter_pk() {
                    let (key_index, multipath_index_left, multipath_index_right) =
                        parse_wallet_policy_pk(&pk).or(Err(Error::InvalidInput))?;

                    if derivations_seen.contains(&(key_index, multipath_index_left)) {
                        return Err(Error::InvalidInput);
                    }
                    derivations_seen.push((key_index, multipath_index_left));
                    if derivations_seen.contains(&(key_index, multipath_index_right)) {
                        return Err(Error::InvalidInput);
                    }
                    derivations_seen.push((key_index, multipath_index_right));

                    *keys_seen.get_mut(key_index).ok_or(Error::InvalidInput)? = true;
                }

                if !keys_seen.into_iter().all(|b| b) {
                    return Err(Error::InvalidInput);
                }
                Ok(())
            }
        }
    }

    /// Validate a policy.
    /// - Coin is supported (only Bitcoin testnet for now)
    /// - Number of keys
    /// - At least one of the keys is ours
    /// - There are no duplicate or missing xpubs
    /// - No duplicate keys in the policy
    pub fn validate(&self, coin: BtcCoin) -> Result<(), Error> {
        check_enabled(coin)?;

        let policy = self.get_policy();

        if policy.keys.len() > MAX_KEYS {
            return Err(Error::InvalidInput);
        }

        self.validate_keys()?;

        // Check that at least one key is ours.
        let has_our_key = 'block: {
            for key in policy.keys.iter() {
                if is_our_key(key)? {
                    break 'block true;
                }
            }
            false
        };
        if !has_our_key {
            return Err(Error::InvalidInput);
        }

        // Check for duplicate xpubs and that all keys contain an xpub.
        // Extract all xpubs first.
        let xpubs: Vec<&pb::XPub> = policy
            .keys
            .iter()
            .map(|key| match key {
                pb::KeyOriginInfo {
                    xpub: Some(xpub), ..
                } => Ok(xpub),
                _ => Err(Error::InvalidInput),
            })
            .collect::<Result<Vec<&pb::XPub>, Error>>()?;
        if (1..xpubs.len()).any(|i| xpubs[i..].contains(&xpubs[i - 1])) {
            return Err(Error::InvalidInput);
        }

        Ok(())
    }

    /// Derive the witness script of the policy derived at a receive or change path.
    /// If is_change is false, the witness script for the receive address is derived.
    /// If is_change is true, the witness script for the change address is derived.
    /// Example: wsh(and_v(v:pk(@0/**),pk(@1/<20;21>/*))) derived using `is_change=false, address_index=5` derives
    /// wsh(and_v(v:pk(@0/0/5),pk(@1/20/5))).
    /// The same derived using `is_change=true` derives: wsh(and_v(v:pk(@0/1/5),pk(@1/21/5)))
    pub fn witness_script(&self, is_change: bool, address_index: u32) -> Result<Vec<u8>, Error> {
        match self {
            Self::Wsh(Wsh {
                policy,
                miniscript_expr,
            }) => {
                let mut translator = WalletPolicyPkTranslator {
                    keys: policy.keys.as_ref(),
                    is_change,
                    address_index,
                };
                let miniscript_expr = match miniscript_expr.translate_pk(&mut translator) {
                    Ok(m) => m,
                    Err(miniscript::TranslateErr::TranslatorErr(e)) => return Err(e),
                    Err(miniscript::TranslateErr::OuterError(_)) => return Err(Error::Generic),
                };
                Ok(miniscript_expr.encode().as_bytes().to_vec())
            }
        }
    }

    /// Derive the witness script of the policy derived at the given full keypath.
    /// Example: wsh(and_v(v:pk(@0/<10;11>/*),pk(@1/<20;21>/*))) with our key [fp/48'/1'/0'/3']xpub...]
    /// derived using keypath m/48'/1'/0'/3'/11/5 derives:
    /// wsh(and_v(v:pk(@0/11/5),pk(@1/21/5))).
    pub fn witness_script_at_keypath(&self, keypath: &[u32]) -> Result<Vec<u8>, Error> {
        match self {
            Self::Wsh(Wsh {
                ref policy,
                ref miniscript_expr,
            }) => {
                let (is_change, address_index) =
                    get_change_and_address_index(miniscript_expr.iter_pk(), &policy.keys, keypath)?;
                self.witness_script(is_change, address_index)
            }
        }
    }

    /// Returns true if the address-level keypath points to a change address.
    pub fn is_change_keypath(&self, keypath: &[u32]) -> Result<bool, Error> {
        match self {
            Self::Wsh(Wsh {
                policy,
                miniscript_expr,
            }) => {
                let (is_change, _) =
                    get_change_and_address_index(miniscript_expr.iter_pk(), &policy.keys, keypath)?;
                Ok(is_change)
            }
        }
    }
}

/// Parses a policy as specified by 'Wallet policies': https://github.com/bitcoin/bips/pull/1389.
/// Only `wsh(<miniscript expression>)` is supported for now.
/// Example: `wsh(pk(@0/**))`.
///
/// The parsed output keeps the key strings as is (e.g. "@0/**"). They will be processed and
/// replaced with actual pubkeys in a later step.
pub fn parse(policy: &Policy) -> Result<ParsedPolicy, Error> {
    let desc = policy.policy.as_str();
    match desc.as_bytes() {
        // Match wsh(...).
        [b'w', b's', b'h', b'(', .., b')'] => {
            let miniscript_expr: miniscript::Miniscript<String, miniscript::Segwitv0> =
                miniscript::Miniscript::from_str(&desc[4..desc.len() - 1])
                    .or(Err(Error::InvalidInput))?;

            Ok(ParsedPolicy::Wsh(Wsh {
                policy,
                miniscript_expr,
            }))
        }
        _ => Err(Error::InvalidInput),
    }
}

/// Confirmation mode.
pub enum Mode {
    /// Confirm coin, number of keys and account name and optionally the advanced details.
    Basic,
    /// Confirm coin, number of keys, account name, the policy string, and the key origin infos.
    Advanced,
}

/// Confirm the policy. In advanced mode, all details are shown. In basic mode, the advanced details
/// are optional. Used to verify the policy during account registration (advanced mode), creating a
/// receive address (basic mode) and signing a transaction (basic mode).
pub async fn confirm(
    title: &str,
    params: &Params,
    name: &str,
    policy: &Policy,
    mode: Mode,
) -> Result<(), Error> {
    confirm::confirm(&confirm::Params {
        title,
        body: &format!("{}\npolicy with\n{} keys", params.name, policy.keys.len(),),
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title: "Name",
        body: name,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    if matches!(mode, Mode::Basic) {
        if let Err(confirm::UserAbort) = confirm::confirm(&confirm::Params {
            body: "Show policy\ndetails?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await
        {
            return Ok(());
        }
    }

    confirm::confirm(&confirm::Params {
        title: "Policy",
        body: &policy.policy,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    let num_keys = policy.keys.len();
    for (i, key) in policy.keys.iter().enumerate() {
        let key_str = match key {
            pb::KeyOriginInfo {
                root_fingerprint,
                keypath,
                xpub: Some(xpub),
            } => {
                let xpub_str = bip32::Xpub::from(xpub)
                    .serialize_str(bip32::XPubType::Xpub)
                    .or(Err(Error::InvalidInput))?;
                if root_fingerprint.is_empty() {
                    xpub_str
                } else if root_fingerprint.len() != 4 {
                    return Err(Error::InvalidInput);
                } else {
                    format!(
                        "[{}/{}]{}",
                        hex::encode(root_fingerprint),
                        util::bip32::to_string_no_prefix(keypath),
                        xpub_str
                    )
                }
            }
            _ => return Err(Error::InvalidInput),
        };
        confirm::confirm(&confirm::Params {
            title: &format!("Key {}/{}", i + 1, num_keys),
            body: (if is_our_key(key)? {
                format!("This device: {}", key_str)
            } else {
                key_str
            })
            .as_str(),
            scrollable: true,
            longtouch: i == num_keys - 1 && matches!(mode, Mode::Advanced),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    }
    Ok(())
}

/// Creates a hash of this policy config, useful for registration and identification.
pub fn get_hash(coin: BtcCoin, policy: &Policy) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    {
        // 1. Type of registration: policy.
        // It is chosen to never conflict with multisig hashes which start with the coin (0x00-0x03).
        hasher.update(&[0xff]);
    }
    {
        // 2. coin
        let byte: u8 = match coin {
            BtcCoin::Btc => 0x00,
            BtcCoin::Tbtc => 0x01,
            BtcCoin::Ltc => 0x02,
            BtcCoin::Tltc => 0x03,
        };
        hasher.update(byte.to_le_bytes());
    }
    {
        // 3. policy
        let len: u32 = policy.policy.len() as _;
        hasher.update(len.to_le_bytes());
        hasher.update(policy.policy.as_bytes());
    }
    {
        // 4. keys
        let num: u32 = policy.keys.len() as _;
        hasher.update(num.to_le_bytes());
        for key in policy.keys.iter() {
            hasher.update(&bip32::Xpub::from(key.xpub.as_ref().unwrap()).serialize(None)?);
        }
    }
    Ok(hasher.finalize().as_slice().into())
}

/// Get the name of a registered policy account. The poliy is not validated, it must be
/// pre-validated!
///
/// Returns the name of the registered policy account if it exists or None otherwise.
pub fn get_name(coin: BtcCoin, policy: &Policy) -> Result<Option<String>, ()> {
    Ok(bitbox02::memory::multisig_get_by_hash(&get_hash(
        coin, policy,
    )?))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bip32::parse_xpub;
    use bitbox02::testing::{mock_unlocked, mock_unlocked_using_mnemonic};

    const SOME_XPUB_1: &str = "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo";

    const KEYPATH_ACCOUNT: &[u32] = &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED];

    // Creates a policy key without fingerprint/keypath from an xpub string.
    fn make_key(xpub: &str) -> pb::KeyOriginInfo {
        pb::KeyOriginInfo {
            root_fingerprint: vec![],
            keypath: vec![],
            xpub: Some(parse_xpub(xpub).unwrap()),
        }
    }

    // Creates a policy for one of our own keys at keypath.
    fn make_our_key(keypath: &[u32]) -> pb::KeyOriginInfo {
        let our_xpub = crate::keystore::get_xpub(keypath).unwrap();
        pb::KeyOriginInfo {
            root_fingerprint: crate::keystore::root_fingerprint().unwrap(),
            keypath: keypath.to_vec(),
            xpub: Some(our_xpub.into()),
        }
    }

    fn make_policy(policy: &str, keys: &[pb::KeyOriginInfo]) -> Policy {
        Policy {
            policy: policy.into(),
            keys: keys.to_vec(),
        }
    }

    #[test]
    fn test_parse_wallet_policy_pk() {
        assert_eq!(parse_wallet_policy_pk("@0/**"), Ok((0, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@1/**"), Ok((1, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@100/**"), Ok((100, 0, 1)));

        assert_eq!(parse_wallet_policy_pk("@0/<0;1>/*"), Ok((0, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@0/<1;2>/*"), Ok((0, 1, 2)));
        assert_eq!(parse_wallet_policy_pk("@0/<100;101>/*"), Ok((0, 100, 101)));
        assert_eq!(
            parse_wallet_policy_pk("@50/<100;101>/*"),
            Ok((50, 100, 101))
        );

        assert!(parse_wallet_policy_pk("@00/**").is_err());
        assert!(parse_wallet_policy_pk("@01/**").is_err());
        assert!(parse_wallet_policy_pk("@0").is_err());
        assert!(parse_wallet_policy_pk("@0/").is_err());
        assert!(parse_wallet_policy_pk("@0/*").is_err());
        assert!(parse_wallet_policy_pk("0/**").is_err());
        assert!(parse_wallet_policy_pk("@-1/**").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>/*/*").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>/").is_err());
        assert!(parse_wallet_policy_pk("@0/<100;100>/*").is_err());
        // 2147483648 = HARDENED offset.
        assert!(parse_wallet_policy_pk("@0/<100;2147483648>/*").is_err());
        assert!(parse_wallet_policy_pk("@0/<2147483648;100>/*").is_err());
    }

    #[test]
    fn test_parse_wsh_miniscript() {
        // Parse a valid example and check that the keys are collected as is as strings.
        let policy = make_policy("wsh(pk(@0/**))", &[]);
        match parse(&policy).unwrap() {
            ParsedPolicy::Wsh(Wsh {
                ref miniscript_expr,
                ..
            }) => {
                assert_eq!(
                    miniscript_expr.iter_pk().collect::<Vec<String>>(),
                    vec!["@0/**"]
                );
            }
        }

        // Parse another valid example and check that the keys are collected as is as strings.
        let policy = make_policy("wsh(or_b(pk(@0/**),s:pk(@1/**)))", &[]);
        match parse(&policy).unwrap() {
            ParsedPolicy::Wsh(Wsh {
                ref miniscript_expr,
                ..
            }) => {
                assert_eq!(
                    miniscript_expr.iter_pk().collect::<Vec<String>>(),
                    vec!["@0/**", "@1/**"]
                );
            }
        }

        // Unknown top-level fragment.
        assert_eq!(
            parse(&make_policy("unknown(pk(@0/**))", &[])).unwrap_err(),
            Error::InvalidInput,
        );

        // Unknown script fragment.
        assert_eq!(
            parse(&make_policy("wsh(unknown(@0/**))", &[])).unwrap_err(),
            Error::InvalidInput,
        );

        // Miniscript type-check fails (should be `or_b(pk(@0/**),s:pk(@1/**))`).
        assert_eq!(
            parse(&make_policy("wsh(or_b(pk(@0/**),pk(@1/**)))", &[])).unwrap_err(),
            Error::InvalidInput,
        );
    }

    #[test]
    fn test_parse_validate() {
        mock_unlocked();

        let our_key = make_our_key(KEYPATH_ACCOUNT);

        // All good.
        assert!(parse(&make_policy("wsh(pk(@0/**))", &[our_key.clone()]))
            .unwrap()
            .validate(BtcCoin::Tbtc)
            .is_ok());

        // Unsupported coins
        for coin in [BtcCoin::Btc, BtcCoin::Ltc, BtcCoin::Tltc] {
            assert_eq!(
                parse(&make_policy("wsh(pk(@0/**))", &[our_key.clone()]))
                    .unwrap()
                    .validate(coin),
                Err(Error::InvalidInput)
            );
        }

        // Too many keys.
        let many_keys: Vec<pb::KeyOriginInfo> = (0..=20)
            .map(|i| make_our_key(&[48 + HARDENED, 1 + HARDENED, i + HARDENED, 3 + HARDENED]))
            .collect();
        assert_eq!(
            parse(&make_policy("wsh(pk(@0/**))", &many_keys))
                .unwrap()
                .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Our key is not present - fingerprint missing.
        assert_eq!(
            parse(&make_policy("wsh(pk(@0/**))", &[make_key(SOME_XPUB_1)]))
                .unwrap()
                .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Our key is not present - fingerprint and keypath exit but xpub does not match.
        let mut wrong_key = our_key.clone();
        wrong_key.xpub = Some(parse_xpub(SOME_XPUB_1).unwrap());
        assert_eq!(
            parse(&make_policy("wsh(pk(@0/**))", &[wrong_key]))
                .unwrap()
                .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Contains duplicate keys.
        assert_eq!(
            parse(&make_policy(
                "wsh(multi(2,@0/**,@1/**,@2/**))",
                &[
                    make_key(SOME_XPUB_1),
                    our_key.clone(),
                    make_key(SOME_XPUB_1)
                ]
            ))
            .unwrap()
            .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Contains a key with missing xpub.
        assert_eq!(
            parse(&make_policy(
                "wsh(multi(2,@0/**,@1/**))",
                &[
                    our_key.clone(),
                    pb::KeyOriginInfo {
                        root_fingerprint: vec![],
                        keypath: vec![],
                        xpub: None // missing
                    }
                ]
            ))
            .unwrap()
            .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Not all keys are used.
        assert_eq!(
            parse(&make_policy(
                "wsh(pk(@0/**))",
                &[our_key.clone(), make_key(SOME_XPUB_1)]
            ))
            .unwrap()
            .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );

        // Referenced key does not exist
        assert_eq!(
            parse(&make_policy("wsh(pk(@1/**))", &[our_key.clone()]))
                .unwrap()
                .validate(BtcCoin::Tbtc),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    fn test_parse_check_dups_in_policy() {
        mock_unlocked();

        let coin = BtcCoin::Tbtc;
        let our_key = make_our_key(KEYPATH_ACCOUNT);

        // Ok, one key.
        let pol = make_policy("wsh(pk(@0/**))", &[our_key.clone()]);
        assert!(parse(&pol).unwrap().validate(coin).is_ok());

        // Ok, two keys.
        let pol = make_policy(
            "wsh(or_b(pk(@0/**),s:pk(@1/**)))",
            &[our_key.clone(), make_key(SOME_XPUB_1)],
        );
        assert!(parse(&pol).unwrap().validate(coin).is_ok());

        // Ok, one key with different derivations
        let pol = make_policy(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;3>/*)))",
            &[our_key.clone()],
        );
        assert!(parse(&pol).unwrap().validate(coin).is_ok());

        // Duplicate path, one time in change, one time in receive. While the keys technically are
        // never duplicate in the final miniscript with the pubkeys inserted, we still prohibit, as
        // it does not look like there would be a sane use case for this and would likely be an
        // accident.
        let pol = make_policy(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<1;2>/*)))",
            &[our_key.clone()],
        );
        assert!(parse(&pol).unwrap().validate(coin).is_err());

        // Duplicate key inside policy.
        let pol = make_policy("wsh(or_b(pk(@0/**),s:pk(@0/**)))", &[our_key.clone()]);
        assert!(parse(&pol).is_err());

        // Duplicate key inside policy (same change and receive).
        let pol = make_policy("wsh(pk(@0/<0;0>/*))", &[our_key.clone()]);
        assert!(parse(&pol).unwrap().validate(coin).is_err());

        // Duplicate key inside policy, using different notations for the same thing.
        let pol = make_policy("wsh(or_b(pk(@0/**),s:pk(@0/<0;1>/*)))", &[our_key.clone()]);
        assert!(parse(&pol).unwrap().validate(coin).is_err());

        // Duplicate key inside policy, using same receive but different change.
        let pol = make_policy(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<0;2>/*)))",
            &[our_key.clone()],
        );
        assert!(parse(&pol).unwrap().validate(coin).is_err());

        // Duplicate key inside policy, using same change but different receive.
        let pol = make_policy(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;1>/*)))",
            &[our_key.clone()],
        );
        assert!(parse(&pol).unwrap().validate(coin).is_err());
    }

    #[test]
    fn test_get_change_and_address_index() {
        mock_unlocked();
        let our_key = make_our_key(KEYPATH_ACCOUNT);
        let some_key = make_key(SOME_XPUB_1);

        assert_eq!(
            get_change_and_address_index(
                ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
                &[our_key.clone(), some_key.clone()],
                &[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    10,
                    0,
                ],
            ),
            Ok((false, 0))
        );

        assert_eq!(
            get_change_and_address_index(
                ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
                &[our_key.clone(), some_key.clone()],
                &[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    11,
                    5,
                ],
            ),
            Ok((true, 5))
        );

        // Account keypath does not match.
        assert!(get_change_and_address_index(
            ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
            &[our_key.clone(), some_key.clone()],
            &[
                48 + HARDENED,
                1 + HARDENED,
                0 + HARDENED,
                0 + HARDENED,
                11,
                5,
            ],
        )
        .is_err());

        // Keypath change/receive element does not match.
        assert!(get_change_and_address_index(
            ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
            &[our_key.clone(), some_key.clone()],
            &[
                48 + HARDENED,
                1 + HARDENED,
                0 + HARDENED,
                3 + HARDENED,
                20,
                5,
            ],
        )
        .is_err());

        // Keypath too long
        assert!(get_change_and_address_index(
            ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
            &[our_key.clone(), some_key.clone()],
            &[
                48 + HARDENED,
                1 + HARDENED,
                0 + HARDENED,
                3 + HARDENED,
                10,
                5,
                0,
            ],
        )
        .is_err());

        // Keypath too short
        assert!(get_change_and_address_index(
            ["@0/<10;11>/*", "@1/<20;21>/*"].iter(),
            &[our_key.clone(), some_key.clone()],
            &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED, 10,],
        )
        .is_err());
    }

    #[test]
    fn test_witness_script() {
        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
            "",
        );

        let our_key = make_our_key(KEYPATH_ACCOUNT);
        let our_xpub = bip32::Xpub::from(our_key.xpub.as_ref().unwrap());

        let some_key = make_key(SOME_XPUB_1);
        let some_xpub = bip32::Xpub::from(some_key.xpub.as_ref().unwrap());
        let address_index = 5;

        let witness_script = |pol: &str, keys: &[pb::KeyOriginInfo], is_change: bool| {
            hex::encode(
                &parse(&make_policy(pol, keys))
                    .unwrap()
                    .witness_script(is_change, address_index)
                    .unwrap(),
            )
        };
        let witness_script_at_keypath = |pol: &str, keys: &[pb::KeyOriginInfo], keypath: &[u32]| {
            hex::encode(
                &parse(&make_policy(pol, keys))
                    .unwrap()
                    .witness_script_at_keypath(keypath)
                    .unwrap(),
            )
        };

        // pk(key) => <key> OP_CHECKSIG
        let result = witness_script("wsh(pk(@0/**))", &[our_key.clone()], false);
        let expected_derived_pubkey =
            "039d626054b8fd7e8371ee7341549846cc7703b5530d6b7ddc08dc8a3b78455924";
        assert_eq!(
            hex::encode(our_xpub.derive(&[0, address_index]).unwrap().public_key()).as_str(),
            expected_derived_pubkey
        );
        assert_eq!(result, format!("21{}ac", expected_derived_pubkey));

        // multi(2,key1,key2) => OP_1 <key1> <key2> OP_2 CHECKMULTISIGVERIFY OP_1 = 0x51, OP_2 =
        // 0x52 Use <10;11> and <20;21> for receive/change instead of the usual <0;1> to test these
        // derivations.
        {
            // 1. Test the receive path
            let result = witness_script(
                "wsh(multi(1,@0/<10;11>/*,@1/<20;21>/*))",
                &[our_key.clone(), some_key.clone()],
                false,
            );
            let expected_derived_pubkey1 =
                "0290ad738002018d6e9551603f1913983bd52145e3a026b79b133b9d36bacc7f25";
            let expected_derived_pubkey2 =
                "02d56a3aeb73509ddaea764d2af3094092a80ab5d282ac35c7c42a03c397302a1b";
            assert_eq!(
                hex::encode(our_xpub.derive(&[10, address_index]).unwrap().public_key()).as_str(),
                expected_derived_pubkey1
            );
            assert_eq!(
                hex::encode(some_xpub.derive(&[20, address_index]).unwrap().public_key()).as_str(),
                expected_derived_pubkey2
            );
            let expected_witness_script = format!(
                "5121{}21{}52ae",
                expected_derived_pubkey1, expected_derived_pubkey2
            );
            assert_eq!(result, expected_witness_script);

            // Test the same using a full keypath.
            assert_eq!(
                witness_script_at_keypath(
                    "wsh(multi(1,@0/<10;11>/*,@1/<20;21>/*))",
                    &[our_key.clone(), some_key.clone()],
                    &[
                        48 + HARDENED,
                        1 + HARDENED,
                        0 + HARDENED,
                        3 + HARDENED,
                        10,
                        address_index,
                    ],
                ),
                expected_witness_script,
            );
        }
        {
            // 2. Test the change path
            let result = witness_script(
                "wsh(multi(1,@0/<10;11>/*,@1/<20;21>/*))",
                &[our_key.clone(), some_key.clone()],
                true,
            );
            let expected_derived_pubkey1 =
                "038294e6b0f046e869c3211b8c937ccb19ab0913e3170b7ec32d07d241d97d0e07";
            let expected_derived_pubkey2 =
                "029684141cf8eb01224cbe0470cca0ad4dae482c70d8e5c1601686f9b2b69f3d0f";
            assert_eq!(
                hex::encode(our_xpub.derive(&[11, address_index]).unwrap().public_key()).as_str(),
                expected_derived_pubkey1
            );
            assert_eq!(
                hex::encode(some_xpub.derive(&[21, address_index]).unwrap().public_key()).as_str(),
                expected_derived_pubkey2
            );
            let expected_witness_script = format!(
                "5121{}21{}52ae",
                expected_derived_pubkey1, expected_derived_pubkey2
            );
            assert_eq!(result, expected_witness_script);

            // Test the same using a full keypath.
            assert_eq!(
                witness_script_at_keypath(
                    "wsh(multi(1,@0/<10;11>/*,@1/<20;21>/*))",
                    &[our_key.clone(), some_key.clone()],
                    &[
                        48 + HARDENED,
                        1 + HARDENED,
                        0 + HARDENED,
                        3 + HARDENED,
                        11,
                        address_index,
                    ],
                ),
                expected_witness_script,
            );
        }
    }

    #[test]
    fn test_get_hash() {
        // Fixture below verified with:
        // import hashlib
        // import base58
        //
        // xpubs = [
        //     "xpub6EMfjyGVUvwhqh719Z4b9j4hxgWwZg9NrNRhs4s4QHP4qMkYfkxD3i3fcQuE51i99G1AhSyoSymjbEZMfa1bcPJK281gPNCS7VPQ7YmovG4",
        //     "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
        // ]
        //
        // policy = b'wsh(multi(2,@0/**,@1/**))'
        //
        // i32 = lambda i: i.to_bytes(4, 'little')
        //
        // msg = []
        // msg.append(b'\xff') # registration type
        // msg.append(b'\x00') # coin
        // msg.append(i32(len(policy)))
        // msg.append(policy) # script config type
        // msg.append(i32(len(xpubs)))
        // msg.extend(base58.b58decode_check(xpub)[4:] for xpub in xpubs)
        // print(hashlib.sha256(b''.join(msg)).hexdigest())

        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
            "",
        );

        let pol = make_policy(
            "wsh(multi(2,@0/**,@1/**))",
            &[make_our_key(KEYPATH_ACCOUNT), make_key(SOME_XPUB_1)],
        );

        assert_eq!(
            hex::encode(get_hash(BtcCoin::Btc, &pol).unwrap()).as_str(),
            "26b4cd47ee808288cebf95b77b31cafa2ab88ec377eb94f01aa8860abf67b6d6",
        );
        assert_eq!(
            hex::encode(get_hash(BtcCoin::Tbtc, &pol).unwrap()).as_str(),
            "3c2e1194a9c5ebe0703f580e1493818b2af3eb30368f1dddcaddb2b4a93fbcf3",
        );
        assert_eq!(
            hex::encode(get_hash(BtcCoin::Ltc, &pol).unwrap()).as_str(),
            "7538418014c911a2812afabca3f725d32a076fb756a867d0a2f7bf23879bd474",
        );
        assert_eq!(
            hex::encode(get_hash(BtcCoin::Tltc, &pol).unwrap()).as_str(),
            "6160dc5cf72b79380e9e715c75ae54573b81dcb4ed8ab2e90fde5d661e443781",
        );
    }
}
