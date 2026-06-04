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
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{
    Amount, OutPoint, Script, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn, TxOut, Txid,
    Witness, absolute::LockTime, transaction::Version,
};

use super::bip341;
use super::pb;
use super::Error;

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

/// Which sighash algorithm to use for the BIP-322 `to_sign` virtual transaction.
///
/// The choice has to match what a BIP-322 verifier runs through the standard script
/// interpreter: BIP-341 for taproot inputs and BIP-143 for v0 segwit (P2WPKH, P2WPKH-P2SH,
/// P2WSH, P2WSH-P2SH).
pub enum SighashMode<'a> {
    /// BIP-341 taproot key-path spend (SIGHASH_DEFAULT, no annex, no tapleaf).
    Taproot,
    /// BIP-143 v0 segwit (P2WPKH/P2WSH and their P2SH-wrapped forms). `script_code` is the
    /// script that goes into BIP-143 step 5 (the P2PKH-form `0x76a914<hash>88ac` for P2WPKH,
    /// or the witness script for P2WSH).
    SegwitV0 { script_code: &'a [u8] },
}

/// Compute the sighash for the BIP-322 `to_sign` virtual transaction.
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
    mode: SighashMode<'_>,
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

    let mut cache = SighashCache::new(&to_sign);
    match mode {
        SighashMode::Taproot => {
            let prevout = TxOut {
                value: Amount::ZERO,
                script_pubkey: ScriptBuf::from_bytes(script_pubkey.to_vec()),
            };
            cache
                .taproot_key_spend_signature_hash(
                    0,
                    &bitcoin::sighash::Prevouts::All(&[prevout]),
                    TapSighashType::Default,
                )
                .expect("sighash computation failed")
                .to_byte_array()
        }
        SighashMode::SegwitV0 { script_code } => {
            // `p2wsh_signature_hash` is a thin wrapper over `segwit_v0_encode_signing_data_to`
            // that uses its `witness_script` argument as the BIP-143 script_code unchanged, so
            // it works as a generic v0 segwit sighash for both P2WPKH (with P2PKH-form script
            // code) and P2WSH (with the witness script).
            cache
                .p2wsh_signature_hash(
                    0,
                    Script::from_bytes(script_code),
                    Amount::ZERO,
                    EcdsaSighashType::All,
                )
                .expect("sighash computation failed")
                .to_byte_array()
        }
    }
}

/// Validate the BIP-322 init request against the spec-level rules for `to_sign`.
///
/// Per BIP-322 v1.0.0:
///   - version must be 0 or 2 (upgradeable rule §4)
///   - exactly one output (the OP_RETURN)
///   - locktime is unrestricted (full format may set it for timelocks)
///
/// `num_inputs` is currently restricted to 1: the streaming flow only signs the first input,
/// so accepting more would silently drop signatures for the additional inputs. This restriction
/// can be lifted once Proof-of-Funds (multi-input) signing is implemented.
pub fn validate_init(request: &pb::BtcSignInitRequest) -> Result<(), Error> {
    if request.version != 0 && request.version != 2 {
        return Err(Error::InvalidInput);
    }
    if request.num_outputs != 1 {
        return Err(Error::InvalidInput);
    }
    // TODO: relax to `>= 1` when multi-input (Proof-of-Funds) signing is implemented.
    if request.num_inputs != 1 {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

/// Validate the BIP-322 first input.
///
/// Checks: prevOutIndex=0, prevOutValue=0, and prevOutHash matches the computed
/// to_spend txid from the message and scriptPubKey. `sequence` is unrestricted (full format
/// may set it for timelocks).
pub fn validate_input(
    input: &pb::BtcSignInputRequest,
    message: &[u8],
    script_pubkey: &[u8],
) -> Result<(), Error> {
    if input.prev_out_index != 0 {
        return Err(Error::InvalidInput);
    }
    if input.prev_out_value != 0 {
        return Err(Error::InvalidInput);
    }
    let expected_txid = create_to_spend_txid(message, script_pubkey);
    if input.prev_out_hash.as_slice() != expected_txid {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

/// Validate the BIP-322 output.
///
/// Checks: value=0, type=OP_RETURN.
pub fn validate_output(output: &pb::BtcSignOutputRequest) -> Result<(), Error> {
    if output.value != 0 {
        return Err(Error::InvalidInput);
    }
    if pb::BtcOutputType::try_from(output.r#type)? != pb::BtcOutputType::OpReturn {
        return Err(Error::InvalidInput);
    }
    Ok(())
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
    let b64 = binascii::b64encode(&witness_stack, &mut b64_buf)
        .expect("base64 output buffer too small");

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
        ctx.update(&tag);
        ctx.update(&tag);
        let expected: [u8; 32] = ctx.finalize().into();
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_create_to_spend_txid() {
        let script_pubkey = hex_lit::hex!(
            "5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274"
        );
        let txid = create_to_spend_txid(b"", &script_pubkey);
        assert_eq!(txid.len(), 32);
        // Deterministic.
        assert_eq!(txid, create_to_spend_txid(b"", &script_pubkey));
        // Different message produces different txid.
        assert_ne!(txid, create_to_spend_txid(b"hello", &script_pubkey));
    }

    #[test]
    fn test_sighash_taproot() {
        let script_pubkey = hex_lit::hex!(
            "5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274"
        );
        let hash = sighash(b"", &script_pubkey, 0, 0, 0, SighashMode::Taproot);
        assert_eq!(hash.len(), 32);
        // Deterministic.
        assert_eq!(hash, sighash(b"", &script_pubkey, 0, 0, 0, SighashMode::Taproot));
        // Different message produces different sighash.
        assert_ne!(hash, sighash(b"hello", &script_pubkey, 0, 0, 0, SighashMode::Taproot));
        // Different version/locktime/sequence produce different sighashes (full format).
        assert_ne!(hash, sighash(b"", &script_pubkey, 2, 0, 0, SighashMode::Taproot));
        assert_ne!(hash, sighash(b"", &script_pubkey, 0, 1, 0, SighashMode::Taproot));
        assert_ne!(hash, sighash(b"", &script_pubkey, 0, 0, 1, SighashMode::Taproot));
    }

    #[test]
    fn test_sighash_segwit_v0() {
        // P2SH-P2WPKH challenge: scriptPubKey is OP_HASH160 PUSH20 <hash160(redeem)> OP_EQUAL.
        let script_pubkey =
            hex_lit::hex!("a91464096dffec1b1b52addf3020a9f01be8b812c3f987");
        // The BIP-143 script_code for a P2WPKH(-P2SH) input is the P2PKH-form script:
        // 0x76a914 <hash160(pubkey)> 88ac.
        let script_code =
            hex_lit::hex!("76a91464096dffec1b1b52addf3020a9f01be8b812c3f988ac");
        let mode = SighashMode::SegwitV0 {
            script_code: &script_code,
        };
        let hash = sighash(b"", &script_pubkey, 0, 0, 0, mode);
        assert_eq!(hash.len(), 32);
        // Deterministic.
        let hash2 = sighash(
            b"",
            &script_pubkey,
            0,
            0,
            0,
            SighashMode::SegwitV0 {
                script_code: &script_code,
            },
        );
        assert_eq!(hash, hash2);
        // Different message produces different sighash.
        let hash3 = sighash(
            b"hello",
            &script_pubkey,
            0,
            0,
            0,
            SighashMode::SegwitV0 {
                script_code: &script_code,
            },
        );
        assert_ne!(hash, hash3);
        // Taproot mode produces a different sighash for the same inputs.
        let hash_tr = sighash(b"", &script_pubkey, 0, 0, 0, SighashMode::Taproot);
        assert_ne!(hash, hash_tr);
    }

    #[test]
    fn test_validate_init_ok() {
        // Simple format defaults.
        let request = pb::BtcSignInitRequest {
            version: 0,
            locktime: 0,
            num_inputs: 1,
            num_outputs: 1,
            ..Default::default()
        };
        assert!(validate_init(&request).is_ok());

        // Full format: version=2, non-zero locktime is allowed.
        let request = pb::BtcSignInitRequest {
            version: 2,
            locktime: 100,
            num_inputs: 1,
            num_outputs: 1,
            ..Default::default()
        };
        assert!(validate_init(&request).is_ok());
    }

    #[test]
    fn test_validate_init_bad_version() {
        // Per spec, only version 0 or 2 is allowed.
        let request = pb::BtcSignInitRequest {
            version: 1,
            locktime: 0,
            num_inputs: 1,
            num_outputs: 1,
            ..Default::default()
        };
        assert!(validate_init(&request).is_err());

        let request = pb::BtcSignInitRequest {
            version: 3,
            locktime: 0,
            num_inputs: 1,
            num_outputs: 1,
            ..Default::default()
        };
        assert!(validate_init(&request).is_err());
    }

    #[test]
    fn test_validate_init_bad_num_inputs() {
        let request = pb::BtcSignInitRequest {
            version: 0,
            locktime: 0,
            num_inputs: 2,
            num_outputs: 1,
            ..Default::default()
        };
        assert!(validate_init(&request).is_err());
    }

    #[test]
    fn test_validate_input_ok() {
        let script_pubkey = hex_lit::hex!(
            "5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274"
        );
        let txid = create_to_spend_txid(b"hello", &script_pubkey);
        let input = pb::BtcSignInputRequest {
            prev_out_hash: txid.to_vec(),
            prev_out_index: 0,
            prev_out_value: 0,
            sequence: 0,
            ..Default::default()
        };
        assert!(validate_input(&input, b"hello", &script_pubkey).is_ok());
    }

    #[test]
    fn test_validate_input_bad_txid() {
        let script_pubkey = hex_lit::hex!(
            "5120a60869f0dbcf1dc659c9cecbee8b89cea43c4a2906acdb10a681b4bbaef14274"
        );
        let input = pb::BtcSignInputRequest {
            prev_out_hash: vec![0u8; 32], // wrong txid
            prev_out_index: 0,
            prev_out_value: 0,
            sequence: 0,
            ..Default::default()
        };
        assert!(validate_input(&input, b"hello", &script_pubkey).is_err());
    }

    #[test]
    fn test_validate_output_ok() {
        let output = pb::BtcSignOutputRequest {
            value: 0,
            r#type: pb::BtcOutputType::OpReturn as _,
            ..Default::default()
        };
        assert!(validate_output(&output).is_ok());
    }

    #[test]
    fn test_validate_output_bad_value() {
        let output = pb::BtcSignOutputRequest {
            value: 100,
            r#type: pb::BtcOutputType::OpReturn as _,
            ..Default::default()
        };
        assert!(validate_output(&output).is_err());
    }

    #[test]
    fn test_validate_output_bad_type() {
        let output = pb::BtcSignOutputRequest {
            value: 0,
            r#type: pb::BtcOutputType::P2tr as _,
            ..Default::default()
        };
        assert!(validate_output(&output).is_err());
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
