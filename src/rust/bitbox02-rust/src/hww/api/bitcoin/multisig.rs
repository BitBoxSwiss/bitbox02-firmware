// Copyright 2022 Shift Crypto AG
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

use pb::btc_response::Response;
use pb::btc_script_config::{multisig::ScriptType, Config, Multisig};
use pb::BtcCoin;

use crate::workflow::confirm;

use alloc::string::String;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

pub const MAX_SIGNERS: usize = 15;

pub enum SortXpubs {
    No,
    Yes,
}

/// Creates a hash of this multisig config, useful for multisig account registration and
/// identification. The individual params are not validated, they must be pre-validated!
///
/// If `sort_xpubs` is `SortXpubs::Yes`, the xpubs are sorted before hashing.
/// The keypath is the account-level keypath.
pub fn get_hash(
    coin: BtcCoin,
    multisig: &Multisig,
    sort_xpubs: SortXpubs,
    keypath: &[u32],
) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    {
        // 1. coin
        let byte: u8 = match coin {
            BtcCoin::Btc => 0x00,
            BtcCoin::Tbtc => 0x01,
            BtcCoin::Ltc => 0x02,
            BtcCoin::Tltc => 0x03,
        };
        hasher.update(byte.to_le_bytes());
    }
    {
        // 2. script config type
        let byte: u8 = match ScriptType::from_i32(multisig.script_type).ok_or(())? {
            ScriptType::P2wsh => 0x00,
            ScriptType::P2wshP2sh => 0x01,
        };
        hasher.update(byte.to_le_bytes());
    }
    {
        // 3. threshold
        hasher.update(multisig.threshold.to_le_bytes());
    }
    {
        // 4. num xpubs
        let num: u32 = multisig.xpubs.len() as _;
        hasher.update(num.to_le_bytes());
    }
    {
        // 5. xpubs
        let mut xpubs_serialized: Vec<Vec<u8>> = multisig
            .xpubs
            .iter()
            .map(|xpub| crate::bip32::serialize_xpub(xpub, None))
            .collect::<Result<Vec<Vec<u8>>, ()>>()?;
        if let SortXpubs::Yes = sort_xpubs {
            xpubs_serialized.sort();
        }
        for xpub in xpubs_serialized.iter() {
            hasher.update(xpub);
        }
    }
    {
        // 6. keypath len
        let num: u32 = keypath.len() as _;
        hasher.update(num.to_le_bytes());
    }
    {
        // 7. keypath
        for el in keypath.iter() {
            hasher.update(el.to_le_bytes());
        }
    }
    Ok(hasher.finalize().as_slice().into())
}

/// Get the name of a registered multisig account. The individual params are not validated, they
/// must be pre-validated!
///
/// The keypath is the account-level keypath.
///
/// Returns the name of the registered multisig account if it exists or None otherwise.
pub fn get_name(coin: BtcCoin, multisig: &Multisig, keypath: &[u32]) -> Result<Option<String>, ()> {
    // First try using sorted xpubs (the default registration since v9.3.0).
    if let Some(name) =
        bitbox02::memory::multisig_get_by_hash(&get_hash(coin, multisig, SortXpubs::Yes, keypath)?)
    {
        return Ok(Some(name));
    }
    // If that did not exist, try with unsorted xpubs for backwards compatibility.
    Ok(bitbox02::memory::multisig_get_by_hash(&get_hash(
        coin,
        multisig,
        SortXpubs::No,
        keypath,
    )?))
}

/// Confirms a multisig setup with the user during send/receive.
/// Verified are:
/// - coin
/// - multisig type (m-of-n)
/// - name given by the user
pub async fn confirm(
    title: &str,
    params: &Params,
    name: &str,
    multisig: &Multisig,
) -> Result<(), Error> {
    confirm::confirm(&confirm::Params {
        title,
        body: &format!(
            "{}-of-{}\n{} multisig",
            multisig.threshold,
            multisig.xpubs.len(),
            params.name
        ),
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;
    confirm::confirm(&confirm::Params {
        title,
        body: name,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;
    Ok(())
}

/// Validate a m-of-n multisig account. This includes checking that:
/// - 0 < m <= n <= 15
/// - the keypath conforms to bip48 for p2wsh: m/48'/coin'/account'/script_type'
/// - our designated xpub is actually ours (corresponds to the xpub of the currenty unlocked
///   keystore).
/// - no two xpubs are the same.
/// keypath: account-level keypath, e.g. m/48'/0'/10'/2'
/// expected:_coin expected bip44 coin in the keypath.
pub fn validate(multisig: &Multisig, keypath: &[u32], expected_coin: u32) -> Result<(), Error> {
    if multisig.xpubs.len() < 2 || multisig.xpubs.len() > MAX_SIGNERS {
        return Err(Error::InvalidInput);
    }
    if multisig.threshold == 0 || multisig.threshold > multisig.xpubs.len() as _ {
        return Err(Error::InvalidInput);
    }
    if multisig.our_xpub_index >= multisig.xpubs.len() as _ {
        return Err(Error::InvalidInput);
    }
    super::keypath::validate_account_multisig(
        keypath,
        expected_coin,
        ScriptType::from_i32(multisig.script_type).ok_or(Error::InvalidInput)?,
    )
    .or(Err(Error::InvalidInput))?;

    let our_xpub = {
        let our_xpub_str = bitbox02::keystore::encode_xpub_at_keypath(
            keypath,
            bitbox02::keystore::xpub_type_t::XPUB,
        )?;

        bs58::decode(&our_xpub_str)
            .with_check(None)
            .into_vec()
            .or(Err(Error::Generic))?
    };
    let maybe_our_xpub = crate::bip32::serialize_xpub(
        &multisig.xpubs[multisig.our_xpub_index as usize],
        Some(crate::bip32::XPubType::Xpub),
    )?;
    if our_xpub != maybe_our_xpub {
        return Err(Error::InvalidInput);
    }

    // Check for duplicates.
    if (1..multisig.xpubs.len()).any(|i| multisig.xpubs[i..].contains(&multisig.xpubs[i - 1])) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

pub fn process_is_script_config_registered(
    request: &pb::BtcIsScriptConfigRegisteredRequest,
) -> Result<Response, Error> {
    match request.registration.as_ref() {
        Some(pb::BtcScriptConfigRegistration {
            coin,
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::Multisig(multisig)),
                }),
            keypath,
        }) => {
            let coin = BtcCoin::from_i32(*coin).ok_or(Error::InvalidInput)?;
            Ok(Response::IsScriptConfigRegistered(
                pb::BtcIsScriptConfigRegisteredResponse {
                    is_registered: get_name(coin, multisig, keypath)?.is_some(),
                },
            ))
        }
        _ => Err(Error::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bip32::parse_xpub;
    use bitbox02::testing::{mock_memory, mock_unlocked_using_mnemonic};
    use util::bip32::HARDENED;

    #[test]
    fn test_get_hash() {
        /* Fixture below verified with:
        import hashlib
        import base58

        threshold = 1
        xpubs = [
            "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
        ]

        keypath = [48 + 0x80000000, 0 + 0x80000000, 10 + 0x80000000, 2 + 0x80000000]

        i32 = lambda i: i.to_bytes(4, 'little')

        msg = []
        msg.append(b'\x00') # coin
        msg.append(b'\x00') # script config type
        msg.append(i32(threshold))
        msg.append(i32(len(xpubs)))
        msg.extend(base58.b58decode_check(xpub)[4:] for xpub in xpubs)
        msg.append(i32(len(keypath)))
        msg.extend(i32(k) for k in keypath)
        print(hashlib.sha256(b''.join(msg)).hexdigest())
        */

        let keypath: &[u32] = &[48 + HARDENED, 0 + HARDENED, 10 + HARDENED, 2 + HARDENED];
        let multisig = Multisig {
            threshold: 1,
            xpubs: vec![
                parse_xpub("xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo").unwrap(),
                parse_xpub("xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF").unwrap(),
            ],
            our_xpub_index: 0,
            script_type: ScriptType::P2wsh as _,
        };

        assert_eq!(
            get_hash(BtcCoin::Btc, &multisig, SortXpubs::No, keypath).unwrap(),
            hex::decode("b0267fbb26ba0e74bad825c987949f58ba22aa75f63b539986dd937607bb4dc3")
                .unwrap(),
        );
        assert_eq!(
            get_hash(BtcCoin::Tbtc, &multisig, SortXpubs::No, keypath).unwrap(),
            hex::decode("3800cb87a1e346eb4a61e25c4775e663f613090aa2bf3fddb057462d174b56ef")
                .unwrap(),
        );
        assert_eq!(
            get_hash(BtcCoin::Ltc, &multisig, SortXpubs::No, keypath).unwrap(),
            hex::decode("6cf181d3e131eafefd4258084e5e48366a32d59be80a0afb13345589294ccf2d")
                .unwrap(),
        );
        assert_eq!(
            get_hash(BtcCoin::Tltc, &multisig, SortXpubs::No, keypath).unwrap(),
            hex::decode("0e5ee1d18a74d22cf7e3255a3529b9a453e9b080005ca0bd886f6decf9e4b845")
                .unwrap(),
        );

        let multisig_p2wsh_p2sh = Multisig {
            threshold: 1,
            xpubs: vec![
                parse_xpub("xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo").unwrap(),
                parse_xpub("xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF").unwrap(),
            ],
            our_xpub_index: 0,
            script_type: ScriptType::P2wshP2sh as _,
        };
        assert_eq!(
            get_hash(BtcCoin::Btc, &multisig_p2wsh_p2sh, SortXpubs::No, keypath).unwrap(),
            hex::decode("24513114c36f5c1f82d7b30c1431fad248d062dfa133d0f52ca85708b5a3fc2c")
                .unwrap(),
        );

        // Test that the hash is correct, and the same for all xpubs permutations if xpubs sort is
        // enabled.
        //
        // Generated with the help of:
        // import pprint, itertools; pprint.pprint(list(itertools.permutations(xpubs, len(xpubs))))
        let permutations = &[
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
            ],
            &[
                "xpub6EKzE8gaKnHbQ2HcBFXqBrnFc8LCjSUYs7x6iSh8CZNeM9EG6GzSCPTnmEKp4uHxiWZnXW7k3VDbtvPVp9B3JX3ZQM58BPSa3dPr47BU3Me",
                "xpub6CSYrpTrpg21tLhTfvDBRKjfgz6EjC3UYoS9ZtUaoyc3XDqLAEi5SnKALd4gcPpfmbFxHWYJDuFbTwyivqNQc5PgZXv4yJECu8fjnxEKeRn",
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
            ],
        ];

        for xpubs in permutations.iter() {
            let multisig = Multisig {
                threshold: 1,
                xpubs: xpubs.iter().map(|xpub| parse_xpub(xpub).unwrap()).collect(),
                our_xpub_index: 0,
                script_type: ScriptType::P2wsh as _,
            };
            assert_eq!(
                get_hash(BtcCoin::Btc, &multisig, SortXpubs::Yes, keypath).unwrap(),
                hex::decode("e09011232d85b49a9fd5b83d6bef42ff60a50b69b56218333cb61d93c1567fbe")
                    .unwrap(),
            );
        }
    }

    #[test]
    fn test_process_is_script_config_registered() {
        fn test(sort_xpubs: SortXpubs) {
            mock_memory();

            let keypath = &[48 + HARDENED, 0 + HARDENED, 10 + HARDENED, 2 + HARDENED];
            // The xpubs in this test are deliberately not ordered correctly to test that ordering
            // does not matter.
            let multisig = Multisig {
                threshold: 1,
                xpubs: vec![
                    parse_xpub("xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo").unwrap(),
                    parse_xpub("xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF").unwrap(),
                ],
                our_xpub_index: 0,
                script_type: ScriptType::P2wsh as _,
            };
            let hash = &get_hash(BtcCoin::Btc, &multisig, sort_xpubs, keypath).unwrap();
            let request = pb::BtcIsScriptConfigRegisteredRequest {
                registration: Some(pb::BtcScriptConfigRegistration {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::Multisig(multisig)),
                    }),
                    keypath: keypath.to_vec(),
                }),
            };
            assert_eq!(
                process_is_script_config_registered(&request),
                Ok(Response::IsScriptConfigRegistered(
                    pb::BtcIsScriptConfigRegisteredResponse {
                        is_registered: false,
                    },
                ))
            );

            bitbox02::memory::multisig_set_by_hash(hash, "some name").unwrap();
            assert_eq!(
                process_is_script_config_registered(&request),
                Ok(Response::IsScriptConfigRegistered(
                    pb::BtcIsScriptConfigRegisteredResponse {
                        is_registered: true,
                    },
                ))
            );
        }

        // Registration based on the hash using unsorted xpubs for backwards compatbility.
        test(SortXpubs::No);
        test(SortXpubs::Yes);
    }

    #[test]
    fn test_validate() {
        let expected_coin = 1 + HARDENED;
        let keypath = &[48 + HARDENED, expected_coin, 0 + HARDENED, 2 + HARDENED];
        let our_xpub_str = "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF";
        let multisig = Multisig {
            threshold: 1,
            xpubs: vec![
                parse_xpub("xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo").unwrap(),
                // this xpub corresponds to the mocked seed above at m/48'/1'/0'/2.
                parse_xpub(our_xpub_str).unwrap(),
            ],
            our_xpub_index: 1,
            script_type: ScriptType::P2wsh as _,
        };

        // Keystore locked.
        bitbox02::keystore::lock();
        assert!(validate(&multisig, keypath, expected_coin).is_err());

        // ok.
        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
        );
        assert!(validate(&multisig, keypath, expected_coin).is_ok());

        {
            // number of cosigners too large

            let mut invalid = multisig.clone();
            invalid.xpubs = [
                "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo",
                our_xpub_str,
                "xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ",
                "xpub6EQcxF2jFkYGn89AwoQJEEJkYMbRjED9AZgt7bkxQA5BLhZEoaQpUHcADbB5GxcMrTdDSGmjP7M3u462Q9otyE2PPam66P5KFLWitPVfYz9",
                "xpub6EP4EycVS5dq1PN7ZqsxBtptkYhfLvLGokZjnB3fvPshMiAohh6E5TaJjAafZWoPRjo6uiZxhtDXLgCuk81ooQgwrsnEdfSWSfa4VUtX8nu",
                "xpub6Eszd4BGGmHShcGtys5gbvV2zrBtW1gaorKf9YuvV4L3bePw7XePyyb2DKswZ5AhFfkcQwjQsiJEUTKhfRstRdHZUjQnJ2RJoQqL8g7FS4b",
                "xpub6Df3nbvH6P3FTvjgKaZcSuydyEofK545U4Bb15JY8R9MtFkKrhYrc3bpEF6fHtNM7xQ1qHwsVpS56TJWUjbKcmRwPkQr17ovV2RaVSJaBq3",
                "xpub6FQQ62gUYzS9wnHWHMPLWrpVnzS8xAf8XvfW1xzXEXTkTCtBrfbeww2zNeCgm3PbueMoq8opQvQDzp5Yf9EtiqVd7d1ASDoWSC1m7g1KHza",
                "xpub6EQNZUUAzJAoFAVVetYUrFVrf7mLyYsnHiQihkA3KPhoRHx7m6SgKBYV4z5Rd9CvUc11ACN8Ap5Wxigt6GYRPUqXGFfm3833ezJpjAmvJKt",
                "xpub6EGZy7cizYn2zUf9NT4qJ3Kr1ZrxdzPRcv2CwAnB1BTGWw7n9ZgDYvwmzzJXM6V7AgZ6CL3DrARZk5DzM9o8tz2RVTeC7QoHh9SxbW3b7Pw",
                "xpub6DaV7oCAkm4HJQMoProrrKYq1RvcgpStgYUCzLRaaeJSBSy9WBRFMNnQyAWJUYy9myUFRTvogq1C2f7x4A2yhtYgr7gL6eZXv2eJvzU12pe",
                "xpub6FFVRbdHt5DgHqR69KuWXRVDp93e1xKxv8rRLwhhCGnWaoF1ecnfdxpg2Nf1pvJTgT1UYg28CVt7YbUXFJL86vi9FaPN9QGtWLeCmf9dA24",
                "xpub6FNywxebMjvSSginZrk7DfNmAHvPJAy3j6pJ9FmUQCoh4FKPzNymdHnkA1z77Ke4GK7g5GkdrBhpyXfWTbZkH6Yo1t4v524wDwF8SAKny9J",
                "xpub6F1V9y6gXejomurTy2hN1UDCJidYahVkqtQJSZLYmcPcPDWkxGgWTrrLnCrCkGESSUSq6GpVVQx9kejPV97BEa9F85utABNL9r6xyPZFiDm",
                "xpub6ECHc4kmTC2tQg2ZoAoazwyag9C4V6yFsZEhjwMJixdVNsUibot6uEvsZY38ZLVqWCtyc9gbzFEwHQLHCT8EiDDKSNNsFAB8NQYRgkiAQwu",
                "xpub6F7CaxXzBCtvXwpRi61KYyhBRkgT1856ujHV5AbJK6ySCUYoDruBH6Pnsi6eHkDiuKuAJ2tSc9x3emP7aax9Dc3u7nP7RCQXEjLKihQu6w1",
            ].iter().map(|s| parse_xpub(s).unwrap()).collect();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // number of cosigners too small

            let mut invalid = multisig.clone();
            invalid.xpubs = vec![];
            assert!(validate(&invalid, keypath, expected_coin).is_err());
            invalid.our_xpub_index = 0;
            invalid.xpubs = vec![parse_xpub(our_xpub_str).unwrap()];
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // threshold larger than number of cosigners
            let mut invalid = multisig.clone();
            invalid.threshold = 3;
            assert!(validate(&invalid, keypath, expected_coin).is_err());

            // threshold zero
            invalid.threshold = 0;
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // our xpub index larger than number of cosigners (xpubs[our_xpb_index] would be out of
            // bounds).
            let mut invalid = multisig.clone();
            invalid.our_xpub_index = 2;
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // invalid keypath, wrong purpose
            let mut invalid = multisig.clone();
            let keypath = &[49 + HARDENED, expected_coin, 0 + HARDENED, 2 + HARDENED];
            invalid.xpubs[1] = parse_xpub(
                &bitbox02::keystore::encode_xpub_at_keypath(
                    keypath,
                    bitbox02::keystore::xpub_type_t::XPUB,
                )
                .unwrap(),
            )
            .unwrap();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // invalid keypath, wrong coin

            let mut invalid = multisig.clone();
            let keypath = &[48 + HARDENED, expected_coin + 1, 0 + HARDENED, 2 + HARDENED];
            invalid.xpubs[1] = parse_xpub(
                &bitbox02::keystore::encode_xpub_at_keypath(
                    keypath,
                    bitbox02::keystore::xpub_type_t::XPUB,
                )
                .unwrap(),
            )
            .unwrap();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // invalid keypath, account too large

            let mut invalid = multisig.clone();
            let keypath = &[48 + HARDENED, expected_coin, 100 + HARDENED, 2 + HARDENED];
            invalid.xpubs[1] = parse_xpub(
                &bitbox02::keystore::encode_xpub_at_keypath(
                    keypath,
                    bitbox02::keystore::xpub_type_t::XPUB,
                )
                .unwrap(),
            )
            .unwrap();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // invalid keypath, wrong script type

            let mut invalid = multisig.clone();
            let keypath = &[48 + HARDENED, expected_coin, 0 + HARDENED, 1 + HARDENED];
            invalid.xpubs[1] = parse_xpub(
                &bitbox02::keystore::encode_xpub_at_keypath(
                    keypath,
                    bitbox02::keystore::xpub_type_t::XPUB,
                )
                .unwrap(),
            )
            .unwrap();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // our xpub is not part of the multisig (overwrite our xpub with an arbitrary other one).

            let mut invalid = multisig.clone();
            invalid.xpubs[1] = parse_xpub("xpub6FNT7x2ZEBMhs4jvZJSEBV2qBCBnRidNsyqe7inT9V2wmEn4sqidTEudB4dVSvEjXz2NytcymwWJb8PPYExRycNf9SH8fAHzPWUsQJAmbR3").unwrap();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }

        {
            // duplicate

            let mut invalid = multisig.clone();
            invalid.xpubs[0] = invalid.xpubs[1].clone();
            assert!(validate(&invalid, keypath, expected_coin).is_err());
        }
    }
}
