// SPDX-License-Identifier: Apache-2.0

//! PSBT scenarios covering output metadata and compatibility behavior.

use super::common::{
    ecdsa_signature, keypath, secp, simulator_xprv, simulator_xpub_at, taproot_key_signature,
    transaction_vector,
};
use super::screens;
use crate::btc_transaction::{
    Coin, PaymentRequest, PaymentRequestMemo, PsbtOutputOptions, PsbtSignOptions, TestVector,
};
use bitcoin::bip32::DerivationPath;
use bitcoin::consensus::encode::VarInt;
use bitcoin::hashes::{Hash, HashEngine, sha256};
use bitcoin::psbt::Psbt;
use bitcoin::secp256k1::{Message, SecretKey};
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness, transaction,
};
use sha3::Digest;
use std::collections::BTreeMap;

const SILENT_PAYMENT_ADDRESS: &str = "sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv";

pub fn all() -> Vec<TestVector> {
    vec![
        taproot_spends_to_non_taproot(),
        silent_payment(),
        send_self(false),
        send_self(true),
        payment_request(),
        payment_request_rejects_owned_output(),
    ]
}

fn path(value: &str) -> DerivationPath {
    value.parse().unwrap()
}

fn dummy_input() -> TxIn {
    TxIn {
        previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
            .parse()
            .unwrap(),
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::default(),
    }
}

fn unsigned_input(prev_tx: &Transaction, vout: u32) -> TxIn {
    TxIn {
        previous_output: OutPoint {
            txid: prev_tx.compute_txid(),
            vout,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::default(),
    }
}

// All inputs are Taproot, but the change output is not Taproot. Some firmware versions
// conservatively request the previous transactions in this case.
fn taproot_spends_to_non_taproot() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let input0_path = path("m/86'/1'/0'/0/0");
    let input1_path = path("m/86'/1'/0'/0/1");
    let change_path = path("m/84'/1'/0'/1/0");
    let input0_xpub = simulator_xpub_at(&secp, &input0_path);
    let input1_xpub = simulator_xpub_at(&secp, &input1_path);
    let change_xpub = simulator_xpub_at(&secp, &change_path);

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![dummy_input()],
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, input0_xpub.to_x_only_pub(), None),
            },
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, input1_xpub.to_x_only_pub(), None),
            },
        ],
    };
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![unsigned_input(&prev_tx, 0), unsigned_input(&prev_tx, 1)],
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&change_xpub.to_pub().wpubkey_hash()),
            },
            TxOut {
                value: Amount::from_sat(20_000_000),
                script_pubkey: ScriptBuf::new_p2tr(
                    &secp,
                    // random private key:
                    // 9dbb534622a6100a39b73dece43c6d4db14b9a612eb46a6c64c2bb849e283ce8
                    "e4adbb12c3426ec71ebb10688d8ae69d531ca822a2b790acee216a7f1b95b576"
                        .parse()
                        .unwrap(),
                    None,
                ),
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    for (index, (xpub, input_path)) in [(input0_xpub, input0_path), (input1_xpub, input1_path)]
        .into_iter()
        .enumerate()
    {
        psbt.inputs[index].witness_utxo = Some(prev_tx.output[index].clone());
        psbt.inputs[index].non_witness_utxo = Some(prev_tx.clone());
        psbt.inputs[index].tap_internal_key = Some(xpub.to_x_only_pub());
        psbt.inputs[index]
            .tap_key_origins
            .insert(xpub.to_x_only_pub(), (vec![], (fingerprint, input_path)));
    }
    psbt.outputs[0]
        .bip32_derivation
        .insert(change_xpub.to_pub().0, (fingerprint, change_path));

    transaction_vector(
        "taproot-to-non-taproot-change",
        "Signs all-Taproot inputs with P2WPKH change, requiring previous transactions for the added non-Taproot output config.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions::default(),
        vec![
            taproot_key_signature(0, input0_xpub.to_x_only_pub()),
            taproot_key_signature(1, input1_xpub.to_x_only_pub()),
        ],
        screens::taproot_to_non_taproot_change(),
    )
}

// Test that a mixed-input PSBT can ask the device to generate a BIP352 output.
fn silent_payment() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let change_path = path("m/86'/0'/0'/1/0");
    let input0_path = path("m/86'/0'/0'/0/0"); // P2TR
    let input1_path = path("m/84'/0'/0'/0/0"); // P2WPKH
    let input2_path = path("m/49'/0'/0'/0/0"); // P2SH-P2WPKH
    let change_xpub = simulator_xpub_at(&secp, &change_path);
    let input0_xpub = simulator_xpub_at(&secp, &input0_path);
    let input1_xpub = simulator_xpub_at(&secp, &input1_path);
    let input2_xpub = simulator_xpub_at(&secp, &input2_path);
    let input2_redeem = ScriptBuf::new_p2wpkh(&input2_xpub.to_pub().wpubkey_hash());

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![dummy_input()],
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, input0_xpub.to_x_only_pub(), None),
            },
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&input1_xpub.to_pub().wpubkey_hash()),
            },
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2sh(&input2_redeem.script_hash()),
            },
        ],
    };
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: (0..3).map(|vout| unsigned_input(&prev_tx, vout)).collect(),
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, change_xpub.to_x_only_pub(), None),
            },
            TxOut {
                value: Amount::from_sat(20_000_000),
                // The device fills this script from the silent-payment address.
                script_pubkey: ScriptBuf::new(),
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].tap_internal_key = Some(input0_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input0_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input0_path)),
    );
    psbt.inputs[1].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[1].witness_utxo = Some(prev_tx.output[1].clone());
    psbt.inputs[1]
        .bip32_derivation
        .insert(input1_xpub.to_pub().0, (fingerprint, input1_path));
    psbt.inputs[2].non_witness_utxo = Some(prev_tx);
    psbt.inputs[2].witness_utxo = psbt.inputs[2]
        .non_witness_utxo
        .as_ref()
        .map(|tx| tx.output[2].clone());
    psbt.inputs[2].redeem_script = Some(input2_redeem);
    psbt.inputs[2]
        .bip32_derivation
        .insert(input2_xpub.to_pub().0, (fingerprint, input2_path));
    psbt.outputs[0].tap_internal_key = Some(change_xpub.to_x_only_pub());
    psbt.outputs[0].tap_key_origins.insert(
        change_xpub.to_x_only_pub(),
        (vec![], (fingerprint, change_path)),
    );

    let mut outputs = BTreeMap::new();
    outputs.insert(
        1,
        PsbtOutputOptions {
            silent_payment_address: Some(SILENT_PAYMENT_ADDRESS.into()),
            payment_request_index: None,
        },
    );
    let mut result = transaction_vector(
        "silent-payment",
        "Signs mixed inputs with BIP352 metadata and a device-generated silent-payment output.",
        Coin::Btc,
        psbt,
        PsbtSignOptions {
            outputs,
            ..Default::default()
        },
        vec![
            taproot_key_signature(0, input0_xpub.to_x_only_pub()),
            ecdsa_signature(1, input1_xpub.to_pub().0),
            ecdsa_signature(2, input2_xpub.to_pub().0),
        ],
        screens::silent_payment(),
    );
    result.expected_generated_outputs.insert(
        1,
        "5120d826829cb603fc008e5ef99d0818f2126d3569c3ab8a6cd069f07a20e892bd59".into(),
    );
    result
}

// Tests that the output is recognized as the same account or another account on this device.
fn send_self(different_account: bool) -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let input_path = path("m/86'/1'/0'/0/0");
    let change_path = path("m/49'/1'/0'/1/0");
    let send_self_path = if different_account {
        path("m/84'/1'/1'/0/0")
    } else {
        path("m/84'/1'/0'/0/0")
    };
    let input_xpub = simulator_xpub_at(&secp, &input_path);
    let change_xpub = simulator_xpub_at(&secp, &change_path);
    let send_self_xpub = simulator_xpub_at(&secp, &send_self_path);
    let change_redeem = ScriptBuf::new_p2wpkh(&change_xpub.to_pub().wpubkey_hash());

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![dummy_input()],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: ScriptBuf::new_p2tr(&secp, input_xpub.to_x_only_pub(), None),
        }],
    };
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![unsigned_input(&prev_tx, 0)],
        output: vec![
            TxOut {
                value: Amount::from_sat(50_000_000),
                script_pubkey: ScriptBuf::new_p2sh(&change_redeem.script_hash()),
            },
            TxOut {
                value: Amount::from_sat(20_000_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&send_self_xpub.to_pub().wpubkey_hash()),
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].tap_internal_key = Some(input_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input_path)),
    );
    psbt.outputs[0].redeem_script = Some(change_redeem);
    psbt.outputs[0]
        .bip32_derivation
        .insert(change_xpub.to_pub().0, (fingerprint, change_path));
    psbt.outputs[1]
        .bip32_derivation
        .insert(send_self_xpub.to_pub().0, (fingerprint, send_self_path));

    transaction_vector(
        if different_account {
            "send-self-different-account"
        } else {
            "send-self-same-account"
        },
        if different_account {
            "Covers ownership detection for another account and version-specific account labels."
        } else {
            "Covers ownership detection for the input account, suppression of change confirmation and version-specific account labels."
        },
        Coin::Tbtc,
        psbt,
        PsbtSignOptions::default(),
        vec![taproot_key_signature(0, input_xpub.to_x_only_pub())],
        if different_account {
            screens::send_self_different_account()
        } else {
            screens::send_self_same_account()
        },
    )
}

fn hash_len_prefixed(engine: &mut sha256::HashEngine, value: &[u8]) {
    engine.input(&bitcoin::consensus::serialize(&VarInt(value.len() as u64)));
    engine.input(value);
}

pub(super) fn payment_request_signature(value: u64, address: &str) -> Vec<u8> {
    let mut sighash = sha256::Hash::engine();
    sighash.input(b"SL\x00\x24");
    hash_len_prefixed(&mut sighash, b"");
    hash_len_prefixed(&mut sighash, b"Test Merchant");
    sighash.input(&bitcoin::consensus::serialize(&VarInt(1)));
    sighash.input(&1u32.to_le_bytes());
    hash_len_prefixed(&mut sighash, b"TextMemo line1\nTextMemo line2");
    sighash.input(&1u32.to_le_bytes());

    let mut output_hash = sha256::Hash::engine();
    output_hash.input(&value.to_le_bytes());
    hash_len_prefixed(&mut output_hash, address.as_bytes());
    sighash.input(sha256::Hash::from_engine(output_hash).as_byte_array());

    let digest = sha256::Hash::from_engine(sighash).to_byte_array();
    let secret_key = SecretKey::from_slice(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    secp()
        .sign_ecdsa(&Message::from_digest(digest), &secret_key)
        .serialize_compact()
        .to_vec()
}

fn checksummed_eth_address(pubkey: &bitcoin::secp256k1::PublicKey) -> String {
    let hash = sha3::Keccak256::digest(&pubkey.serialize_uncompressed()[1..]);
    let mut address = hex::encode(&hash[hash.len() - 20..]).into_bytes();
    let checksum = sha3::Keccak256::digest(&address);
    for (index, byte) in address.iter_mut().enumerate() {
        let nibble = if index % 2 == 0 {
            checksum[index / 2] >> 4
        } else {
            checksum[index / 2] & 0x0f
        };
        if *byte > b'9' && nibble > 7 {
            *byte -= 32;
        }
    }
    format!("0x{}", String::from_utf8(address).unwrap())
}

pub(super) fn coin_purchase_payment_request(
    source_coin_type: u32,
    value: u64,
    source_address: &str,
) -> PaymentRequest {
    let address_keypath = path("m/44'/60'/0'/0/0");
    let private_key = simulator_xprv()
        .derive_priv(&secp(), &address_keypath)
        .unwrap()
        .private_key;
    let destination_address = checksummed_eth_address(
        &bitcoin::secp256k1::PublicKey::from_secret_key(&secp(), &private_key),
    );
    let destination_amount = "0.25 ETH";

    let mut sighash = sha256::Hash::engine();
    sighash.input(b"SL\x00\x24");
    hash_len_prefixed(&mut sighash, b"");
    hash_len_prefixed(&mut sighash, b"Test Merchant");
    sighash.input(&bitcoin::consensus::serialize(&VarInt(1)));
    sighash.input(&3u32.to_le_bytes());
    sighash.input(&60u32.to_le_bytes());
    hash_len_prefixed(&mut sighash, destination_amount.as_bytes());
    hash_len_prefixed(&mut sighash, destination_address.as_bytes());
    sighash.input(&source_coin_type.to_le_bytes());

    let mut output_hash = sha256::Hash::engine();
    output_hash.input(&value.to_le_bytes());
    hash_len_prefixed(&mut output_hash, source_address.as_bytes());
    sighash.input(sha256::Hash::from_engine(output_hash).as_byte_array());

    let digest = sha256::Hash::from_engine(sighash).to_byte_array();
    let secret_key = SecretKey::from_slice(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    let signature = secp()
        .sign_ecdsa(&Message::from_digest(digest), &secret_key)
        .serialize_compact();
    PaymentRequest {
        recipient_name: "Test Merchant".into(),
        total_amount: value,
        nonce: String::new(),
        memos: vec![PaymentRequestMemo::CoinPurchase {
            coin_type: 60,
            amount: destination_amount.into(),
            address: destination_address,
            address_keypath: keypath(&address_keypath),
        }],
        signature: hex::encode(signature),
    }
}

fn payment_request_options(output_index: usize, value: u64, address: &str) -> PsbtSignOptions {
    PsbtSignOptions {
        outputs: BTreeMap::from([(
            output_index,
            PsbtOutputOptions {
                silent_payment_address: None,
                payment_request_index: Some(0),
            },
        )]),
        payment_requests: vec![PaymentRequest {
            recipient_name: "Test Merchant".into(),
            total_amount: value,
            nonce: String::new(),
            memos: vec![PaymentRequestMemo::Text {
                note: "TextMemo line1\nTextMemo line2".into(),
            }],
            signature: hex::encode(payment_request_signature(value, address)),
        }],
        ..Default::default()
    }
}

fn payment_request() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let input_path = path("m/86'/1'/0'/0/0");
    let change_path = path("m/49'/1'/0'/1/0");
    let input_xpub = simulator_xpub_at(&secp, &input_path);
    let change_xpub = simulator_xpub_at(&secp, &change_path);
    let change_redeem = ScriptBuf::new_p2wpkh(&change_xpub.to_pub().wpubkey_hash());
    let address = "tb1q9kvhpyd32aqhpsc8yrdm48gx5dnadq63lservm";
    let recipient_script = address
        .parse::<bitcoin::Address<_>>()
        .unwrap()
        .assume_checked()
        .script_pubkey();
    let value = 20_000_000;

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![dummy_input()],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: ScriptBuf::new_p2tr(&secp, input_xpub.to_x_only_pub(), None),
        }],
    };
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![unsigned_input(&prev_tx, 0)],
        output: vec![
            TxOut {
                value: Amount::from_sat(50_000_000),
                script_pubkey: ScriptBuf::new_p2sh(&change_redeem.script_hash()),
            },
            TxOut {
                value: Amount::from_sat(value),
                script_pubkey: recipient_script,
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].tap_internal_key = Some(input_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input_path)),
    );
    psbt.outputs[0].redeem_script = Some(change_redeem);
    psbt.outputs[0]
        .bip32_derivation
        .insert(change_xpub.to_pub().0, (fingerprint, change_path));

    transaction_vector(
        "payment-request",
        "Covers signed SLIP-24 payment-request metadata, merchant and multiline memo screens, address suppression and the pre-v9.24 simulator merchant limitation.",
        Coin::Tbtc,
        psbt,
        payment_request_options(1, value, address),
        vec![taproot_key_signature(0, input_xpub.to_x_only_pub())],
        screens::payment_request(),
    )
}

fn payment_request_rejects_owned_output() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let input_path = path("m/86'/1'/0'/0/0");
    let change_path = path("m/86'/1'/0'/1/0");
    let receive_path = path("m/84'/1'/0'/0/0");
    let input_xpub = simulator_xpub_at(&secp, &input_path);
    let change_xpub = simulator_xpub_at(&secp, &change_path);
    let receive_xpub = simulator_xpub_at(&secp, &receive_path);
    let receive_address = bitcoin::Address::p2wpkh(
        &bitcoin::CompressedPublicKey(receive_xpub.public_key),
        bitcoin::Network::Testnet,
    )
    .to_string();

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![dummy_input()],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: ScriptBuf::new_p2tr(&secp, input_xpub.to_x_only_pub(), None),
        }],
    };
    let value = 20_000_000;
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![unsigned_input(&prev_tx, 0)],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, change_xpub.to_x_only_pub(), None),
            },
            TxOut {
                value: Amount::from_sat(value),
                script_pubkey: ScriptBuf::new_p2wpkh(&receive_xpub.to_pub().wpubkey_hash()),
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].tap_internal_key = Some(input_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input_path)),
    );
    psbt.outputs[0].tap_internal_key = Some(change_xpub.to_x_only_pub());
    psbt.outputs[0].tap_key_origins.insert(
        change_xpub.to_x_only_pub(),
        (vec![], (fingerprint, change_path)),
    );
    psbt.outputs[1]
        .bip32_derivation
        .insert(receive_xpub.public_key, (fingerprint, receive_path));

    transaction_vector(
        "payment-request-owned-output",
        "Covers version-specific handling of payment-request metadata attached to an output owned by this device, rejected since v9.26.3.",
        Coin::Tbtc,
        psbt,
        payment_request_options(1, value, &receive_address),
        vec![taproot_key_signature(0, input_xpub.to_x_only_pub())],
        screens::payment_request_owned_output(),
    )
}
