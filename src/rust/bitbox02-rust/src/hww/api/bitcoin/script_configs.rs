// Copyright 2024 Shift Crypto AG
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

use alloc::string::String;

use super::pb;
use super::Error;
use pb::btc_script_config::{Multisig, SimpleType};

use super::policies::ParsedPolicy;

use util::bip32::HARDENED;

/// Parsed and validated form of `pb::BtcScriptConfig`.
pub enum ValidatedScriptConfig<'a> {
    SimpleType(SimpleType),
    Multisig {
        name: String,
        multisig: &'a Multisig,
    },
    Policy {
        name: String,
        parsed_policy: ParsedPolicy<'a>,
    },
}

/// Parsed and validated form of `pb::BtcScriptConfigWithKeypath`.
pub struct ValidatedScriptConfigWithKeypath<'a> {
    pub keypath: &'a [u32],
    pub config: ValidatedScriptConfig<'a>,
}

impl ValidatedScriptConfigWithKeypath<'_> {
    /// Get a string representation of the account script config to show to the user when they send
    /// coins to an address belonging to the same keystore.
    pub fn self_transfer_representation(&self) -> Result<String, Error> {
        match self {
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(_),
            } => {
                let keypath_account_element = keypath.get(2).ok_or(Error::Generic)?;
                Ok(format!(
                    "This BitBox (account #{})",
                    keypath_account_element
                        .checked_sub(HARDENED)
                        .ok_or(Error::Generic)?
                        + 1
                ))
            }
            ValidatedScriptConfigWithKeypath {
                config: ValidatedScriptConfig::Multisig { name, .. },
                ..
            }
            | ValidatedScriptConfigWithKeypath {
                config: ValidatedScriptConfig::Policy { name, .. },
                ..
            } => Ok(format!("This BitBox - {}", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bip32::parse_xpub;
    use bitbox02::testing::{mock_memory, mock_unlocked};

    #[test]
    fn test_self_transfer_representation_simple_type() {
        // Same text repr for all simple types.
        for simple_type in [SimpleType::P2wpkhP2sh, SimpleType::P2wpkh, SimpleType::P2tr] {
            let config = ValidatedScriptConfigWithKeypath {
                keypath: &[84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                config: ValidatedScriptConfig::SimpleType(simple_type),
            };
            assert_eq!(
                config.self_transfer_representation().unwrap(),
                "This BitBox (account #11)"
            )
        }
    }

    #[test]
    fn test_self_transfer_representation_multisig() {
        let xpub1 = parse_xpub("xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF").unwrap();
        let xpub2 = parse_xpub("xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM7wT3KxLKX8Q8keZPd67kRGmrD1WJj").unwrap();
        let multisig = pb::btc_script_config::Multisig {
            threshold: 1,
            xpubs: vec![xpub1, xpub2],
            our_xpub_index: 0,
            script_type: pb::btc_script_config::multisig::ScriptType::P2wsh as _,
        };
        let keypath = &[48 + HARDENED, 0 + HARDENED, 0 + HARDENED, 2 + HARDENED];
        let config = ValidatedScriptConfigWithKeypath {
            keypath,
            config: ValidatedScriptConfig::Multisig {
                name: "test multisig account name".into(),
                multisig: &multisig,
            },
        };

        assert_eq!(
            config.self_transfer_representation().unwrap().as_str(),
            "This BitBox - test multisig account name",
        )
    }

    #[test]
    fn test_self_transfer_representation_policy() {
        let keypath = &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED];
        let policy = pb::btc_script_config::Policy {
            policy: "wsh(multi(2,@0/**,@1/**))".into(),
            keys: vec![
                pb::KeyOriginInfo {
                    root_fingerprint: crate::keystore::root_fingerprint().unwrap(),
                    keypath: keypath.to_vec(),
                    xpub: Some(crate::keystore::get_xpub(keypath).unwrap().into()),
                },
                pb::KeyOriginInfo {
                    root_fingerprint: vec![],
                    keypath: vec![],
                    xpub: Some(parse_xpub("tpubDFGkUYFfEhAALSXQ9VNssUq71HWYLWLK7sAEqFyqJBQxQ4uGSBW1RSBkoVfijE6iEHZFs2kZrVzzV1nZCSEXYKudtsfEWcWKVXvjjLeRyd8").unwrap()),
                },
            ],
        };

        let parsed_policy = super::super::policies::parse(&policy, pb::BtcCoin::Btc).unwrap();
        let config = ValidatedScriptConfigWithKeypath {
            keypath,
            config: ValidatedScriptConfig::Policy {
                name: "test policy account name".into(),
                parsed_policy,
            },
        };

        assert_eq!(
            config.self_transfer_representation().unwrap().as_str(),
            "This BitBox - test policy account name",
        )
    }
}
