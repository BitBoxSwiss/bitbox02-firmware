// SPDX-License-Identifier: Apache-2.0

//! Additional portable scenarios migrated from the firmware transaction-signing tests.

use super::common::{
    ecdsa_signature, secp, simulator_xprv, simulator_xpub_at, taproot_key_signature,
    transaction_vector,
};
use super::metadata_psbt::{coin_purchase_payment_request, payment_request_signature};
use super::screens;
use crate::btc_transaction::{
    Coin, ExpectedSignature, FormatUnit, PaymentRequest, PaymentRequestMemo, PsbtOutputOptions,
    PsbtSignOptions, Screen, SimpleType, TestVector,
};
use bitcoin::bip32::DerivationPath;
use bitcoin::blockdata::script::Builder;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::OP_RETURN;
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness, transaction,
};
use std::collections::BTreeMap;

const TBTC_EXTERNAL_XONLY: &str =
    "e4adbb12c3426ec71ebb10688d8ae69d531ca822a2b790acee216a7f1b95b576";
const BTC_EXTERNAL_XONLY: &str = "a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c";
const SILENT_PAYMENT_ADDRESS: &str = "sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv";

struct SimpleSpend {
    psbt: bitcoin::psbt::Psbt,
    expected_signature: ExpectedSignature,
}

fn path(value: &str) -> DerivationPath {
    value.parse().unwrap()
}

fn simple_script(
    secp: &bitcoin::secp256k1::Secp256k1<bitcoin::secp256k1::All>,
    script_type: SimpleType,
    xpub: &bitcoin::bip32::Xpub,
) -> (ScriptBuf, Option<ScriptBuf>) {
    match script_type {
        SimpleType::P2wpkh => (ScriptBuf::new_p2wpkh(&xpub.to_pub().wpubkey_hash()), None),
        SimpleType::P2wpkhP2sh => {
            let redeem_script = ScriptBuf::new_p2wpkh(&xpub.to_pub().wpubkey_hash());
            (
                ScriptBuf::new_p2sh(&redeem_script.script_hash()),
                Some(redeem_script),
            )
        }
        SimpleType::P2tr => (ScriptBuf::new_p2tr(secp, xpub.to_x_only_pub(), None), None),
    }
}

fn simple_spend(
    script_type: SimpleType,
    account: &str,
    input_index: u32,
    sequence: u32,
    locktime: u32,
    external_script: ScriptBuf,
) -> SimpleSpend {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let input_path = path(&format!("{account}/0/{input_index}"));
    let change_path = path(&format!("{account}/1/0"));
    let input_xpub = simulator_xpub_at(&secp, &input_path);
    let change_xpub = simulator_xpub_at(&secp, &change_path);
    let (input_script, input_redeem_script) = simple_script(&secp, script_type, &input_xpub);
    let (change_script, change_redeem_script) = simple_script(&secp, script_type, &change_xpub);

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: input_script,
        }],
    };
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::from_consensus(locktime),
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(sequence),
            witness: Witness::new(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_script,
            },
            TxOut {
                value: Amount::from_sat(20_000_000),
                script_pubkey: external_script,
            },
        ],
    };
    let mut psbt = bitcoin::psbt::Psbt::from_unsigned_tx(tx).unwrap();
    // Keep the full previous transaction available even when this particular form only needs the
    // witness UTXO. The derived firmware request may need it because of a non-Taproot owned output
    // config.
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].redeem_script = input_redeem_script;
    psbt.outputs[0].redeem_script = change_redeem_script;

    let expected_signature = match script_type {
        SimpleType::P2tr => {
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
            taproot_key_signature(0, input_xpub.to_x_only_pub())
        }
        SimpleType::P2wpkh | SimpleType::P2wpkhP2sh => {
            psbt.inputs[0]
                .bip32_derivation
                .insert(input_xpub.public_key, (fingerprint, input_path));
            psbt.outputs[0]
                .bip32_derivation
                .insert(change_xpub.public_key, (fingerprint, change_path));
            ecdsa_signature(0, input_xpub.public_key)
        }
    };

    SimpleSpend {
        psbt,
        expected_signature,
    }
}

fn p2tr_script(xonly: &str) -> ScriptBuf {
    ScriptBuf::new_p2tr(&secp(), xonly.parse().unwrap(), None)
}

fn ltc_external_script() -> ScriptBuf {
    let pubkey: bitcoin::PublicKey =
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
            .parse()
            .unwrap();
    ScriptBuf::new_p2wpkh(&pubkey.wpubkey_hash().unwrap())
}

fn op_return_spend(payload: &[u8], value: u64) -> SimpleSpend {
    let mut spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/1'/0'",
        0,
        Sequence::MAX.0,
        0,
        p2tr_script(TBTC_EXTERNAL_XONLY),
    );
    let script = Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(bitcoin::script::PushBytesBuf::try_from(payload.to_vec()).unwrap())
        .into_script();
    spend.psbt.unsigned_tx.output.insert(
        0,
        TxOut {
            value: Amount::from_sat(value),
            script_pubkey: script,
        },
    );
    spend.psbt.outputs.insert(0, Default::default());
    spend
}

fn op_return_nonascii() -> TestVector {
    let spend = op_return_spend(&[1, 2, 3, 4, 5], 0);
    transaction_vector(
        "op-return-nonascii",
        "Displays a non-ASCII OP_RETURN payload as hexadecimal and signs the transaction.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::op_return_nonascii(),
    )
}

fn op_return_nonzero_value() -> TestVector {
    let spend = op_return_spend(b"hello world", 100);
    transaction_vector(
        "op-return-nonzero-value",
        "Rejects a nonzero-value OP_RETURN output.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![],
        screens::unsupported_then_invalid_input("9.24.0"),
    )
}

fn op_return_silent_payment() -> TestVector {
    let spend = op_return_spend(b"hello world", 0);
    transaction_vector(
        "op-return-silent-payment",
        "Rejects silent-payment metadata on an OP_RETURN output.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions {
            outputs: BTreeMap::from([(
                0,
                PsbtOutputOptions {
                    silent_payment_address: Some(SILENT_PAYMENT_ADDRESS.into()),
                    payment_request_index: None,
                },
            )]),
            ..Default::default()
        },
        vec![],
        screens::unsupported_then_invalid_input("9.24.0"),
    )
}

fn op_return_payment_request() -> TestVector {
    let spend = op_return_spend(b"hello world", 0);
    let signed_address = "tb1pff8vkq80pu2cgtu7ttgad2znw62v2lguhw6ptrppwns6nrpqau2qcuz37d";
    transaction_vector(
        "op-return-payment-request",
        "Rejects payment-request metadata on an OP_RETURN output.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions {
            outputs: BTreeMap::from([(
                0,
                PsbtOutputOptions {
                    silent_payment_address: None,
                    payment_request_index: Some(0),
                },
            )]),
            payment_requests: vec![PaymentRequest {
                recipient_name: "Test Merchant".into(),
                total_amount: 1,
                nonce: String::new(),
                memos: vec![PaymentRequestMemo::Text {
                    note: "TextMemo line1\nTextMemo line2".into(),
                }],
                signature: hex::encode(payment_request_signature(1, signed_address)),
            }],
            ..Default::default()
        },
        vec![],
        screens::unsupported_then_invalid_input("9.24.0"),
    )
}

fn multiple_output_types(
    coin: Coin,
    format_unit: FormatUnit,
    high_fee_warning: bool,
) -> TestVector {
    assert!(!high_fee_warning || coin == Coin::Btc && format_unit == FormatUnit::Default);
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let coin_type = match coin {
        Coin::Btc => 0,
        Coin::Ltc => 2,
        Coin::Tbtc => panic!("multiple-output fixture has no TBTC address set"),
    };
    let account = format!("m/84'/{coin_type}'/10'");
    let input_path = path(&format!("{account}/0/5"));
    let change0_path = path(&format!("{account}/1/3"));
    let change1_path = path(&format!("{account}/1/30"));
    let input_xpub = simulator_xpub_at(&secp, &input_path);
    let change0_xpub = simulator_xpub_at(&secp, &change0_path);
    let change1_xpub = simulator_xpub_at(&secp, &change1_path);
    let prev_tx = Transaction {
        version: transaction::Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(2_030_000_000),
            script_pubkey: ScriptBuf::new_p2wpkh(&input_xpub.to_pub().wpubkey_hash()),
        }],
    };
    let tx = Transaction {
        version: transaction::Version::ONE,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2pkh(&bitcoin::PubkeyHash::from_byte_array(
                    [0x11; 20],
                )),
            },
            TxOut {
                value: Amount::from_sat(if high_fee_warning {
                    1_034_567_890
                } else {
                    1_234_567_890
                }),
                script_pubkey: ScriptBuf::new_p2sh(&bitcoin::ScriptHash::from_byte_array(
                    [0x22; 20],
                )),
            },
            TxOut {
                value: Amount::from_sat(6_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&bitcoin::WPubkeyHash::from_byte_array(
                    [0x33; 20],
                )),
            },
            TxOut {
                value: Amount::from_sat(7_000),
                script_pubkey: ScriptBuf::new_p2wsh(&bitcoin::WScriptHash::from_byte_array(
                    [0x44; 32],
                )),
            },
            TxOut {
                value: Amount::from_sat(690_000_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&change0_xpub.to_pub().wpubkey_hash()),
            },
            TxOut {
                value: Amount::from_sat(100),
                script_pubkey: ScriptBuf::new_p2wpkh(&change1_xpub.to_pub().wpubkey_hash()),
            },
        ],
    };
    let mut psbt = bitcoin::psbt::Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0]
        .bip32_derivation
        .insert(input_xpub.public_key, (fingerprint, input_path));
    psbt.outputs[4]
        .bip32_derivation
        .insert(change0_xpub.public_key, (fingerprint, change0_path));
    psbt.outputs[5]
        .bip32_derivation
        .insert(change1_xpub.public_key, (fingerprint, change1_path));

    let unit_name = match format_unit {
        FormatUnit::Default => "coin",
        FormatUnit::Sat => "satoshi",
    };
    let id = if high_fee_warning {
        "high-fee-rounding".into()
    } else {
        format!(
            "multiple-output-types-{}-{unit_name}",
            match coin {
                Coin::Btc => "bitcoin",
                Coin::Ltc => "litecoin",
                Coin::Tbtc => unreachable!(),
            }
        )
    };
    transaction_vector(
        &id,
        if high_fee_warning {
            "Displays the 18.1% high-fee warning and requires the final longtouch on the warning instead of the fee screen."
        } else {
            "Displays P2PKH, P2SH, P2WPKH and P2WSH recipients, warns about two change outputs, and signs the transaction."
        },
        coin,
        psbt,
        PsbtSignOptions {
            format_unit,
            ..Default::default()
        },
        vec![ecdsa_signature(0, input_xpub.public_key)],
        screens::multiple_output_types(coin, format_unit == FormatUnit::Sat, high_fee_warning),
    )
}

fn swap_payment_request(unsupported_source: bool) -> TestVector {
    let (coin, account, external_xonly, source_address, source_coin_type) = if unsupported_source {
        (
            Coin::Tbtc,
            "m/84'/1'/0'",
            TBTC_EXTERNAL_XONLY,
            "tb1pff8vkq80pu2cgtu7ttgad2znw62v2lguhw6ptrppwns6nrpqau2qcuz37d",
            1,
        )
    } else {
        (
            Coin::Btc,
            "m/84'/0'/0'",
            BTC_EXTERNAL_XONLY,
            "bc1pmg5dhafms6h9nts4dtehgkanym6yeccfmk5hx3ts3jxnm4zh2knqv80ha5",
            0,
        )
    };
    let spend = simple_spend(
        SimpleType::P2wpkh,
        account,
        0,
        Sequence::MAX.0,
        0,
        p2tr_script(external_xonly),
    );
    let options = PsbtSignOptions {
        outputs: BTreeMap::from([(
            1,
            PsbtOutputOptions {
                silent_payment_address: None,
                payment_request_index: Some(0),
            },
        )]),
        payment_requests: vec![coin_purchase_payment_request(
            source_coin_type,
            20_000_000,
            source_address,
        )],
        ..Default::default()
    };
    transaction_vector(
        if unsupported_source {
            "swap-payment-request-unsupported-source"
        } else {
            "swap-payment-request"
        },
        if unsupported_source {
            "Rejects a coin-purchase payment request whose source account is Bitcoin testnet."
        } else {
            "Displays and signs a Bitcoin-to-Ethereum coin-purchase payment request."
        },
        coin,
        spend.psbt,
        options,
        (!unsupported_source)
            .then_some(spend.expected_signature)
            .into_iter()
            .collect(),
        if unsupported_source {
            screens::swap_payment_request_unsupported_source()
        } else {
            screens::swap_payment_request()
        },
    )
}

fn p2wpkh_p2sh() -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkhP2sh,
        "m/49'/1'/0'",
        0,
        Sequence::MAX.0,
        0,
        p2tr_script(TBTC_EXTERNAL_XONLY),
    );
    transaction_vector(
        "p2wpkh-p2sh",
        "Signs a nested SegWit input and recognizes nested SegWit change.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::simple_tbtc(&[]),
    )
}

fn high_address_index() -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/1'/0'",
        100_000,
        Sequence::MAX.0,
        0,
        p2tr_script(TBTC_EXTERNAL_XONLY),
    );
    transaction_vector(
        "high-input-address-index",
        "Spends an owned input at address index 100000 without treating the high spend path as a receive-address verification request.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::simple_tbtc(&[]),
    )
}

fn locktime(block: u32, sequence: u32, rbf: bool) -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/1'/0'",
        0,
        sequence,
        block,
        p2tr_script(TBTC_EXTERNAL_XONLY),
    );
    let qualifier = if rbf { "rbf" } else { "non-rbf" };
    let locktime_screen = Screen::Confirm {
        title: String::new(),
        body: format!(
            "Locktime on block:\n{block}\nTransaction is {}RBF",
            if rbf { "" } else { "not " }
        ),
        longtouch: false,
    };
    transaction_vector(
        &format!("locktime-{qualifier}"),
        &format!("Displays block locktime {block} and its {qualifier} sequence semantics."),
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::simple_tbtc(&[locktime_screen]),
    )
}

fn zero_locktime() -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/1'/0'",
        0,
        Sequence::MAX.0 - 2,
        0,
        p2tr_script(TBTC_EXTERNAL_XONLY),
    );
    transaction_vector(
        "locktime-zero",
        "Suppresses the locktime confirmation when locktime is zero even if the sequence signals RBF.",
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::simple_tbtc(&[]),
    )
}

fn p2tr_output_btc() -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/0'/0'",
        0,
        Sequence::MAX.0,
        0,
        p2tr_script(BTC_EXTERNAL_XONLY),
    );
    transaction_vector(
        "p2tr-output-mainnet",
        "Displays and signs a mainnet P2TR recipient output from a native SegWit account.",
        Coin::Btc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::p2tr_output_btc(),
    )
}

fn silent_payment_rejects_owned_output() -> TestVector {
    let mut spend = simple_spend(
        SimpleType::P2tr,
        "m/86'/0'/0'",
        0,
        Sequence::MAX.0,
        0,
        p2tr_script(BTC_EXTERNAL_XONLY),
    );
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let receive_path = path("m/84'/0'/0'/0/0");
    let receive_xpub = simulator_xpub_at(&secp, &receive_path);
    spend.psbt.unsigned_tx.output[1].script_pubkey =
        ScriptBuf::new_p2wpkh(&receive_xpub.to_pub().wpubkey_hash());
    spend.psbt.outputs[1]
        .bip32_derivation
        .insert(receive_xpub.public_key, (fingerprint, receive_path));

    let options = PsbtSignOptions {
        outputs: BTreeMap::from([(
            1,
            PsbtOutputOptions {
                silent_payment_address: Some(SILENT_PAYMENT_ADDRESS.into()),
                payment_request_index: None,
            },
        )]),
        ..Default::default()
    };
    transaction_vector(
        "silent-payment-owned-output",
        "Covers version-specific handling of silent-payment metadata attached to an output owned by this device, rejected since v9.26.3.",
        Coin::Btc,
        spend.psbt,
        options,
        vec![spend.expected_signature],
        screens::silent_payment_owned_output(),
    )
}

fn ltc_locktime(sequence: u32, qualifier: &str) -> TestVector {
    let spend = simple_spend(
        SimpleType::P2wpkh,
        "m/84'/2'/0'",
        0,
        sequence,
        10,
        ltc_external_script(),
    );
    let locktime_screen = Screen::Confirm {
        title: String::new(),
        body: "Locktime on block:\n10\n".into(),
        longtouch: false,
    };
    transaction_vector(
        &format!("locktime-litecoin-{qualifier}"),
        "Displays a Litecoin block locktime without Bitcoin-specific RBF wording.",
        Coin::Ltc,
        spend.psbt,
        PsbtSignOptions::default(),
        vec![spend.expected_signature],
        screens::simple_ltc(&[locktime_screen]),
    )
}

pub fn all() -> Vec<TestVector> {
    vec![
        multiple_output_types(Coin::Btc, FormatUnit::Default, false),
        multiple_output_types(Coin::Btc, FormatUnit::Sat, false),
        multiple_output_types(Coin::Ltc, FormatUnit::Default, false),
        multiple_output_types(Coin::Btc, FormatUnit::Default, true),
        swap_payment_request(false),
        swap_payment_request(true),
        p2wpkh_p2sh(),
        high_address_index(),
        zero_locktime(),
        locktime(10, Sequence::MAX.0 - 1, false),
        locktime(10, Sequence::MAX.0 - 2, true),
        ltc_locktime(Sequence::MAX.0 - 1, "non-rbf-sequence"),
        ltc_locktime(Sequence::MAX.0 - 2, "rbf-sequence"),
        p2tr_output_btc(),
        silent_payment_rejects_owned_output(),
        op_return_nonascii(),
        op_return_nonzero_value(),
        op_return_silent_payment(),
        op_return_payment_request(),
    ]
}
