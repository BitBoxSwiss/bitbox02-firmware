// Copyright 2024 Shift Crypto AG
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

use super::pb;
use super::Error;

use alloc::vec::Vec;

use super::common::format_amount;
use super::params;
use super::script::serialize_varint;

use pb::btc_payment_request_request::{memo, Memo};
use pb::btc_sign_init_request::FormatUnit;

use crate::hal::Ui;
use crate::workflow::{confirm, verify_message};

use sha2::{Digest, Sha256};

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
        public_key: b"\x02\x29\x02\xb4\xed\xe4\x82\xa9\x07\xce\x16\xa1\xc6\x34\x14\x5e\x72\x8f\x1d\xe4\xf2\x49\x04\x3a\x8b\xe4\x7d\xf2\x7d\xb9\x32\x0c\x2c",
    },
    #[cfg(feature = "testing")]
    Identity {
        name: "Test Merchant",
        // private_key: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        public_key: b"\x02\xe5\xa0\x18\xb3\xa2\xe1\x55\x31\x61\x09\xd9\xcd\xc5\xea\xb7\x39\x75\x9c\x0e\x07\xe0\xc0\x0b\xf9\xfc\xcb\x82\x37\xfe\x4d\x7f\x02",
    },
];

fn find_identity(name: &str) -> Option<&Identity> {
    IDENTITIES.iter().find(|identity| identity.name == name)
}

/// Prompt user to verify the payment request.
pub async fn user_verify(
    hal: &mut impl crate::hal::Hal,
    coin_params: &params::Params,
    payment_request: &pb::BtcPaymentRequestRequest,
    format_unit: FormatUnit,
) -> Result<(), Error> {
    if find_identity(&payment_request.recipient_name).is_none() {
        return Err(Error::InvalidInput);
    }
    hal.ui()
        .verify_recipient(
            &payment_request.recipient_name,
            &format_amount(coin_params, format_unit, payment_request.total_amount)?,
        )
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
                    .confirm(&confirm::Params {
                        title: "",
                        body: &format!("Memo from\n\n{}", payment_request.recipient_name),
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
                verify_message::verify(hal, "Memo", "Memo", text_memo.note.as_bytes(), false)
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
    Other,
}

fn hash_data_lenprefixed<U: digest::Update>(hasher: &mut U, data: &[u8]) {
    hasher.update(&serialize_varint(data.len() as u64));
    hasher.update(data);
}

// Compute payment request sighash.  See
// https://github.com/satoshilabs/slips/blob/master/slip-0024.md#signature-generation For now, only
// one output for a payment request is supported - for multiple output support, the data must be
// streamed into the hasher.
fn compute_sighash(
    coin_params: &params::Params,
    payment_request: &pb::BtcPaymentRequestRequest,
    output_value: u64,
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
    sighash.update(serialize_varint(payment_request.memos.len() as u64));
    for memo in payment_request.memos.iter() {
        match memo {
            Memo {
                memo: Some(memo::Memo::TextMemo(text_memo)),
            } => {
                sighash.update(1u32.to_le_bytes());
                hash_data_lenprefixed(&mut sighash, text_memo.note.as_bytes());
            }
            _ => return Err(ValidationError::Other),
        }
    }
    // coinType
    sighash.update(coin_params.slip44().to_le_bytes());
    // outputsHash (only one output for now).
    sighash.update({
        let mut output_hasher = Sha256::new();
        output_hasher.update(output_value.to_le_bytes());
        hash_data_lenprefixed(&mut output_hasher, output_address.as_bytes());
        output_hasher.finalize()
    });
    Ok(sighash.finalize().to_vec())
}

#[cfg(feature = "testing")]
#[allow(dead_code)]
pub fn tst_sign_payment_request(
    coin_params: &params::Params,
    payment_request: &mut pb::BtcPaymentRequestRequest,
    output_value: u64,
    output_address: &str,
) {
    let sighash =
        compute_sighash(coin_params, payment_request, output_value, output_address).unwrap();

    let privkey = secp256k1::SecretKey::from_slice(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    let msg = secp256k1::Message::from_digest_slice(&sighash).unwrap();
    let secp = secp256k1::Secp256k1::new();
    let sig = secp.sign_ecdsa(&msg, &privkey);
    payment_request.signature = sig.serialize_compact().to_vec();
}

fn ecdsa_verify(sig64: &[u8], msg32: &[u8], pubkey33: &[u8]) -> Result<(), ValidationError> {
    let secp = secp256k1::Secp256k1::new();
    let pubkey = secp256k1::PublicKey::from_slice(pubkey33)
        .map_err(|_| ValidationError::InvalidSignature)?;
    let msg = secp256k1::Message::from_digest_slice(msg32).unwrap();
    let sig = secp256k1::ecdsa::Signature::from_compact(sig64)
        .map_err(|_| ValidationError::InvalidSignature)?;
    secp.verify_ecdsa(&msg, &sig, &pubkey)
        .map_err(|_| ValidationError::InvalidSignature)
}

/// Validate the payment request: amount, signature, etc.
pub fn validate(
    coin_params: &params::Params,
    payment_request: &pb::BtcPaymentRequestRequest,
    output_value: u64,
    output_address: &str,
) -> Result<(), ValidationError> {
    let identity =
        find_identity(&payment_request.recipient_name).ok_or(ValidationError::UnknownRecipient)?;
    if payment_request.total_amount != output_value {
        return Err(ValidationError::Other);
    }
    if !payment_request.nonce.is_empty() {
        // No support for nonces yet.
        return Err(ValidationError::Other);
    }
    if payment_request.memos.len() > MAX_MEMOS_NUM {
        return Err(ValidationError::Other);
    }
    let sighash = compute_sighash(coin_params, payment_request, output_value, output_address)?;
    ecdsa_verify(&payment_request.signature, &sighash, identity.public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_text_memo(note: &str) -> Memo {
        Memo {
            memo: Some(memo::Memo::TextMemo(memo::TextMemo { note: note.into() })),
        }
    }

    #[test]
    fn test_sighash() {
        let coin_params = params::get(pb::BtcCoin::Tbtc);

        let sighash = compute_sighash(
            coin_params,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            },
            123456,
            "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
        )
        .unwrap();
        assert_eq!(
            hex::encode(sighash),
            "d6b996da9ea1129b601e4ec2edf54aa67cf917e1e3bca82be0f8302af9138fac"
        );

        let sighash = compute_sighash(
            coin_params,
            &pb::BtcPaymentRequestRequest {
                recipient_name: "Merchant".into(),
                memos: vec![make_text_memo("TextMemo 1"), make_text_memo("TextMemo 2")],
                nonce: vec![],
                total_amount: 123456,
                signature: vec![],
            },
            123456,
            "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup",
        )
        .unwrap();
        assert_eq!(
            hex::encode(sighash),
            "9303ef0189ab78e92b7518ebf9851bf567ca06ddce242fb33220c3b31a489251"
        );
    }

    #[test]
    fn test_validate() {
        let coin_params = params::get(pb::BtcCoin::Tbtc);

        let value = 123456u64;
        let address = "tb1q2q0j6gmfxynj40p0kxsr9jkagcvgpuqvqynnup";

        let mut payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Test Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        tst_sign_payment_request(coin_params, &mut payment_request, value, address);

        assert!(validate(coin_params, &payment_request, value, address).is_ok());

        // Unhappy cases:

        // Unnown recipient
        let payment_request = pb::BtcPaymentRequestRequest {
            recipient_name: "Unknown Merchant".into(),
            memos: vec![make_text_memo("TextMemo")],
            nonce: vec![],
            total_amount: value,
            signature: vec![],
        };
        assert!(matches!(
            validate(coin_params, &payment_request, value, address),
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
            validate(coin_params, &payment_request, value + 1, address),
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
            validate(coin_params, &payment_request, value, address),
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
            validate(coin_params, &payment_request, value, address),
            Err(ValidationError::InvalidSignature)
        ));
    }
}
