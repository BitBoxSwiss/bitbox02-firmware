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

    let basic_info = format!("Coin: {}", super::coin_name(coin));
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

    let host_nonce = [0; 32]; // TODO: get nonce contribution from host.
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
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;
    use util::bip32::HARDENED;

    const ADDRESS: &str = "<address>";
    const KEYPATH: &[u32] = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
    const MESSAGE: &[u8] = b"message";
    const EXPECTED_SIGHASH: &[u8; 32] = b"\x13\x89\x5e\x0c\x3a\xe0\xe7\x98\xe4\x0a\x6c\x3e\x4b\x2b\x2e\x48\x7d\x1c\xe4\xdc\x13\x31\xa4\xe1\x5b\x2f\x6c\xe0\x96\x97\x65\x51";

    #[test]
    pub fn test_process() {
        let _guard = MUTEX.lock().unwrap();

        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: KEYPATH.to_vec(),
            }),
            msg: MESSAGE.to_vec(),
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            btc_address_simple: Some(Box::new(|coin, _, _| {
                assert_eq!(coin, BtcCoin::Btc as _);
                Ok(ADDRESS.into())
            })),
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
                        assert_eq!(params.body, ADDRESS);
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
            keystore_secp256k1_sign: Some(Box::new(|keypath, sighash, _host_nonce| {
                assert_eq!(keypath, KEYPATH);
                assert_eq!(sighash, EXPECTED_SIGHASH);
                Ok(bitbox02::keystore::SignResult {
                    signature: [b'1'; 64],
                    recid: 3,
                })
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&request)),
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: b"1111111111111111111111111111111111111111111111111111111111111111\x03"
                    .to_vec()
            }))
        );
    }

    #[test]
    pub fn test_process_user_aborted() {
        let _guard = MUTEX.lock().unwrap();

        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: KEYPATH.to_vec(),
            }),
            msg: MESSAGE.to_vec(),
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        // Basic info dialog aborted.
        mock(Data {
            btc_address_simple: Some(Box::new(|_, _, _| Ok(ADDRESS.into()))),
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
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // Address verification aborted.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
            btc_address_simple: Some(Box::new(|_, _, _| Ok(ADDRESS.into()))),
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => true,
                    2 => {
                        assert_eq!(params.title, "Address");
                        assert_eq!(params.body, ADDRESS);
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // Message verification aborted.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
            btc_address_simple: Some(Box::new(|_, _, _| Ok(ADDRESS.into()))),
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
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));
    }

    #[test]
    pub fn test_process_failures() {
        let _guard = MUTEX.lock().unwrap();

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
            })),
            Err(Error::InvalidInput)
        );

        // Address could not be generated
        mock(Data {
            btc_address_simple: Some(Box::new(|_, _, _| Err(()))),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::BtcSignMessageRequest {
                coin: BtcCoin::Btc as _,
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                    }),
                    keypath: KEYPATH.to_vec(),
                }),
                msg: MESSAGE.to_vec()
            })),
            Err(Error::InvalidInput)
        );
    }
}
