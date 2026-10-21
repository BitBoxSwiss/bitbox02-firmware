// SPDX-License-Identifier: Apache-2.0

use super::super::Error;
use super::super::keypath::validate_address_shelley_stake;
use super::super::params;
use super::super::pb;
use crate::hal::ui::ConfirmParams;
use crate::i18n::I18n as _;

use alloc::string::ToString;
use alloc::vec::Vec;

use pb::cardano_sign_transaction_request::{
    Certificate, certificate,
    certificate::Cert::{StakeDelegation, StakeDeregistration, StakeRegistration, VoteDelegation},
};

use crate::hal::Ui;
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
                let account = (keypath[2] + 1 - HARDENED).to_string();
                let body =
                    crate::tr_format!(hal, "Register staking key for account #{}?", &[&account]);
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: params.name,
                        body: &body,
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
                let account = (keypath[2] + 1 - HARDENED).to_string();
                let body =
                    crate::tr_format!(hal, "Stop stake delegation for account #{}?", &[&account]);
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: params.name,
                        body: &body,
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
                let account = (keypath[2] + 1 - HARDENED).to_string();
                let pool = hex::encode(pool_keyhash);
                let body = crate::tr_format!(
                    hal,
                    "Delegate staking for account #{} to pool {}?",
                    &[&account, &pool],
                );
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: params.name,
                        body: &body,
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
                        certificate::vote_delegation::CardanoDRepType::KeyHash => {
                            crate::tr!(hal, "Key Hash")
                        }
                        certificate::vote_delegation::CardanoDRepType::ScriptHash => {
                            crate::tr!(hal, "Script Hash")
                        }
                        certificate::vote_delegation::CardanoDRepType::AlwaysAbstain => {
                            crate::tr!(hal, "Always Abstain")
                        }
                        certificate::vote_delegation::CardanoDRepType::AlwaysNoConfidence => {
                            crate::tr!(hal, "Always No Confidence")
                        }
                    };
                let account = (keypath[2] + 1 - HARDENED).to_string();
                match drep_credhash {
                    Some(hash) => {
                        let hash = hex::encode(hash);
                        let body = crate::tr_format!(
                            hal,
                            "Delegate voting for account #{} to type {} and drep {}?",
                            &[&account, drep_type_name.as_ref(), &hash],
                        );
                        hal.ui()
                            .confirm(&ConfirmParams {
                                title: params.name,
                                body: &body,
                                scrollable: true,
                                accept_is_nextarrow: true,
                                ..Default::default()
                            })
                            .await?;
                    }
                    None => {
                        let body = crate::tr_format!(
                            hal,
                            "Delegate voting for account #{} to type {}?",
                            &[&account, drep_type_name.as_ref()],
                        );
                        hal.ui()
                            .confirm(&ConfirmParams {
                                title: params.name,
                                body: &body,
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
