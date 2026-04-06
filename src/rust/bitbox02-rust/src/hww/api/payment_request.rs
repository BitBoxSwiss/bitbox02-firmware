// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::ConfirmParams;
use crate::pb;

use alloc::vec::Vec;
#[cfg(feature = "app-ethereum")]
use num_bigint::BigUint;

use pb::btc_payment_request_request::{Memo, memo};

use crate::hal::Ui;
use crate::secp256k1::SECP256K1;
use crate::workflow::verify_message;

use hex_lit::hex;
use sha2::{Digest, Sha256};

use bitcoin::consensus::encode::{VarInt, serialize};
use bitcoin::secp256k1;

// Arbitrary limit on number of memos that a payment request can show to the user.
const MAX_MEMOS_NUM: usize = 3;

struct Identity {
    name: &'static str,
    public_key: &'static [u8],
}

const IDENTITIES: &[Identity] = &[
    Identity {
        name: "POCKET",
        public_key: &hex!("022902b4ede482a907ce16a1c634145e728f1de4f249043a8be47df27db9320c2c"),
    },
    Identity {
        name: "SWAPKIT",
        public_key: &hex!("03098cba9cde720171796a5c58cb774b0cd19deb62e9b51df5967aefeba34632ff"),
    },
    #[cfg(any(feature = "testing", feature = "c-unit-testing"))]
    Identity {
        name: "Test Merchant",
        // private_key: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        public_key: &hex!("02e5a018b3a2e155316109d9cdc5eab739759c0e07e0c00bf9fccb8237fe4d7f02"),
    },
];

/// Looks up the signing identity for a payment request recipient name.
///
/// Most recipients must match an entry in `IDENTITIES` exactly. `SWAPKIT` is a
/// special case: any recipient name containing `swapkit`, in any ASCII case,
/// is matched to the fixed `SWAPKIT` identity so provider-specific or legacy
/// naming variants are accepted.
fn find_identity(name: &str) -> Option<&Identity> {
    if name.to_ascii_uppercase().contains("SWAPKIT") {
        return IDENTITIES
            .iter()
            .find(|identity| identity.name == "SWAPKIT");
    }
    IDENTITIES.iter().find(|identity| identity.name == name)
}

pub(super) fn contains_coin_purchase_memo(payment_request: &pb::BtcPaymentRequestRequest) -> bool {
    payment_request.memos.iter().any(|memo| {
        matches!(
            memo,
            Memo {
                memo: Some(memo::Memo::CoinPurchaseMemo(_)),
            }
        )
    })
}

/// Parses a human-readable coin purchase amount of the form
/// "<positive-number> <unit>", where the number may be an integer or decimal,
/// and returns the amount/unit parts.
fn parse_coin_purchase_amount(amount: &str) -> Result<(&str, &str), Error> {
    let mut parts = amount.split_ascii_whitespace();
    let destination_amount = parts.next().ok_or(Error::InvalidInput)?;
    let destination_unit = parts.next().ok_or(Error::InvalidInput)?;
    if parts.next().is_some() {
        return Err(Error::InvalidInput);
    }

    let mut decimal_parts = destination_amount.split('.');
    let integer = match decimal_parts.next() {
        Some(integer) if !integer.is_empty() => integer,
        _ => return Err(Error::InvalidInput),
    };
    if !integer.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::InvalidInput);
    }

    let fractional = match decimal_parts.next() {
        Some(fractional) => {
            if fractional.is_empty() || !fractional.bytes().all(|b| b.is_ascii_digit()) {
                return Err(Error::InvalidInput);
            }
            fractional
        }
        None => "",
    };
    if decimal_parts.next().is_some() {
        return Err(Error::InvalidInput);
    }
    if integer.bytes().chain(fractional.bytes()).all(|b| b == b'0') {
        return Err(Error::InvalidInput);
    }

    Ok((destination_amount, destination_unit))
}

/// Prompt the user to verify the payment request UI flow.
/// The caller is responsible for formatting `payment_request.total_amount`
/// into the final display string for the sent amount, e.g. `"0.1 BTC"`.
pub async fn user_verify(
    hal: &mut impl crate::hal::Hal,
    payment_request: &pb::BtcPaymentRequestRequest,
    displayed_source_amount: &str,
) -> Result<(), Error> {
    if find_identity(&payment_request.recipient_name).is_none() {
        return Err(Error::InvalidInput);
    }
    hal.ui()
        .verify_recipient(&payment_request.recipient_name, displayed_source_amount)
        .await?;
    for memo in payment_request.memos.iter() {
        match memo {
            Memo {
                memo: Some(memo::Memo::TextMemo(text_memo)),
            } => {
                if !util::ascii::is_printable_ascii(
                    &text_memo.note,
                    util::ascii::Charset::AllNewline,
                ) {
                    return Err(Error::InvalidInput);
                }
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: "",
                        body: &format!("Memo from\n\n{}", payment_request.recipient_name),
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
                verify_message::verify(hal, "Memo", "Memo", text_memo.note.as_bytes(), false)
                    .await?;
            }
            Memo {
                memo: Some(memo::Memo::CoinPurchaseMemo(coin_purchase_memo)),
            } => {
                let swap_body = format!(
                    "{displayed_source_amount}\nto\n{}",
                    coin_purchase_memo.amount
                );
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: "SWAP",
                        body: &swap_body,
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
                let (_, destination_unit) = parse_coin_purchase_amount(&coin_purchase_memo.amount)?;
                let address_derivation = coin_purchase_memo
                    .address_derivation
                    .as_ref()
                    .ok_or(Error::InvalidInput)?;
                let destination_account = match address_derivation {
                    memo::coin_purchase_memo::AddressDerivation::Eth(eth) => {
                        eth.keypath
                            .get(2)
                            .ok_or(Error::InvalidInput)?
                            .checked_sub(util::bip32::HARDENED)
                            .ok_or(Error::InvalidInput)?
                            + 1
                    }
                    memo::coin_purchase_memo::AddressDerivation::Btc(btc) => {
                        if !matches!(
                            (coin_purchase_memo.coin_type, destination_unit),
                            (0, "BTC") | (2, "LTC")
                        ) {
                            return Err(Error::InvalidInput);
                        }

                        let script_config =
                            btc.script_config.as_ref().ok_or(Error::InvalidInput)?;

                        script_config
                            .keypath
                            .get(2)
                            .ok_or(Error::InvalidInput)?
                            .checked_sub(util::bip32::HARDENED)
                            .ok_or(Error::InvalidInput)?
                            + 1
                    }
                };
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: "Receive to",
                        body: &format!("{destination_unit} account #{destination_account}"),
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
            }
            _ => return Err(Error::InvalidInput),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum ValidationError {
    UnknownRecipient,
    InvalidSignature,
    #[cfg(feature = "app-ethereum")]
    AddressMismatch,
    #[cfg(not(feature = "app-ethereum"))]
    Disabled,
    Other,
}

fn hash_data_lenprefixed<U: digest::Update>(hasher: &mut U, data: &[u8]) {
    hasher.update(&serialize(&VarInt(data.len() as u64)));
    hasher.update(data);
}

// Compute payment request sighash.  See
// https://github.com/satoshilabs/slips/blob/master/slip-0024.md#signature-generation For now, only
// one output for a payment request is supported - for multiple output support, the data must be
// streamed into the hasher.
fn compute_sighash(
    source_coin_type: u32,
    payment_request: &pb::BtcPaymentRequestRequest,
    output_value: &[u8],
    output_address: &str,
) -> Result<Vec<u8>, ValidationError> {
    let mut sighash = Sha256::new();
    // versionMagic
    sighash.update(b"SL\x00\x24");
    // nonce
    hash_data_lenprefixed(&mut sighash, &payment_request.nonce);
    // recipientName
    hash_data_lenprefixed(&mut sighash, payment_request.recipient_name.as_bytes());
    // memos
    sighash.update(serialize(&VarInt(payment_request.memos.len() as u64)));
    for memo in payment_request.memos.iter() {
        match memo {
            Memo {
                memo: Some(memo::Memo::TextMemo(text_memo)),
            } => {
                sighash.update(1u32.to_le_bytes());
                hash_data_lenprefixed(&mut sighash, text_memo.note.as_bytes());
            }
            #[cfg(feature = "app-ethereum")]
            Memo {
                memo: Some(memo::Memo::CoinPurchaseMemo(coin_purchase_memo)),
            } => {
                // Only hash SLIP-24 fields. address_derivation is BitBox-specific and not
                // part of the signed payload.
                sighash.update(3u32.to_le_bytes()); // CoinPurchaseMemo type, not Protobuf field.
                sighash.update(coin_purchase_memo.coin_type.to_le_bytes());
                hash_data_lenprefixed(&mut sighash, coin_purchase_memo.amount.as_bytes());
                hash_data_lenprefixed(&mut sighash, coin_purchase_memo.address.as_bytes());
            }
            _ => return Err(ValidationError::Other),
        }
    }
    // coinType
    sighash.update(source_coin_type.to_le_bytes());
    // outputsHash (only one output for now).
    sighash.update({
        let mut output_hasher = Sha256::new();
        output_hasher.update(output_value);
        hash_data_lenprefixed(&mut output_hasher, output_address.as_bytes());
        output_hasher.finalize()
    });
    Ok(sighash.finalize().to_vec())
}

#[cfg(feature = "testing")]
#[allow(dead_code)]
pub fn tst_sign_payment_request_btc(
    coin: pb::BtcCoin,
    payment_request: &mut pb::BtcPaymentRequestRequest,
    total_value: u64,
    output_address: &str,
) {
    let total_value_bytes = total_value.to_le_bytes();
    tst_sign_payment_request(
        super::bitcoin::params::get(coin).slip44(),
        payment_request,
        &total_value_bytes,
        output_address,
    );
}

#[cfg(feature = "testing")]
#[allow(dead_code)]
pub fn tst_sign_payment_request_eth(
    source_coin_type: u32,
    payment_request: &mut pb::BtcPaymentRequestRequest,
    total_value_bytes: &[u8; 32],
    output_address: &str,
) {
    tst_sign_payment_request(
        source_coin_type,
        payment_request,
        total_value_bytes,
        output_address,
    );
}

#[cfg(feature = "testing")]
#[allow(dead_code)]
fn tst_sign_payment_request(
    source_coin_type: u32,
    payment_request: &mut pb::BtcPaymentRequestRequest,
    output_value: &[u8],
    output_address: &str,
) {
    let sighash = compute_sighash(
        source_coin_type,
        payment_request,
        output_value,
        output_address,
    )
    .unwrap();

    let privkey = secp256k1::SecretKey::from_slice(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    let msg = secp256k1::Message::from_digest_slice(&sighash).unwrap();
    let sig = SECP256K1.sign_ecdsa(&msg, &privkey);
    payment_request.signature = sig.serialize_compact().to_vec();
}

fn ecdsa_verify(sig64: &[u8], msg32: &[u8], pubkey33: &[u8]) -> Result<(), ValidationError> {
    let pubkey = secp256k1::PublicKey::from_slice(pubkey33)
        .map_err(|_| ValidationError::InvalidSignature)?;
    let msg = secp256k1::Message::from_digest_slice(msg32).unwrap();
    let sig = secp256k1::ecdsa::Signature::from_compact(sig64)
        .map_err(|_| ValidationError::InvalidSignature)?;
    SECP256K1
        .verify_ecdsa(&msg, &sig, &pubkey)
        .map_err(|_| ValidationError::InvalidSignature)
}

/// Validate a BTC payment request against the parsed BTC output total.
pub fn validate_btc(
    #[cfg_attr(not(feature = "app-ethereum"), allow(unused_variables))] hal: &mut impl crate::hal::Hal,
    coin_params: &super::bitcoin::params::Params,
    payment_request: &pb::BtcPaymentRequestRequest,
    total_value: u64,
    output_address: &str,
) -> Result<(), ValidationError> {
    let total_value_bytes = total_value.to_le_bytes();
    if total_value_bytes.as_slice() != payment_request.total_amount.to_le_bytes().as_slice() {
        return Err(ValidationError::Other);
    }
    validate_common(
        hal,
        coin_params.slip44(),
        payment_request,
        &total_value_bytes,
        output_address,
    )
}

/// Validate an ETH/EVM payment request against the parsed source-side transaction.
#[cfg(feature = "app-ethereum")]
pub fn validate_eth(
    hal: &mut impl crate::hal::Hal,
    coin_params: &super::ethereum::params::Params,
    payment_request: &pb::BtcPaymentRequestRequest,
    output_value: &BigUint,
    output_address: &str,
) -> Result<(), ValidationError> {
    let output_value = output_value.to_bytes_be();
    if output_value.len() > 32 {
        return Err(ValidationError::Other);
    }
    let mut output_value_padded = [0u8; 32];
    output_value_padded[32 - output_value.len()..].copy_from_slice(&output_value);
    validate_common(
        hal,
        coin_params.slip44(),
        payment_request,
        &output_value_padded,
        output_address,
    )
}

/// Validate that the parsed source-side transaction matches the signed payment request.
///
/// The caller provides the normalized source-side facts that are actually being
/// signed: the SLIP-44 source coin type, the canonical source amount bytes, and
/// the deposit/vault address.
///
/// Destination ownership checks for `CoinPurchaseMemo.address` still happen
/// here, because they are derived from the memo's keypath metadata rather than
/// from the source transaction itself.
fn validate_common(
    #[cfg_attr(not(feature = "app-ethereum"), allow(unused_variables))] hal: &mut impl crate::hal::Hal,
    source_coin_type: u32,
    payment_request: &pb::BtcPaymentRequestRequest,
    output_value: &[u8],
    output_address: &str,
) -> Result<(), ValidationError> {
    let identity =
        find_identity(&payment_request.recipient_name).ok_or(ValidationError::UnknownRecipient)?;
    if !payment_request.nonce.is_empty() {
        // No support for nonces yet.
        return Err(ValidationError::Other);
    }
    if payment_request.memos.len() > MAX_MEMOS_NUM {
        return Err(ValidationError::Other);
    }
    if contains_coin_purchase_memo(payment_request) && payment_request.memos.len() != 1 {
        return Err(ValidationError::Other);
    }
    for memo in payment_request.memos.iter() {
        if let Memo {
            memo: Some(memo::Memo::CoinPurchaseMemo(coin_purchase_memo)),
        } = memo
        {
            match &coin_purchase_memo.address_derivation {
                Some(memo::coin_purchase_memo::AddressDerivation::Eth(_eth)) => {
                    #[cfg(feature = "app-ethereum")]
                    {
                        let derived_address = super::ethereum::derive_address(hal, &_eth.keypath)
                            .map_err(|_| ValidationError::Other)?;
                        if derived_address != coin_purchase_memo.address {
                            return Err(ValidationError::AddressMismatch);
                        }
                    }
                    #[cfg(not(feature = "app-ethereum"))]
                    return Err(ValidationError::Disabled);
                }
                Some(memo::coin_purchase_memo::AddressDerivation::Btc(_btc)) => {
                    #[cfg(feature = "app-litecoin")]
                    {
                        let (_, destination_unit) =
                            parse_coin_purchase_amount(&coin_purchase_memo.amount)
                                .map_err(|_| ValidationError::Other)?;

                        let destination_coin =
                            match (coin_purchase_memo.coin_type, destination_unit) {
                                (0, "BTC") => pb::BtcCoin::Btc,
                                (2, "LTC") => pb::BtcCoin::Ltc,
                                _ => return Err(ValidationError::Other),
                            };

                        let script_config =
                            _btc.script_config.as_ref().ok_or(ValidationError::Other)?;

                        let simple_type = match script_config.script_config.as_ref() {
                            Some(pb::BtcScriptConfig {
                                config: Some(pb::btc_script_config::Config::SimpleType(simple_type)),
                            }) => pb::btc_script_config::SimpleType::try_from(*simple_type)
                                .map_err(|_| ValidationError::Other)?,
                            _ => return Err(ValidationError::Other),
                        };

                        let derived_address = super::bitcoin::derive_address_simple(
                            hal,
                            destination_coin,
                            simple_type,
                            &script_config.keypath,
                        )
                        .map_err(|_| ValidationError::Other)?;

                        if derived_address != coin_purchase_memo.address {
                            return Err(ValidationError::AddressMismatch);
                        }
                    }
                    #[cfg(not(feature = "app-litecoin"))]
                    return Err(ValidationError::Disabled);
                }
                None => return Err(ValidationError::Other),
            }
        }
    }
    let sighash = compute_sighash(
        source_coin_type,
        payment_request,
        output_value,
        output_address,
    )?;
    ecdsa_verify(&payment_request.signature, &sighash, identity.public_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::hww::api::bitcoin::params;
    #[cfg(feature = "app-ethereum")]
    use crate::hww::api::ethereum::params as eth_params;

    fn make_text_memo(note: &str) -> Memo {
        Memo {
            memo: Some(memo::Memo::TextMemo(memo::TextMemo { note: note.into() })),
        }
    }

    #[cfg(feature = "app-ethereum")]
    fn make_coin_purchase_memo(
        coin_type: u32,
        amount: &str,
        address: &str,
        address_derivation: Option<memo::coin_purchase_memo::AddressDerivation>,
    ) -> Memo {
        Memo {
            memo: Some(memo::Memo::CoinPurchaseMemo(memo::CoinPurchaseMemo {
                coin_type,
                amount: amount.into(),
                address: address.into(),
                address_derivation,
            })),
        }
    }

    #[cfg(feature = "app-litecoin")]
    fn dummy_eth_address_derivation(valid: bool) -> memo::coin_purchase_memo::AddressDerivation {
        let coin_type = if valid { 60 } else { 0 };
        memo::coin_purchase_memo::AddressDerivation::Eth(
            memo::coin_purchase_memo::EthAddressDerivation {
                keypath: vec![
                    44 + util::bip32::HARDENED,
                    coin_type + util::bip32::HARDENED,
                    0 + util::bip32::HARDENED,
                    0,
                    0,
                ],
            },
        )
    }

    #[cfg(feature = "app-ethereum")]
    fn dummy_btc_address_derivation(
        simple_type: pb::btc_script_config::SimpleType,
        keypath: &[u32],
    ) -> memo::coin_purchase_memo::AddressDerivation {
        memo::coin_purchase_memo::AddressDerivation::Btc(
            memo::coin_purchase_memo::BtcAddressDerivation {
                script_config: Some(pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(simple_type as _)),
                    }),
                    keypath: keypath.to_vec(),
                }),
            },
        )
    }

    #[test]
    fn test_find_identity() {
        assert_eq!(find_identity("POCKET").unwrap().name, "POCKET");

        let swapkit_identity = find_identity("SWAPKIT (Provider)").unwrap();
        assert_eq!(swapkit_identity.name, "SWAPKIT");
        assert_eq!(
            swapkit_identity.public_key,
            hex!("03098cba9cde720171796a5c58cb774b0cd19deb62e9b51df5967aefeba34632ff")
        );

        assert_eq!(find_identity("SWAPKIT Provider").unwrap().name, "SWAPKIT");
        assert_eq!(find_identity("swapkit (Provider)").unwrap().name, "SWAPKIT");
        assert_eq!(find_identity("SWAPKIT").unwrap().name, "SWAPKIT");
        assert_eq!(find_identity("SwapKit").unwrap().name, "SWAPKIT");

        assert!(find_identity("Provider").is_none());
    }

    #[test]
    fn test_parse_coin_purchase_amount() {
        assert_eq!(parse_coin_purchase_amount("0.25 ETH"), Ok(("0.25", "ETH")));
        assert_eq!(parse_coin_purchase_amount("1 ETH"), Ok(("1", "ETH")));
        assert_eq!(
            parse_coin_purchase_amount("14128 eth"),
            Ok(("14128", "eth"))
        );
        assert_eq!(
            parse_coin_purchase_amount("3481471947 SC"),
            Ok(("3481471947", "SC"))
        );

        for amount in [
            "",
            "ETH",
            "0 ETH",
            "0.0 ETH",
            "-1 ETH",
            ".25 ETH",
            "1. ETH",
            "1.2.3 ETH",
            "foo ETH",
            "foo bar baz",
            "1 ETH extra",
        ] {
            assert_eq!(parse_coin_purchase_amount(amount), Err(Error::InvalidInput));
        }
    }

    #[test]
    fn test_sighash() {
        let source_coin_type = params::get(pb::BtcCoin::Tbtc).slip44();
        let output_value = 123456u64.to_le_bytes().to_vec();

        let sighash = compute_sighash(
            source_coin_type,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            },
            &output_value,
            "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
        )
        .unwrap();
        assert_eq!(
            hex::encode(sighash),
            "d6b996da9ea1129b601e4ec2edf54aa67cf917e1e3bca82be0f8302af9138fac"
        );

        let sighash = compute_sighash(
            source_coin_type,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![make_text_memo("TextMemo 1"), make_text_memo("TextMemo 2")],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            },
            &output_value,
            "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
        )
        .unwrap();
        assert_eq!(
            hex::encode(sighash),
            "9303ef0189ab78e92b7518ebf9851bf567ca06ddce242fb33220c3b31a489251"
        );

        #[cfg(feature = "app-ethereum")]
        {
            // Verify that keypath does not influence the sighash.
            let payment_request_without = pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0xabc1234567890",
                    None,
                )],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            };
            let sighash_without = compute_sighash(
                source_coin_type,
                &payment_request_without,
                &output_value,
                "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
            )
            .unwrap();

            let payment_request_with = pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0xabc1234567890",
                    Some(dummy_eth_address_derivation(/*valid=*/ true)),
                )],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            };
            let sighash_with = compute_sighash(
                source_coin_type,
                &payment_request_with,
                &output_value,
                "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
            )
            .unwrap();

            assert_eq!(sighash_without, sighash_with);

            assert_eq!(
                hex::encode(sighash_without),
                "1806caf7c518aad69eb38f25fd418d507c6a3e01719a7d77be94cd50a2790872"
            );
        }
    }

    #[test]
    fn test_validate() {
        let source_coin_type = params::get(pb::BtcCoin::Tbtc).slip44();
        let mut mock_hal = TestingHal::new();

        let value = 123456u64;
        let value_bytes = value.to_le_bytes().to_vec();
        let address = "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup";

        let mut payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        tst_sign_payment_request(
            source_coin_type,
            &mut payment_request,
            &value_bytes,
            address,
        );

        assert!(
            validate_common(
                &mut mock_hal,
                source_coin_type,
                &payment_request,
                &value_bytes,
                address
            )
            .is_ok()
        );

        #[cfg(feature = "app-ethereum")]
        {
            // CoinPurchase memo with matching keypath and address. The
            // address results from the keypath which is used in dummy_eth_address_derivation().
            // See src/rust/bitbox02-rust/src/hww/api/ethereum/pubrequest.rs
            use crate::keystore::testing::mock_unlocked;
            mock_unlocked();
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0x773A77b9D32589be03f9132AF759e294f7851be9",
                    Some(dummy_eth_address_derivation(/*valid=*/ true)),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                )
                .is_ok()
            );
        }

        #[cfg(feature = "app-litecoin")]
        {
            // BTC -> LTC swap
            let source_keypath = [
                84 + util::bip32::HARDENED,
                0 + util::bip32::HARDENED,
                11 + util::bip32::HARDENED,
                0,
                0,
            ];
            let source_address = super::super::bitcoin::derive_address_simple(
                &mut mock_hal,
                pb::BtcCoin::Btc,
                pb::btc_script_config::SimpleType::P2wpkh,
                &source_keypath,
            )
            .unwrap();

            let destination_keypath = [
                84 + util::bip32::HARDENED,
                2 + util::bip32::HARDENED,
                0 + util::bip32::HARDENED,
                0,
                0,
            ];
            let destination_address = super::super::bitcoin::derive_address_simple(
                &mut mock_hal,
                pb::BtcCoin::Ltc,
                pb::btc_script_config::SimpleType::P2wpkh,
                &destination_keypath,
            )
            .unwrap();

            let source_coin_type = params::get(pb::BtcCoin::Btc).slip44();

            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    2,
                    "0.25 LTC",
                    &destination_address,
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &destination_keypath,
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };

            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                &source_address,
            );

            assert!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    &source_address,
                )
                .is_ok()
            );
        }

        #[cfg(feature = "app-litecoin")]
        {
            // LTC -> BTC swap
            let source_keypath = [
                84 + util::bip32::HARDENED,
                2 + util::bip32::HARDENED,
                11 + util::bip32::HARDENED,
                0,
                0,
            ];
            let source_address = super::super::bitcoin::derive_address_simple(
                &mut mock_hal,
                pb::BtcCoin::Ltc,
                pb::btc_script_config::SimpleType::P2wpkh,
                &source_keypath,
            )
            .unwrap();

            let destination_keypath = [
                84 + util::bip32::HARDENED,
                0 + util::bip32::HARDENED,
                0 + util::bip32::HARDENED,
                0,
                0,
            ];
            let destination_address = super::super::bitcoin::derive_address_simple(
                &mut mock_hal,
                pb::BtcCoin::Btc,
                pb::btc_script_config::SimpleType::P2wpkh,
                &destination_keypath,
            )
            .unwrap();

            let source_coin_type = params::get(pb::BtcCoin::Ltc).slip44();

            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    0,
                    "0.25 BTC",
                    &destination_address,
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &destination_keypath,
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };

            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                &source_address,
            );

            assert!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    &source_address,
                )
                .is_ok()
            );
        }

        // Unhappy cases:

        #[cfg(feature = "app-ethereum")]
        {
            // Invalid ETH keypath in CoinPurchaseMemo
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0xabc1234567890",
                    Some(dummy_eth_address_derivation(/*valid=*/ false)),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            // Sign it so the only failure reason is the keypath validation.
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        #[cfg(feature = "app-ethereum")]
        {
            // Valid keypath but address mismatch
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0xWRONG_ADDRESS_THAT_DOESNT_MATCH",
                    Some(dummy_eth_address_derivation(/*valid=*/ true)),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut TestingHal::new(),
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::AddressMismatch)
            ));
        }

        #[cfg(feature = "app-ethereum")]
        {
            // Missing address_derivation in CoinPurchaseMemo
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0xabc1234567890",
                    None,
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        #[cfg(feature = "app-ethereum")]
        {
            // CoinPurchaseMemo must be the only memo in a payment request.
            for mut payment_request in [
                pb::BtcPaymentRequestRequest {
                    recipient_name: "Test Merchant".into(),
                    memos: vec![
                        make_text_memo("memo"),
                        make_coin_purchase_memo(
                            60,
                            "0.25 ETH",
                            "0x773A77b9D32589be03f9132AF759e294f7851be9",
                            Some(dummy_eth_address_derivation(/*valid=*/ true)),
                        ),
                    ],
                    nonce: vec![],
                    total_amount: value,
                    signature: vec![],
                },
                pb::BtcPaymentRequestRequest {
                    recipient_name: "Test Merchant".into(),
                    memos: vec![
                        make_coin_purchase_memo(
                            60,
                            "0.25 ETH",
                            "0x773A77b9D32589be03f9132AF759e294f7851be9",
                            Some(dummy_eth_address_derivation(/*valid=*/ true)),
                        ),
                        make_coin_purchase_memo(
                            60,
                            "0.50 ETH",
                            "0x773A77b9D32589be03f9132AF759e294f7851be9",
                            Some(dummy_eth_address_derivation(/*valid=*/ true)),
                        ),
                    ],
                    nonce: vec![],
                    total_amount: value,
                    signature: vec![],
                },
            ] {
                tst_sign_payment_request(
                    source_coin_type,
                    &mut payment_request,
                    &value_bytes,
                    address,
                );
                assert!(matches!(
                    validate_common(
                        &mut mock_hal,
                        source_coin_type,
                        &payment_request,
                        &value_bytes,
                        address
                    ),
                    Err(ValidationError::Other)
                ));
            }
        }

        #[cfg(feature = "app-litecoin")]
        {
            // BTC-like destination keypath is valid, but claimed address does not match.
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    2,
                    "0.25 LTC",
                    "ltc1qwrongdestinationaddressthatdoesnotmatch4w7g4j",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[
                            84 + util::bip32::HARDENED,
                            2 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0,
                            0,
                        ],
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::AddressMismatch)
            ));
        }

        #[cfg(feature = "app-ethereum")]
        {
            // BTC-like destinations only support BTC/LTC mainnet coin_type values.
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    1,
                    "0.25 BTC",
                    "bc1qanything",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[
                            84 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0,
                            0,
                        ],
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        #[cfg(feature = "app-litecoin")]
        {
            // BTC-like amount unit must agree with the destination coin_type.
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    2,
                    "0.25 BTC",
                    "bc1qanything",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[
                            84 + util::bip32::HARDENED,
                            2 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0,
                            0,
                        ],
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        #[cfg(feature = "app-litecoin")]
        {
            // Invalid BTC keypath in CoinPurchaseMemo
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    0,
                    "0.25 BTC",
                    "bc1qanything",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[
                            84 + util::bip32::HARDENED,
                            2 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0,
                            0,
                        ],
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        #[cfg(feature = "app-litecoin")]
        {
            // BTC-like destinations are simple singlesig only.
            let mut payment_request = pb::BtcPaymentRequestRequest {
                recipient_name: "Test Merchant".into(),
                memos: vec![make_coin_purchase_memo(
                    0,
                    "0.25 BTC",
                    "bc1qanything",
                    Some(memo::coin_purchase_memo::AddressDerivation::Btc(
                        memo::coin_purchase_memo::BtcAddressDerivation {
                            script_config: Some(pb::BtcScriptConfigWithKeypath {
                                script_config: Some(pb::BtcScriptConfig {
                                    config: Some(pb::btc_script_config::Config::Multisig(
                                        pb::btc_script_config::Multisig {
                                            threshold: 1,
                                            xpubs: vec![],
                                            our_xpub_index: 0,
                                            script_type:
                                                pb::btc_script_config::multisig::ScriptType::P2wsh
                                                    as _,
                                        },
                                    )),
                                }),
                                keypath: vec![
                                    48 + util::bip32::HARDENED,
                                    0 + util::bip32::HARDENED,
                                    0 + util::bip32::HARDENED,
                                    2 + util::bip32::HARDENED,
                                    0,
                                    0,
                                ],
                            }),
                        },
                    )),
                )],
                nonce: vec![],
                total_amount: value,
                signature: vec![],
            };
            tst_sign_payment_request(
                source_coin_type,
                &mut payment_request,
                &value_bytes,
                address,
            );
            assert!(matches!(
                validate_common(
                    &mut mock_hal,
                    source_coin_type,
                    &payment_request,
                    &value_bytes,
                    address
                ),
                Err(ValidationError::Other)
            ));
        }

        // Unknown recipient
        let payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Unknown Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        assert!(matches!(
            validate_common(
                &mut mock_hal,
                source_coin_type,
                &payment_request,
                &value_bytes,
                address
            ),
            Err(ValidationError::UnknownRecipient)
        ));

        // Wrong output value
        let payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        assert!(matches!(
            validate_btc(
                &mut mock_hal,
                params::get(pb::BtcCoin::Tbtc),
                &payment_request,
                value + 1,
                address
            ),
            Err(ValidationError::Other)
        ));

        // Nonzero nonce (not supported yet)
        let payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![0xaa],
            total_amount: value,
            signature: vec![],
        };
        assert!(matches!(
            validate_common(
                &mut mock_hal,
                source_coin_type,
                &payment_request,
                &value_bytes,
                address
            ),
            Err(ValidationError::Other)
        ));

        // Invalid signature
        let payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        assert!(matches!(
            validate_common(
                &mut mock_hal,
                source_coin_type,
                &payment_request,
                &value_bytes,
                address
            ),
            Err(ValidationError::InvalidSignature)
        ));
    }

    #[tokio::test]
    async fn test_user_verify_text_memos() {
        // Baseline Pocket flow: recipient screen, memo intro, memo contents.
        let mut mock_hal = TestingHal::new();
        user_verify(
            &mut mock_hal,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "POCKET".into(),
                memos: vec![make_text_memo("Pocket memo")],
                nonce: vec![],
                total_amount: 1234567890,
                signature: vec![],
            },
            "12.34567890 BTC",
        )
        .await
        .unwrap();

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Recipient {
                    recipient: "POCKET".into(),
                    amount: "12.34567890 BTC".into(),
                },
                Screen::Confirm {
                    title: "".into(),
                    body: "Memo from\n\nPOCKET".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Memo".into(),
                    body: "Pocket memo".into(),
                    longtouch: false,
                },
            ]
        );
    }

    #[cfg(feature = "app-ethereum")]
    #[tokio::test]
    async fn test_user_verify_swap() {
        // Happy-path swap flow: recipient screen plus two swap-specific confirms.
        let mut mock_hal = TestingHal::new();
        user_verify(
            &mut mock_hal,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0x123",
                    Some(dummy_eth_address_derivation(/*valid=*/ true)),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            "0.25000000 BTC",
        )
        .await
        .unwrap();

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Recipient {
                    recipient: "SWAPKIT (Provider)".into(),
                    amount: "0.25000000 BTC".into(),
                },
                Screen::Confirm {
                    title: "SWAP".into(),
                    body: "0.25000000 BTC\nto\n0.25 ETH".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Receive to".into(),
                    body: "ETH account #1".into(),
                    longtouch: false,
                },
            ]
        );
    }

    #[cfg(feature = "app-litecoin")]
    #[tokio::test]
    async fn test_user_verify_swap_btc_destination() {
        // BTC -> LTC swap
        let mut mock_hal = TestingHal::new();
        user_verify(
            &mut mock_hal,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    2,
                    "0.25 LTC",
                    "ltc1qdestination",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[
                            84 + util::bip32::HARDENED,
                            2 + util::bip32::HARDENED,
                            0 + util::bip32::HARDENED,
                            0,
                            0,
                        ],
                    )),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            "0.25000000 BTC",
        )
        .await
        .unwrap();

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Recipient {
                    recipient: "SWAPKIT (Provider)".into(),
                    amount: "0.25000000 BTC".into(),
                },
                Screen::Confirm {
                    title: "SWAP".into(),
                    body: "0.25000000 BTC\nto\n0.25 LTC".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Receive to".into(),
                    body: "LTC account #1".into(),
                    longtouch: false,
                },
            ]
        );
    }

    #[cfg(feature = "app-ethereum")]
    #[test]
    fn test_validate_eth() {
        let mut mock_hal = TestingHal::new();
        let params = eth_params::Params {
            coin: Some(pb::EthCoin::Eth),
            bip44_coin: 60 + util::bip32::HARDENED,
            chain_id: 1,
            name: "Ethereum",
            unit: "ETH",
        };
        let output_value = hex!("000000000000000000000000000000000000000000000000075cf1259e9c4000");
        let output_address = "0x04F264Cf34440313B4A0192A352814FBe927b885";
        let mut payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_coin_purchase_memo(
                60,
                "0.25 ETH",
                "0x773A77b9D32589be03f9132AF759e294f7851be9",
                Some(dummy_eth_address_derivation(/*valid=*/ true)),
            )],
            nonce: vec![],
            total_amount: 0,
            signature: vec![],
        };
        tst_sign_payment_request_eth(
            params.slip44(),
            &mut payment_request,
            &output_value,
            output_address,
        );

        // Fully normalized 32-byte EVM amount bytes validate as-is.
        assert!(
            validate_eth(
                &mut mock_hal,
                &params,
                &payment_request,
                &BigUint::from_bytes_be(&output_value),
                output_address,
            )
            .is_ok()
        );

        // Native ETH values may be shorter than 32 bytes.
        assert!(matches!(
            validate_eth(
                &mut TestingHal::new(),
                &params,
                &payment_request,
                &BigUint::from_bytes_be(&output_value[24..]),
                output_address,
            ),
            Ok(())
        ));

        let wrong_output_value =
            hex!("000000000000000000000000000000000000000000000000075cf1259e9c4001");
        // Wrong source amount must produce wrong signature.
        assert!(matches!(
            validate_eth(
                &mut TestingHal::new(),
                &params,
                &payment_request,
                &BigUint::from_bytes_be(&wrong_output_value),
                output_address,
            ),
            Err(ValidationError::InvalidSignature)
        ));

        //Wrong source-side deposit address must produce wrong signature.
        assert!(matches!(
            validate_eth(
                &mut TestingHal::new(),
                &params,
                &payment_request,
                &BigUint::from_bytes_be(&output_value),
                "0x1111111111111111111111111111111111111111",
            ),
            Err(ValidationError::InvalidSignature)
        ));

        // Wrong source coin type must produce wrong signature.
        assert!(matches!(
            validate_eth(
                &mut TestingHal::new(),
                &eth_params::Params {
                    coin: Some(pb::EthCoin::Eth),
                    bip44_coin: 1 + util::bip32::HARDENED,
                    chain_id: 1,
                    name: "Ethereum",
                    unit: "ETH",
                },
                &payment_request,
                &BigUint::from_bytes_be(&output_value),
                output_address,
            ),
            Err(ValidationError::InvalidSignature)
        ));
    }

    #[cfg(all(feature = "app-litecoin", feature = "app-ethereum"))]
    #[tokio::test]
    async fn test_user_verify_swap_invalid() {
        // Invalid swap requests that user_verify must reject because the
        // UI cannot render them safely.
        for payment_request in [
            // Missing destination derivation, so "Receive to" cannot be built.
            pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(60, "0.25 ETH", "0x123", None)],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            // Destination keypath is too short to contain an account element.
            pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "0.25 ETH",
                    "0x123",
                    Some(memo::coin_purchase_memo::AddressDerivation::Eth(
                        memo::coin_purchase_memo::EthAddressDerivation {
                            keypath: vec![44 + util::bip32::HARDENED, 60 + util::bip32::HARDENED],
                        },
                    )),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            // BTC-like derivation requires script_config.
            pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    0,
                    "0.25 BTC",
                    "bc1qdestination",
                    Some(memo::coin_purchase_memo::AddressDerivation::Btc(
                        memo::coin_purchase_memo::BtcAddressDerivation {
                            script_config: None,
                        },
                    )),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            // BTC-like destination keypath is too short to contain an account element.
            pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    0,
                    "0.25 BTC",
                    "bc1qdestination",
                    Some(dummy_btc_address_derivation(
                        pb::btc_script_config::SimpleType::P2wpkh,
                        &[84 + util::bip32::HARDENED, 0 + util::bip32::HARDENED],
                    )),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
            // Display amount must be exactly "<positive-decimal> <unit>".
            pb::BtcPaymentRequestRequest {
                recipient_name: "SWAPKIT (Provider)".into(),
                memos: vec![make_coin_purchase_memo(
                    60,
                    "foo bar baz",
                    "0x123",
                    Some(dummy_eth_address_derivation(/*valid=*/ true)),
                )],
                nonce: vec![],
                total_amount: 25000000,
                signature: vec![],
            },
        ] {
            let mut mock_hal = TestingHal::new();
            assert_eq!(
                user_verify(&mut mock_hal, &payment_request, "0.25000000 BTC",).await,
                Err(Error::InvalidInput)
            );
        }
    }
}
