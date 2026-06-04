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
use bitcoin::hashes::Hash as _;

use super::{bip322, common, params};
use crate::keystore::Compute;

const MAX_MESSAGE_SIZE: usize = 1024;

/// Compute the BIP-322 signature for a P2TR key-path spend.
///
/// Uses bip322::sighash() for the sighash and bip322::encode_simple_witness() for encoding.
async fn sign_bip322_p2tr(
    hal: &mut impl crate::hal::Hal,
    coin: BtcCoin,
    keypath: &[u32],
    msg: &[u8],
) -> Result<Vec<u8>, Error> {
    let coin_params = params::get(coin);
    let mut xpub_cache = crate::xpubcache::XpubCache::new(Compute::Twice);

    let script_pubkey =
        common::Payload::from_simple(hal, &mut xpub_cache, coin_params, SimpleType::P2tr, keypath)
            .await?
            .pk_script(coin_params)?;

    // Simple format: version=0, locktime=0, sequence=0.
    let sighash = bip322::sighash(msg, &script_pubkey, 0, 0, 0, bip322::SighashMode::Taproot);

    // BIP-86 key-path spend: tweak private key by hash of public key (no merkle root).
    let xpub = xpub_cache.get_xpub(hal, keypath).await?;
    let pubkey = bitcoin::PublicKey::from_slice(xpub.public_key()).map_err(|_| Error::Generic)?;
    let tweak = bitcoin::TapTweakHash::from_key_and_tweak(pubkey.into(), None);

    let sig =
        keystore::secp256k1_schnorr_sign(hal, keypath, &sighash, Some(tweak.as_byte_array()))
        .await?;

    Ok(bip322::encode_simple_witness(&sig))
}

/// Compute the legacy "Bitcoin Signed Message" signature (ECDSA).
///
/// Returns a 65-byte signature: 64 bytes secp256k1 compact (R, S) + 1 byte recovery id.
async fn sign_legacy(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
    msg: &[u8],
    host_nonce_commitment: &Option<pb::AntiKleptoHostNonceCommitment>,
) -> Result<Vec<u8>, Error> {
    // See
    // https://github.com/spesmilo/electrum/blob/84dc181b6e7bb20e88ef6b98fb8925c5f645a765/electrum/ecc.py#L355-L358.
    // This is the message format that is widespread for p2pkh addresses.
    // Electrum re-used it for p2wpkh-p2sh and p2wpkh addresses.
    let mut extended_msg: Vec<u8> = Vec::new();
    extended_msg.extend(b"\x18Bitcoin Signed Message:\n");
    extended_msg.extend(serialize(&VarInt(msg.len() as _)));
    extended_msg.extend(msg);

    let sighash: [u8; 32] = Sha256::digest(Sha256::digest(extended_msg)).into();

    let host_nonce = match host_nonce_commitment {
        // Engage in the anti-klepto protocol if the host sends a host nonce commitment.
        Some(pb::AntiKleptoHostNonceCommitment { commitment }) => {
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

        // Return signature directly without the anti-klepto protocol, for backwards compatibility.
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
    Ok(signature)
}

/// Process a sign message request.
///
/// For non-taproot types, the result contains a 65 byte signature. The first 64 bytes are the
/// secp256k1 signature in compact format (R and S values), and the last byte is the recoverable id
/// (recid).
///
/// For P2TR (taproot), the result is a BIP-322 "simple" encoded signature: the 3-byte ASCII
/// variant prefix `smp` followed by a serialized witness containing a single 64-byte Schnorr
/// signature (69 bytes total).
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
    if request.msg.len() > MAX_MESSAGE_SIZE {
        return Err(Error::InvalidInput);
    }

    // Keypath and script_config are validated in address_simple().
    let address = super::derive_address_simple(
        hal,
        coin,
        simple_type,
        keypath,
        crate::keystore::Compute::Twice,
    )
    .await?;
    let address_formatted = util::strings::format_address(&address);

    let basic_info = format!("Coin: {}", super::params::get(coin).name);
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

    let signature = if simple_type == SimpleType::P2tr {
        sign_bip322_p2tr(hal, coin, keypath, &request.msg).await?
    } else {
        sign_legacy(hal, keypath, &request.msg, &request.host_nonce_commitment).await?
    };

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
    use alloc::boxed::Box;
    use util::bip32::HARDENED;

    const MESSAGE: &str = "message";

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
    pub async fn test_p2tr() {
        let request = pb::BtcSignMessageRequest {
            coin: BtcCoin::Btc as _,
            script_config: Some(pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(Config::SimpleType(SimpleType::P2tr as _)),
                }),
                keypath: vec![86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
            }),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
        };

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        let result = process(&mut mock_hal, &request).await;
        assert!(result.is_ok());
        match result.unwrap() {
            Response::SignMessage(pb::BtcSignMessageResponse { signature }) => {
                // BIP-322 simple encoding: "smp" prefix (3 bytes) + base64 of consensus-encoded
                // witness stack (0x01 || 0x40 || 64-byte sig = 66 bytes => 88 base64 chars).
                assert_eq!(signature.len(), 91);
                assert_eq!(&signature[..3], b"smp");
            }
            _ => panic!("expected SignMessage response"),
        }
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
                    body: "bc1p z6xe mnzk 9nzj pt5a z3v8 27st 6e72 emt3 364v z6u3 p7gl rg56 r39q et59 hc".into(),
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
