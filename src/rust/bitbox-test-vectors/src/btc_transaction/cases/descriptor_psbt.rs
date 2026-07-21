// SPDX-License-Identifier: Apache-2.0

//! Portable multisig and policy scenarios backed by descriptors.

use super::common::{
    ecdsa_signature, keypath, secp, simulator_xprv, simulator_xpub_at, taproot_key_signature,
    taproot_script_signature, transaction_vector,
};
use super::screens;
use crate::btc_transaction::{
    Coin, KeyOriginInfo, MultisigScriptType, PsbtSignOptions, Registration, ScriptConfig,
    ScriptConfigWithKeypath, TestVector,
};
use bitcoin::bip32::{ChainCode, ChildNumber, DerivationPath, Fingerprint, KeySource, Xpriv, Xpub};
use bitcoin::hashes::{Hash, sha256};
use bitcoin::psbt::Psbt;
use bitcoin::secp256k1::{PublicKey, XOnlyPublicKey};
use bitcoin::taproot::TapLeafHash;
use bitcoin::{
    Amount, Network, NetworkKind, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
    transaction,
};
use miniscript::psbt::PsbtExt;
use std::collections::BTreeMap;

const EXTERNAL_XONLY: &str = "e4adbb12c3426ec71ebb10688d8ae69d531ca822a2b790acee216a7f1b95b576";
const POLICY_ACCOUNT: &str = "m/48'/1'/0'/3'";

struct MultisigSpend {
    psbt: Psbt,
    config: ScriptConfig,
    account: DerivationPath,
    input_pubkey: PublicKey,
}

struct PolicySpend {
    psbt: Psbt,
    config: ScriptConfig,
    account: DerivationPath,
    input_pubkey: PublicKey,
    display_keys: Vec<String>,
}

fn path(value: &str) -> DerivationPath {
    value.parse().unwrap()
}

fn seeded_xpriv(index: u8) -> Xpriv {
    let mut seed = [0u8; 32];
    seed[0] = index;
    Xpriv::new_master(Network::Testnet, &seed).unwrap()
}

fn account_xpub(root: &Xpriv, account: &DerivationPath) -> Xpub {
    Xpub::from_priv(&secp(), &root.derive_priv(&secp(), account).unwrap())
}

fn descriptor_key(root: &Xpriv, account: &DerivationPath, xpub: Xpub, branches: &str) -> String {
    format!(
        "[{}/{}]{xpub}/{branches}/*",
        root.fingerprint(&secp()),
        account
    )
}

fn descriptor_psbt(descriptor: &str, change_index: u32) -> Psbt {
    let descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey> =
        descriptor.parse().unwrap();
    assert!(descriptor.sanity_check().is_ok());
    let [receive, change] = descriptor
        .into_single_descriptors()
        .unwrap()
        .try_into()
        .unwrap();
    let input_descriptor = receive.at_derivation_index(0).unwrap();
    let change_descriptor = change.at_derivation_index(change_index).unwrap();

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
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![
            TxOut {
                value: Amount::from_sat(70_000_000),
                script_pubkey: change_descriptor.script_pubkey(),
            },
            TxOut {
                value: Amount::from_sat(20_000_000),
                script_pubkey: ScriptBuf::new_p2tr(&secp(), EXTERNAL_XONLY.parse().unwrap(), None),
            },
        ],
    };
    let mut psbt = Psbt::from_unsigned_tx(tx).unwrap();
    psbt.inputs[0].witness_utxo = Some(prev_tx.output[0].clone());
    if !input_descriptor.script_pubkey().is_p2tr() {
        psbt.inputs[0].non_witness_utxo = Some(prev_tx);
    }
    psbt.update_input_with_descriptor(0, &input_descriptor)
        .unwrap();
    psbt.update_output_with_descriptor(0, &change_descriptor)
        .unwrap();
    psbt
}

fn add_cosigner_signature(psbt: &mut Psbt, root: &Xpriv) {
    let ecdsa_before = psbt.inputs[0].partial_sigs.len();
    let taproot_before = psbt.inputs[0].tap_script_sigs.len();
    psbt.sign(root, &secp()).unwrap();
    assert_eq!(
        psbt.inputs[0].partial_sigs.len() + psbt.inputs[0].tap_script_sigs.len(),
        ecdsa_before + taproot_before + 1
    );
}

fn multisig_spend(
    threshold: u32,
    xpub_count: usize,
    script_type: MultisigScriptType,
    cosigner_signatures: usize,
) -> MultisigSpend {
    assert!(threshold as usize <= xpub_count);
    assert!(cosigner_signatures < threshold as usize);

    let secp = secp();
    let account = match script_type {
        MultisigScriptType::P2wsh => path("m/48'/1'/0'/2'"),
        MultisigScriptType::P2wshP2sh => path("m/48'/1'/0'/1'"),
    };
    let our_xpub = simulator_xpub_at(&secp, &account);
    let cosigner_roots = (1..xpub_count)
        .map(|index| seeded_xpriv(u8::try_from(index).unwrap()))
        .collect::<Vec<_>>();
    let cosigner_xpubs = cosigner_roots
        .iter()
        .map(|root| account_xpub(root, &account))
        .collect::<Vec<_>>();

    let mut descriptor_keys = vec![format!(
        "[{}/{}]{our_xpub}/<0;1>/*",
        simulator_xprv().fingerprint(&secp),
        account
    )];
    descriptor_keys.extend(
        cosigner_roots
            .iter()
            .zip(&cosigner_xpubs)
            .map(|(root, xpub)| descriptor_key(root, &account, *xpub, "<0;1>")),
    );
    let sortedmulti = format!("sortedmulti({threshold},{})", descriptor_keys.join(","));
    let descriptor = match script_type {
        MultisigScriptType::P2wsh => format!("wsh({sortedmulti})"),
        MultisigScriptType::P2wshP2sh => format!("sh(wsh({sortedmulti}))"),
    };
    let mut psbt = descriptor_psbt(&descriptor, 0);
    for root in cosigner_roots.iter().take(cosigner_signatures) {
        add_cosigner_signature(&mut psbt, root);
    }

    let config = ScriptConfig::Multisig {
        threshold,
        xpubs: std::iter::once(our_xpub.to_string())
            .chain(cosigner_xpubs.iter().map(ToString::to_string))
            .collect(),
        our_xpub_index: 0,
        script_type,
    };
    let input_path = path(&format!("{account}/0/0"));
    MultisigSpend {
        psbt,
        config,
        account,
        input_pubkey: simulator_xpub_at(&secp, &input_path).public_key,
    }
}

fn multisig_vector(
    id: &str,
    description: &str,
    threshold: u32,
    xpub_count: usize,
    script_type: MultisigScriptType,
    cosigner_signatures: usize,
    name: Option<&str>,
) -> TestVector {
    let spend = multisig_spend(threshold, xpub_count, script_type, cosigner_signatures);
    let success = name.is_some();
    let mut vector = transaction_vector(
        id,
        description,
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: spend.config.clone(),
                keypath: keypath(&spend.account),
            }),
            ..Default::default()
        },
        success
            .then(|| ecdsa_signature(0, spend.input_pubkey))
            .into_iter()
            .collect(),
        match name {
            Some(name) => screens::multisig(threshold, xpub_count, name),
            None => screens::always_invalid_input(),
        },
    );
    if let Some(name) = name {
        vector.registrations.push(Registration {
            script_config: spend.config,
            keypath: Some(keypath(&spend.account)),
            name: name.into(),
        });
    }
    vector
}

fn policy_spend(policy: &str, change_index: u32, sign_cosigner: bool) -> PolicySpend {
    let secp = secp();
    let account = path(POLICY_ACCOUNT);
    let our_xpub = simulator_xpub_at(&secp, &account);
    let cosigner_root = seeded_xpriv(100);
    let cosigner_xpub = account_xpub(&cosigner_root, &account);
    let descriptor = policy
        .replace("/**", "/<0;1>/*")
        .replace(
            "@0",
            &format!(
                "[{}/{}]{our_xpub}",
                simulator_xprv().fingerprint(&secp),
                account
            ),
        )
        .replace(
            "@1",
            &format!(
                "[{}/{}]{cosigner_xpub}",
                cosigner_root.fingerprint(&secp),
                account
            ),
        );
    let mut psbt = descriptor_psbt(&descriptor, change_index);
    if sign_cosigner {
        add_cosigner_signature(&mut psbt, &cosigner_root);
    }
    let config = ScriptConfig::Policy {
        policy: policy.into(),
        keys: vec![
            KeyOriginInfo {
                root_fingerprint: Some(simulator_xprv().fingerprint(&secp).to_string()),
                keypath: Some(keypath(&account)),
                xpub: our_xpub.to_string(),
            },
            KeyOriginInfo {
                root_fingerprint: None,
                keypath: None,
                xpub: cosigner_xpub.to_string(),
            },
        ],
    };
    let input_branch = if policy.contains("<10;11>") { 10 } else { 0 };
    let input_path = path(&format!("{account}/{input_branch}/0"));
    PolicySpend {
        psbt,
        config,
        account,
        input_pubkey: simulator_xpub_at(&secp, &input_path).public_key,
        display_keys: vec![
            format!("This device: {}", screens::DEVICE_POLICY_XPUB),
            cosigner_xpub.to_string(),
        ],
    }
}

fn policy_vector(
    id: &str,
    description: &str,
    policy: &str,
    name: &str,
    spend: PolicySpend,
    account_override: Option<DerivationPath>,
    success: bool,
) -> TestVector {
    let config_account = account_override.as_ref().unwrap_or(&spend.account);
    let prefix = screens::policy_prefix(policy, name, &spend.display_keys);
    let mut vector = transaction_vector(
        id,
        description,
        Coin::Tbtc,
        spend.psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: spend.config.clone(),
                keypath: keypath(config_account),
            }),
            ..Default::default()
        },
        success
            .then(|| ecdsa_signature(0, spend.input_pubkey))
            .into_iter()
            .collect(),
        if success {
            screens::policy(policy, name, &spend.display_keys, false)
        } else {
            screens::always_invalid_input_with_screens(prefix)
        },
    );
    vector.registrations.push(Registration {
        script_config: spend.config,
        keypath: None,
        name: name.into(),
    });
    vector
}

fn unspendable_xpub(cosigner: Xpub, ours: Xpub, script_key_pair_repetitions: usize) -> Xpub {
    let mut keys = Vec::with_capacity(66 * script_key_pair_repetitions);
    for _ in 0..script_key_pair_repetitions {
        keys.extend_from_slice(&cosigner.public_key.serialize());
        keys.extend_from_slice(&ours.public_key.serialize());
    }
    Xpub {
        network: NetworkKind::Test,
        depth: 0,
        parent_fingerprint: Default::default(),
        child_number: ChildNumber::from_normal_idx(0).unwrap(),
        public_key: "0250929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0"
            .parse()
            .unwrap(),
        chain_code: ChainCode::from(sha256::Hash::hash(&keys).to_byte_array()),
    }
}

fn policy_tr_keyspend_with_script_tree() -> TestVector {
    let policy = "tr(@0/**,pk(@1/**))";
    let name = "test tr tweaked keyspend";
    let PolicySpend {
        psbt,
        config,
        account,
        input_pubkey,
        display_keys,
    } = policy_spend(policy, 0, false);
    let mut vector = transaction_vector(
        "policy-tr-keyspend-with-script-tree",
        "Signs a Taproot policy key path whose tweak commits to a script tree.",
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: config.clone(),
                keypath: keypath(&account),
            }),
            ..Default::default()
        },
        vec![taproot_key_signature(0, input_pubkey.x_only_public_key().0)],
        screens::policy(policy, name, &display_keys, true),
    );
    vector.registrations.push(Registration {
        script_config: config,
        keypath: None,
        name: name.into(),
    });
    vector
}

fn taproot_script_key(
    origins: &BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)>,
    fingerprint: Fingerprint,
    keypath: &DerivationPath,
) -> (XOnlyPublicKey, TapLeafHash) {
    let (pubkey, (leaf_hashes, _)) = origins
        .iter()
        .find(|(_, (_, (candidate_fingerprint, candidate_keypath)))| {
            *candidate_fingerprint == fingerprint && candidate_keypath == keypath
        })
        .unwrap();
    assert_eq!(leaf_hashes.len(), 1);
    (*pubkey, leaf_hashes[0])
}

struct UnspendablePolicy<'a> {
    id: &'a str,
    description: &'a str,
    policy: &'a str,
    name: &'a str,
    cosigner_seed: u8,
    cosigner_account: &'a str,
    display_cosigner_origin: bool,
    script_key_pair_repetitions: usize,
}

fn unspendable_policy_vector(spec: UnspendablePolicy<'_>) -> TestVector {
    let secp = secp();
    let account = path(POLICY_ACCOUNT);
    let cosigner_account = path(spec.cosigner_account);
    let our_xpub = simulator_xpub_at(&secp, &account);
    let cosigner_root = seeded_xpriv(spec.cosigner_seed);
    let cosigner_xpub = account_xpub(&cosigner_root, &cosigner_account);
    let nums_xpub = unspendable_xpub(cosigner_xpub, our_xpub, spec.script_key_pair_repetitions);
    let descriptor = spec
        .policy
        .replace("@0", &nums_xpub.to_string())
        .replace(
            "@1",
            &format!(
                "[{}/{}]{cosigner_xpub}",
                cosigner_root.fingerprint(&secp),
                cosigner_account
            ),
        )
        .replace(
            "@2",
            &format!(
                "[{}/{}]{our_xpub}",
                simulator_xprv().fingerprint(&secp),
                account
            ),
        );
    let mut psbt = descriptor_psbt(&descriptor, 0);

    let input_path = path(&format!("{account}/0/0"));
    let cosigner_input_path = path(&format!("{cosigner_account}/0/0"));
    let our_fingerprint = simulator_xprv().fingerprint(&secp);
    let cosigner_fingerprint = cosigner_root.fingerprint(&secp);
    let (input_pubkey, leaf_hash) = taproot_script_key(
        &psbt.inputs[0].tap_key_origins,
        our_fingerprint,
        &input_path,
    );
    let (cosigner_input_pubkey, cosigner_leaf_hash) = taproot_script_key(
        &psbt.inputs[0].tap_key_origins,
        cosigner_fingerprint,
        &cosigner_input_path,
    );
    assert_eq!(leaf_hash, cosigner_leaf_hash);

    psbt.sign(&cosigner_root, &secp).unwrap();
    assert!(
        psbt.inputs[0]
            .tap_script_sigs
            .contains_key(&(cosigner_input_pubkey, leaf_hash))
    );
    psbt.inputs[0]
        .tap_script_sigs
        .retain(|key, _| key == &(cosigner_input_pubkey, leaf_hash));

    let output_path = path(&format!("{account}/1/0"));
    let cosigner_output_path = path(&format!("{cosigner_account}/1/0"));
    for (origins, our_path, cosigner_path) in [
        (
            &mut psbt.inputs[0].tap_key_origins,
            &input_path,
            &cosigner_input_path,
        ),
        (
            &mut psbt.outputs[0].tap_key_origins,
            &output_path,
            &cosigner_output_path,
        ),
    ] {
        origins.retain(|_, (_, (fingerprint, keypath))| {
            (*fingerprint != our_fingerprint || keypath == our_path)
                && (*fingerprint != cosigner_fingerprint || keypath == cosigner_path)
        });
    }

    let cosigner_origin = format!("[{cosigner_fingerprint}/{cosigner_account}]{cosigner_xpub}");
    let config = ScriptConfig::Policy {
        policy: spec.policy.into(),
        keys: vec![
            KeyOriginInfo {
                root_fingerprint: None,
                keypath: None,
                xpub: nums_xpub.to_string(),
            },
            KeyOriginInfo {
                root_fingerprint: spec
                    .display_cosigner_origin
                    .then(|| cosigner_fingerprint.to_string()),
                keypath: spec
                    .display_cosigner_origin
                    .then(|| keypath(&cosigner_account)),
                xpub: cosigner_xpub.to_string(),
            },
            KeyOriginInfo {
                root_fingerprint: Some(our_fingerprint.to_string()),
                keypath: Some(keypath(&account)),
                xpub: our_xpub.to_string(),
            },
        ],
    };
    let display_keys = vec![
        format!("Provably unspendable: {nums_xpub}"),
        if spec.display_cosigner_origin {
            cosigner_origin
        } else {
            cosigner_xpub.to_string()
        },
        format!("This device: {}", screens::DEVICE_POLICY_XPUB),
    ];
    let mut vector = transaction_vector(
        spec.id,
        spec.description,
        Coin::Tbtc,
        psbt,
        PsbtSignOptions {
            force_script_config: Some(ScriptConfigWithKeypath {
                script_config: config.clone(),
                keypath: keypath(&account),
            }),
            ..Default::default()
        },
        vec![taproot_script_signature(0, input_pubkey, leaf_hash)],
        screens::policy(spec.policy, spec.name, &display_keys, true),
    );
    vector.registrations.push(Registration {
        script_config: config,
        keypath: None,
        name: spec.name.into(),
    });
    vector
}

fn policy_tr_unspendable_internal_key() -> TestVector {
    unspendable_policy_vector(UnspendablePolicy {
        id: "policy-tr-unspendable-internal-key",
        description: "Signs a Taproot script path with a provably unspendable internal key and displays that property in the policy review.",
        policy: "tr(@0/<0;1>/*,multi_a(2,@1/<0;1>/*,@2/<0;1>/*))",
        name: "test unspendable policy",
        cosigner_seed: 101,
        cosigner_account: POLICY_ACCOUNT,
        display_cosigner_origin: false,
        script_key_pair_repetitions: 1,
    })
}

fn policy_tr_unspendable_internal_key_complex() -> TestVector {
    unspendable_policy_vector(UnspendablePolicy {
        id: "policy-tr-unspendable-internal-key-complex",
        description: "Signs the satisfiable branch of a multi-leaf Taproot policy with a provably unspendable internal key, distinct multipaths and a relative-timelock sibling branch.",
        policy: "tr(@0/<0;1>/*,{and_v(v:multi_a(1,@1/<2;3>/*,@2/<2;3>/*),older(2)),multi_a(2,@1/<0;1>/*,@2/<0;1>/*)})",
        name: "test complex unspendable",
        cosigner_seed: 102,
        cosigner_account: "m/48'/1'/0'/2'",
        display_cosigner_origin: true,
        script_key_pair_repetitions: 2,
    })
}

pub fn all() -> Vec<TestVector> {
    let multipath_policy = "wsh(multi(2,@0/<10;11>/*,@1/<20;21>/*))";
    let standard_policy = "wsh(multi(2,@0/**,@1/**))";
    vec![
        multisig_vector(
            "multisig-not-registered",
            "Rejects a valid 1-of-2 P2WSH multisig transaction when its account has not been registered.",
            1,
            2,
            MultisigScriptType::P2wsh,
            0,
            None,
        ),
        multisig_vector(
            "multisig-p2wsh-p2sh",
            "Signs and finalizes a registered 1-of-2 nested P2SH-P2WSH multisig transaction.",
            1,
            2,
            MultisigScriptType::P2wshP2sh,
            0,
            Some("test sh-wsh multisig"),
        ),
        multisig_vector(
            "multisig-large",
            "Adds the seventh signature to a registered 7-of-15 P2WSH multisig transaction whose PSBT already contains six cosigner signatures.",
            7,
            15,
            MultisigScriptType::P2wsh,
            6,
            Some("test large multisig"),
        ),
        policy_tr_keyspend_with_script_tree(),
        policy_tr_unspendable_internal_key(),
        policy_tr_unspendable_internal_key_complex(),
        policy_vector(
            "policy-different-multipath-derivations",
            "Signs and finalizes a registered WSH policy whose two keys use different receive and change branches.",
            multipath_policy,
            "test multipath policy",
            policy_spend(multipath_policy, 0, true),
            None,
            true,
        ),
        policy_vector(
            "policy-wrong-account-keypath",
            "Rejects a registered policy when the signing account keypath does not match the device key in the policy.",
            standard_policy,
            "test policy account",
            policy_spend(standard_policy, 0, false),
            Some(path("m/48'/1'/0'/4'")),
            false,
        ),
        policy_vector(
            "policy-change-index-too-high",
            "Rejects a registered policy change output at address index 10000.",
            standard_policy,
            "test policy account",
            policy_spend(standard_policy, 10_000, false),
            None,
            false,
        ),
    ]
}
