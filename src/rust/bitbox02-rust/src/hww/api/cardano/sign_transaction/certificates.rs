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

use super::super::keypath::validate_address_shelley_stake;
use super::super::params;
use super::super::pb;
use super::super::Error;

use alloc::vec::Vec;

use pb::cardano_sign_transaction_request::{
    certificate,
    certificate::Cert::{StakeDelegation, StakeDeregistration, StakeRegistration, VoteDelegation},
    Certificate,
};

use crate::hal::Ui;
use crate::workflow::confirm;
use util::bip32::HARDENED;

pub async fn verify<'a>(
    hal: &mut impl crate::hal::Hal,
    params: &params::Params,
    certificates: &'a [Certificate],
    bip44_account: u32,
    signing_keypaths: &mut Vec<&'a [u32]>,
) -> Result<(), Error> {
    for Certificate { cert } in certificates {
        let cert = cert.as_ref().ok_or(Error::InvalidInput)?;
        match cert {
            StakeRegistration(pb::Keypath { keypath }) => {
                validate_address_shelley_stake(keypath, Some(bip44_account))?;
                signing_keypaths.push(keypath);
                // 2 ADA will be deposited and refunded once delegation stops, independent of the staking rewards.
                hal.ui()
                    .confirm(&confirm::Params {
                        title: params.name,
                        body: &format!(
                            "Register staking key for account #{}?",
                            keypath[2] + 1 - HARDENED
                        ),
                        scrollable: true,
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
            }
            StakeDeregistration(pb::Keypath { keypath }) => {
                validate_address_shelley_stake(keypath, Some(bip44_account))?;
                signing_keypaths.push(keypath);
                // 2 ADA will be refunded back, independent of the staking rewards.
                hal.ui()
                    .confirm(&confirm::Params {
                        title: params.name,
                        body: &format!(
                            "Stop stake delegation for account #{}?",
                            keypath[2] + 1 - HARDENED
                        ),
                        scrollable: true,
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
            }
            StakeDelegation(certificate::StakeDelegation {
                keypath,
                pool_keyhash,
            }) => {
                validate_address_shelley_stake(keypath, Some(bip44_account))?;
                signing_keypaths.push(keypath);
                hal.ui()
                    .confirm(&confirm::Params {
                        title: params.name,
                        body: &format!(
                            "Delegate staking for account #{} to pool {}?",
                            keypath[2] + 1 - HARDENED,
                            hex::encode(pool_keyhash),
                        ),
                        scrollable: true,
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
            }
            VoteDelegation(certificate::VoteDelegation {
                keypath,
                r#type,
                drep_credhash,
            }) => {
                validate_address_shelley_stake(keypath, Some(bip44_account))?;
                signing_keypaths.push(keypath);
                let drep_type_name =
                    match certificate::vote_delegation::CardanoDRepType::try_from(*r#type)? {
                        certificate::vote_delegation::CardanoDRepType::KeyHash => "Key Hash",
                        certificate::vote_delegation::CardanoDRepType::ScriptHash => "Script Hash",
                        certificate::vote_delegation::CardanoDRepType::AlwaysAbstain => {
                            "Always Abstain"
                        }
                        certificate::vote_delegation::CardanoDRepType::AlwaysNoConfidence => {
                            "Always No Confidence"
                        }
                    };
                match drep_credhash {
                    Some(hash) => {
                        hal.ui()
                            .confirm(&confirm::Params {
                                title: params.name,
                                body: &format!(
                                    "Delegate voting for account #{} to type {} and drep {}?",
                                    keypath[2] + 1 - HARDENED,
                                    drep_type_name,
                                    hex::encode(hash),
                                ),
                                scrollable: true,
                                accept_is_nextarrow: true,
                                ..Default::default()
                            })
                            .await?;
                    }
                    None => {
                        hal.ui()
                            .confirm(&confirm::Params {
                                title: params.name,
                                body: &format!(
                                    "Delegate voting for account #{} to type {}?",
                                    keypath[2] + 1 - HARDENED,
                                    drep_type_name,
                                ),
                                scrollable: true,
                                accept_is_nextarrow: true,
                                ..Default::default()
                            })
                            .await?;
                    }
                }
            }
        };
    }
    Ok(())
}
