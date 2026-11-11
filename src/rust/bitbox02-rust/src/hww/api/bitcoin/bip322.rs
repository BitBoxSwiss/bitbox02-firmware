// SPDX-License-Identifier: Apache-2.0

//! BIP-322 generic signed message format.
//!
//! https://github.com/bitcoin/bips/blob/master/bip-0322.mediawiki

use alloc::vec::Vec;

use bitcoin::consensus::encode::{VarInt, serialize};
use sha2::{Digest, Sha256};

use super::bip341;

/// Computes `message_hash`, the BIP-340 tagged hash of the message with the tag
/// `BIP0322-signed-message`.
fn message_hash(msg: &[u8]) -> [u8; 32] {
    let mut ctx = bip341::tagged_hash_engine(b"BIP0322-signed-message");
    ctx.update(msg);
    ctx.finalize().into()
}

/// Computes the txid of the `to_spend` virtual transaction, which commits to the message and to
/// `script_pubkey`, the scriptPubKey of the address the message is signed with (the
/// `message_challenge`):
///
/// ```text
/// nVersion = 0
/// nLockTime = 0
/// vin[0].prevout.hash = 0000...000
/// vin[0].prevout.n = 0xFFFFFFFF
/// vin[0].nSequence = 0
/// vin[0].scriptSig = OP_0 PUSH32[ message_hash ]
/// vin[0].scriptWitness = []
/// vout[0].nValue = 0
/// vout[0].scriptPubKey = message_challenge
/// ```
///
/// The txid is returned in the byte order in which it is serialized in a transaction input.
pub fn to_spend_txid(msg: &[u8], script_pubkey: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    // nVersion
    hasher.update(0u32.to_le_bytes());
    // Number of inputs
    hasher.update(serialize(&VarInt(1)));
    // vin[0].prevout.hash
    hasher.update([0u8; 32]);
    // vin[0].prevout.n
    hasher.update(0xffffffffu32.to_le_bytes());
    // vin[0].scriptSig, prefixed with its length: OP_0 PUSH32[ message_hash ]
    hasher.update(serialize(&VarInt(34)));
    hasher.update([
        bitcoin::opcodes::OP_0.to_u8(),
        bitcoin::opcodes::all::OP_PUSHBYTES_32.to_u8(),
    ]);
    hasher.update(message_hash(msg));
    // vin[0].nSequence
    hasher.update(0u32.to_le_bytes());
    // Number of outputs
    hasher.update(serialize(&VarInt(1)));
    // vout[0].nValue
    hasher.update(0u64.to_le_bytes());
    // vout[0].scriptPubKey
    hasher.update(serialize(&VarInt(script_pubkey.len() as u64)));
    hasher.update(script_pubkey);
    // nLockTime
    hasher.update(0u32.to_le_bytes());
    // Hash again to produce the final double-hash.
    Sha256::digest(hasher.finalize()).into()
}

/// Computes the SHA256 of the serialized outputs of the `to_sign` virtual transaction. It has a
/// single output of value 0 with the scriptPubKey `OP_RETURN`.
pub fn to_sign_hash_outputs() -> [u8; 32] {
    let mut hasher = Sha256::new();
    // vout[0].nValue
    hasher.update(0u64.to_le_bytes());
    // vout[0].scriptPubKey
    hasher.update(serialize(&VarInt(1)));
    hasher.update([bitcoin::opcodes::all::OP_RETURN.to_u8()]);
    hasher.finalize().into()
}

/// Computes the BIP-341 sighash (`SIGHASH_DEFAULT`, key path spend) of the `to_sign` virtual
/// transaction of a "simple" signature for the P2TR scriptPubKey `script_pubkey`:
///
/// ```text
/// nVersion = 0
/// nLockTime = 0
/// vin[0].prevout.hash = to_spend.txid
/// vin[0].prevout.n = 0
/// vin[0].nSequence = 0
/// vin[0].scriptSig = []
/// vin[0].scriptWitness = message_signature
/// vout[0].nValue = 0
/// vout[0].scriptPubKey = OP_RETURN
/// ```
pub fn sighash_p2tr(msg: &[u8], script_pubkey: &[u8]) -> [u8; 32] {
    let mut hasher_prevouts = Sha256::new();
    hasher_prevouts.update(to_spend_txid(msg, script_pubkey));
    hasher_prevouts.update(0u32.to_le_bytes());

    let mut hasher_scriptpubkeys = Sha256::new();
    hasher_scriptpubkeys.update(serialize(&VarInt(script_pubkey.len() as u64)));
    hasher_scriptpubkeys.update(script_pubkey);

    bip341::sighash(&bip341::Args {
        version: 0,
        locktime: 0,
        hash_prevouts: hasher_prevouts.finalize().into(),
        hash_amounts: Sha256::digest(0u64.to_le_bytes()).into(),
        hash_scriptpubkeys: hasher_scriptpubkeys.finalize().into(),
        hash_sequences: Sha256::digest(0u32.to_le_bytes()).into(),
        hash_outputs: to_sign_hash_outputs(),
        input_index: 0,
        tapleaf_hash: None,
    })
}

/// Encodes a "simple" signature whose witness stack consists of a single Schnorr signature, i.e.
/// the signature of a taproot key path spend: the variant prefix `smp`, followed by the base64
/// encoding of the consensus-encoded witness stack.
pub fn encode_simple_p2tr(signature: &[u8; 64]) -> Vec<u8> {
    const PREFIX: &[u8] = b"smp";

    // Number of witness stack items, followed by the length-prefixed item.
    let mut witness_stack = serialize(&VarInt(1));
    witness_stack.extend(serialize(&VarInt(signature.len() as u64)));
    witness_stack.extend_from_slice(signature);

    let mut result = PREFIX.to_vec();
    // One byte more than the padded base64 length: `b64encode` panics if the output buffer has
    // no room after the unpadded data.
    result.resize(PREFIX.len() + witness_stack.len().div_ceil(3) * 4 + 1, 0);
    let encoded_len = binascii::b64encode(&witness_stack, &mut result[PREFIX.len()..])
        .unwrap()
        .len();
    result.truncate(PREFIX.len() + encoded_len);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitcoin::hashes::Hash;
    use hex_lit::hex;

    // Test vectors from
    // https://github.com/bitcoin/bips/blob/master/bip-0322/basic-test-vectors.json

    /// scriptPubKey of bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l.
    const SCRIPT_PUBKEY_P2WPKH: [u8; 22] = hex!("00142b05d564e6a7a33c087f16e0f730d1440123799d");
    /// scriptPubKey of bc1pss0zhytly75awhm6x2hhvd5lnzv3vssgrf9axfheq8ldyzn88ges79fler.
    const SCRIPT_PUBKEY_P2TR: [u8; 34] =
        hex!("5120841e2b917f27a9d75f7a32af76369f98991642081a4bd326f901fed20a673a33");
    const MESSAGE_UTF8: &str = "UTF-8 support: öäüéàè 测试文本 😄";

    /// Builds the `to_sign` transaction of a "simple" signature using the `bitcoin` crate.
    fn to_sign(msg: &[u8], script_pubkey: &[u8]) -> bitcoin::Transaction {
        bitcoin::Transaction {
            version: bitcoin::transaction::Version(0),
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![bitcoin::TxIn {
                previous_output: bitcoin::OutPoint::new(
                    bitcoin::Txid::from_byte_array(to_spend_txid(msg, script_pubkey)),
                    0,
                ),
                sequence: bitcoin::Sequence::ZERO,
                ..Default::default()
            }],
            output: vec![bitcoin::TxOut {
                value: bitcoin::Amount::ZERO,
                script_pubkey: bitcoin::ScriptBuf::from_bytes(vec![
                    bitcoin::opcodes::all::OP_RETURN.to_u8(),
                ]),
            }],
        }
    }

    #[test]
    fn test_message_hash() {
        assert_eq!(
            message_hash(b""),
            hex!("c90c269c4f8fcbe6880f72a721ddfbf1914268a794cbb21cfafee13770ae19f1"),
        );
        assert_eq!(
            message_hash(b"Hello World"),
            hex!("f0eb03b1a75ac6d9847f55c624a99169b5dccba2a31f5b23bea77ba270de0a7a"),
        );
        assert_eq!(
            message_hash(MESSAGE_UTF8.as_bytes()),
            hex!("43936b237ea38c7794eb5d755e0d220b6db92ebfc5c8f482759d22b1286376d7"),
        );
    }

    #[test]
    fn test_to_spend_txid() {
        // The test vectors contain the txid in the reverse byte order, in which it is displayed.
        let txid = |msg: &[u8]| {
            let mut txid = to_spend_txid(msg, &SCRIPT_PUBKEY_P2WPKH);
            txid.reverse();
            txid
        };
        assert_eq!(
            txid(b""),
            hex!("c5680aa69bb8d860bf82d4e9cd3504b55dde018de765a91bb566283c545a99a7"),
        );
        assert_eq!(
            txid(b"Hello World"),
            hex!("b79d196740ad5217771c1098fc4a4b51e0535c32236c71f1ea4d61a2d603352b"),
        );
        assert_eq!(
            txid(MESSAGE_UTF8.as_bytes()),
            hex!("c8f4f525fe8afb1bc09b44175bd2096f079c98425e8a1be676b712add1fb62f0"),
        );
    }

    #[test]
    fn test_to_sign_hash_outputs() {
        let to_sign = to_sign(b"Hello World", &SCRIPT_PUBKEY_P2WPKH);
        // The `to_sign` transaction of the test vector, so its outputs are the expected ones.
        let mut txid = to_sign.compute_txid().to_byte_array();
        txid.reverse();
        assert_eq!(
            txid,
            hex!("88737ae86f2077145f93cc4b153ae9a1cb8d56afa511988c149c5c8c9d93bddf"),
        );
        let expected: [u8; 32] = Sha256::digest(serialize(&to_sign.output[0])).into();
        assert_eq!(to_sign_hash_outputs(), expected);
    }

    #[test]
    fn test_sighash_p2tr() {
        let msg = b"No prefix fallback";
        let sighash = sighash_p2tr(msg, &SCRIPT_PUBKEY_P2TR);

        // Same sighash as computed by the `bitcoin` crate.
        let to_sign = to_sign(msg, &SCRIPT_PUBKEY_P2TR);
        let expected = bitcoin::sighash::SighashCache::new(&to_sign)
            .taproot_key_spend_signature_hash(
                0,
                &bitcoin::sighash::Prevouts::All(&[bitcoin::TxOut {
                    value: bitcoin::Amount::ZERO,
                    script_pubkey: bitcoin::ScriptBuf::from_bytes(SCRIPT_PUBKEY_P2TR.to_vec()),
                }]),
                bitcoin::TapSighashType::Default,
            )
            .unwrap();
        assert_eq!(sighash, expected.to_byte_array());

        // The signature of the test vector, which is the only item of the witness stack, is a
        // valid signature of the sighash for the output key.
        let signature = bitcoin::secp256k1::schnorr::Signature::from_slice(&hex!(
            "8960ec0e8f1600bdab5300661a565357055c85b9ce0cc0d4243ae2b6b65f3185f28338d7743637d8c8d498711a9e0a9f21b009c210f1c054490ef61eaf6d0fd1"
        ))
        .unwrap();
        let output_key =
            bitcoin::secp256k1::XOnlyPublicKey::from_slice(&SCRIPT_PUBKEY_P2TR[2..]).unwrap();
        bitcoin::secp256k1::Secp256k1::verification_only()
            .verify_schnorr(
                &signature,
                &bitcoin::secp256k1::Message::from_digest(sighash),
                &output_key,
            )
            .unwrap();
    }

    #[test]
    fn test_encode_simple_p2tr() {
        // Signature of the "No prefix fallback" test vector, which lacks the prefix.
        assert_eq!(
            encode_simple_p2tr(&hex!(
                "8960ec0e8f1600bdab5300661a565357055c85b9ce0cc0d4243ae2b6b65f3185f28338d7743637d8c8d498711a9e0a9f21b009c210f1c054490ef61eaf6d0fd1"
            )),
            b"smpAUCJYOwOjxYAvatTAGYaVlNXBVyFuc4MwNQkOuK2tl8xhfKDONd0NjfYyNSYcRqeCp8hsAnCEPHAVEkO9h6vbQ/R",
        );
    }
}
