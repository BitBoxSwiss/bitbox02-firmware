// SPDX-License-Identifier: Apache-2.0

use super::common::{
    SOME_XPUB, ecdsa_signature, keypath, secp, simulator_xprv, simulator_xpub_at,
    taproot_key_signature, taproot_script_signature, transaction_vector,
};
use super::screens;
use crate::btc_transaction::{
    Coin, KeyOriginInfo, MultisigScriptType, PsbtSignOptions, Registration, ScriptConfig,
    ScriptConfigWithKeypath, TestVector,
};
use bitcoin::bip32::{DerivationPath, Xpub};
use bitcoin::opcodes::all;
use bitcoin::psbt::Psbt;
use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
    blockdata::script::Builder, transaction,
};
use miniscript::psbt::PsbtExt;

pub fn all() -> Vec<TestVector> {
    vec![
        taproot_key_spend(),
        mixed_spend(),
        op_return(),
        multisig_p2wsh(),
        policy_wsh(),
        policy_tr_keyspend(),
        policy_tr_scriptspend(),
    ]
}

/// Test signing where all inputs are BIP86 Taproot keyspends.
fn taproot_key_spend() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);

    let change_path: DerivationPath = "m/86'/1'/0'/1/0".parse().unwrap();
    let change_xpub = simulator_xpub_at(&secp, &change_path);

    let input0_path: DerivationPath = "m/86'/1'/0'/0/0".parse().unwrap();
    let input0_xpub = simulator_xpub_at(&secp, &input0_path);

    let input1_path: DerivationPath = "m/86'/1'/0'/0/1".parse().unwrap();
    let input1_xpub = simulator_xpub_at(&secp, &input1_path);

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
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
        input: vec![
            TxIn {
                previous_output: OutPoint {
                    txid: prev_tx.compute_txid(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence(0xFFFFFFFF),
                witness: Witness::default(),
            },
            TxIn {
                previous_output: OutPoint {
                    txid: prev_tx.compute_txid(),
                    vout: 1,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence(0xFFFFFFFF),
                witness: Witness::default(),
            },
        ],
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, change_xpub.to_x_only_pub(), None),
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

    // Add input and change infos.
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0].tap_internal_key = Some(input0_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input0_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input0_path.clone())),
    );
    psbt.inputs[1].witness_utxo = Some(prev_tx.output[1].clone());
    psbt.inputs[1].tap_internal_key = Some(input1_xpub.to_x_only_pub());
    psbt.inputs[1].tap_key_origins.insert(
        input1_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input1_path.clone())),
    );

    psbt.outputs[0].tap_internal_key = Some(change_xpub.to_x_only_pub());
    psbt.outputs[0].tap_key_origins.insert(
        change_xpub.to_x_only_pub(),
        (vec![], (fingerprint, change_path)),
    );

    transaction_vector(
        "taproot-key-spend",
        "Signs two BIP86 Taproot key-spend inputs and recognizes a BIP86 change output.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions::default(),
        vec![
            taproot_key_signature(0, input0_xpub.to_x_only_pub()),
            taproot_key_signature(1, input1_xpub.to_x_only_pub()),
        ],
        screens::taproot_key_spend(),
    )
}

/// Test signing with mixed input types: P2TR, P2WPKH and P2SH-P2WPKH.
fn mixed_spend() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);

    let change_path: DerivationPath = "m/86'/1'/0'/1/0".parse().unwrap();
    let change_xpub = simulator_xpub_at(&secp, &change_path);

    let input0_path: DerivationPath = "m/86'/1'/0'/0/0".parse().unwrap();
    let input0_xpub = simulator_xpub_at(&secp, &input0_path);

    let input1_path: DerivationPath = "m/84'/1'/0'/0/0".parse().unwrap();
    let input1_xpub = simulator_xpub_at(&secp, &input1_path);

    let input2_path: DerivationPath = "m/49'/1'/0'/0/0".parse().unwrap();
    let input2_xpub = simulator_xpub_at(&secp, &input2_path);

    let input2_redeemscript = ScriptBuf::new_p2wpkh(&input2_xpub.to_pub().wpubkey_hash());

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
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
                script_pubkey: ScriptBuf::new_p2sh(&input2_redeemscript.clone().into()),
            },
        ],
    };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: (0..3)
            .map(|vout| TxIn {
                previous_output: OutPoint {
                    txid: prev_tx.compute_txid(),
                    vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence(0xFFFFFFFF),
                witness: Witness::default(),
            })
            .collect(),
        output: vec![
            TxOut {
                value: Amount::from_sat(100_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp, change_xpub.to_x_only_pub(), None),
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

    // Add input and change infos.
    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].tap_internal_key = Some(input0_xpub.to_x_only_pub());
    psbt.inputs[0].tap_key_origins.insert(
        input0_xpub.to_x_only_pub(),
        (vec![], (fingerprint, input0_path)),
    );

    psbt.inputs[1].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[1]
        .bip32_derivation
        .insert(input1_xpub.to_pub().0, (fingerprint, input1_path));

    psbt.inputs[2].non_witness_utxo = Some(prev_tx);
    psbt.inputs[2].redeem_script = Some(input2_redeemscript);
    psbt.inputs[2]
        .bip32_derivation
        .insert(input2_xpub.to_pub().0, (fingerprint, input2_path));

    psbt.outputs[0].tap_internal_key = Some(change_xpub.to_x_only_pub());
    psbt.outputs[0].tap_key_origins.insert(
        change_xpub.to_x_only_pub(),
        (vec![], (fingerprint, change_path)),
    );

    transaction_vector(
        "mixed-spend",
        "Signs P2TR, native P2WPKH and nested P2SH-P2WPKH inputs from one previous transaction and recognizes BIP86 change.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions::default(),
        vec![
            taproot_key_signature(0, input0_xpub.to_x_only_pub()),
            ecdsa_signature(1, input1_xpub.to_pub().0),
            ecdsa_signature(2, input2_xpub.to_pub().0),
        ],
        screens::mixed_spend(),
    )
}

fn op_return() -> TestVector {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);

    let input_path: DerivationPath = "m/84'/1'/0'/0/5".parse().unwrap();
    let change_path: DerivationPath = "m/84'/1'/0'/1/0".parse().unwrap();

    let input_pub = simulator_xpub_at(&secp, &input_path).to_pub();
    let change_pub = simulator_xpub_at(&secp, &change_path).to_pub();

    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(50_000_000),
            script_pubkey: ScriptBuf::new_p2wpkh(&input_pub.wpubkey_hash()),
        }],
    };

    let op_return_data = b"hello world";
    let op_return_script = Builder::new()
        .push_opcode(all::OP_RETURN)
        .push_slice(op_return_data)
        .into_script();

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(49_000_000),
                script_pubkey: ScriptBuf::new_p2wpkh(&change_pub.wpubkey_hash()),
            },
            TxOut {
                value: Amount::from_sat(0),
                script_pubkey: op_return_script,
            },
        ],
    };

    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();

    psbt.inputs[0].non_witness_utxo = Some(prev_tx.clone());
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    psbt.inputs[0]
        .bip32_derivation
        .insert(input_pub.0, (fingerprint, input_path));

    psbt.outputs[0]
        .bip32_derivation
        .insert(change_pub.0, (fingerprint, change_path));

    transaction_vector(
        "op-return",
        "Signs a P2WPKH spend with a zero-value OP_RETURN output containing one printable data push and a P2WPKH change output.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions::default(),
        vec![ecdsa_signature(0, input_pub.0)],
        screens::op_return(),
    )
}

/// Test a registered 1-of-2 P2WSH multisig account. The historical test name says 1-of-3, but the
/// client test has always used two xpubs; this vector retains the exact transaction construction.
fn multisig_p2wsh() -> TestVector {
    let secp = secp();
    let our_root_fingerprint = simulator_xprv().fingerprint(&secp);

    let threshold: u32 = 1;
    let keypath_account: DerivationPath = "m/48'/1'/0'/2'".parse().unwrap();

    let our_xpub: Xpub = simulator_xpub_at(&secp, &keypath_account);
    let some_xpub: Xpub = SOME_XPUB.parse().unwrap();

    // We use the miniscript library to build a multipath descriptor including key origin so we can
    // easily derive the receive/change descriptor, pubkey scripts, populate the PSBT input key
    // infos and convert the sigs to final witnesses.
    let multi_descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey> = format!(
        "wsh(sortedmulti({},[{}/48'/1'/0'/2']{}/<0;1>/*,{}/<0;1>/*))",
        threshold, our_root_fingerprint, our_xpub, some_xpub
    )
    .parse()
    .unwrap();
    assert!(multi_descriptor.sanity_check().is_ok());

    let [descriptor_receive, descriptor_change] = multi_descriptor
        .into_single_descriptors()
        .unwrap()
        .try_into()
        .unwrap();
    // Derive /0/0 (first receive) and /1/0 (first change) descriptors.
    let input_descriptor = descriptor_receive.at_derivation_index(0).unwrap();
    let change_descriptor = descriptor_change.at_derivation_index(0).unwrap();

    let multisig_config = ScriptConfig::Multisig {
        threshold,
        xpubs: vec![our_xpub.to_string(), some_xpub.to_string()],
        our_xpub_index: 0,
        script_type: MultisigScriptType::P2wsh,
    };

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: input_descriptor.script_pubkey(),
        }],
    };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_descriptor.script_pubkey(),
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

    // Add input and change infos.
    psbt.inputs[0].non_witness_utxo = Some(prev_tx);
    // These add the input/output bip32_derivation entries / key infos.
    psbt.update_input_with_descriptor(0, &input_descriptor)
        .unwrap();
    psbt.update_output_with_descriptor(0, &change_descriptor)
        .unwrap();

    let input_path: DerivationPath = "m/48'/1'/0'/2'/0/0".parse().unwrap();
    let input_pubkey = simulator_xpub_at(&secp, &input_path).public_key;
    let mut vector = transaction_vector(
        "multisig-p2wsh",
        "Signs the device branch of a registered 1-of-2 sortedmulti P2WSH input, recognizes descriptor-derived change, and finalizes the witness.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: multisig_config.clone(),
                keypath: keypath(&keypath_account),
            }),
            ..Default::default()
        },
        vec![ecdsa_signature(0, input_pubkey)],
        screens::multisig_p2wsh(),
    );
    vector.registrations = vec![Registration {
        script_config: multisig_config,
        keypath: Some(keypath(&keypath_account)),
        name: "test wsh multisig".into(),
    }];
    vector
}

fn policy_wsh() -> TestVector {
    let secp = secp();
    // Policy string following BIP-388 syntax, input to the BitBox.
    let policy = "wsh(or_b(pk(@0/<0;1>/*),s:pk(@1/<0;1>/*)))";

    let our_root_fingerprint = simulator_xprv().fingerprint(&secp);
    let keypath_account: DerivationPath = "m/48'/1'/0'/3'".parse().unwrap();
    let our_xpub: Xpub = simulator_xpub_at(&secp, &keypath_account);
    let some_xpub: Xpub = SOME_XPUB.parse().unwrap();

    // We use the miniscript library to build a multipath descriptor including key origin so we can
    // easily derive the receive/change descriptor, pubkey scripts, populate the PSBT input key
    // infos and convert the sigs to final witnesses.
    let multi_descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey> = policy
        .replace(
            "@0",
            &format!("[{}/48'/1'/0'/3']{}", our_root_fingerprint, our_xpub),
        )
        .replace("@1", &some_xpub.to_string())
        .parse()
        .unwrap();
    assert!(multi_descriptor.sanity_check().is_ok());

    let [descriptor_receive, descriptor_change] = multi_descriptor
        .into_single_descriptors()
        .unwrap()
        .try_into()
        .unwrap();
    // Derive /0/0 (first receive) and /1/0 (first change) descriptors.
    let input_descriptor = descriptor_receive.at_derivation_index(0).unwrap();
    let change_descriptor = descriptor_change.at_derivation_index(0).unwrap();

    let policy_config = ScriptConfig::Policy {
        policy: policy.into(),
        keys: vec![
            // Our key: root fingerprint and keypath are required.
            KeyOriginInfo {
                root_fingerprint: Some(our_root_fingerprint.to_string()),
                keypath: Some(keypath(&keypath_account)),
                xpub: our_xpub.to_string(),
            },
            // Foreign key: root fingerprint and keypath are optional.
            KeyOriginInfo {
                root_fingerprint: None,
                keypath: None,
                xpub: some_xpub.to_string(),
            },
        ],
    };

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: input_descriptor.script_pubkey(),
        }],
    };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_descriptor.script_pubkey(),
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

    // Add input and change infos.
    psbt.inputs[0].non_witness_utxo = Some(prev_tx);
    // These add the input/output bip32_derivation entries / key infos.
    psbt.update_input_with_descriptor(0, &input_descriptor)
        .unwrap();
    psbt.update_output_with_descriptor(0, &change_descriptor)
        .unwrap();

    let input_path: DerivationPath = "m/48'/1'/0'/3'/0/0".parse().unwrap();
    let input_pubkey = simulator_xpub_at(&secp, &input_path).public_key;
    let mut vector = transaction_vector(
        "policy-wsh",
        "Signs the device branch of a registered BIP388 WSH or-policy and recognizes descriptor-derived change.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: policy_config.clone(),
                keypath: keypath(&keypath_account),
            }),
            ..Default::default()
        },
        vec![ecdsa_signature(0, input_pubkey)],
        screens::policy_wsh(),
    );
    vector.registrations = vec![Registration {
        script_config: policy_config,
        keypath: None,
        name: "test wsh policy".into(),
    }];
    vector
}

fn policy_tr_keyspend() -> TestVector {
    let secp = secp();
    // Policy string following BIP-388 syntax, input to the BitBox.
    let policy = "tr(@0/<0;1>/*)";

    let our_root_fingerprint = simulator_xprv().fingerprint(&secp);
    let keypath_account: DerivationPath = "m/48'/1'/0'/3'".parse().unwrap();
    let our_xpub: Xpub = simulator_xpub_at(&secp, &keypath_account);

    // We use the miniscript library to build a multipath descriptor including key origin so we can
    // easily derive the receive/change descriptor, pubkey scripts, populate the PSBT input key
    // infos and convert the sigs to final witnesses.
    let multi_descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey> = policy
        .replace(
            "@0",
            &format!("[{}/48'/1'/0'/3']{}", our_root_fingerprint, our_xpub),
        )
        .parse()
        .unwrap();
    assert!(multi_descriptor.sanity_check().is_ok());

    let [descriptor_receive, descriptor_change] = multi_descriptor
        .into_single_descriptors()
        .unwrap()
        .try_into()
        .unwrap();
    // Derive /0/0 (first receive) and /1/0 (first change) descriptors.
    let input_descriptor = descriptor_receive.at_derivation_index(0).unwrap();
    let change_descriptor = descriptor_change.at_derivation_index(0).unwrap();

    let policy_config = ScriptConfig::Policy {
        policy: policy.into(),
        keys: vec![
            // Our key: root fingerprint and keypath are required.
            KeyOriginInfo {
                root_fingerprint: Some(our_root_fingerprint.to_string()),
                keypath: Some(keypath(&keypath_account)),
                xpub: our_xpub.to_string(),
            },
        ],
    };

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: input_descriptor.script_pubkey(),
        }],
    };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_descriptor.script_pubkey(),
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

    // Add input and change infos.
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    // These add the input/output bip32_derivation entries / key infos.
    psbt.update_input_with_descriptor(0, &input_descriptor)
        .unwrap();
    psbt.update_output_with_descriptor(0, &change_descriptor)
        .unwrap();

    let input_path: DerivationPath = "m/48'/1'/0'/3'/0/0".parse().unwrap();
    let input_pubkey = simulator_xpub_at(&secp, &input_path).to_x_only_pub();
    let mut vector = transaction_vector(
        "policy-tr-keyspend",
        "Signs the key-spend path of a registered BIP388 Taproot policy using only a witness UTXO and recognizes descriptor-derived change.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: policy_config.clone(),
                keypath: keypath(&keypath_account),
            }),
            ..Default::default()
        },
        vec![taproot_key_signature(0, input_pubkey)],
        screens::policy_tr_keyspend(),
    );
    vector.registrations = vec![Registration {
        script_config: policy_config,
        keypath: None,
        name: "test tr keyspend policy".into(),
    }];
    vector
}

fn policy_tr_scriptspend() -> TestVector {
    let secp = secp();
    // Policy string following BIP-388 syntax, input to the BitBox.
    let policy = "tr(@0/<0;1>/*,pk(@1/<0;1>/*))";

    let our_root_fingerprint = simulator_xprv().fingerprint(&secp);
    let keypath_account: DerivationPath = "m/48'/1'/0'/3'".parse().unwrap();
    let our_xpub: Xpub = simulator_xpub_at(&secp, &keypath_account);
    let some_xpub: Xpub = SOME_XPUB.parse().unwrap();

    // We use the miniscript library to build a multipath descriptor including key origin so we can
    // easily derive the receive/change descriptor, pubkey scripts, populate the PSBT input key
    // infos and convert the sigs to final witnesses.
    let multi_descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey> = policy
        .replace(
            "@1",
            &format!("[{}/48'/1'/0'/3']{}", our_root_fingerprint, our_xpub),
        )
        .replace("@0", &some_xpub.to_string())
        .parse()
        .unwrap();
    assert!(multi_descriptor.sanity_check().is_ok());

    let [descriptor_receive, descriptor_change] = multi_descriptor
        .into_single_descriptors()
        .unwrap()
        .try_into()
        .unwrap();
    // Derive /0/0 (first receive) and /1/0 (first change) descriptors.
    let input_descriptor = descriptor_receive.at_derivation_index(0).unwrap();
    let change_descriptor = descriptor_change.at_derivation_index(0).unwrap();

    let policy_config = ScriptConfig::Policy {
        policy: policy.into(),
        keys: vec![
            // Foreign key: root fingerprint and keypath are optional.
            KeyOriginInfo {
                root_fingerprint: None,
                keypath: None,
                xpub: some_xpub.to_string(),
            },
            // Our key: root fingerprint and keypath are required.
            KeyOriginInfo {
                root_fingerprint: Some(our_root_fingerprint.to_string()),
                keypath: Some(keypath(&keypath_account)),
                xpub: our_xpub.to_string(),
            },
        ],
    };

    // A previous tx which creates some UTXOs we can reference later.
    let prev_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: "3131313131313131313131313131313131313131313131313131313131313131:0"
                .parse()
                .unwrap(),
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(100_000_000),
            script_pubkey: input_descriptor.script_pubkey(),
        }],
    };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx.compute_txid(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence(0xFFFFFFFF),
            witness: Witness::default(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_descriptor.script_pubkey(),
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

    // Add input and change infos.
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    // These add the input/output bip32_derivation entries / key infos.
    psbt.update_input_with_descriptor(0, &input_descriptor)
        .unwrap();
    psbt.update_output_with_descriptor(0, &change_descriptor)
        .unwrap();

    let input_path: DerivationPath = "m/48'/1'/0'/3'/0/0".parse().unwrap();
    let input_pubkey = simulator_xpub_at(&secp, &input_path).to_x_only_pub();
    let leaf_hash = psbt.inputs[0].tap_key_origins.get(&input_pubkey).unwrap().0[0];
    let mut vector = transaction_vector(
        "policy-tr-scriptspend",
        "Signs the script path owned by the device in a registered BIP388 Taproot policy whose internal key belongs to another signer.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: policy_config.clone(),
                keypath: keypath(&keypath_account),
            }),
            ..Default::default()
        },
        vec![taproot_script_signature(0, input_pubkey, leaf_hash)],
        screens::policy_tr_scriptspend(),
    );
    vector.registrations = vec![Registration {
        script_config: policy_config,
        keypath: None,
        name: "test tr scriptspend policy".into(),
    }];
    vector
}
