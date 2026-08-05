// SPDX-License-Identifier: Apache-2.0

use crate::btc_transaction::{
    Coin, ExpectedSignature, FirmwareInput, FirmwareOutput, FirmwareSignRequest, OutputType,
    PsbtForm, PsbtSignOptions, ScriptConfig, ScriptConfigWithKeypath, Sighash, SignatureKind,
    SimpleType, TestVector, VersionExpectation,
};
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::key::TapTweak;
use bitcoin::script::Instruction;
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::{ScriptBuf, TxOut};

use std::collections::BTreeMap;

pub const SOME_XPUB: &str = "tpubDFgycCkexSxkdZfeyaasDHityE97kiYM1BeCNoivDHvydGugKtoNobt4vEX6YSHNPy2cqmWQHKjKxciJuocepsGPGxcDZVmiMBnxgA1JKQk";

pub fn secp() -> Secp256k1<secp256k1::All> {
    Secp256k1::new()
}

pub fn simulator_xprv() -> Xpriv {
    crate::btc_transaction::SIMULATOR_BIP32_XPRV
        .parse()
        .unwrap()
}

pub fn simulator_xpub_at<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    path: &DerivationPath,
) -> Xpub {
    Xpub::from_priv(secp, &simulator_xprv().derive_priv(secp, path).unwrap())
}

pub fn keypath(path: &DerivationPath) -> String {
    format!("m/{path}")
}

pub fn transaction_vector(
    id: &str,
    description: &str,
    coin: Coin,
    psbt: bitcoin::psbt::Psbt,
    psbt_options: PsbtSignOptions,
    expected_signatures: Vec<ExpectedSignature>,
    expectations: Vec<VersionExpectation>,
) -> TestVector {
    let expected_needs_prevtxs = firmware_request_from_psbt(&psbt, &psbt_options)
        .unwrap()
        .needs_prevtxs();
    TestVector {
        id: id.into(),
        description: description.into(),
        coin,
        psbt: PsbtForm {
            transaction: hex::encode(psbt.serialize()),
            options: psbt_options,
        },
        expected_needs_prevtxs,
        expectations,
        registrations: vec![],
        expected_signatures,
        expected_generated_outputs: BTreeMap::new(),
    }
}

#[derive(Clone, Copy)]
enum OurKey {
    Segwit(bitcoin::secp256k1::PublicKey),
    Taproot(bitcoin::secp256k1::XOnlyPublicKey),
}

impl OurKey {
    fn bip352_pubkey(self, secp: &Secp256k1<secp256k1::All>) -> Vec<u8> {
        match self {
            Self::Segwit(pubkey) => pubkey.serialize().to_vec(),
            // Taproot silent-payment inputs use the tweaked key-spend private key.
            Self::Taproot(pubkey) => pubkey.tap_tweak(secp, None).0.serialize().to_vec(),
        }
    }
}

fn input_our_key(
    input: &bitcoin::psbt::Input,
    fingerprint: bitcoin::bip32::Fingerprint,
) -> Option<(OurKey, DerivationPath)> {
    input
        .tap_key_origins
        .iter()
        .find_map(|(pubkey, (_, (candidate, keypath)))| {
            (*candidate == fingerprint).then(|| (OurKey::Taproot(*pubkey), keypath.clone()))
        })
        .or_else(|| {
            input
                .bip32_derivation
                .iter()
                .find_map(|(pubkey, (candidate, keypath))| {
                    (*candidate == fingerprint).then(|| (OurKey::Segwit(*pubkey), keypath.clone()))
                })
        })
}

fn output_our_key(
    output: &bitcoin::psbt::Output,
    fingerprint: bitcoin::bip32::Fingerprint,
) -> Option<(OurKey, DerivationPath)> {
    output
        .tap_key_origins
        .iter()
        .find_map(|(pubkey, (_, (candidate, keypath)))| {
            (*candidate == fingerprint).then(|| (OurKey::Taproot(*pubkey), keypath.clone()))
        })
        .or_else(|| {
            output
                .bip32_derivation
                .iter()
                .find_map(|(pubkey, (candidate, keypath))| {
                    (*candidate == fingerprint).then(|| (OurKey::Segwit(*pubkey), keypath.clone()))
                })
        })
}

fn account_keypath(path: &DerivationPath) -> Result<String, String> {
    let components = path.as_ref();
    if components.len() < 3 {
        return Err(format!("keypath m/{path} has no account prefix"));
    }
    Ok(keypath(&DerivationPath::from(components[..3].to_vec())))
}

fn script_config_from_utxo(
    output: &TxOut,
    keypath: &DerivationPath,
    redeem_script: Option<&ScriptBuf>,
) -> Result<ScriptConfigWithKeypath, String> {
    let script_type = if output.script_pubkey.is_p2wpkh() {
        SimpleType::P2wpkh
    } else if output.script_pubkey.is_p2sh()
        && redeem_script.is_some_and(|script| script.is_p2wpkh())
    {
        SimpleType::P2wpkhP2sh
    } else if output.script_pubkey.is_p2tr() {
        SimpleType::P2tr
    } else {
        return Err(format!(
            "cannot infer a simple script config for {}",
            output.script_pubkey
        ));
    };
    Ok(ScriptConfigWithKeypath {
        script_config: ScriptConfig::Simple { script_type },
        keypath: account_keypath(keypath)?,
    })
}

fn find_or_add_config(
    configs: &mut Vec<ScriptConfigWithKeypath>,
    config: ScriptConfigWithKeypath,
) -> usize {
    if let Some(index) = configs.iter().position(|candidate| candidate == &config) {
        index
    } else {
        configs.push(config);
        configs.len() - 1
    }
}

fn same_account(
    input_configs: &[ScriptConfigWithKeypath],
    output_config: &ScriptConfigWithKeypath,
) -> Result<bool, String> {
    for input_config in input_configs {
        if matches!(input_config.script_config, ScriptConfig::Simple { .. }) {
            let input_path = input_config
                .keypath
                .parse::<DerivationPath>()
                .map_err(|err| err.to_string())?;
            let output_path = output_config
                .keypath
                .parse::<DerivationPath>()
                .map_err(|err| err.to_string())?;
            if input_path.as_ref().get(2) != output_path.as_ref().get(2) {
                return Ok(false);
            }
        } else if input_config != output_config {
            return Ok(false);
        }
    }
    Ok(true)
}

fn external_output(output: &TxOut) -> Result<(OutputType, Vec<u8>), String> {
    let script = output.script_pubkey.as_bytes();
    if output.script_pubkey.is_p2pkh() {
        Ok((OutputType::P2pkh, script[3..23].to_vec()))
    } else if output.script_pubkey.is_p2sh() {
        Ok((OutputType::P2sh, script[2..22].to_vec()))
    } else if output.script_pubkey.is_p2wpkh() {
        Ok((OutputType::P2wpkh, script[2..].to_vec()))
    } else if output.script_pubkey.is_p2wsh() {
        Ok((OutputType::P2wsh, script[2..].to_vec()))
    } else if output.script_pubkey.is_p2tr() {
        Ok((OutputType::P2tr, script[2..].to_vec()))
    } else if output.script_pubkey.is_op_return() {
        let mut instructions = output.script_pubkey.instructions_minimal();
        match (
            instructions.next(),
            instructions.next(),
            instructions.next(),
        ) {
            (Some(Ok(Instruction::Op(op))), Some(Ok(Instruction::PushBytes(payload))), None)
                if op == bitcoin::opcodes::all::OP_RETURN =>
            {
                Ok((OutputType::OpReturn, payload.as_bytes().to_vec()))
            }
            _ => Err("unsupported OP_RETURN script".into()),
        }
    } else {
        Err(format!(
            "unsupported output script {}",
            output.script_pubkey
        ))
    }
}

pub(in crate::btc_transaction) fn firmware_request_from_psbt(
    psbt: &bitcoin::psbt::Psbt,
    options: &PsbtSignOptions,
) -> Result<FirmwareSignRequest, String> {
    let secp = secp();
    let fingerprint = simulator_xprv().fingerprint(&secp);
    let contains_silent_payment = options
        .outputs
        .values()
        .any(|output| output.silent_payment_address.is_some());
    let mut script_configs = options
        .force_script_config
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let forced_config = options.force_script_config.is_some();
    let mut input_prev_txs = Vec::with_capacity(psbt.inputs.len());
    let mut inputs = Vec::with_capacity(psbt.inputs.len());

    for (index, (tx_input, psbt_input)) in
        psbt.unsigned_tx.input.iter().zip(&psbt.inputs).enumerate()
    {
        let prev_tx = psbt_input.non_witness_utxo.as_ref();
        let utxo = psbt_input.witness_utxo.as_ref().or_else(|| {
            prev_tx.and_then(|tx| tx.output.get(tx_input.previous_output.vout as usize))
        });
        let utxo = utxo.ok_or_else(|| format!("PSBT input {index} has no spend UTXO"))?;
        let (our_key, input_keypath) = input_our_key(psbt_input, fingerprint)
            .ok_or_else(|| format!("PSBT input {index} has no simulator key"))?;
        let script_config_index = if forced_config {
            0
        } else {
            let config =
                script_config_from_utxo(utxo, &input_keypath, psbt_input.redeem_script.as_ref())?;
            find_or_add_config(&mut script_configs, config)
        };
        input_prev_txs.push(prev_tx);
        inputs.push(FirmwareInput {
            prev_out_hash: tx_input.previous_output.txid,
            prev_out_index: tx_input.previous_output.vout,
            prev_out_value: utxo.value.to_sat(),
            sequence: tx_input.sequence.to_consensus_u32(),
            keypath: input_keypath,
            script_config_index: script_config_index as u32,
            prev_tx: None,
            bip352_pubkey: contains_silent_payment.then(|| our_key.bip352_pubkey(&secp)),
        });
    }

    let mut output_script_configs = Vec::new();
    let mut outputs = Vec::with_capacity(psbt.outputs.len());
    for (index, (tx_output, psbt_output)) in psbt
        .unsigned_tx
        .output
        .iter()
        .zip(&psbt.outputs)
        .enumerate()
    {
        let output_options = options.outputs.get(&index);
        let silent_payment_address =
            output_options.and_then(|output| output.silent_payment_address.clone());
        let payment_request_index = output_options.and_then(|output| output.payment_request_index);
        if let Some((_, output_keypath)) = output_our_key(psbt_output, fingerprint) {
            let config = if let Some(config) = &options.force_script_config {
                config.clone()
            } else {
                script_config_from_utxo(
                    tx_output,
                    &output_keypath,
                    psbt_output.redeem_script.as_ref(),
                )?
            };
            let (script_config_index, output_script_config_index) =
                if same_account(&script_configs, &config)? {
                    (find_or_add_config(&mut script_configs, config) as u32, None)
                } else {
                    let index = find_or_add_config(&mut output_script_configs, config);
                    (0, Some(index as u32))
                };
            outputs.push(FirmwareOutput {
                ours: true,
                value: tx_output.value.to_sat(),
                output_type: OutputType::Unknown,
                payload: Vec::new(),
                keypath: Some(output_keypath),
                script_config_index,
                output_script_config_index,
                silent_payment_address,
                payment_request_index,
            });
        } else {
            let (output_type, payload) =
                if silent_payment_address.is_some() && !tx_output.script_pubkey.is_op_return() {
                    (OutputType::Unknown, Vec::new())
                } else {
                    external_output(tx_output)?
                };
            outputs.push(FirmwareOutput {
                ours: false,
                value: tx_output.value.to_sat(),
                output_type,
                payload,
                keypath: None,
                script_config_index: 0,
                output_script_config_index: None,
                silent_payment_address,
                payment_request_index,
            });
        }
    }

    let needs_prevtxs = script_configs
        .iter()
        .any(|config| !config.script_config.is_taproot());
    if needs_prevtxs {
        for (index, prev_tx) in input_prev_txs.into_iter().enumerate() {
            inputs[index].prev_tx = Some(
                prev_tx
                    .ok_or_else(|| format!("PSBT input {index} needs a non-witness UTXO"))?
                    .clone(),
            );
        }
    }

    Ok(FirmwareSignRequest {
        script_configs,
        output_script_configs,
        version: psbt.unsigned_tx.version.0 as u32,
        inputs,
        outputs,
        locktime: psbt.unsigned_tx.lock_time.to_consensus_u32(),
        payment_requests: options.payment_requests.clone(),
        format_unit: options.format_unit,
    })
}

pub fn ecdsa_signature(
    input_index: usize,
    pubkey: bitcoin::secp256k1::PublicKey,
) -> ExpectedSignature {
    ExpectedSignature {
        input_index,
        kind: SignatureKind::Ecdsa,
        pubkey: Some(pubkey.to_string()),
        leaf_hash: None,
        sighash: Sighash::All,
    }
}

pub fn taproot_key_signature(
    input_index: usize,
    pubkey: bitcoin::secp256k1::XOnlyPublicKey,
) -> ExpectedSignature {
    ExpectedSignature {
        input_index,
        kind: SignatureKind::TaprootKey,
        pubkey: Some(pubkey.to_string()),
        leaf_hash: None,
        sighash: Sighash::Default,
    }
}

pub fn taproot_script_signature(
    input_index: usize,
    pubkey: bitcoin::secp256k1::XOnlyPublicKey,
    leaf_hash: bitcoin::TapLeafHash,
) -> ExpectedSignature {
    ExpectedSignature {
        input_index,
        kind: SignatureKind::TaprootScript,
        pubkey: Some(pubkey.to_string()),
        leaf_hash: Some(leaf_hash.to_string()),
        sighash: Sighash::Default,
    }
}
