// Copyright 2020 Shift Crypto AG
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

use alloc::vec::Vec;
use core::convert::TryInto;

use sha2::{Digest, Sha256};

use super::pb;
use super::Error;

use pb::btc_script_config::{Config, SimpleType};
use pb::BtcCoin;

use pb::btc_response::Response;

use bitbox02::keystore;

use crate::workflow::{confirm, verify_message};

const MAX_MESSAGE_SIZE: usize = 1024;

/// Process a sign message request.
///
/// The result contains a 65 byte signature. The first 64 bytes are the secp256k1 signature in
/// compact format (R and S values), and the last byte is the recoverable id (recid).
pub async fn process(request: &pb::BtcSignMessageRequest) -> Result<Response, Error> {
    let coin = BtcCoin::from_i32(request.coin).ok_or(Error::InvalidInput)?;
    if coin != BtcCoin::Btc {
        return Err(Error::InvalidInput);
    }
    let (keypath, simple_type) = match &request.script_config {
        Some(pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(simple_type)),
                }),
            keypath,
        }) => (
            keypath,
            SimpleType::from_i32(*simple_type).ok_or(Error::InvalidInput)?,
        ),
        _ => return Err(Error::InvalidInput),
    };
    if request.msg.len() > MAX_MESSAGE_SIZE {
        return Err(Error::InvalidInput);
    }

    // Keypath and script_config are validated in address_simple().
    let address = bitbox02::app_btc::address_simple(coin as _, simple_type as _, keypath)
        .or(Err(Error::InvalidInput))?;

    let basic_info = format!("Coin: {}", super::params::get(coin).name);
    let confirm_params = confirm::Params {
        title: "Sign message",
        body: &basic_info,
        accept_is_nextarrow: true,
        ..Default::default()
    };
    confirm::confirm(&confirm_params).await?;

    let confirm_params = confirm::Params {
        title: "Address",
        body: &address,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    };
    confirm::confirm(&confirm_params).await?;

    verify_message::verify(&request.msg).await?;

    // See
    // https://github.com/spesmilo/electrum/blob/84dc181b6e7bb20e88ef6b98fb8925c5f645a765/electrum/ecc.py#L355-L358.
    // This is the message format that is widespread for p2pkh addresses.
    // Electrum re-used it for p2wpkh-p2sh and p2wpkh addresses.
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(b"\x18Bitcoin Signed Message:\n");
    msg.extend(super::script::serialize_varint(request.msg.len() as _));
    msg.extend(&request.msg);

    let sighash: [u8; 32] = Sha256::digest(&Sha256::digest(&msg))
        .as_slice()
        .try_into()
        .unwrap();

    let host_nonce = match request.host_nonce_commitment {
        // Engage in the anti-klepto protocol if the host sends a host nonce commitment.
        Some(pb::AntiKleptoHostNonceCommitment { ref commitment }) => {
            let signer_commitment = keystore::secp256k1_nonce_commit(
                keypath,
                &sighash,
                commitment
                    .as_slice()
                    .try_into()
                    .or(Err(Error::InvalidInput))?,
            )?;

            // Send signer commitment to host and wait for the host nonce from the host.
            super::antiklepto_get_host_nonce(signer_commitment).await?
        }

        // Return signature directly without the anti-klepto protocol, for backwards compatibility.
        None => [0; 32],
    };

    let sign_result = bitbox02::keystore::secp256k1_sign(keypath, &sighash, &host_nonce)?;

    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::SignMessage(pb::BtcSignMessageResponse {
        signature,
    }))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, mock_unlocked, Data};
    use std::boxed::Box;
    use util::bip32::HARDENED;

    const KEYPATH: &[u32] = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
    const MESSAGE: &[u8] = b"message";
    const EXPECTED_ADDRESS: &str = "bc1qk5f9em9qc8yfpks8ngfg3h8h02n2e3yeqdyhpt";
    const EXPECTED_SIGNATURE: &[u8] = b"\x0f\x1d\x54\x2a\x9e\x2f\x37\x4e\xfe\xd4\x57\x8c\xaa\x84\x72\xd1\xc3\x12\x68\xfb\x89\x2d\x39\xa6\x15\x44\x59\x18\x5b\x2d\x35\x4d\x3b\x2b\xff\xf0\xe1\x61\x5c\x77\x25\x73\x4f\x43\x13\x4a\xb4\x51\x6b\x7e\x7c\xb3\x9d\x2d\xba\xaa\x5f\x4e\x8b\x8a\xff\x9f\x97\xd0\x00";

    #[test]
    pub fn test_process() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: KEYPATH.to_vec(),
            }),
            msg: MESSAGE.to_vec(),
            host_nonce_commitment: None,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body, "Coin: Bitcoin");
                        true
                    }
                    2 => {
                        assert_eq!(params.title, "Address");
                        assert_eq!(params.body, EXPECTED_ADDRESS);
                        true
                    }
                    3 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body.as_bytes(), MESSAGE);
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();
        assert_eq!(
            block_on(process(&request)),
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: EXPECTED_SIGNATURE.to_vec(),
            }))
        );
    }

    #[test]
    pub fn test_process_user_aborted() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: KEYPATH.to_vec(),
            }),
            msg: MESSAGE.to_vec(),
            host_nonce_commitment: None,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        // Basic info dialog aborted.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body, "Coin: Bitcoin");
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // Address verification aborted.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => true,
                    2 => {
                        assert_eq!(params.title, "Address");
                        assert_eq!(params.body, EXPECTED_ADDRESS);
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // Message verification aborted.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 | 2 => true,
                    3 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body.as_bytes(), MESSAGE);
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        mock_unlocked();
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));
    }

    #[test]
    pub fn test_process_failures() {
        // Invalid coin
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: -1,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                    }),
                    keypath: KEYPATH.to_vec(),
                }),
                msg: MESSAGE.to_vec(),
                host_nonce_commitment: None,
            })),
            Err(Error::InvalidInput)
        );

        // Invalid script type (invalid simple type)
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: BtcCoin::Btc as _,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::SimpleType(-1))
                    }),
                    keypath: KEYPATH.to_vec(),
                }),
                msg: MESSAGE.to_vec(),
                host_nonce_commitment: None,
            })),
            Err(Error::InvalidInput)
        );

        // Invalid script type (multisig not supported)
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: BtcCoin::Btc as _,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::Multisig(pb::btc_script_config::Multisig {
                            ..Default::default()
                        }))
                    }),
                    keypath: KEYPATH.to_vec(),
                }),
                msg: MESSAGE.to_vec(),
                host_nonce_commitment: None,
            })),
            Err(Error::InvalidInput)
        );

        // Message too long
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: BtcCoin::Btc as _,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                    }),
                    keypath: KEYPATH.to_vec(),
                }),
                msg: [0; 1025].to_vec(),
                host_nonce_commitment: None,
            })),
            Err(Error::InvalidInput)
        );

        // Invalid keypath
        mock(Data {
            ..Default::default()
        });
        mock_unlocked();
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: BtcCoin::Btc as _,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                    }),
                    keypath: [0].to_vec(),
                }),
                msg: MESSAGE.to_vec(),
                host_nonce_commitment: None,
            })),
            Err(Error::InvalidInput)
        );
    }
}
