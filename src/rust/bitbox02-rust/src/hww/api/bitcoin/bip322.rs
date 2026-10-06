// SPDX-License-Identifier: Apache-2.0

//! BIP-322 generic message signing.
//!
//! Uses the `bitcoin` crate for transaction construction and the shared tagged hash engine from
//! `bip341` for the BIP-322 message hash. Can be replaced by the public `bip322` crate
//! (<https://crates.io/crates/bip322>) if it gains no_std support in the future.

use alloc::vec;
use alloc::vec::Vec;
use sha2::Digest;

use bitcoin::hashes::Hash;
use bitcoin::sighash::SighashCache;
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn, TxOut, Txid, Witness,
    absolute::LockTime, transaction::Version,
};

use super::bip341;

/// The BIP-322 tag used for the tagged message hash.
const BIP322_TAG: &[u8] = b"BIP0322-signed-message";

/// Compute the BIP-322 tagged message hash.
///
/// `SHA256(SHA256(tag) || SHA256(tag) || msg)` where tag = "BIP0322-signed-message".
pub fn tagged_hash(msg: &[u8]) -> [u8; 32] {
    let mut ctx = bip341::tagged_hash_engine(BIP322_TAG);
    ctx.update(msg);
    ctx.finalize().into()
}

/// Build the BIP-322 `to_spend` virtual transaction and return its txid.
///
/// The `to_spend` transaction commits to the message and the signer's scriptPubKey:
///   - nVersion=0, nLockTime=0
///   - vin[0]: prevout=(0x00..00, 0xFFFFFFFF), scriptSig=`OP_0 PUSH32 <tagged_hash(msg)>`,
///     nSequence=0
///   - vout[0]: nValue=0, scriptPubKey=`script_pubkey`
pub fn create_to_spend_txid(msg: &[u8], script_pubkey: &[u8]) -> [u8; 32] {
    let msg_hash = tagged_hash(msg);

    let script_sig = bitcoin::script::Builder::new()
        .push_int(0)
        .push_slice(msg_hash)
        .into_script();

    let to_spend = Transaction {
        version: Version(0),
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::new(Txid::all_zeros(), 0xFFFFFFFF),
            script_sig,
            sequence: Sequence::ZERO,
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_bytes(script_pubkey.to_vec()),
        }],
    };

    to_spend.compute_txid().to_byte_array()
}

/// Compute the BIP-341 sighash (SIGHASH_DEFAULT, key-path spend, no annex) of the BIP-322
/// `to_sign` virtual transaction with a single taproot input.
///
/// Used by BTCSignMessage, which signs for one P2TR address.
///
/// The `to_sign` transaction spends the `to_spend` output:
///   - vin[0]: prevout=(to_spend.txid(), 0), scriptSig=empty
///   - vout[0]: nValue=0, scriptPubKey=OP_RETURN
///
/// `version`, `locktime` and `sequence` come from the host request: they are 0 for the simple
/// format, but full-format signers may set them (e.g. version=2, non-zero locktime/sequence for
/// timelocks).
pub fn sighash(
    msg: &[u8],
    script_pubkey: &[u8],
    version: u32,
    locktime: u32,
    sequence: u32,
) -> [u8; 32] {
    let txid = create_to_spend_txid(msg, script_pubkey);

    let to_sign = Transaction {
        version: Version(version as i32),
        lock_time: LockTime::from_consensus(locktime),
        input: vec![TxIn {
            previous_output: OutPoint::new(Txid::from_byte_array(txid), 0),
            script_sig: ScriptBuf::new(),
            sequence: Sequence::from_consensus(sequence),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::ZERO,
            script_pubkey: bitcoin::script::Builder::new()
                .push_opcode(bitcoin::opcodes::all::OP_RETURN)
                .into_script(),
        }],
    };

    let prevout = TxOut {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::from_bytes(script_pubkey.to_vec()),
    };
    SighashCache::new(&to_sign)
        .taproot_key_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&[prevout]),
            TapSighashType::Default,
        )
        .expect("sighash computation failed")
        .to_byte_array()
}

/// The variant prefix for the "simple" BIP-322 signature format.
///
/// Per BIP-322 v1.0.0 §"Types of Signatures", a simple signature is prefixed with `smp`.
pub const SIMPLE_PREFIX: &[u8; 3] = b"smp";

/// Encode a signature as a BIP-322 "simple" signature.
///
/// Per BIP-322 v1.0.0: the witness stack
/// is consensus encoded as
/// `varint(num_items) || varint(item_len) || item_data` for each item, base64-encoded, and
/// prefixed with `smp`.
pub fn encode_simple_witness(sig: &[u8]) -> Vec<u8> {
    // Consensus-encoded witness stack: varint(1) || varint(sig_len) || sig.
    let mut witness_stack = Vec::with_capacity(2 + sig.len());
    witness_stack.push(0x01); // 1 witness stack item
    witness_stack.push(sig.len() as u8); // item length
    witness_stack.extend_from_slice(sig);

    // Base64-encode: output length is ceil(input_len / 3) * 4. We allocate one extra byte to
    // work around a bug in `binascii::b64encode` which writes past the calculated end when the
    // input length is a multiple of 3.
    let b64_len = witness_stack.len().div_ceil(3) * 4;
    let mut b64_buf = vec![0u8; b64_len + 1];
    let b64 =
        binascii::b64encode(&witness_stack, &mut b64_buf).expect("base64 output buffer too small");

    let mut out = Vec::with_capacity(SIMPLE_PREFIX.len() + b64.len());
    out.extend_from_slice(SIMPLE_PREFIX);
    out.extend_from_slice(b64);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagged_hash() {
        use sha2::Sha256;
        let hash = tagged_hash(b"");
        let tag = Sha256::digest(BIP322_TAG);
        let mut ctx = Sha256::new();
        ctx.update(tag);
        ctx.update(tag);
        let expected: [u8; 32] = ctx.finalize().into();
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_create_to_spend_txid() {
        let script_pubkey =
            hex_lit::hex!("5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274");
        let txid = create_to_spend_txid(b"", &script_pubkey);
        assert_eq!(txid.len(), 32);
        // Deterministic.
        assert_eq!(txid, create_to_spend_txid(b"", &script_pubkey));
        // Different message produces different txid.
        assert_ne!(txid, create_to_spend_txid(b"hello", &script_pubkey));
    }

    #[test]
    fn test_sighash_taproot() {
        let script_pubkey =
            hex_lit::hex!("5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274");
        let hash = sighash(b"", &script_pubkey, 0, 0, 0);
        assert_eq!(hash.len(), 32);
        // Deterministic.
        assert_eq!(hash, sighash(b"", &script_pubkey, 0, 0, 0));
        // Different message produces different sighash.
        assert_ne!(hash, sighash(b"hello", &script_pubkey, 0, 0, 0));
        // Different version/locktime/sequence produce different sighashes (full format).
        assert_ne!(hash, sighash(b"", &script_pubkey, 2, 0, 0));
        assert_ne!(hash, sighash(b"", &script_pubkey, 0, 1, 0));
        assert_ne!(hash, sighash(b"", &script_pubkey, 0, 0, 1));
    }

    #[test]
    fn test_encode_simple_witness_schnorr() {
        // 64-byte signature (Schnorr / P2TR with SIGHASH_DEFAULT).
        let sig = [0xABu8; 64];
        let encoded = encode_simple_witness(&sig);
        // Prefix "smp" (3 bytes) + base64 of (1 + 1 + 64 = 66 bytes) = 3 + 88 = 91 bytes.
        assert_eq!(encoded.len(), 91);
        assert_eq!(&encoded[..3], b"smp");
        // Witness stack: 01 40 AB...(64x). Base64 of "01 40 ABAB...AB" is deterministic.
        // 0xABAB...AB in groups of 3 bytes: 0xABABAB = base64 "q6ur" (repeated).
        // The full base64 string starts with the encoded header bytes 01 40 AB...
        // Verify it ends with base64 padding for 66 mod 3 = 0 (no padding).
        assert!(!encoded.ends_with(b"="));
    }

    #[test]
    fn test_encode_simple_witness_ecdsa() {
        // ECDSA DER signature can be 70-72 bytes.
        let sig = [0xCDu8; 71];
        let encoded = encode_simple_witness(&sig);
        // Prefix "smp" + base64 of (1 + 1 + 71 = 73 bytes) = 3 + 100 = 103 bytes.
        assert_eq!(encoded.len(), 103);
        assert_eq!(&encoded[..3], b"smp");
        // 73 mod 3 = 1, so the base64 ends with "==" padding.
        assert!(encoded.ends_with(b"=="));
    }
}
