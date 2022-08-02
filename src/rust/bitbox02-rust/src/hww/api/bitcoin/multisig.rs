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

use super::pb;
use super::Error;

use pb::btc_response::Response;
use pb::btc_script_config::{multisig::ScriptType, Config, Multisig};
use pb::BtcCoin;

use alloc::string::String;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

enum SortXpubs {
    No,
    Yes,
}

/// Creates a hash of this multisig config, useful for multisig account registration and
/// identification. The individual params are not validated, they must be pre-validated!
///
/// If `sort_xpubs` is `SortXpubs::Yes`, the xpubs are sorted before hashing.
/// The keypath is the account-level keypath.
fn get_hash(
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
            .map(super::common::serialize_xpub_no_version)
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
fn get_name(coin: BtcCoin, multisig: &Multisig, keypath: &[u32]) -> Result<Option<String>, ()> {
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

    use super::super::common::parse_xpub;
    use bitbox02::testing::mock_memory;
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
}
