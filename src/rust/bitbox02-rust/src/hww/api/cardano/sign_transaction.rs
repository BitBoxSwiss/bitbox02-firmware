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

mod cbor;
mod certificates;

use super::pb;
use super::Error;

use alloc::string::String;
use alloc::vec::Vec;

use bech32::{ToBase32, Variant};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use crate::workflow::{confirm, status, transaction};

use pb::cardano_response::Response;
use pb::cardano_sign_transaction_response::ShelleyWitness;
use pb::{CardanoNetwork, CardanoScriptConfig};

use super::params;
use crate::keystore::ed25519;

use util::bip32::HARDENED;

// 1 ADA = 1e6 lovelaces
const LOVELACE_DECIMALS: usize = 6;

// Mainnet params.
// Start of Shelley era
const SHELLEY_START_EPOCH: u64 = 208;
// 21600 are the slots per epoch before Shelley.
const SHELLEY_START_SLOT: u64 = SHELLEY_START_EPOCH * 21600;
const SHELLEY_SLOTS_IN_EPOCH: u64 = 432000;

fn format_value(params: &params::Params, value: u64) -> String {
    format!(
        "{} {}",
        util::decimal::format(value, LOVELACE_DECIMALS),
        params.unit
    )
}

fn make_shelley_witness(keypath: &[u32], tx_body_hash: &[u8; 32]) -> Result<ShelleyWitness, ()> {
    let result = ed25519::sign(keypath, tx_body_hash)?;
    Ok(ShelleyWitness {
        public_key: result.public_key.as_ref().to_vec(),
        signature: result.signature.to_vec(),
    })
}

// For Cardano mainnet, this formats a slot number in terms of epoch
// and relative slot number to the epoch and lets the user verify it.
// This should only be called for mainnet.
async fn verify_slot(params: &params::Params, title: &str, slot: u64) -> Result<(), Error> {
    if slot < SHELLEY_START_SLOT {
        return Err(Error::InvalidInput);
    }
    let epoch = SHELLEY_START_EPOCH + (slot - SHELLEY_START_SLOT) / SHELLEY_SLOTS_IN_EPOCH;
    let slot_in_epoch = (slot - SHELLEY_START_SLOT) % SHELLEY_SLOTS_IN_EPOCH;
    confirm::confirm(&confirm::Params {
        title: params.name,
        body: &format!("{}\nslot {} in\nepoch {}", title, slot_in_epoch, epoch),
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;
    Ok(())
}

/// Format an asset fingerprint according to CIP-14.
/// https://github.com/cardano-foundation/CIPs/blob/a2ef32d8a2b485fed7f6ffde2781dd58869ff511/CIP-0014/README.md
fn format_asset(policy_id: &[u8], asset_name: &[u8]) -> String {
    let mut hasher = VarBlake2b::new(20).unwrap();
    hasher.update(policy_id);
    hasher.update(asset_name);
    let mut hash = [0u8; 20];
    hasher.finalize_variable(|res| hash.copy_from_slice(res));
    bech32::encode("asset", hash.to_base32(), Variant::Bech32).unwrap()
}

/// Validate size limits in the asset groups.
fn validate_asset_groups(
    asset_groups: &[pb::cardano_sign_transaction_request::AssetGroup],
) -> Result<(), Error> {
    for asset_group in asset_groups.iter() {
        if asset_group.policy_id.len() != 28 {
            return Err(Error::InvalidInput);
        }
        for token in asset_group.tokens.iter() {
            if token.asset_name.len() > 32 {
                return Err(Error::InvalidInput);
            }
        }
    }
    Ok(())
}

async fn _process(request: &pb::CardanoSignTransactionRequest) -> Result<Response, Error> {
    let network = CardanoNetwork::from_i32(request.network).ok_or(Error::InvalidInput)?;
    let params = params::get(network);
    if request.inputs.is_empty() {
        return Err(Error::InvalidInput);
    }
    // Outputs could be empty if the tx e.g. only contains certificates or withdrawals and no
    // change.

    // Validate that all keypaths (inputs and change outputs, certificates and withdrawals) have the
    // same account element.
    let bip44_account: u32 = *request.inputs[0]
        .keypath
        .get(2)
        .ok_or(Error::InvalidInput)?;

    // Collect all keypaths at which we will sign the transaction. Will include payment keypaths
    // from the inputs and staking keypaths from the certificates and withdrawals.
    let mut signing_keypaths: Vec<&[u32]> = Vec::new();

    for input in request.inputs.iter() {
        super::keypath::validate_address_shelley_payment(&input.keypath, Some(bip44_account))?;
        signing_keypaths.push(&input.keypath);
    }

    if network == CardanoNetwork::CardanoMainnet {
        let validity_interval_start_present = request.validity_interval_start != 0;
        let ttl_present = request.ttl != 0 || request.allow_zero_ttl;
        let cannot_be_mined = (validity_interval_start_present
            && ttl_present
            && (request.validity_interval_start > request.ttl))
            || (ttl_present && request.ttl < SHELLEY_START_SLOT);
        if cannot_be_mined {
            confirm::confirm(&confirm::Params {
                title: params.name,
                body: "Transaction\ncannot be\nmined",
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
        } else {
            if validity_interval_start_present {
                verify_slot(params, "Can be mined from", request.validity_interval_start).await?;
            }
            if ttl_present {
                verify_slot(params, "Can be mined until", request.ttl).await?;
            }
        }
    }
    certificates::verify(
        params,
        &request.certificates,
        bip44_account,
        &mut signing_keypaths,
    )
    .await?;

    for withdrawal in request.withdrawals.iter() {
        super::keypath::validate_address_shelley_stake(&withdrawal.keypath, Some(bip44_account))?;
        if withdrawal.value == 0 {
            return Err(Error::InvalidInput);
        }

        confirm::confirm(&confirm::Params {
            title: params.name,
            body: &format!(
                "Withdraw {} in staking rewards for account #{}?",
                format_value(params, withdrawal.value),
                withdrawal.keypath[2] + 1 - HARDENED,
            ),
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

        signing_keypaths.push(&withdrawal.keypath);
    }

    let mut total: u64 = 0;

    for output in request.outputs.iter() {
        super::address::decode_payment_address(params, &output.encoded_address)?;

        validate_asset_groups(&output.asset_groups)?;

        match output.script_config {
            Some(ref script_config) => match script_config {
                CardanoScriptConfig {
                    config: Some(ref config),
                } => {
                    let encoded_address = super::address::validate_and_encode_payment_address(
                        params,
                        config,
                        Some(bip44_account),
                    )?;
                    if encoded_address != output.encoded_address {
                        return Err(Error::InvalidInput);
                    }
                }
                _ => return Err(Error::InvalidInput),
            },
            None => {
                let formatted_value = format_value(params, output.value);
                transaction::verify_recipient(&output.encoded_address, &formatted_value).await?;
                total += output.value;

                for asset_group in output.asset_groups.iter() {
                    for token in asset_group.tokens.iter() {
                        confirm::confirm(&confirm::Params {
                            title: "Send token",
                            body: &format!(
                                "Amount: {}. Asset: {}",
                                util::decimal::format(token.value, 0),
                                format_asset(&asset_group.policy_id, &token.asset_name),
                            ),
                            accept_is_nextarrow: true,
                            scrollable: true,
                            ..Default::default()
                        })
                        .await?;
                    }
                }
            }
        }
    }

    if total == 0 {
        confirm::confirm(&confirm::Params {
            title: params.name,
            body: &format!("Fee\n{}", format_value(params, request.fee)),
            longtouch: true,
            ..Default::default()
        })
        .await?;
    } else {
        transaction::verify_total_fee(
            &format_value(params, total + request.fee),
            &format_value(params, request.fee),
        )
        .await?;
    }

    status::status("Transaction\nconfirmed", true).await;

    let tx_body_hash: [u8; 32] = {
        let mut hasher = VarBlake2b::new(32).unwrap();
        cbor::encode_transaction_body(request, cbor::HashedWriter::new(&mut hasher))?;

        let mut out = [0u8; 32];
        hasher.finalize_variable(|res| out.copy_from_slice(res));
        out
    };

    signing_keypaths.sort();
    signing_keypaths.dedup();

    let mut shelley_witnesses: Vec<ShelleyWitness> = Vec::with_capacity(signing_keypaths.len());
    for keypath in signing_keypaths {
        shelley_witnesses.push(make_shelley_witness(keypath, &tx_body_hash)?);
    }

    Ok(Response::SignTransaction(
        pb::CardanoSignTransactionResponse { shelley_witnesses },
    ))
}

/// Verify and sign a Cardano transaction.
pub async fn process(request: &pb::CardanoSignTransactionRequest) -> Result<Response, Error> {
    let result = _process(request).await;
    if let Err(Error::UserAbort) = result {
        status::status("Transaction\ncanceled", false).await;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bb02_async::block_on;
    use alloc::boxed::Box;
    use bitbox02::testing::{mock, mock_unlocked, Data};
    use util::bip32::HARDENED;

    use pb::cardano_sign_transaction_request::{certificate, certificate::Cert, Certificate};

    #[test]
    fn test_format_asset() {
        // Test vectors from:
        // https://github.com/cardano-foundation/CIPs/blob/a2ef32d8a2b485fed7f6ffde2781dd58869ff511/CIP-0014/README.md#test-vectors
        assert_eq!(
            format_asset(
                b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73", b""),
            "asset1rjklcrnsdzqp65wjgrg55sy9723kw09mlgvlc3",
        );
        assert_eq!(
            format_asset(
                b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x7e", b""),
            "asset1nl0puwxmhas8fawxp8nx4e2q3wekg969n2auw3",
        );
        assert_eq!(
            format_asset(
                b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09", b""),
            "asset1uyuxku60yqe57nusqzjx38aan3f2wq6s93f6ea",
        );
        assert_eq!(
            format_asset(
                b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73", b"\x50\x41\x54\x41\x54\x45"),
            "asset13n25uv0yaf5kus35fm2k86cqy60z58d9xmde92",
        );
        assert_eq!(
            format_asset(
                b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09", b"\x50\x41\x54\x41\x54\x45"),
            "asset1hv4p5tv2a837mzqrst04d0dcptdjmluqvdx9k3",
        );
        assert_eq!(
            format_asset(
                b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09", b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73"),
            "asset1aqrdypg669jgazruv5ah07nuyqe0wxjhe2el6f",
        );
        assert_eq!(
            format_asset(
                b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73", b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09"),
            "asset17jd78wukhtrnmjh3fngzasxm8rck0l2r4hhyyt",
        );
        assert_eq!(
            format_asset(
                b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73", b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            "asset1pkpwyknlvul7az0xx8czhl60pyel45rpje4z8w",
        );
    }

    #[test]
    fn test_sign_normal_tx() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 170499,
            ttl: 41115811,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        static mut OUTPUT_CONFIRMED: bool = false;
        static mut TOTAL_CONFIRMED: bool = false;
        static mut CONFIRM_COUNTER: u32 = 0;
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Can be mined until\nslot 335011 in\nepoch 292");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),

            ui_transaction_address_create: Some(Box::new(|amount, address| {
                assert_eq!(amount, "1 ADA");
                assert_eq!(address, "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt");
                unsafe {
                    OUTPUT_CONFIRMED = true;
                }
                true
            })),
            ui_transaction_fee_create: Some(Box::new(|total, fee| {
                assert_eq!(total, "1.170499 ADA");
                assert_eq!(fee, "0.170499 ADA");
                unsafe {
                    TOTAL_CONFIRMED = true;
                }
                true
            })),
            ..Default::default()
        });
        mock_unlocked();

        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![ShelleyWitness {
                    public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                    signature: b"\xf0\x8c\xbb\x77\x76\x03\x14\x22\x4d\xa7\x2b\x88\x62\x5d\xae\x78\xbc\x44\xec\x50\xb4\x98\xf7\x14\xf0\xb2\x3f\x86\x0f\x0c\x0c\x16\x89\x8b\x73\xf1\xa3\x77\xcc\x28\xd9\x78\xfa\x47\xfe\xf8\xba\x79\x7a\x35\x60\x9a\x6a\xd1\x2d\xd7\x9d\x51\x8b\x62\xff\x96\x6c\x06".to_vec(),
                }]
            })
        );
        assert_eq!(unsafe { CONFIRM_COUNTER }, 1);
        assert!(unsafe { OUTPUT_CONFIRMED });
        assert!(unsafe { TOTAL_CONFIRMED });
    }

    #[test]
    fn test_sign_stake_registration() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![
                pb::cardano_sign_transaction_request::Input {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                    prev_out_hash: b"\x64\xc3\x9d\x60\xf9\xd6\xb4\xf8\x83\xd0\x5a\xe3\x58\x5d\x06\x21\xd0\xfe\xbc\x06\xad\x0e\xa3\x40\x3b\xdc\x00\xbc\x23\x67\x16\x15".to_vec(),
                    prev_out_index: 1,
                },
                pb::cardano_sign_transaction_request::Input {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 1, 0],
                    prev_out_hash: b"\xb7\xb2\x33\x3e\x72\xf2\x67\x0a\xb8\x20\x51\xf4\x26\xcc\x84\x00\x04\x31\x97\x5a\x34\xe7\x1d\x5e\xdf\x70\xea\x6c\x0d\xdc\x9b\xf8".to_vec(),
                    prev_out_index: 0,
                },
            ],
            outputs: vec![
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 2741512,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 191681,
            ttl: 41539125,
            allow_zero_ttl: false,
            certificates: vec![
                Certificate{
                    cert: Some(Cert::StakeRegistration(
                        pb::Keypath {
                            keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        },
                    )),
                },
                Certificate{
                    cert: Some(Cert::StakeDelegation(
                        certificate::StakeDelegation{
                            keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                            pool_keyhash: b"\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab\xab".to_vec(),
                        }
                    )),
                },
            ],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Can be mined until\nslot 326325 in\nepoch 293");
                        true
                    }
                    2 => {
                        assert_eq!(params.title, "Cardano");
                        assert!(params
                            .body
                            .starts_with("Register staking key for account #1?"));
                        true
                    }
                    3 => {
                        assert_eq!(params.title, "Cardano");
                        assert!(params.body.starts_with(
                            "Delegate staking for account #1 to pool \
                             abababababababababababababababababababababababababababab?"
                        ));
                        true
                    }
                    4 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Fee\n0.191681 ADA");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();

        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![
                    ShelleyWitness {
                         public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                        signature: b"\x6a\xb5\xce\xde\xe3\x11\xa1\x66\x56\xee\x3c\x27\x09\x3a\xb8\x9b\xf2\xbc\xa7\xd4\x3d\xa7\x57\xb9\xab\xc3\xc2\x08\xfb\xce\xef\x1e\x59\x1d\xe3\x4f\x55\xa3\x86\xe1\xee\x34\x1a\xdf\x4f\xd9\x41\x56\x13\x97\x53\xf3\x9d\x81\x3f\xa8\x36\xfd\x0f\x42\xbf\x6b\x6c\x04".to_vec(),
                    },
                    ShelleyWitness {
                        public_key: b"\x32\x49\xff\x97\x5d\xbd\x08\x51\x4e\x34\xc7\x1e\x03\x2b\xec\x8d\x53\xdb\x1a\xf1\x13\xbb\x06\x52\x86\xd7\x1d\xe6\xbb\xe0\x15\x5b".to_vec(),
                        signature: b"\xb9\xec\xb7\x48\x5a\x61\x20\xc7\x9f\x2d\x34\xfd\x85\x9c\xa6\xb5\xf9\x69\x2b\x50\x14\xa2\x73\x4e\x1f\x89\x4b\x49\xfe\x47\x9f\x0b\x8e\xe3\xfd\xff\x5b\x8e\xf7\x2d\xec\xe3\x94\x8d\x3e\xdc\xf2\xa0\x2a\x27\xed\x33\x10\x4d\xcb\x22\x8b\xaa\x9d\x17\x4f\x49\xa9\x0c".to_vec(),
                    },
                    ShelleyWitness {
                        public_key: b"\xb0\xdc\x73\x13\xca\xbf\x4a\x4b\x07\x15\x14\xf4\x86\xd0\xd9\x97\x75\x86\x4e\x73\x77\x70\x0f\xb9\x93\x98\xb3\xf8\x23\x01\x06\x60".to_vec(),
                        signature: b"\x8d\x13\x38\x70\xd5\xa2\x57\x32\x83\x26\x8b\x78\x0c\xdc\x21\xf5\xce\x33\xda\xfe\xe9\x9a\x76\xf2\x5f\x64\x73\x1b\xac\x07\x86\xe9\xd6\x8c\x8e\xdb\x29\x9b\xc7\x17\xdb\x26\xcf\xb8\x35\x00\x6d\x95\xfc\xbd\x74\x3e\x8b\xcd\x55\xae\x85\x78\x9b\x01\xd2\x70\xee\x0a".to_vec(),
                    },
                ]
            })
        );
        assert_eq!(unsafe { CONFIRM_COUNTER }, 4);
    }

    #[test]
    fn test_sign_stake_deregistration() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![
                pb::cardano_sign_transaction_request::Input {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                    prev_out_hash: b"\x64\xc3\x9d\x60\xf9\xd6\xb4\xf8\x83\xd0\x5a\xe3\x58\x5d\x06\x21\xd0\xfe\xbc\x06\xad\x0e\xa3\x40\x3b\xdc\x00\xbc\x23\x67\x16\x15".to_vec(),
                    prev_out_index: 1,
                },
                pb::cardano_sign_transaction_request::Input {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 1, 0],
                    prev_out_hash: b"\xb7\xb2\x33\x3e\x72\xf2\x67\x0a\xb8\x20\x51\xf4\x26\xcc\x84\x00\x04\x31\x97\x5a\x34\xe7\x1d\x5e\xdf\x70\xea\x6c\x0d\xdc\x9b\xf8".to_vec(),
                    prev_out_index: 0,
                },
            ],
            outputs: vec![
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 2741512,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 191681,
            ttl: 41539125,
            allow_zero_ttl: false,
            certificates: vec![
                Certificate{
                    cert: Some(Cert::StakeDeregistration(
                        pb::Keypath {
                            keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        },
                    )),
                },
            ],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Can be mined until\nslot 326325 in\nepoch 293");
                        true
                    }
                    2 => {
                        assert_eq!(params.title, "Cardano");
                        assert!(params
                            .body
                            .starts_with("Stop stake delegation for account #1?"));
                        true
                    }
                    3 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Fee\n0.191681 ADA");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();

        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![
                    ShelleyWitness {
                        public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                        signature: b"\xb6\x60\xd3\x84\xc6\xcf\x29\x70\x49\x77\xab\xde\x77\xaf\x88\xea\xce\xb0\xcd\x6e\xb0\xfb\xb8\x19\x25\xb3\x31\xd7\x43\xae\x54\x61\x0f\x17\xd4\x62\x49\xe8\x52\xf7\x17\x7c\x26\xa0\xca\x01\x08\xa1\x4e\x48\xb8\xac\xb3\x0e\xd5\x55\xa0\xc6\x80\x42\x7e\xf1\x6c\x0e".to_vec(),
                    },
                    ShelleyWitness {
                        public_key: b"\x32\x49\xff\x97\x5d\xbd\x08\x51\x4e\x34\xc7\x1e\x03\x2b\xec\x8d\x53\xdb\x1a\xf1\x13\xbb\x06\x52\x86\xd7\x1d\xe6\xbb\xe0\x15\x5b".to_vec(),
                        signature: b"\x76\x8c\xef\xc1\xa3\x47\x8a\xb8\x11\x67\xf2\xda\xc9\x69\x12\xc5\xe2\x5d\xde\x29\xd3\xd4\x5a\xa8\x49\x2d\x1c\x26\xac\xd3\x9f\x78\xa1\x67\x19\x62\x97\xc1\x01\xb1\x5e\x44\x80\x4d\x5c\x9b\x72\xdd\xd3\xaf\x7f\x93\xf9\xbe\xd2\x17\x49\xe1\x6c\x20\xeb\x8f\xf6\x00".to_vec(),
                    },
                    ShelleyWitness {
                        public_key: b"\xb0\xdc\x73\x13\xca\xbf\x4a\x4b\x07\x15\x14\xf4\x86\xd0\xd9\x97\x75\x86\x4e\x73\x77\x70\x0f\xb9\x93\x98\xb3\xf8\x23\x01\x06\x60".to_vec(),
                        signature: b"\xbf\xce\x07\x7a\xbd\xf7\x3b\xba\xc2\xaf\x1b\x09\x16\x2e\x25\x15\x9a\x8b\xb2\xbb\xe6\x2e\x98\xbc\xaf\xea\x73\xe0\x51\xca\x54\xe0\x8b\x49\xa1\x22\xde\xba\x54\xbb\x2c\xed\xeb\x78\xa8\x7c\x09\x1e\x64\x26\x5f\x84\x73\x8b\xd6\xf6\xfa\xd0\xee\x81\x75\x14\x11\x03".to_vec(),
                    },
                ]
            })
        );
        assert_eq!(unsafe { CONFIRM_COUNTER }, 3);
    }

    #[test]
    fn test_sign_withdrawal() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![
                pb::cardano_sign_transaction_request::Input {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                    prev_out_hash: b"\xb7\xb2\x33\x3e\x72\xf2\x67\x0a\xb8\x20\x51\xf4\x26\xcc\x84\x00\x04\x31\x97\x5a\x34\xe7\x1d\x5e\xdf\x70\xea\x6c\x0d\xdc\x9b\xf8".to_vec(),
                    prev_out_index: 0,
                },
            ],
            outputs: vec![
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4817591,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 175157,
            ttl: 41788708,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![
                pb::cardano_sign_transaction_request::Withdrawal {
                    keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                    value: 1234567,
                },
            ],
            validity_interval_start: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Can be mined until\nslot 143908 in\nepoch 294");
                        true
                    }
                    2 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(
                            params.body,
                            "Withdraw 1.234567 ADA in staking rewards for account #1?"
                        );
                        true
                    }
                    3 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Fee\n0.175157 ADA");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();

        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![
                    ShelleyWitness {
                        public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                        signature: b"\x7f\xa9\x1c\x06\x6b\xc3\x5a\x17\x0d\x06\xb4\x4b\xc9\xed\x81\x79\xdf\x00\x59\x4b\x90\xcb\x56\x08\xf4\x05\x5b\x27\x4f\xd9\x69\x9c\xeb\x9f\x1f\x44\xbb\x3a\x4e\x0f\x27\xe0\x1e\xa3\xd5\xb8\xd9\xc9\xf6\x1e\x7d\xc1\x80\x67\xa2\xa7\x56\x88\x20\x13\x64\x08\xf2\x0e".to_vec(),
                    },
                    ShelleyWitness {
                        public_key: b"\xb0\xdc\x73\x13\xca\xbf\x4a\x4b\x07\x15\x14\xf4\x86\xd0\xd9\x97\x75\x86\x4e\x73\x77\x70\x0f\xb9\x93\x98\xb3\xf8\x23\x01\x06\x60".to_vec(),
                        signature: b"\xc7\xd9\xf4\x88\xab\x46\xc8\x33\x11\xd5\x29\x51\x00\xe8\xef\x6f\x8f\xd7\x8b\xb9\x1f\xb7\xa4\x29\x06\xde\x39\xad\xa0\x6d\x57\x19\xff\x8e\x5a\xef\x3d\xeb\xb3\x9e\x9a\x41\x4c\x96\x0d\x2b\x6d\x8e\x31\xa3\x78\xd3\x97\xaa\x19\xe9\x13\x33\x7d\xc6\xfd\x8b\x0c\x08".to_vec(),
                    },
                ]
            })
        );
        assert_eq!(unsafe { CONFIRM_COUNTER }, 3);
    }

    /// Test that ttl=0 is not included in the transaction if allow_ttl_zero is false. Up to v9.8.0, ttl was not included if it was zero.
    #[test]
    fn test_sign_tx_no_ttl() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 170499,
            ttl: 0,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        mock(Data {
            ui_confirm_create: Some(Box::new(|_params| true)),
            ui_transaction_address_create: Some(Box::new(|_amount, _address| true)),
            ui_transaction_fee_create: Some(Box::new(|_total, _fee| true)),
            ..Default::default()
        });
        mock_unlocked();
        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![ShelleyWitness {
                    public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                    signature: b"\x05\xc0\x20\x83\xd8\x91\x48\xdf\xb5\x55\x87\x46\x6f\x76\xbf\xfa\x4a\x26\x90\x4b\xe2\x0d\x04\x61\x04\x8a\x81\xbc\x01\x64\xf4\x15\xd7\xa4\xae\x4c\x50\xde\x10\x06\x16\xac\x39\xb6\x79\x00\x2b\x7f\xa8\xd6\xa5\x7f\x68\x80\xfa\xd6\x5e\xb4\x37\xc3\xed\x94\xe3\x0f".to_vec(),
                }]
            })
        );
    }

    /// Test that ttl=0 is included in the transaction if allow_ttl_zero is true. Up to v9.8.0, ttl was not included if it was zero.
    /// ttl=0 also means the transaction cannot be mined.
    /// Also test other configurations where the transaction cannot be mined.
    #[test]
    fn test_sign_non_mineable_tx() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 170499,
            ttl: 0,
            allow_zero_ttl: true,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        // Second, test with allow_zero_ttl=true, meaning that a zero ttl will be included as 0.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Transaction\ncannot be\nmined");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),

            ui_transaction_address_create: Some(Box::new(|_amount, _address| true)),
            ui_transaction_fee_create: Some(Box::new(|_total, _fee| true)),
            ..Default::default()
        });
        mock_unlocked();
        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![ShelleyWitness {
                    public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                    signature: b"\x5b\xa3\xc8\x1f\x57\xac\x0c\xb2\x49\x36\xc3\xc6\x7c\xb5\x1e\x86\x7f\xda\x7d\x95\xb4\x57\x22\x59\xbe\x9a\x06\xd0\xb1\x0c\xd4\x3b\x2e\x90\xd5\x32\xd0\x6b\x46\xd0\x5b\x23\x85\xe9\x03\x50\xaf\x2d\x9d\xb1\xc3\x9f\x39\xbf\xe3\x6b\x79\x25\x4e\xcb\xd3\x59\x1b\x0e".to_vec(),
                }]
            })
        );
    }

    #[test]
    fn test_sign_tx_valid_interval_start() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 170499,
            ttl: 0,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 41115811,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Can be mined from\nslot 335011 in\nepoch 292");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ui_transaction_address_create: Some(Box::new(|_amount, _address| true)),
            ui_transaction_fee_create: Some(Box::new(|_total, _fee| true)),
            ..Default::default()
        });
        mock_unlocked();
        assert!(block_on(process(&tx)).is_ok());
    }

    #[test]
    fn test_sign_tx_invalid_interval_start() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![],
                },
            ],
            fee: 170499,
            ttl: 41115810,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 41115811, // start > ttl, invalid
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Cardano");
                        assert_eq!(params.body, "Transaction\ncannot be\nmined");
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ui_transaction_address_create: Some(Box::new(|_amount, _address| true)),
            ui_transaction_fee_create: Some(Box::new(|_total, _fee| true)),
            ..Default::default()
        });
        mock_unlocked();
        assert!(block_on(process(&tx)).is_ok());
    }

    #[test]
    fn test_sign_tx_tokens() {
        let tx = pb::CardanoSignTransactionRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            inputs: vec![pb::cardano_sign_transaction_request::Input {
                keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                prev_out_hash: b"\x59\x86\x4e\xe7\x3c\xa5\xd9\x10\x98\xa3\x2b\x3c\xe9\x81\x1b\xac\x19\x96\xdc\xba\xef\xa6\xb6\x24\x7d\xca\xaf\xb5\x77\x9c\x25\x38".to_vec(),
                prev_out_index: 0,
            }],
            outputs: vec![
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt".into(),
                    value: 1000000,
                    script_config: None,
                    asset_groups: vec![
                        pb::cardano_sign_transaction_request::AssetGroup {
                            policy_id: b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73".to_vec(),
                            tokens: vec![
                                pb::cardano_sign_transaction_request::asset_group::Token {
                                    asset_name: b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09".to_vec(),
                                    value: 3,
                                },
                                pb::cardano_sign_transaction_request::asset_group::Token {
                                    asset_name: b"\x50\x41\x54\x41\x54\x45".to_vec(),
                                    value: 1,
                                },
                            ],
                        },
                        pb::cardano_sign_transaction_request::AssetGroup {
                            policy_id: b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09".to_vec(),
                            tokens: vec![
                                pb::cardano_sign_transaction_request::asset_group::Token {
                                    asset_name: b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73".to_vec(),
                                    value: 5,
                                },
                            ],
                        },
                    ],
                },
                // change
                pb::cardano_sign_transaction_request::Output {
                    encoded_address: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into(),
                    value: 4829501,
                    script_config: Some(CardanoScriptConfig{
                        config: Some(pb::cardano_script_config::Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                            keypath_payment: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                            keypath_stake: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                        }))
                    }),
                    asset_groups: vec![
                        pb::cardano_sign_transaction_request::AssetGroup {
                            policy_id: b"\x7e\xae\x28\xaf\x22\x08\xbe\x85\x6f\x7a\x11\x96\x68\xae\x52\xa4\x9b\x73\x72\x5e\x32\x6d\xc1\x65\x79\xdc\xc3\x73".to_vec(),
                            tokens: vec![
                                pb::cardano_sign_transaction_request::asset_group::Token {
                                    asset_name: b"\x1e\x34\x9c\x9b\xde\xa1\x9f\xd6\xc1\x47\x62\x6a\x52\x60\xbc\x44\xb7\x16\x35\xf3\x98\xb6\x7c\x59\x88\x1d\xf2\x09".to_vec(),
                                    value: 1,
                                },
                            ],
                        },
                    ],
                },
            ],
            fee: 170499,
            ttl: 0,
            allow_zero_ttl: false,
            certificates: vec![],
            withdrawals: vec![],
            validity_interval_start: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    2 => {
                        assert_eq!(params.title, "Send token");
                        assert_eq!(
                            params.body,
                            "Amount: 3. Asset: asset17jd78wukhtrnmjh3fngzasxm8rck0l2r4hhyyt"
                        );
                        true
                    }
                    3 => {
                        assert_eq!(params.title, "Send token");
                        assert_eq!(
                            params.body,
                            "Amount: 1. Asset: asset13n25uv0yaf5kus35fm2k86cqy60z58d9xmde92"
                        );
                        true
                    }
                    4 => {
                        assert_eq!(params.title, "Send token");
                        assert_eq!(
                            params.body,
                            "Amount: 5. Asset: asset1aqrdypg669jgazruv5ah07nuyqe0wxjhe2el6f"
                        );
                        true
                    }
                    _ => panic!("unexpected user confirmation"),
                }
            })),
            ui_transaction_address_create: Some(Box::new(|_amount, address| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(
                            address,
                            "addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt");
                        true
                    }
                    _ => panic!("unexpected user confirmation"),
                }
            })),
            ui_transaction_fee_create: Some(Box::new(|_total, _fee| true)),
            ..Default::default()
        });
        mock_unlocked();
        let result = block_on(process(&tx)).unwrap();
        assert_eq!(
            result,
            Response::SignTransaction(pb::CardanoSignTransactionResponse {
                shelley_witnesses: vec![ShelleyWitness {
                    public_key: b"\x1f\x17\xaf\xff\xe8\x05\x29\x7f\x8e\xc6\x54\x45\x82\xb7\xea\x91\xc3\x0d\xc1\xf9\x11\x9c\x5c\x2b\x26\x3e\x58\xfa\x36\x59\x31\x7d".to_vec(),
                    signature: b"\xfe\xdd\x2d\xdf\x9d\x00\x69\xe9\xb4\xb6\x11\x83\xae\xdd\xb3\xbb\xe7\x02\x19\x0e\xa5\x8d\x4a\x23\x25\xef\xa2\x2b\xf0\xd6\x32\x5a\x82\x89\x10\x53\xa7\x6b\x6a\x2e\xce\x2d\xf2\xd2\x2a\x6b\x65\x78\x07\x42\xa1\x9f\x27\x61\x18\xee\x68\x34\xa0\x05\x2e\xf9\xa4\x08".to_vec(),
                }]
            })
        );
    }
}
