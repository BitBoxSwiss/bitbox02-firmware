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

use super::params;
use super::pb;
use super::Error;

use alloc::string::String;

use pb::btc_register_script_config_request::XPubType;
use pb::btc_response::Response;
use pb::btc_script_config::Config;
use pb::BtcCoin;

use super::multisig::SortXpubs;

use crate::hal::Ui;
use crate::workflow::{confirm, trinary_input_string};

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
            let coin = BtcCoin::try_from(*coin)?;
            Ok(Response::IsScriptConfigRegistered(
                pb::BtcIsScriptConfigRegisteredResponse {
                    is_registered: super::multisig::get_name(coin, multisig, keypath)?.is_some(),
                },
            ))
        }
        Some(pb::BtcScriptConfigRegistration {
            coin,
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::Policy(policy)),
                }),
            ..
        }) => {
            let coin = BtcCoin::try_from(*coin)?;
            Ok(Response::IsScriptConfigRegistered(
                pb::BtcIsScriptConfigRegisteredResponse {
                    is_registered: super::policies::get_name(coin, policy)?.is_some(),
                },
            ))
        }

        _ => Err(Error::InvalidInput),
    }
}

async fn get_name(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcRegisterScriptConfigRequest,
) -> Result<String, Error> {
    let name = if request.name.is_empty() {
        hal.ui()
            .confirm(&confirm::Params {
                title: "Register",
                body: "Please name this\naccount",
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;

        let name = hal
            .ui()
            .enter_string(
                &trinary_input_string::Params {
                    title: "Enter account name",
                    longtouch: true,
                    ..Default::default()
                },
                trinary_input_string::CanCancel::Yes,
                "",
            )
            .await?;
        // We truncate the user input string to fit into the maximum allowed multisig
        // account name length. This is not very nice, but it has to do until we have some
        // sort of indication in the input component.
        bitbox02::util::truncate_str(name.as_str(), bitbox02::memory::MULTISIG_NAME_MAX_LEN).into()
    } else {
        request.name.clone()
    };
    if !util::name::validate(&name, bitbox02::memory::MULTISIG_NAME_MAX_LEN) {
        return Err(Error::InvalidInput);
    }
    Ok(name)
}

pub async fn process_register_script_config(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcRegisterScriptConfigRequest,
) -> Result<Response, Error> {
    let title = "Register";
    match request.registration.as_ref() {
        Some(pb::BtcScriptConfigRegistration {
            coin,
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::Multisig(multisig)),
                }),
            keypath,
        }) => {
            let coin = BtcCoin::try_from(*coin)?;
            let coin_params = params::get(coin);
            let name = get_name(hal, request).await?;
            super::multisig::validate(multisig, keypath)?;
            let xpub_type = XPubType::try_from(request.xpub_type)?;
            super::multisig::confirm_extended(
                hal,
                title,
                coin_params,
                &name,
                multisig,
                xpub_type,
                keypath,
            )
            .await?;
            let hash = super::multisig::get_hash(coin, multisig, SortXpubs::Yes, keypath)?;
            match bitbox02::memory::multisig_set_by_hash(&hash, &name) {
                Ok(()) => {
                    hal.ui().status("Multisig account\nregistered", true).await;
                    Ok(Response::Success(pb::BtcSuccess {}))
                }
                Err(bitbox02::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME) => {
                    Err(Error::Duplicate)
                }
                Err(_) => Err(Error::Generic),
            }
        }
        Some(pb::BtcScriptConfigRegistration {
            coin,
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::Policy(policy)),
                }),
            ..
        }) => {
            let coin = BtcCoin::try_from(*coin)?;
            let coin_params = params::get(coin);
            let name = get_name(hal, request).await?;
            let parsed = super::policies::parse(policy, coin)?;
            parsed
                .confirm(
                    hal,
                    title,
                    coin_params,
                    &name,
                    super::policies::Mode::Advanced,
                )
                .await?;
            let hash = super::policies::get_hash(coin, policy)?;
            match bitbox02::memory::multisig_set_by_hash(&hash, &name) {
                Ok(()) => {
                    hal.ui().status("Policy\nregistered", true).await;
                    Ok(Response::Success(pb::BtcSuccess {}))
                }
                Err(bitbox02::memory::MemoryError::MEMORY_ERR_DUPLICATE_NAME) => {
                    Err(Error::Duplicate)
                }
                Err(_) => Err(Error::Generic),
            }
        }
        // Only multisig and policy registration supported for now.
        _ => Err(Error::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bip32::parse_xpub;
    use bitbox02::testing::{mock_memory, mock_unlocked_using_mnemonic};
    use util::bip32::HARDENED;

    use pb::btc_script_config::{multisig::ScriptType, Multisig};

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
            let hash =
                &super::super::multisig::get_hash(BtcCoin::Btc, &multisig, sort_xpubs, keypath)
                    .unwrap();
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
