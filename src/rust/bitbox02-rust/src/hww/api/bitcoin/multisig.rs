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

use pb::btc_register_script_config_request::XPubType;
use pb::btc_script_config::{multisig::ScriptType, Multisig};
use pb::BtcCoin;

use crate::bip32;

use crate::hal::Ui;
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
            BtcCoin::Rbtc => 0x04,
        };
        hasher.update(byte.to_le_bytes());
    }
    {
        // 2. script config type
        let byte: u8 = match ScriptType::try_from(multisig.script_type).map_err(|_| ())? {
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
            .map(|xpub| bip32::Xpub::from(xpub).serialize(None))
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
    hal: &mut impl crate::hal::Hal,
    title: &str,
    params: &Params,
    name: &str,
    multisig: &Multisig,
) -> Result<(), Error> {
    hal.ui()
        .confirm(&confirm::Params {
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
    hal.ui()
        .confirm(&confirm::Params {
            title,
            body: name,
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    Ok(())
}

/// Confirms a multisig setup with the user during account registration.
/// Verified are:
/// - coin
/// - multisig type (m-of-n)
/// - name given by the user
/// - script type (e.g. p2wsh, p2wsh-p2sh)
/// - account keypath
/// - all xpubs (formatted according to `xpub_type`).
///
/// xpub_type: if AUTO_ELECTRUM, will automatically format xpubs as `Zpub/Vpub`,
/// `Ypub/UPub` depending on the script type, to match Electrum's formatting. If AUTO_XPUB_TPUB,
/// format as xpub (mainnets) or tpub (testnets).
pub async fn confirm_extended(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    params: &Params,
    name: &str,
    multisig: &Multisig,
    xpub_type: XPubType,
    keypath: &[u32],
) -> Result<(), Error> {
    let script_type = ScriptType::try_from(multisig.script_type)?;

    confirm(hal, title, params, name, multisig).await?;
    hal.ui()
        .confirm(&confirm::Params {
            title,
            body: &format!(
                "{}\nat\n{}",
                match ScriptType::try_from(multisig.script_type)? {
                    ScriptType::P2wsh => "p2wsh",
                    ScriptType::P2wshP2sh => "p2wsh-p2sh",
                },
                util::bip32::to_string(keypath)
            ),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    // Confirm cosigners.
    let output_xpub_type: bip32::XPubType = match xpub_type {
        XPubType::AutoElectrum => match params.coin {
            BtcCoin::Btc | BtcCoin::Ltc => match script_type {
                ScriptType::P2wsh => bip32::XPubType::CapitalZpub,
                ScriptType::P2wshP2sh => bip32::XPubType::CapitalYpub,
            },
            BtcCoin::Tbtc | BtcCoin::Rbtc | BtcCoin::Tltc => match script_type {
                ScriptType::P2wsh => bip32::XPubType::CapitalVpub,
                ScriptType::P2wshP2sh => bip32::XPubType::CapitalUpub,
            },
        },
        XPubType::AutoXpubTpub => match params.coin {
            BtcCoin::Btc | BtcCoin::Ltc => bip32::XPubType::Xpub,
            BtcCoin::Tbtc | BtcCoin::Rbtc | BtcCoin::Tltc => bip32::XPubType::Tpub,
        },
    };
    let num_cosigners = multisig.xpubs.len();
    for (i, xpub) in multisig.xpubs.iter().enumerate() {
        let xpub_str = bip32::Xpub::from(xpub)
            .serialize_str(output_xpub_type)
            .or(Err(Error::InvalidInput))?;
        hal.ui()
            .confirm(&confirm::Params {
                title,
                body: (if i == multisig.our_xpub_index as usize {
                    format!(
                        "Cosigner {}/{} (this device): {}",
                        i + 1,
                        num_cosigners,
                        xpub_str
                    )
                } else {
                    format!("Cosigner {}/{}: {}", i + 1, num_cosigners, xpub_str)
                })
                .as_str(),
                scrollable: true,
                longtouch: i == num_cosigners - 1,
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }
    // TODO rest
    Ok(())
}

/// Validate a m-of-n multisig account. This includes checking that:
/// - 0 < m <= n <= 15
/// - the keypath conforms to bip48 for p2wsh: m/48'/coin'/account'/script_type'
/// - our designated xpub is actually ours (corresponds to the xpub of the currenty unlocked
///   keystore).
/// - no two xpubs are the same.
///
/// keypath: account-level keypath, e.g. m/48'/0'/10'/2'
pub fn validate(multisig: &Multisig, keypath: &[u32]) -> Result<(), Error> {
    if multisig.xpubs.len() < 2 || multisig.xpubs.len() > MAX_SIGNERS {
        return Err(Error::InvalidInput);
    }
    if multisig.threshold == 0 || multisig.threshold > multisig.xpubs.len() as _ {
        return Err(Error::InvalidInput);
    }
    if multisig.our_xpub_index >= multisig.xpubs.len() as _ {
        return Err(Error::InvalidInput);
    }

    let our_xpub = crate::keystore::get_xpub(keypath)?.serialize(None)?;
    let maybe_our_xpub =
        bip32::Xpub::from(&multisig.xpubs[multisig.our_xpub_index as usize]).serialize(None)?;
    if our_xpub != maybe_our_xpub {
        return Err(Error::InvalidInput);
    }

    // Check for duplicates.
    if (1..multisig.xpubs.len()).any(|i| multisig.xpubs[i..].contains(&multisig.xpubs[i - 1])) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

/// Creates a n-of-m multisig script based on OP_CHECKMULTISIG. 0<n<=m<=15.
/// Note that the multisig config and keypaths are *not* validated, this must be done before calling.
/// keypath_change is 0 for receive addresses, 1 for change addresses.
/// keypath_address is the receive address index.
pub fn pkscript(
    multisig: &Multisig,
    keypath_change: u32,
    keypath_address: u32,
) -> Result<Vec<u8>, Error> {
    if multisig.xpubs.len() < 2 || multisig.xpubs.len() > MAX_SIGNERS {
        return Err(Error::InvalidInput);
    }
    if multisig.threshold == 0 || multisig.threshold > multisig.xpubs.len() as _ {
        return Err(Error::InvalidInput);
    }
    let mut pubkeys: Vec<Vec<u8>> = multisig
        .xpubs
        .iter()
        .map(|xpub| -> Result<Vec<u8>, ()> {
            Ok(bip32::Xpub::from(xpub)
                .derive(&[keypath_change, keypath_address])?
                .public_key()
                .to_vec())
        })
        .collect::<Result<_, _>>()?;
    pubkeys.sort();

    let mut script_builder = bitcoin::script::Builder::new().push_int(multisig.threshold as _);
    for pk in pubkeys.iter() {
        let pk: &bitcoin::script::PushBytes =
            pk.as_slice().try_into().map_err(|_| Error::Generic)?;
        script_builder = script_builder.push_slice(pk);
    }
    script_builder = script_builder
        .push_int(pubkeys.len() as _)
        .push_opcode(bitcoin::opcodes::all::OP_CHECKMULTISIG);

    Ok(script_builder.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    use bip32::parse_xpub;
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
    fn test_validate() {
        let keypath = &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 2 + HARDENED];
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
        assert!(validate(&multisig, keypath).is_err());

        // Ok.
        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
            "",
        );
        assert!(validate(&multisig, keypath).is_ok());
        // Ok at arbitrary keypath.
        assert!(validate(&Multisig {
            threshold: 1,
            xpubs: vec![
                parse_xpub("xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo").unwrap(),
                // this xpub corresponds to the mocked seed above at m/45'.
                parse_xpub("xpub68yJakxtRe3azab9rb8DJqxDeCG7oBY3zhsNnvZybjTE9qc9Hgw4bCqdLjVGykZrwD6CC6r6xHrnuep5Dmb9uq2R4emCm8YzBuddFyhgvAD").unwrap(),
            ],
            our_xpub_index: 1,
            script_type: ScriptType::P2wsh as _,
        }, &[45 + HARDENED]).is_ok());

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
            assert!(validate(&invalid, keypath).is_err());
        }

        {
            // number of cosigners too small

            let mut invalid = multisig.clone();
            invalid.xpubs = vec![];
            assert!(validate(&invalid, keypath).is_err());
            invalid.our_xpub_index = 0;
            invalid.xpubs = vec![parse_xpub(our_xpub_str).unwrap()];
            assert!(validate(&invalid, keypath).is_err());
        }

        {
            // threshold larger than number of cosigners
            let mut invalid = multisig.clone();
            invalid.threshold = 3;
            assert!(validate(&invalid, keypath).is_err());

            // threshold zero
            invalid.threshold = 0;
            assert!(validate(&invalid, keypath).is_err());
        }

        {
            // our xpub index larger than number of cosigners (xpubs[our_xpb_index] would be out of
            // bounds).
            let mut invalid = multisig.clone();
            invalid.our_xpub_index = 2;
            assert!(validate(&invalid, keypath).is_err());
        }

        {
            // our xpub is not part of the multisig (overwrite our xpub with an arbitrary other one).

            let mut invalid = multisig.clone();
            invalid.xpubs[1] = parse_xpub("xpub6FNT7x2ZEBMhs4jvZJSEBV2qBCBnRidNsyqe7inT9V2wmEn4sqidTEudB4dVSvEjXz2NytcymwWJb8PPYExRycNf9SH8fAHzPWUsQJAmbR3").unwrap();
            assert!(validate(&invalid, keypath).is_err());
        }

        {
            // duplicate

            let mut invalid = multisig.clone();
            invalid.xpubs[0] = invalid.xpubs[1].clone();
            assert!(validate(&invalid, keypath).is_err());
        }
    }

    #[test]
    fn test_pkscript() {
        struct Test<'a> {
            threshold: u32,
            xpubs: &'a [&'a str],
            keypath_change: u32,
            keypath_address: u32,
            expected_script_hex: &'a str,
        }

        let tests = [
            Test {
                threshold: 1,
                xpubs: &[
                    "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                    "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUGh3oWzjHtgfQ3mtjPNFgNMronzTTVR",
                ],
                keypath_change: 0,
                keypath_address: 1,
                expected_script_hex: "51210217fb1e3415108fee2b004c932dc5a89eabf3587e3e7b21165c123de1f37a3a612102ae0826124c98c4e255c1a6cc404ff6d2448a0d9f853e6d72d6b02d9ad2d3565052ae",
            },
            // different xpub order should have the same result.
            Test {
                threshold: 1,
                xpubs: &[
                    "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUGh3oWzjHtgfQ3mtjPNFgNMronzTTVR",
                    "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                ],
                keypath_change: 0,
                keypath_address: 1,
                expected_script_hex: "51210217fb1e3415108fee2b004c932dc5a89eabf3587e3e7b21165c123de1f37a3a612102ae0826124c98c4e255c1a6cc404ff6d2448a0d9f853e6d72d6b02d9ad2d3565052ae",
            },
            Test {
                threshold: 1,
                xpubs: &[
                    "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                    "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUGh3oWzjHtgfQ3mtjPNFgNMronzTTVR",
                ],
                keypath_change: 1,
                keypath_address: 10,
                expected_script_hex: "512102b6da3d9e33c3bcee679ef3bb2fca8e60c4a8ade06519146c77b007778756b2c92103f42b45d0d91039df309ff5d10d0a044fb4eb6595d015281be2d56c288524d68f52ae",
            },
            Test {
                threshold: 2,
                xpubs: &[
                    "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                    "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUGh3oWzjHtgfQ3mtjPNFgNMronzTTVR",
                ],
                keypath_change: 0,
                keypath_address: 1,
                expected_script_hex: "52210217fb1e3415108fee2b004c932dc5a89eabf3587e3e7b21165c123de1f37a3a612102ae0826124c98c4e255c1a6cc404ff6d2448a0d9f853e6d72d6b02d9ad2d3565052ae",
            },
            Test {
                threshold: 15,
                xpubs: &[
                    "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                    "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUGh3oWzjHtgfQ3mtjPNFgNMronzTTVR",
                    "xpub6E9Qk6G1PAZPqheZ85sySQc9fxS8mp2muF9dNaXpnCGvW2NB13rCm4TKLo9vJaCyxcXBJPF2yBSkKuivLGA5fxuXhbRSL2Sp8HfgxEMFYD3",
                    "xpub6DxHJ5evyWcSBrG9zCauY1zrh3J6HkiBGLzgG4wvuRaDQYxF6suuPNh1hD2VktphRhEwWXECaWLXo1PkVkGn7hW6vq6AN3ZgqFUrQ7boHqs",
                    "xpub6EdCXJqHFRVqVCZpain6TMzkpmcU6pLU5jSzjUUouumdkzKUAmvBiTsVeJSwxdBzH5mLU1FEFka7jsrs1JeRbqJnwHE31bVF26gkJQ5SCs3",
                    "xpub6EG6LDy2hGg7NBUKyPzqe8k57Jm6H9WmH85MKKWGVTCbr5tVDt8oaKSAArXga4LbYy6Aawfzr324kXq4ia4vSkRBzPCktDv5XJbPRg3sXsz",
                    "xpub6EVoeT8mm6jfq5mtG3Kuv2ozffH1oRaLYsq88N1x7225QBzfBeZxbdx6sGYpFpkcJohzLHXhM7GjqqyrvxzkfvZjydSCGPbaDxWirKH3TRp",
                    "xpub6FNtqhDqFmHJxZsocfd2LftXzZAcDXK2ijhzcscsrsu46Ccz3uv7rrZYbFEvA98svjzkD49x8K2Mi2BuJuhyZHfTtBfdgeUc66JdCez8KG8",
                    "xpub6F1behqQRigaf5gbFbbdjrNV4M64UTQTrzEU535dURgBMJakSFpiZkXveqEscL6Y6gyveFwxp8PGKn3q9MLtwk1UmyRRkFCQb2X6hfvGYWt",
                    "xpub6FG3mVwCkRmtmFuCKZa6MXc4kCPEd5bKrjrNAPgwcmekysnsHBaadhuzo2jV2AjYyg4QjGmu3LgyEUAw4bUXPUsQJG61ZtKM7MVkBxbxcVj",
                    "xpub6DoEJEeUNBkLF7zmKKu8YewqK1PcXWfuek2J2Y8USdGh2McQStsGbVn2oqv521KdJiESeRW4mBBtpBamKHNaD6yZhAbyPwy51VyqHS4EFq6",
                    "xpub6EzUjWSuWk7kBKZTKsdXkFMUropKFLq1iWabRtQpXckxf6s9NMR8UrmY6aYQUuvHyXpYo78RJhyZ1sK9Re4ZbdzpG4Awm6yW221N2AQM6ZU",
                    "xpub6DeznbrZVRaZ4P5Xr79JBs8dNyBMamFmAgAX52o73Pap5RLkMmUi9oQH1sopigbSr6gwUoDMd3EhpoB5tBZXzu4HWJiGETKQGneYtRpjaJB",
                    "xpub6EYf1KXzjaTgcNZFq7pVXGtGDkqHFPvEGBDygkDodz94ZpDazWppGe57hDhTA94z6zeGEubqyLqUMP67ubdd8hf6BbKYA9qtdDf3yM5wdJX",
                    "xpub6ELR9CAGqxwbKcCh591AfKs74neEY9UjtNbvLjrpsxH2FakqE238J1DmsFHePtXXyYhkZshW3qTWWwhENTQgWb6KHaf7SQnVovsKxtvZQaG",
                ],
                keypath_change: 0,
                keypath_address: 1,
                expected_script_hex: "5f210210e4a9e6d84a7d4b88d5f0450ade30de2046f824374f9b4954a6f03bd37b7269210217fb1e341\
                                      5108fee2b004c932dc5a89eabf3587e3e7b21165c123de1f37a3a61210219ad58aa89a3e1669b0757b7\
                                      b87d72350cd94675421365a9b7ae781fabeb04ec210230a8551d874b4a3633195c1ba80d0fd5d4e6cf7\
                                      917b07f00379893490f795fbe210242f82d15933cf3487567405699910eae5c4b5b24821eeaceeac0ea\
                                      da231a760421024be1e5f4fd6c4248b05df752d19754aad4ca663f62f20fd7ac54616899870ebc21024\
                                      d5cae14247c53ec7943a78ddb016a939e98756526587ec4bb72789334e698292102ae0826124c98c4e2\
                                      55c1a6cc404ff6d2448a0d9f853e6d72d6b02d9ad2d356502102cd014c5921c2f40c0b8de3cf32f9b67\
                                      89737e2a06677c4da7325623bcb0af89421033f63c02d09195b9c7efb7b75e18da8b768b5c3e0517082\
                                      98d6580634284c28122103410a5da3477482eea7be703bd81d00d4498b7babfbd25f7c930a137a5025c\
                                      0b721035b0322eeec4356d59edf4b6213cf78409c6f2e05c26e65b04c503f98a38ec78b21037ff295f8\
                                      45fabf9eb4ada869bfa62bde1ede38f074b12bf12a2a2f214282cef82103aef77f1780440ba2445aef6\
                                      d3ecf5d0b8dae3b6f22abc44734e1d4c257dc631f2103cd01c7cd59d6956bf07f1e7acba7c41a126ba5\
                                      49c07d0c88988c94846ecd88005fae",
            },
        ];

        for test in tests {
            assert_eq!(
                hex::encode(
                    pkscript(
                        &Multisig {
                            threshold: test.threshold,
                            xpubs: test
                                .xpubs
                                .iter()
                                .map(|xpub| parse_xpub(xpub).unwrap())
                                .collect(),
                            our_xpub_index: 0,
                            script_type: ScriptType::P2wsh as _
                        },
                        test.keypath_change,
                        test.keypath_address
                    )
                    .unwrap()
                )
                .as_str(),
                test.expected_script_hex
            );
        }
    }

    #[test]
    fn test_pkscript_unhappy() {
        struct Test<'a> {
            threshold: u32,
            xpubs: &'a [&'a str],
        }

        let tests = [
            Test {
                threshold: 1,
                xpubs: &[],
            },
            Test {
                threshold: 0,
                xpubs: &[
                    "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                    "xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM7wT3KxLKX8Q8keZPd67kRGmrD1WJj",
                ],
            },
            Test {
                threshold: 3,
                xpubs: &[
                    "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                    "xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM7wT3KxLKX8Q8keZPd67kRGmrD1WJj",
                ],
            },
        ];

        for test in tests {
            assert!(pkscript(
                &Multisig {
                    threshold: test.threshold,
                    xpubs: test
                        .xpubs
                        .iter()
                        .map(|xpub| parse_xpub(xpub).unwrap())
                        .collect(),
                    our_xpub_index: 0,
                    script_type: ScriptType::P2wsh as _
                },
                1,
                2,
            )
            .is_err());
        }
    }
}
