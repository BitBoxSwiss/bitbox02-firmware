// SPDX-License-Identifier: Apache-2.0

//! Readable source and deterministic generator for Bitcoin transaction test vectors.

mod cases;

use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GENERATED_FILENAME: &str = "btc-transaction-test-vectors.json";

/// BIP32 xprv derived from the mnemonic restored by all client simulator tests:
/// boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple
/// enact box walk height pull today solid off enable tide
const SIMULATOR_BIP32_XPRV: &str = "xprv9s21ZrQH143K2qxpAMxVdyeza5dUBxY11XbJ7eKvRF51sQyhiFXgmn4P4ALi3Nf6bcG8cmPDvMMEFiAVjtXsqeZ47PJfBJif7uSYycMsx9c";
const SIMULATOR_SEED: &str = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";

#[derive(Debug, Serialize, Deserialize)]
pub struct TestVectorFile {
    pub simulator_seed: String,
    pub vectors: Vec<TestVector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestVector {
    pub id: String,
    pub description: String,
    pub coin: Coin,
    pub psbt: PsbtForm,
    pub expected_needs_prevtxs: bool,
    pub expectations: Vec<VersionExpectation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registrations: Vec<Registration>,
    /// Signature slots that signing must newly insert. Signature bytes are deliberately omitted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expected_signatures: Vec<ExpectedSignature>,
    /// Output scripts that successful signing must generate, keyed by output index.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub expected_generated_outputs: BTreeMap<usize, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PsbtForm {
    pub transaction: String,
    #[serde(default, skip_serializing_if = "PsbtSignOptions::is_empty")]
    pub options: PsbtSignOptions,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Coin {
    Btc,
    Tbtc,
    Ltc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionExpectation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version_exclusive: Option<String>,
    pub outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_version: Option<String>,
    pub screens: Vec<Screen>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Success,
    Unsupported,
    InvalidInput,
}

/// Screens expected from the firmware UI. Released simulator stdout omits `longtouch`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Screen {
    Confirm {
        title: String,
        body: String,
        longtouch: bool,
    },
    TransactionAddress {
        amount: String,
        address: String,
    },
    TransactionFee {
        amount: String,
        fee: String,
        longtouch: bool,
    },
    Status {
        title: String,
        body: String,
    },
    Swap {
        title: String,
        from: String,
        to: String,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PsbtSignOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_script_config: Option<ScriptConfigWithKeypath>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub outputs: BTreeMap<usize, PsbtOutputOptions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_requests: Vec<PaymentRequest>,
    #[serde(default, skip_serializing_if = "FormatUnit::is_default")]
    pub format_unit: FormatUnit,
}

impl PsbtSignOptions {
    fn is_empty(&self) -> bool {
        self.force_script_config.is_none()
            && self.outputs.is_empty()
            && self.payment_requests.is_empty()
            && self.format_unit == FormatUnit::Default
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsbtOutputOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_payment_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_request_index: Option<u32>,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormatUnit {
    #[default]
    Default,
    Sat,
}

impl FormatUnit {
    fn is_default(&self) -> bool {
        *self == Self::Default
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub recipient_name: String,
    pub total_amount: u64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nonce: String,
    pub memos: Vec<PaymentRequestMemo>,
    pub signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentRequestMemo {
    Text {
        note: String,
    },
    CoinPurchase {
        coin_type: u32,
        amount: String,
        address: String,
        address_keypath: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScriptConfigWithKeypath {
    pub script_config: ScriptConfig,
    pub keypath: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScriptConfig {
    Simple {
        script_type: SimpleType,
    },
    Multisig {
        threshold: u32,
        xpubs: Vec<String>,
        our_xpub_index: u32,
        script_type: MultisigScriptType,
    },
    Policy {
        policy: String,
        keys: Vec<KeyOriginInfo>,
    },
}

impl ScriptConfig {
    fn is_taproot(&self) -> bool {
        match self {
            Self::Simple {
                script_type: SimpleType::P2tr,
            } => true,
            Self::Policy { policy, .. } => policy.starts_with("tr("),
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimpleType {
    P2wpkh,
    P2wpkhP2sh,
    P2tr,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultisigScriptType {
    P2wsh,
    P2wshP2sh,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct KeyOriginInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypath: Option<String>,
    pub xpub: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Registration {
    pub script_config: ScriptConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keypath: Option<String>,
    pub name: String,
}

#[derive(Debug)]
pub struct FirmwareSignRequest {
    pub script_configs: Vec<ScriptConfigWithKeypath>,
    pub output_script_configs: Vec<ScriptConfigWithKeypath>,
    pub version: u32,
    pub inputs: Vec<FirmwareInput>,
    pub outputs: Vec<FirmwareOutput>,
    pub locktime: u32,
    pub payment_requests: Vec<PaymentRequest>,
    pub format_unit: FormatUnit,
}

impl FirmwareSignRequest {
    fn needs_prevtxs(&self) -> bool {
        self.script_configs
            .iter()
            .any(|config| !config.script_config.is_taproot())
    }
}

#[derive(Debug)]
pub struct FirmwareInput {
    pub prev_out_hash: bitcoin::Txid,
    pub prev_out_index: u32,
    pub prev_out_value: u64,
    pub sequence: u32,
    pub keypath: bitcoin::bip32::DerivationPath,
    pub script_config_index: u32,
    pub prev_tx: Option<bitcoin::Transaction>,
    pub bip352_pubkey: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct FirmwareOutput {
    pub ours: bool,
    pub value: u64,
    pub output_type: OutputType,
    pub payload: Vec<u8>,
    pub keypath: Option<bitcoin::bip32::DerivationPath>,
    pub script_config_index: u32,
    pub output_script_config_index: Option<u32>,
    pub silent_payment_address: Option<String>,
    pub payment_request_index: Option<u32>,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputType {
    Unknown,
    P2pkh,
    P2sh,
    P2wpkh,
    P2wsh,
    P2tr,
    OpReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedSignature {
    pub input_index: usize,
    pub kind: SignatureKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash: Option<String>,
    pub sighash: Sighash,
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureKind {
    Ecdsa,
    TaprootKey,
    TaprootScript,
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sighash {
    All,
    Default,
}

/// Validate invariants that execution alone cannot establish.
fn validate(file: &TestVectorFile) -> Result<(), String> {
    if file.vectors.is_empty() {
        return Err("test vector file contains no vectors".into());
    }

    let mut ids = BTreeSet::new();
    for vector in &file.vectors {
        let id = vector.id.trim();
        if id.is_empty() {
            return Err("test vector has an empty id".into());
        }
        if !ids.insert(id) {
            return Err(format!("duplicate test vector id '{id}'"));
        }
        validate_version_expectations(id, &vector.expectations)?;
    }
    Ok(())
}

fn parse_psbt(vector: &TestVector) -> Result<bitcoin::psbt::Psbt, String> {
    let id = &vector.id;
    let serialized = hex::decode(&vector.psbt.transaction)
        .map_err(|err| format!("test vector '{id}' contains invalid PSBT hex: {err}"))?;
    bitcoin::psbt::Psbt::deserialize(&serialized)
        .map_err(|err| format!("test vector '{id}' contains an invalid PSBT: {err}"))
}

/// Derive the firmware signing request represented by a test vector.
pub fn derive_sign_request(vector: &TestVector) -> Result<FirmwareSignRequest, String> {
    let psbt = parse_psbt(vector)?;
    cases::firmware_request_from_psbt(&psbt, &vector.psbt.options).map_err(|err| {
        format!(
            "test vector '{}' cannot derive its firmware signing request: {err}",
            vector.id
        )
    })
}

fn validate_version_expectations(
    vector_id: &str,
    expectations: &[VersionExpectation],
) -> Result<(), String> {
    if expectations.is_empty() {
        return Err(format!("test vector '{vector_id}' has no expectations"));
    }

    let mut previous_max = None;
    for (index, expectation) in expectations.iter().enumerate() {
        let min =
            parse_version_bound(vector_id, "min_version", expectation.min_version.as_deref())?;
        let max = parse_version_bound(
            vector_id,
            "max_version_exclusive",
            expectation.max_version_exclusive.as_deref(),
        )?;
        if min != previous_max {
            return Err(format!(
                "test vector '{vector_id}' has a gap or unordered version expectations"
            ));
        }
        if let (Some(min), Some(max)) = (&min, &max)
            && min >= max
        {
            return Err(format!(
                "test vector '{vector_id}' has an empty or reversed version range [{min}, {max})"
            ));
        }
        if index + 1 < expectations.len() && max.is_none() {
            return Err(format!(
                "test vector '{vector_id}' has an unbounded non-final expectation"
            ));
        }

        previous_max = max;
    }

    if previous_max.is_some() {
        return Err(format!(
            "test vector '{vector_id}' version expectations do not cover all versions"
        ));
    }
    Ok(())
}

fn parse_version_bound(
    vector_id: &str,
    field: &str,
    value: Option<&str>,
) -> Result<Option<Version>, String> {
    value
        .map(|value| {
            Version::parse(value).map_err(|err| {
                format!("test vector '{vector_id}' has invalid {field} '{value}': {err}")
            })
        })
        .transpose()
}

pub fn test_vectors() -> TestVectorFile {
    TestVectorFile {
        simulator_seed: SIMULATOR_SEED.into(),
        vectors: cases::all(),
    }
}

pub fn try_generate_json() -> Result<String, String> {
    let file = test_vectors();
    validate(&file)?;
    let mut result = serde_json::to_string_pretty(&file).map_err(|err| err.to_string())?;
    result.push('\n');
    Ok(result)
}

#[cfg(test)]
mod tests {
    fn assert_invalid(update: impl FnOnce(&mut super::TestVectorFile), expected: &str) {
        let mut file = super::test_vectors();
        update(&mut file);
        let error = super::validate(&file).unwrap_err();
        assert!(
            error.contains(expected),
            "expected validation error containing '{expected}', got '{error}'"
        );
    }

    #[test]
    fn test_generated_vectors_are_current() {
        let expected = include_str!("../../testdata/btc-transaction-test-vectors.json");
        assert_eq!(super::try_generate_json().unwrap(), expected);
    }

    #[test]
    fn test_validate_vector_identity() {
        assert_invalid(|file| file.vectors[0].id.clear(), "empty id");
        assert_invalid(
            |file| file.vectors[1].id = file.vectors[0].id.clone(),
            "duplicate test vector id",
        );
    }

    #[test]
    fn test_validate_version_expectations() {
        assert_invalid(
            |file| {
                file.vectors[0].expectations[1].min_version = Some("99.0.0".into());
            },
            "gap or unordered version expectations",
        );
    }
}
