// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::ConfirmParams;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

use super::Error;
use super::pb;

use pb::BtcCoin;
use pb::btc_script_config::{Config, SimpleType};

use pb::btc_response::Response;

use crate::keystore;

use crate::hal::Ui;
use crate::workflow::verify_message;
use bitcoin::consensus::encode::{VarInt, serialize};
use util::bip32::HARDENED;

const MAX_MESSAGE_SIZE: usize = 1024;
const MAX_KEYPATH_DEPTH: usize = 10;
const EXTERNAL_SERVICE_PURPOSE_M45: u32 = 45 + HARDENED;
const EXTERNAL_SERVICE_PURPOSE_M48: u32 = 48 + HARDENED;
const EXTERNAL_SERVICE_PURPOSES: [u32; 2] =
    [EXTERNAL_SERVICE_PURPOSE_M45, EXTERNAL_SERVICE_PURPOSE_M48];

/// Validate a keypath in the external service application namespace.
fn validate_external_service_keypath(keypath: &[u32]) -> Result<(), Error> {
    match keypath.first() {
        Some(first)
            if EXTERNAL_SERVICE_PURPOSES.contains(first) && keypath.len() <= MAX_KEYPATH_DEPTH =>
        {
            Ok(())
        }
        _ => Err(Error::InvalidInput),
    }
}

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

    // Standard Bitcoin keypaths keep the existing UI. Keypaths in the external service
    // application namespace are accepted and identified as external service keys.
    let coin_params = super::params::get(coin);
    let is_standard_keypath = super::keypath::validate_address_simple(
        keypath,
        coin_params.bip44_coin,
        simple_type,
        coin_params.taproot_support,
        super::keypath::ReceiveSpend::Receive,
    )
    .is_ok();
    if !is_standard_keypath {
        validate_external_service_keypath(keypath)?;
    }

    let address = super::derive_address_simple_unvalidated(
        hal,
        coin,
        simple_type,
        keypath,
        crate::keystore::Compute::Twice,
    )
    .await?;
    let address_formatted = util::strings::format_address(&address);

    let basic_info = if is_standard_keypath {
        format!("Coin: {}", coin_params.name)
    } else {
        format!("Coin: {}\nExternal service key", coin_params.name)
    };
    let confirm_params = ConfirmParams {
        title: "Sign message",
        body: &basic_info,
        accept_is_nextarrow: true,
        ..Default::default()
    };
    hal.ui().confirm(&confirm_params).await?;

    let confirm_params = ConfirmParams {
        title: "Address",
        body: &address_formatted,
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
    msg.extend(serialize(&VarInt(request.msg.len() as _)));
    msg.extend(&request.msg);

    let sighash: [u8; 32] = Sha256::digest(Sha256::digest(msg)).into();

    let host_nonce = match request.host_nonce_commitment {
        // Engage in the anti-klepto protocol if the host sends a host nonce commitment.
        Some(pb::AntiKleptoHostNonceCommitment { ref commitment }) => {
            let signer_commitment = crate::secp256k1::secp256k1_nonce_commit(
                keystore::secp256k1_get_private_key(hal, keypath)
                    .await?
                    .as_slice()
                    .try_into()
                    .unwrap(),
                &sighash,
                commitment
                    .as_slice()
                    .try_into()
                    .or(Err(Error::InvalidInput))?,
            )?;

            // Send signer commitment to host and wait for the host nonce from the host.
            super::antiklepto_get_host_nonce(signer_commitment).await?
        }

        // Return the signature directly without the anti-klepto protocol for backwards
        // compatibility. Preserve the historical zero-contribution S2C signature; this differs
        // from plain RFC6979 and does not provide anti-klepto protection.
        None => [0; 32],
    };

    let sign_result = crate::secp256k1::secp256k1_sign(
        keystore::secp256k1_get_private_key(hal, keypath)
            .await?
            .as_slice()
            .try_into()
            .unwrap(),
        &sighash,
        Some(&host_nonce),
    )?;
    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::SignMessage(pb::BtcSignMessageResponse {
        signature,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use crate::workflow::confirm::{MAX_CONFIRM_BODY_SIZE, TRUNCATION_WARNING_BODY};
    use alloc::boxed::Box;
    use hex_lit::hex;

    const MESSAGE: &str = "message";

    #[test]
    fn test_validate_external_service_keypath() {
        assert!(validate_external_service_keypath(&[EXTERNAL_SERVICE_PURPOSE_M45]).is_ok());
        assert!(validate_external_service_keypath(&[EXTERNAL_SERVICE_PURPOSE_M48]).is_ok());
        assert!(
            validate_external_service_keypath(&[
                EXTERNAL_SERVICE_PURPOSE_M45,
                0 + HARDENED,
                0,
                1 + HARDENED,
                1,
                2 + HARDENED,
                2,
                3 + HARDENED,
                3,
                4,
            ])
            .is_ok()
        );

        assert_eq!(
            validate_external_service_keypath(&[]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_external_service_keypath(&[45]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_external_service_keypath(&[48]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_external_service_keypath(&[44 + HARDENED]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_external_service_keypath(&[46 + HARDENED]),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_external_service_keypath(
                &[EXTERNAL_SERVICE_PURPOSE_M48; MAX_KEYPATH_DEPTH + 1]
            ),
            Err(Error::InvalidInput)
        );
    }

    #[async_test::test]
    pub async fn test_p2wpkh() {
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
            process(&mut mock_hal, &request).await,
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
                    body: "bc1q k5f9 em9q c8yf pks8 ngfg 3h8h 02n2 e3ye qdyh pt".into(),
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

    #[async_test::test]
    pub async fn test_p2wpkh_m45_keypath() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: vec![
                    EXTERNAL_SERVICE_PURPOSE_M45,
                    0 + HARDENED,
                    0 + HARDENED,
                    0,
                    0,
                ],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            process(&mut mock_hal, &request).await,
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: hex!(
                    "29eae5774a7cd1393746121a5533a683d7927fe29066982b88b342a52d216217720610ef451ab9d60e8bbe519057c6619d16a06351aa76d695fe6fb13c5b1f1b00"
                )
                .to_vec(),
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: "Coin: Bitcoin\nExternal service key".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Address".into(),
                    body: "bc1q y5mk a6rf x0ek uwfx 5nkk 098y kfec xxfn 6uja 5z".into(),
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

    #[async_test::test]
    pub async fn test_p2wpkh_m48_keypath() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: vec![
                    EXTERNAL_SERVICE_PURPOSE_M48,
                    0 + HARDENED,
                    0 + HARDENED,
                    0,
                    0,
                ],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            process(&mut mock_hal, &request).await,
            Ok(Response::SignMessage(pb::BtcSignMessageResponse {
                signature: hex!(
                    "10be07206be68b250970f88faee73e666fa7cfc071965889617b68badef5291d2aee2404a36d5e2e43511b03825e146ade250a8a317accb93bfa2ca10dbbeb1301"
                )
                .to_vec(),
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: "Coin: Bitcoin\nExternal service key".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Address".into(),
                    body: "bc1q unjs etyg npj5 fq08 74ex 9qgx d9ue tyk4 lgqf x3".into(),
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

    #[async_test::test]
    pub async fn test_p2wpkh_long_message_warning() {
        let msg = "m".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2wpkh as _)),
                }),
                keypath: vec![84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: msg.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert!(process(&mut mock_hal, &request).await.is_ok());
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
                    body: "bc1q k5f9 em9q c8yf pks8 ngfg 3h8h 02n2 e3ye qdyh pt".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Warning".into(),
                    body: TRUNCATION_WARNING_BODY.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: msg,
                    longtouch: true,
                },
            ]
        );
    }

    #[async_test::test]
    pub async fn test_p2wpkh_testnet() {
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
        assert!(process(&mut mock_hal, &request).await.is_ok());
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
                    body: "tb1q nlyr q9ps hg0v 0lsu udjg ga4n vmjx hcvk etqw dg".into(),
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

    #[async_test::test]
    pub async fn test_p2wpkh_p2sh() {
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
            process(&mut mock_hal, &request).await,
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
                    body: "3BaL 6Xec vLAi dPTo UDhX o1zx D99Z UrEr pd".into(),
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

    #[async_test::test]
    pub async fn test_process_user_aborted() {
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
            process(&mut mock_hal, &request).await,
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
            process(&mut mock_hal, &request).await,
            Err(Error::UserAbort)
        );
        assert_eq!(mock_hal.ui.screens.len(), 2);

        // Message verification aborted.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(2);
        assert_eq!(
            process(&mut mock_hal, &request).await,
            Err(Error::UserAbort)
        );
        assert_eq!(mock_hal.ui.screens.len(), 3);
    }

    #[async_test::test]
    pub async fn test_process_failures() {
        const KEYPATH: &[u32] = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        // Invalid coin
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Invalid script type (invalid simple type)
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Invalid script type (taproot not supported)
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Invalid script type (multisig not supported)
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Message too long
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Invalid keypath
        mock_unlocked();
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );
        // Invalid keypath (mainnet keypath on testnet)
        mock_unlocked();
        assert_eq!(
            process(
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
            )
            .await,
            Err(Error::InvalidInput)
        );
    }
}
