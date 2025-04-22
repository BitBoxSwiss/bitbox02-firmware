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

use sha2::{Digest, Sha256};

use super::pb;
use super::Error;

use pb::btc_script_config::{Config, SimpleType};
use pb::BtcCoin;

use pb::btc_response::Response;

use bitbox02::keystore;

use crate::hal::Ui;
use crate::workflow::{confirm, verify_message};

const MAX_MESSAGE_SIZE: usize = 1024;

/// Process a sign message request.
///
/// The result contains a 65 byte signature. The first 64 bytes are the secp256k1 signature in
/// compact format (R and S values), and the last byte is the recoverable id (recid).
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcSignMessageRequest,
) -> Result<Response, Error> {
    let coin = BtcCoin::try_from(request.coin)?;
    if !matches!(coin, BtcCoin::Btc | BtcCoin::Tbtc | BtcCoin::Rbtc) {
        return Err(Error::InvalidInput);
    }
    let (keypath, simple_type) = match &request.script_config {
        Some(pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(simple_type)),
                }),
            keypath,
        }) => (keypath, SimpleType::try_from(*simple_type)?),
        _ => return Err(Error::InvalidInput),
    };
    if simple_type == SimpleType::P2tr {
        return Err(Error::InvalidInput);
    }
    if request.msg.len() > MAX_MESSAGE_SIZE {
        return Err(Error::InvalidInput);
    }

    // Keypath and script_config are validated in address_simple().
    let address = super::derive_address_simple(coin, simple_type, keypath)?;

    let basic_info = format!("Coin: {}", super::params::get(coin).name);
    let confirm_params = confirm::Params {
        title: "Sign message",
        body: &basic_info,
        accept_is_nextarrow: true,
        ..Default::default()
    };
    hal.ui().confirm(&confirm_params).await?;

    let confirm_params = confirm::Params {
        title: "Address",
        body: &address,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    };
    hal.ui().confirm(&confirm_params).await?;

    verify_message::verify(hal, "Sign message", "Sign", &request.msg, true).await?;

    // See
    // https://github.com/spesmilo/electrum/blob/84dc181b6e7bb20e88ef6b98fb8925c5f645a765/electrum/ecc.py#L355-L358.
    // This is the message format that is widespread for p2pkh addresses.
    // Electrum re-used it for p2wpkh-p2sh and p2wpkh addresses.
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(b"\x18Bitcoin Signed Message:\n");
    msg.extend(super::script::serialize_varint(request.msg.len() as _));
    msg.extend(&request.msg);

    let sighash: [u8; 32] = Sha256::digest(Sha256::digest(msg)).into();

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
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_unlocked;
    use util::bip32::HARDENED;

    const MESSAGE: &str = "message";

    #[test]
    pub fn test_p2wpkh() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: vec![84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: b"\x0f\x1d\x54\x2a\x9e\x2f\x37\x4e\xfe\xd4\x57\x8c\xaa\x84\x72\xd1\xc3\x12\x68\xfb\x89\x2d\x39\xa6\x15\x44\x59\x18\x5b\x2d\x35\x4d\x3b\x2b\xff\xf0\xe1\x61\x5c\x77\x25\x73\x4f\x43\x13\x4a\xb4\x51\x6b\x7e\x7c\xb3\x9d\x2d\xba\xaa\x5f\x4e\x8b\x8a\xff\x9f\x97\xd0\x00".to_vec(),
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: "Coin: Bitcoin".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Address".into(),
                    body: "bc1qk5f9em9qc8yfpks8ngfg3h8h02n2e3yeqdyhpt".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[test]
    pub fn test_p2wpkh_testnet() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Tbtc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: vec![84 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert!(block_on(process(&mut mock_hal, &request)).is_ok());
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: "Coin: BTC Testnet".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Address".into(),
                    body: "tb1qnlyrq9pshg0v0lsuudjgga4nvmjxhcvketqwdg".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[test]
    pub fn test_p2wpkh_p2sh() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkhP2sh as _)),
                }),
                keypath: vec![49 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: b"\x87\x19\x05\x3c\x29\xff\xcf\x54\x31\x40\x69\x86\x75\x8a\xc8\xed\x80\x1c\xff\x3d\x61\x46\xe4\x8c\x46\x25\x75\xb6\x47\x34\x46\xf8\x44\xf1\x38\x7d\x48\xe1\x36\x88\x42\x09\x43\xfa\x8e\x4f\x0a\x23\xaa\x2e\x49\xa8\x3a\xf8\x88\x52\x2c\xec\xa9\x05\x0b\xe6\xc3\x47\x00".to_vec(),
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: "Coin: Bitcoin".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Address".into(),
                    body: "3BaL6XecvLAidPToUDhXo1zxD99ZUrErpd".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ]
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
                keypath: vec![84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();

        let mut mock_hal = TestingHal::new();
        // Basic info dialog aborted.
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Err(Error::UserAbort)
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Sign message".into(),
                body: "Coin: Bitcoin".into(),
                longtouch: false,
            },],
        );

        // Basic info dialog aborted.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(1);
        mock_unlocked();
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Err(Error::UserAbort)
        );
        assert_eq!(mock_hal.ui.screens.len(), 2);

        // Message verification aborted.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(2);
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Err(Error::UserAbort)
        );
        assert_eq!(mock_hal.ui.screens.len(), 3);
    }

    #[test]
    pub fn test_process_failures() {
        const KEYPATH: &[u32] = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        // Invalid coin
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: -1,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                        }),
                        keypath: KEYPATH.to_vec(),
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Invalid script type (invalid simple type)
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(-1))
                        }),
                        keypath: KEYPATH.to_vec(),
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Invalid script type (taproot not supported)
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(SimpleType::P2tr as _)),
                        }),
                        keypath: vec![86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Invalid script type (multisig not supported)
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::Multisig(pb::btc_script_config::Multisig {
                                ..Default::default()
                            }))
                        }),
                        keypath: KEYPATH.to_vec(),
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Message too long
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                        }),
                        keypath: KEYPATH.to_vec(),
                    }),
                    msg: [0; 1025].to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Invalid keypath
        mock_unlocked();
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Btc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                        }),
                        keypath: [0].to_vec(),
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );
        // Invalid keypath (mainnet keypath on testnet)
        mock_unlocked();
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::BtcSignMessageRequest {
                    coin: BtcCoin::Tbtc as _,
                    script_config: Some(pb::BtcScriptConfigWithKeypath {
                        script_config: Some(pb::BtcScriptConfig {
                            config: Some(Config::SimpleType(SimpleType::P2wpkh as _))
                        }),
                        keypath: vec![84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                    }),
                    msg: MESSAGE.as_bytes().to_vec(),
                    host_nonce_commitment: None,
                }
            )),
            Err(Error::InvalidInput)
        );
    }
}
