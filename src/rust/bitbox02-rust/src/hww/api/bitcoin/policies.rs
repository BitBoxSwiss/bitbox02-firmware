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

use super::pb;
use super::Error;
use pb::BtcCoin;

use pb::btc_script_config::Policy;

use alloc::string::String;
use alloc::vec::Vec;

use core::str::FromStr;

use util::bip32::HARDENED;

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
            let maybe_our_xpub = crate::bip32::Xpub::from(xpub).serialize(None)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bip32::parse_xpub;
    use bitbox02::testing::mock_unlocked;

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
}