// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;
use crate::hal::ui::{ConfirmParams, Progress};

use super::super::payment_request;
use super::common::format_amount;
use super::policies::TaprootSpendInfo;
use super::script_configs::{ValidatedScriptConfig, ValidatedScriptConfigWithKeypath};
use super::{bip143, bip341, common, keypath};

use crate::hal::Ui;
use crate::keystore::Compute;
use crate::secp256k1::SECP256K1;
use crate::workflow::transaction;
use crate::xpubcache::Bip32XpubCache;

use alloc::string::String;
use alloc::vec::Vec;

use pb::request::Request;
use pb::response::Response;

use pb::btc_script_config::SimpleType;
use pb::btc_sign_init_request::FormatUnit;
use pb::btc_sign_next_response::Type as NextType;
use sha2::{Digest, Sha256};

use bitcoin::consensus::encode::{VarInt, serialize};
use bitcoin::hashes::Hash;
use bitcoin::key::TapTweak;

use streaming_silent_payments::SilentPayment;

/// After each request from the host, we send a `BtcSignNextResponse` response back to the host,
/// containing information which request we want next, and containing additional metadata if
/// available (e.g. a signature after signing an input).
struct NextResponse {
    next: pb::BtcSignNextResponse,
    /// If true, `next` is wrapped in the `BTCResponse` protobuf message, otherwise it is sent
    /// directly in a `Response` message.
    wrap: bool,
}

impl NextResponse {
    fn to_protobuf(&self) -> Response {
        if self.wrap {
            Response::Btc(pb::BtcResponse {
                response: Some(pb::btc_response::Response::SignNext(self.next.clone())),
            })
        } else {
            Response::BtcSignNext(self.next.clone())
        }
    }
}

/// Wait for the next request sent by the host. Since host<->device communication is a
/// request/response pattern, we have to send a response (to the previous request) before getting
/// the next request.
///
/// In BTC signing, the response is always a `BtcSignNextResponse`, but depending on the previous
/// request, it is either a direct response result (hww.proto:Response), or a a result wrapped in a
/// `BTCResponse` (which was introduced latter, hence the messages are scattered). `response.wrap`
/// is set so the next call to this function wraps the response correctly.
///
/// The NextResponse contains information for the host as to which request we need, but also
/// additional results, e.g. a signature after an input is signed. The response is reset to default
/// values after this call so that this additional data is only sent once.
async fn get_request(
    typ: NextType,
    index: u32,
    prev_index: Option<u32>,
    response: &mut NextResponse,
) -> Result<Request, Error> {
    response.next.r#type = typ as _;
    response.next.index = index;
    if let Some(prev_index) = prev_index {
        response.next.prev_index = prev_index;
    }
    let request = crate::hww::next_request(response.to_protobuf()).await?;
    response.next = Default::default();
    Ok(request)
}

async fn get_tx_input(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcSignInputRequest, Error> {
    let request = get_request(NextType::Input, index, None, response).await?;
    response.wrap = false;
    match request {
        Request::BtcSignInput(request) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_init(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxInitRequest, Error> {
    response.next.r#type = NextType::PrevtxInit as _;
    response.next.index = index;
    let request = get_request(NextType::PrevtxInit, index, None, response).await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxInit(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_input(
    input_index: u32,
    prevtx_input_index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxInputRequest, Error> {
    let request = get_request(
        NextType::PrevtxInput,
        input_index,
        Some(prevtx_input_index),
        response,
    )
    .await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxInput(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_output(
    output_index: u32,
    prevtx_output_index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxOutputRequest, Error> {
    let request = get_request(
        NextType::PrevtxOutput,
        output_index,
        Some(prevtx_output_index),
        response,
    )
    .await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxOutput(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_tx_output(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcSignOutputRequest, Error> {
    let request = get_request(NextType::Output, index, None, response).await?;
    response.wrap = false;
    match request {
        Request::BtcSignOutput(request) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_payment_request(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPaymentRequestRequest, Error> {
    let request = get_request(NextType::PaymentRequest, index, None, response).await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PaymentRequest(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_antiklepto_host_nonce(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::AntiKleptoSignatureRequest, Error> {
    let request = get_request(NextType::HostNonce, index, None, response).await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::AntikleptoSignature(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

/// Validates swap-specific source account constraints after a CoinPurchaseMemo was detected.
/// Coin must be BTC/LTC and all selected source configs must be single-sig.
#[cfg(feature = "app-ethereum")]
fn validate_swap_source_account(
    coin: pb::BtcCoin,
    script_configs: &[ValidatedScriptConfigWithKeypath],
) -> Result<(), Error> {
    match coin {
        pb::BtcCoin::Btc | pb::BtcCoin::Ltc => {}
        _ => return Err(Error::InvalidInput),
    }

    if script_configs.is_empty() {
        return Err(Error::InvalidInput);
    }

    for script_config in script_configs {
        match script_config {
            ValidatedScriptConfigWithKeypath {
                config: ValidatedScriptConfig::SimpleType(_),
                ..
            } => {}
            _ => return Err(Error::InvalidInput),
        }
    }

    Ok(())
}

/// CoinPurchaseMemo-backed swaps require Ethereum support.
#[cfg(not(feature = "app-ethereum"))]
fn validate_swap_source_account(
    _coin: pb::BtcCoin,
    _script_configs: &[ValidatedScriptConfigWithKeypath],
) -> Result<(), Error> {
    Err(Error::Disabled)
}

fn validate_keypath(
    params: &super::params::Params,
    script_config_account: &ValidatedScriptConfigWithKeypath<'_>,
    keypath: &[u32],
    mode: keypath::ReceiveSpend,
) -> Result<(), Error> {
    match &script_config_account.config {
        ValidatedScriptConfig::SimpleType(simple_type) => {
            keypath::validate_address_simple(
                keypath,
                params.bip44_coin,
                *simple_type,
                params.taproot_support,
                mode,
            )
            .or(Err(Error::InvalidInput))?;
        }
        ValidatedScriptConfig::Multisig { .. } | ValidatedScriptConfig::Policy { .. } => {
            keypath::validate_address_policy(keypath, mode).or(Err(Error::InvalidInput))?;
        }
    }
    // Check that keypath_account is a prefix to keypath with two elements left (change, address).
    if script_config_account.keypath.len() + 2 != keypath.len() {
        return Err(Error::InvalidInput);
    }
    if &keypath[..script_config_account.keypath.len()] != script_config_account.keypath {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn validate_input(
    input: &pb::BtcSignInputRequest,
    params: &super::params::Params,
    script_config_account: &ValidatedScriptConfigWithKeypath,
) -> Result<(), Error> {
    if input.prev_out_value == 0 {
        return Err(Error::InvalidInput);
    }
    validate_keypath(
        params,
        script_config_account,
        &input.keypath,
        keypath::ReceiveSpend::Spend,
    )
}

fn is_taproot(script_config_account: &ValidatedScriptConfigWithKeypath) -> bool {
    matches!(
        script_config_account.config,
        ValidatedScriptConfig::SimpleType(SimpleType::P2tr)
            | ValidatedScriptConfig::Policy {
                parsed_policy: super::policies::ParsedPolicy {
                    descriptor: super::policies::Descriptor::Tr(_),
                    ..
                },
                ..
            }
    )
}

/// Generates the subscript (scriptCode without the length prefix) used in the bip143 sighash algo.
///
/// See https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification, item 5:
async fn sighash_script(
    hal: &mut impl crate::hal::Hal,
    xpub_cache: &mut Bip32XpubCache,
    script_config_account: &ValidatedScriptConfigWithKeypath<'_>,
    keypath: &[u32],
) -> Result<Vec<u8>, Error> {
    match script_config_account {
        ValidatedScriptConfigWithKeypath {
            config: ValidatedScriptConfig::SimpleType(simple_type),
            ..
        } => {
            match simple_type {
                SimpleType::P2wpkhP2sh | SimpleType::P2wpkh => {
                    // See https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification, item 5:
                    // > For P2WPKH witness program, the scriptCode is 0x1976a914{20-byte-pubkey-hash}88ac.
                    let pubkey_hash160 = xpub_cache.get_xpub(hal, keypath).await?.pubkey_hash160();
                    let mut result = Vec::<u8>::new();
                    result.extend_from_slice(b"\x76\xa9\x14");
                    result.extend_from_slice(&pubkey_hash160);
                    result.extend_from_slice(b"\x88\xac");
                    Ok(result)
                }
                _ => Err(Error::Generic),
            }
        }
        ValidatedScriptConfigWithKeypath {
            config: ValidatedScriptConfig::Multisig { multisig, .. },
            ..
        } => Ok(super::multisig::pkscript(
            multisig,
            keypath[keypath.len() - 2],
            keypath[keypath.len() - 1],
        )?),
        ValidatedScriptConfigWithKeypath {
            config: ValidatedScriptConfig::Policy { parsed_policy, .. },
            ..
        } => match parsed_policy.derive_at_keypath(keypath)? {
            super::policies::Descriptor::Wsh(wsh) => Ok(wsh.witness_script()),
            // This function is only called for SegWit v0 inputs.
            _ => Err(Error::Generic),
        },
    }
}

/// Stream an input's previous transaction and verify that the prev_out_hash in the input matches
/// the hash of the previous transaction, as well as that the amount provided in the input is correct.
async fn handle_prevtx(
    input_index: u32,
    input: &pb::BtcSignInputRequest,
    num_inputs: u32,
    progress_component: &mut impl Progress,
    next_response: &mut NextResponse,
) -> Result<(), Error> {
    let prevtx_init = get_prevtx_init(input_index, next_response).await?;

    if prevtx_init.num_inputs < 1
        || prevtx_init.num_outputs < 1
        || input.prev_out_index >= prevtx_init.num_outputs
    {
        return Err(Error::InvalidInput);
    }

    let mut hasher = Sha256::new();
    hasher.update(prevtx_init.version.to_le_bytes());

    let prevtx_total_ios = prevtx_init
        .num_inputs
        .checked_add(prevtx_init.num_outputs)
        .ok_or(Error::InvalidInput)?;
    let prevtx_progress_denominator = num_inputs
        .checked_mul(prevtx_total_ios)
        .ok_or(Error::InvalidInput)?;
    let prevtx_progress_input_start = input_index
        .checked_mul(prevtx_total_ios)
        .ok_or(Error::InvalidInput)?;

    hasher.update(serialize(&VarInt(prevtx_init.num_inputs as u64)));
    for prevtx_input_index in 0..prevtx_init.num_inputs {
        // Update progress.
        progress_component.set_fraction(
            prevtx_progress_input_start
                .checked_add(prevtx_input_index)
                .ok_or(Error::InvalidInput)?,
            prevtx_progress_denominator,
        );

        let prevtx_input = get_prevtx_input(input_index, prevtx_input_index, next_response).await?;
        hasher.update(prevtx_input.prev_out_hash.as_slice());
        hasher.update(prevtx_input.prev_out_index.to_le_bytes());
        hasher.update(serialize(&VarInt(
            prevtx_input.signature_script.len() as u64
        )));
        hasher.update(prevtx_input.signature_script.as_slice());
        hasher.update(prevtx_input.sequence.to_le_bytes());
    }

    hasher.update(serialize(&VarInt(prevtx_init.num_outputs as u64)));
    for prevtx_output_index in 0..prevtx_init.num_outputs {
        // Update progress.
        progress_component.set_fraction(
            prevtx_progress_input_start
                .checked_add(prevtx_init.num_inputs)
                .and_then(|progress| progress.checked_add(prevtx_output_index))
                .ok_or(Error::InvalidInput)?,
            prevtx_progress_denominator,
        );

        let prevtx_output =
            get_prevtx_output(input_index, prevtx_output_index, next_response).await?;
        if prevtx_output_index == input.prev_out_index
            && input.prev_out_value != prevtx_output.value
        {
            return Err(Error::InvalidInput);
        }
        hasher.update(prevtx_output.value.to_le_bytes());
        hasher.update(serialize(&VarInt(prevtx_output.pubkey_script.len() as u64)));
        hasher.update(prevtx_output.pubkey_script.as_slice());
    }

    hasher.update(prevtx_init.locktime.to_le_bytes());
    // Hash again to produce the final double-hash.
    let hash = Sha256::digest(hasher.finalize());
    if hash.as_slice() != input.prev_out_hash.as_slice() {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

async fn validate_script_config<'a>(
    hal: &mut impl crate::hal::Hal,
    script_config: &'a pb::BtcScriptConfigWithKeypath,
    coin_params: &super::params::Params,
) -> Result<ValidatedScriptConfigWithKeypath<'a>, Error> {
    match script_config {
        pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::Multisig(multisig)),
                }),
            keypath,
        } => {
            super::multisig::validate(hal, multisig, keypath).await?;
            let name = super::multisig::get_name(hal, coin_params.coin, multisig, keypath)?
                .ok_or(Error::InvalidInput)?;
            Ok(ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::Multisig { name, multisig },
            })
        }
        pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::Policy(policy)),
                }),
            keypath,
        } => {
            let parsed_policy = super::policies::parse(hal, policy, coin_params.coin).await?;
            let name = parsed_policy
                .name(hal, coin_params)?
                .ok_or(Error::InvalidInput)?;
            Ok(ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::Policy {
                    name,
                    parsed_policy,
                },
            })
        }
        pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::SimpleType(simple_type)),
                }),
            keypath,
        } => {
            let simple_type = SimpleType::try_from(*simple_type)?;
            keypath::validate_account_simple(
                keypath,
                coin_params.bip44_coin,
                simple_type,
                coin_params.taproot_support,
            )
            .or(Err(Error::InvalidInput))?;
            Ok(ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(simple_type),
            })
        }
        _ => Err(Error::InvalidInput),
    }
}

async fn validate_script_configs<'a>(
    hal: &mut impl crate::hal::Hal,
    coin_params: &super::params::Params,
    script_configs: &'a [pb::BtcScriptConfigWithKeypath],
) -> Result<Vec<ValidatedScriptConfigWithKeypath<'a>>, Error> {
    let mut validated = Vec::with_capacity(script_configs.len());
    for config in script_configs.iter() {
        validated.push(validate_script_config(hal, config, coin_params).await?);
    }
    Ok(validated)
}

async fn validate_input_script_configs<'a>(
    hal: &mut impl crate::hal::Hal,
    coin_params: &super::params::Params,
    script_configs: &'a [pb::BtcScriptConfigWithKeypath],
) -> Result<Vec<ValidatedScriptConfigWithKeypath<'a>>, Error> {
    if script_configs.is_empty() {
        return Err(Error::InvalidInput);
    }

    let script_configs = validate_script_configs(hal, coin_params, script_configs).await?;

    // If there are multiple script configs, only SimpleType (single sig, no additional inputs)
    // configs are allowed, so e.g. mixing p2wpkh and pw2wpkh-p2sh is okay, but mixing p2wpkh with
    // multisig-pw2sh is not.

    // We get multisig out of the way first.

    if let [
        ValidatedScriptConfigWithKeypath {
            config: ValidatedScriptConfig::Multisig { name, multisig },
            ..
        },
    ] = script_configs.as_slice()
    {
        super::multisig::confirm(hal, "Spend from", coin_params, name, multisig).await?;
        return Ok(script_configs);
    }

    // Then we get policies out of the way.

    if let [
        ValidatedScriptConfigWithKeypath {
            config:
                ValidatedScriptConfig::Policy {
                    name,
                    parsed_policy,
                },
            ..
        },
    ] = script_configs.as_slice()
    {
        // We could check here that the account keypath matches one of our keys in the policy and
        // abort early, but we don't have to - if the keypath does not match we will fail when
        // processing the first input, where it is checked that the account keypath is a prefix of
        // the input keypath, and the computation of the pk_script checks that full keypath is
        // valid.

        parsed_policy
            .confirm(
                hal,
                "Spend from",
                coin_params,
                name,
                super::policies::Mode::Basic,
            )
            .await?;

        return Ok(script_configs);
    }

    // Only allow simple single sig configs here.
    for script_config in script_configs.iter() {
        match script_config {
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(_),
            } => {
                // Check that the bip44 account is the same for all. While we allow mixing input
                // types (bip44 purpose), we do not allow mixing accounts.

                if keypath[2] != script_configs[0].keypath[2] {
                    return Err(Error::InvalidInput);
                }
            }
            _ => return Err(Error::InvalidInput),
        }
    }
    Ok(script_configs)
}

// We configure the xpub cache to cache up to change/receive level. If e.g. there is an input/change
// with keypath `m/84'/0'/0'/0/0`, we want the xpub cache to return the xpub at `m/84'/0'/0'/0`, so
// only one child derivation needs to be done for the input/change.
//
// Only xpubs for simple configs (single-sig) are cached - multisig already provides all necessary
// xpubs in the script config itself.
fn setup_xpub_cache(cache: &mut Bip32XpubCache, script_configs: &[pb::BtcScriptConfigWithKeypath]) {
    for script_config in script_configs.iter() {
        match script_config {
            pb::BtcScriptConfigWithKeypath {
                script_config:
                    Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(_)),
                    }),
                keypath,
            } => {
                let mut receive = keypath.to_vec();
                receive.push(0);
                let mut change = keypath.to_vec();
                change.push(1);
                // Cache xpubs at the receive chain, e.g. m/84'/0'/0'/0.
                cache.add_keypath(&receive);
                // Cache xpubs at change chain, e.g. m/84'/0'/0'/1.
                cache.add_keypath(&change);
                // Also cache xpubs at the account level, e.g. m/84'/0'/0', so that the above two
                // xpubs (change and receive) can reuse the xpub the account-level.
                cache.add_keypath(keypath);
            }
            _ => {
                // We don't need to cache anything for multisig, as there, the xpubs are already
                // provided in the script config.
            }
        }
    }
}

fn silent_payments_network(
    value: pb::BtcCoin,
) -> Result<streaming_silent_payments::Network, Error> {
    match value {
        pb::BtcCoin::Btc => Ok(streaming_silent_payments::Network::Btc),
        pb::BtcCoin::Tbtc => Ok(streaming_silent_payments::Network::Tbtc),
        _ => Err(Error::InvalidInput),
    }
}

fn silent_payments_input_type(
    value: &pb::btc_script_config::SimpleType,
) -> streaming_silent_payments::InputType {
    match value {
        pb::btc_script_config::SimpleType::P2wpkhP2sh => {
            streaming_silent_payments::InputType::P2wpkhP2sh
        }
        pb::btc_script_config::SimpleType::P2wpkh => streaming_silent_payments::InputType::P2wpkh,
        pb::btc_script_config::SimpleType::P2tr => {
            streaming_silent_payments::InputType::P2trKeypathSpend
        }
    }
}

impl<'a> TryFrom<&'a ValidatedScriptConfigWithKeypath<'a>>
    for streaming_silent_payments::InputType
{
    type Error = Error;
    fn try_from(
        value: &'a ValidatedScriptConfigWithKeypath,
    ) -> Result<streaming_silent_payments::InputType, Self::Error> {
        match value {
            ValidatedScriptConfigWithKeypath {
                config: ValidatedScriptConfig::SimpleType(simple_type),
                ..
            } => Ok(silent_payments_input_type(simple_type)),
            _ => Err(Error::InvalidInput),
        }
    }
}

/// Singing flow:
///
/// init
/// for each input:
///    inputs_pass1
///    prevtx init
///    for each prevtx input:
///        prevtx inputs
///    for each prevtx output:
///        prevtx outputs
/// for each output:
///    outputs
/// for each input:
///    inputs_pass2
///    if input contains a host nonce commitment, the anti-klepto protocol is active:
///       inputs_pass2_antiklepto_host_nonce
///
/// The hash_prevout and hash_sequence and total_in are accumulated in inputs_pass1.
///
/// For each input in pass1, the input's prevtx is streamed to compute and compare the prevOutHash
/// and input amount. This only happens if the script_configs in the init request contain
/// non-taproot (legacy and v0 segwit) configs. If all inputs are taproot, this step is not needed
/// as the input amounts and pubkey scripts are committed to in the signature hash. With
/// SIGHASH_ALL/SIGHASH_DEFAULT, it would technically be enough if there was only one taproot input
/// to skip streaming the previous transactions, even if there are non-taproot inputs (every input
/// commits to the taproot inputs presence, and the taproot input commits to all amounts and pubkey
/// scripts), but we only skip streaming previous transactions if all inputs are taproot, for
/// simplicity.
///
/// For each output, the recipient is confirmed. At the last output, the total out, fee, locktime/RBF
/// are confirmed.
///
/// The inputs are signed in inputs_pass2.
///
/// IMPORTANT assumptions:
///
/// - In the 2nd pass, if the inputs provided by the host are not the same as in the 1st pass,
///   nothing bad will happen because the sighash uses the prevout and sequence hashes from the first
///   pass, and the value from the 2nd pass. The BTC consensus rules will reject the tx if there is a
///   mismatch.
///
/// - Only SIGHASH_ALL (SIGHASH_DEFAULT in taproot inputs). Other sighash types must be carefully
///   studied and might not be secure with the above flow or the above assumption.
async fn _process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcSignInitRequest,
) -> Result<Response, Error> {
    if crate::keystore::is_locked() {
        return Err(Error::InvalidState);
    }
    // Validate the coin.
    let coin = pb::BtcCoin::try_from(request.coin)?;
    let coin_params = super::params::get(coin);
    // Validate the format_unit.
    let format_unit = FormatUnit::try_from(request.format_unit)?;
    // Currently we do not support time-based nlocktime
    if request.locktime >= 500000000 {
        return Err(Error::InvalidInput);
    }
    // Currently only support version 1 or version 2 tx.
    // Version 2: https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
    if request.version != 1 && request.version != 2 {
        return Err(Error::InvalidInput);
    }
    if request.num_inputs < 1 || request.num_outputs < 1 {
        return Err(Error::InvalidInput);
    }
    let validated_script_configs =
        validate_input_script_configs(hal, coin_params, &request.script_configs).await?;
    let validated_output_script_configs =
        validate_script_configs(hal, coin_params, &request.output_script_configs).await?;

    let mut xpub_cache = Bip32XpubCache::new(Compute::Once);
    setup_xpub_cache(&mut xpub_cache, &request.script_configs);

    // For now we only allow one payment request with one output per transaction.  In the future,
    // this could be extended to allow multiple outputs per payment request (payment request
    // requests payout to multiple addresses/outputs), as well as multiple payment requests per
    // transaction.
    let mut payment_request_seen = false;

    let mut progress_component = Some(hal.ui().progress_create("Loading transaction..."));

    let mut next_response = NextResponse {
        next: Default::default(),
        wrap: false,
    };

    // Will contain the sum of all spent output values in the first inputs pass.
    let mut inputs_sum_pass1: u64 = 0;

    let mut locktime_applies: bool = false;
    let mut rbf: bool = false;

    let mut hasher_prevouts = Sha256::new();
    let mut hasher_sequence = Sha256::new();
    let mut hasher_amounts = Sha256::new();
    let mut hasher_scriptpubkeys = Sha256::new();

    // Are all inputs taproot?
    let taproot_only = validated_script_configs.iter().all(is_taproot);

    let mut silent_payment = if request.contains_silent_payment_outputs {
        Some(SilentPayment::new(
            SECP256K1,
            silent_payments_network(coin)?,
        ))
    } else {
        None
    };

    for input_index in 0..request.num_inputs {
        // Update progress.
        progress_component
            .as_mut()
            .unwrap()
            .set_fraction(input_index, request.num_inputs);

        let tx_input = get_tx_input(input_index, &mut next_response).await?;
        let script_config_account = validated_script_configs
            .get(tx_input.script_config_index as usize)
            .ok_or(Error::InvalidInput)?;
        validate_input(&tx_input, coin_params, script_config_account)?;
        if tx_input.sequence < 0xffffffff - 1 {
            rbf = true;
        }
        if tx_input.sequence < 0xffffffff {
            locktime_applies = true;
        }
        inputs_sum_pass1 = inputs_sum_pass1
            .checked_add(tx_input.prev_out_value)
            .ok_or(Error::InvalidInput)?;

        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 2: accumulate hashPrevouts
        // ANYONECANPAY not supported.
        hasher_prevouts.update(tx_input.prev_out_hash.as_slice());
        hasher_prevouts.update(tx_input.prev_out_index.to_le_bytes());

        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 3: accumulate hashSequence
        // only SIGHASH_ALL supported.
        hasher_sequence.update(tx_input.sequence.to_le_bytes());

        // https://github.com/bitcoin/bips/blob/bb8dc57da9b3c6539b88378348728a2ff43f7e9c/bip-0341.mediawiki#common-signature-message
        // accumulate `sha_amounts`
        hasher_amounts.update(tx_input.prev_out_value.to_le_bytes());

        // https://github.com/bitcoin/bips/blob/bb8dc57da9b3c6539b88378348728a2ff43f7e9c/bip-0341.mediawiki#common-signature-message
        // accumulate `sha_scriptpubkeys`
        let pk_script = common::Payload::from(
            hal,
            &mut xpub_cache,
            coin_params,
            &tx_input.keypath,
            script_config_account,
        )
        .await?
        .pk_script(coin_params)?;
        hasher_scriptpubkeys.update(serialize(&VarInt(pk_script.len() as u64)));
        hasher_scriptpubkeys.update(pk_script.as_slice());

        if !taproot_only {
            handle_prevtx(
                input_index,
                &tx_input,
                request.num_inputs,
                progress_component.as_mut().unwrap(),
                &mut next_response,
            )
            .await?;
        }

        if let Some(ref mut silent_payment) = silent_payment {
            let keypair = bitcoin::key::UntweakedKeypair::from_seckey_slice(
                SECP256K1,
                &crate::keystore::secp256k1_get_private_key(hal, &tx_input.keypath).await?,
            )
            .unwrap();
            // For Taproot, only key path spends are allowed in silent payments, and we need to
            // provide the key path spend private key, which means the internal key plus the tap
            // tweak.
            let private_key = if is_taproot(script_config_account) {
                keypair.tap_tweak(SECP256K1, None).to_keypair().secret_key()
            } else {
                keypair.secret_key()
            };

            silent_payment
                .add_input(
                    script_config_account.try_into()?,
                    &private_key,
                    bitcoin::OutPoint::new(
                        bitcoin::Txid::from_slice(&tx_input.prev_out_hash)
                            .map_err(|_| Error::InvalidInput)?,
                        tx_input.prev_out_index,
                    ),
                )
                .map_err(|_| Error::InvalidInput)?;
        }
    }

    // The progress for loading the inputs is 100%.
    progress_component.as_mut().unwrap().set_fraction(1, 1);

    let hash_prevouts = hasher_prevouts.finalize();
    let hash_sequence = hasher_sequence.finalize();
    let hash_amounts = hasher_amounts.finalize();
    let hash_scriptpubkeys = hasher_scriptpubkeys.finalize();

    // Base component on the screen stack during signing, which is shown while the device is waiting
    // for the next signing api call. Without this, the 'See the BitBoxApp' waiting screen would
    // flicker in between user confirmations. All user input happens during output processing.
    //
    // We only start rendering this (and stop rendering the inputs progress bar) after we receive
    // the first output, otherwise there is a noticable delay between processing the last input and
    // receiving the first output.
    let mut empty_component = None;

    // Will contain the sum of all change output values.
    let mut outputs_sum_change: u64 = 0;
    // Will contain the sum of all outgoing output values (non-change outputs).
    let mut outputs_sum_out: u64 = 0;

    let mut num_changes: u32 = 0;

    let mut hasher_outputs = Sha256::new();
    for output_index in 0..request.num_outputs {
        let tx_output = get_tx_output(output_index, &mut next_response).await?;
        if output_index == 0 {
            // Stop rendering inputs progress update.
            drop(progress_component.take());

            empty_component = Some(hal.ui().empty_create());
        }

        let output_type = pb::BtcOutputType::try_from(tx_output.r#type)?;

        // We don't allow regular outputs to have 0 value.
        // OP_RETURN outputs however we require to have 0 value.
        if output_type == pb::BtcOutputType::OpReturn {
            if tx_output.value != 0
                || tx_output.ours
                || tx_output.silent_payment.is_some()
                || tx_output.payment_request_index.is_some()
            {
                return Err(Error::InvalidInput);
            }
        } else if tx_output.value == 0 {
            return Err(Error::InvalidInput);
        }

        // Get payload. If the output is marked ours, we compute the payload from the keystore,
        // otherwise it is provided in tx_output.payload.
        let payload: common::Payload = if tx_output.ours {
            // Only external outputs can belong to a payment request. Receiving on silent payments
            // is not supported yet.
            if tx_output.payment_request_index.is_some() || tx_output.silent_payment.is_some() {
                return Err(Error::InvalidInput);
            }

            // Compute the payload from the keystore.
            let script_config_account =
                if let Some(output_script_config_index) = tx_output.output_script_config_index {
                    validated_output_script_configs
                        .get(output_script_config_index as usize)
                        .ok_or(Error::InvalidInput)?
                } else {
                    validated_script_configs
                        .get(tx_output.script_config_index as usize)
                        .ok_or(Error::InvalidInput)?
                };

            validate_keypath(
                coin_params,
                script_config_account,
                &tx_output.keypath,
                keypath::ReceiveSpend::Receive,
            )?;

            common::Payload::from(
                hal,
                &mut xpub_cache,
                coin_params,
                &tx_output.keypath,
                script_config_account,
            )
            .await?
        } else {
            // Take payload from provided output.

            // Create silent payment output.
            if let Some(output_silent_payment) = tx_output.silent_payment.as_ref() {
                match silent_payment {
                    None => return Err(Error::InvalidInput),
                    Some(ref mut silent_payment) => {
                        let sp_output = silent_payment
                            .create_output(&output_silent_payment.address)
                            .map_err(|_| Error::InvalidInput)?;
                        let payload = common::Payload {
                            data: sp_output.pubkey.serialize().to_vec(),
                            output_type: pb::BtcOutputType::P2tr,
                        };
                        next_response.next.generated_output_pkscript =
                            payload.pk_script(coin_params)?;
                        next_response.next.silent_payment_dleq_proof =
                            sp_output.dleq_proof.to_vec();
                        payload
                    }
                }
            } else {
                common::Payload {
                    data: tx_output.payload.clone(),
                    output_type,
                }
            }
        };

        let is_change = if tx_output.ours && tx_output.output_script_config_index.is_none() {
            let script_config_account = validated_script_configs
                .get(tx_output.script_config_index as usize)
                .ok_or(Error::InvalidInput)?;

            match &script_config_account.config {
                // Policy.
                ValidatedScriptConfig::Policy { parsed_policy, .. } => {
                    parsed_policy.is_change_keypath(&tx_output.keypath)?
                }
                // Everything else.
                _ => {
                    let change = tx_output.keypath[tx_output.keypath.len() - 2];
                    change == 1
                }
            }
        } else {
            false
        };

        if !is_change {
            // Verify output if it is not a change output.
            // Assemble address to display, get user confirmation.
            let address = || -> Result<String, Error> {
                if let Some(sp) = tx_output.silent_payment.as_ref() {
                    Ok(sp.address.clone())
                } else {
                    Ok(payload.address(coin_params)?)
                }
            };

            if let Some(output_payment_request_index) = tx_output.payment_request_index {
                if output_payment_request_index != 0 {
                    return Err(Error::InvalidInput);
                }
                if output_type == pb::BtcOutputType::OpReturn {
                    return Err(Error::InvalidInput);
                }
                if payment_request_seen {
                    return Err(Error::InvalidInput);
                }
                let payment_request: pb::BtcPaymentRequestRequest =
                    get_payment_request(output_payment_request_index, &mut next_response).await?;
                if payment_request::contains_coin_purchase_memo(&payment_request) {
                    validate_swap_source_account(coin, &validated_script_configs)?;
                }
                // Only one output per payment request is supported for now, so the
                // payment-request total equals the current output value.
                let total_value = tx_output.value;
                let displayed_source_amount = format_amount(coin_params, format_unit, total_value)?;
                payment_request::user_verify(hal, &payment_request, &displayed_source_amount)
                    .await?;
                match payment_request::validate_btc(
                    hal,
                    coin_params,
                    &payment_request,
                    total_value,
                    &address()?,
                )
                .await
                {
                    Ok(()) => {}
                    #[cfg(not(feature = "app-ethereum"))]
                    Err(payment_request::ValidationError::Disabled) => {
                        return Err(Error::Disabled);
                    }
                    Err(_) => {
                        hal.ui().status("Invalid\npayment request", false).await;
                        return Err(Error::InvalidInput);
                    }
                }

                payment_request_seen = true;
            } else if output_type == pb::BtcOutputType::OpReturn {
                // OP_RETURN value was validated to be 0 above, so we don't need to show the amount.
                crate::workflow::verify_message::verify(
                    hal,
                    "OP_RETURN",
                    "OP_RETURN",
                    &tx_output.payload,
                    false,
                )
                .await?;
            } else {
                // When sending coins back to the same account (non-change), or another account of
                // the same keystore (change or non-change), we show a prefix to let the user know.
                let prefix: Option<String> = if tx_output.ours {
                    if let Some(output_script_config_index) = tx_output.output_script_config_index {
                        // Any address belonging to any account of the same keystore.
                        let output_script_config = validated_output_script_configs
                            .get(output_script_config_index as usize)
                            .ok_or(Error::InvalidInput)?;
                        Some(output_script_config.self_transfer_representation()?)
                    } else {
                        // Non-change output of the same account.
                        Some("This BitBox (same account)".into())
                    }
                } else {
                    // Regular outgoing payment, no prefix.
                    None
                };

                let address_formatted = util::strings::format_address(&address()?);
                let recipient = if let Some(prefix) = prefix {
                    format!("{}: {}", prefix, address_formatted)
                } else {
                    address_formatted
                };
                hal.ui()
                    .verify_recipient(
                        &recipient,
                        &format_amount(coin_params, format_unit, tx_output.value)?,
                    )
                    .await?;
            }
        }

        if is_change {
            num_changes += 1;
            outputs_sum_change = outputs_sum_change
                .checked_add(tx_output.value)
                .ok_or(Error::InvalidInput)?;
        } else {
            outputs_sum_out = outputs_sum_out
                .checked_add(tx_output.value)
                .ok_or(Error::InvalidInput)?;
        }

        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 8: accumulate hashOutputs
        // only SIGHASH_ALL supported.
        hasher_outputs.update(tx_output.value.to_le_bytes());
        let pk_script = payload.pk_script(coin_params)?;
        hasher_outputs.update(serialize(&VarInt(pk_script.len() as u64)));
        hasher_outputs.update(pk_script.as_slice());
    }

    if num_changes > 1 {
        hal.ui()
            .confirm(&ConfirmParams {
                title: "Warning",
                body: &format!("There are {}\nchange outputs.\nProceed?", num_changes),
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }

    // Verify locktime/rbf.
    // A locktime of 0 will also not be verified, as it's certainly in the past and can't do any
    // harm.
    //
    // This is not a security feature, the extra locktime/RBF user confirmation is skipped if the tx
    // is not rbf or has a locktime of 0.
    if request.locktime > 0 && locktime_applies {
        // The RBF nsequence bytes are often set in conjunction with a locktime,
        // so verify both simultaneously.
        hal.ui()
            .confirm(&ConfirmParams {
                body: &format!(
                    "Locktime on block:\n{}\n{}",
                    request.locktime,
                    if coin_params.rbf_support {
                        if rbf {
                            "Transaction is RBF"
                        } else {
                            "Transaction is not RBF"
                        }
                    } else {
                        // There is no RBF in Litecoin.
                        ""
                    }
                ),
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }

    // Total out, including fee.
    let total_out: u64 = inputs_sum_pass1
        .checked_sub(outputs_sum_change)
        .ok_or(Error::InvalidInput)?;
    let fee: u64 = total_out
        .checked_sub(outputs_sum_out)
        .ok_or(Error::InvalidInput)?;
    let fee_percentage: Option<f64> = if outputs_sum_out == 0 {
        None
    } else {
        Some(100. * (fee as f64) / (outputs_sum_out as f64))
    };
    transaction::verify_total_fee_maybe_warn(
        hal,
        &format_amount(coin_params, format_unit, total_out)?,
        &format_amount(coin_params, format_unit, fee)?,
        fee_percentage,
    )
    .await?;
    hal.ui().status("Transaction\nconfirmed", true).await;

    let hash_outputs = hasher_outputs.finalize();

    // Stop rendering the empty component.
    drop(empty_component);

    // Show progress of signing inputs if there are more than 2 inputs. This is an arbitrary cutoff;
    // less or equal to 2 inputs is fast enough so it does not need a progress bar.
    let mut progress_component = if request.num_inputs > 2 {
        Some(hal.ui().progress_create("Signing transaction..."))
    } else {
        None
    };

    // Will contain the sum of all spent output values in the second inputs pass.
    let mut inputs_sum_pass2: u64 = 0;
    for input_index in 0..request.num_inputs {
        let tx_input = get_tx_input(input_index, &mut next_response).await?;
        let script_config_account = validated_script_configs
            .get(tx_input.script_config_index as usize)
            .ok_or(Error::InvalidInput)?;

        validate_input(&tx_input, coin_params, script_config_account)?;

        inputs_sum_pass2 = inputs_sum_pass2
            .checked_add(tx_input.prev_out_value)
            .ok_or(Error::InvalidInput)?;
        if inputs_sum_pass2 > inputs_sum_pass1 {
            return Err(Error::InvalidInput);
        }

        if is_taproot(script_config_account) {
            // This is a taproot (P2TR) input.

            // Anti-Klepto protocol not supported yet for Schnorr signatures.
            if tx_input.host_nonce_commitment.is_some() {
                return Err(Error::InvalidInput);
            }

            let spend_info = match &script_config_account.config {
                ValidatedScriptConfig::SimpleType(SimpleType::P2tr) => {
                    // This is a BIP-86 spend, so we tweak the private key by the hash of the public
                    // key only, as there is no Taproot merkle root.
                    let xpub = xpub_cache.get_xpub(hal, &tx_input.keypath).await?;
                    let pubkey = bitcoin::PublicKey::from_slice(xpub.public_key())
                        .map_err(|_| Error::Generic)?;
                    TaprootSpendInfo::KeySpend(bitcoin::TapTweakHash::from_key_and_tweak(
                        pubkey.into(),
                        None,
                    ))
                }
                ValidatedScriptConfig::Policy { parsed_policy, .. } => {
                    // Get the Taproot tweak based on whether we spend using the internal key (key
                    // path spend) or if we spend using a leaf script. For key path spends, we must
                    // first tweak the private key to match the Taproot output key. For leaf
                    // scripts, we do not tweak.

                    parsed_policy
                        .taproot_spend_info(hal, &mut xpub_cache, &tx_input.keypath)
                        .await?
                }
                _ => return Err(Error::Generic),
            };
            let sighash = bip341::sighash(&bip341::Args {
                version: request.version,
                locktime: request.locktime,
                hash_prevouts: hash_prevouts.into(),
                hash_amounts: hash_amounts.into(),
                hash_scriptpubkeys: hash_scriptpubkeys.into(),
                hash_sequences: hash_sequence.into(),
                hash_outputs: hash_outputs.into(),
                input_index,
                tapleaf_hash: if let TaprootSpendInfo::ScriptSpend(leaf_hash) = &spend_info {
                    Some(leaf_hash.to_byte_array())
                } else {
                    None
                },
            });

            next_response.next.has_signature = true;
            next_response.next.signature = crate::keystore::secp256k1_schnorr_sign(
                hal,
                &tx_input.keypath,
                &sighash,
                if let TaprootSpendInfo::KeySpend(tweak_hash) = &spend_info {
                    Some(tweak_hash.as_byte_array())
                } else {
                    None
                },
            )
            .await?
            .to_vec();
        } else {
            // Sign all other supported inputs.

            const SIGHASH_ALL: u32 = 0x01;
            let sighash = bip143::sighash(&bip143::Args {
                version: request.version,
                hash_prevouts: Sha256::digest(hash_prevouts).into(),
                hash_sequence: Sha256::digest(hash_sequence).into(),
                outpoint_hash: tx_input.prev_out_hash.as_slice().try_into().unwrap(),
                outpoint_index: tx_input.prev_out_index,
                sighash_script: &sighash_script(
                    hal,
                    &mut xpub_cache,
                    script_config_account,
                    &tx_input.keypath,
                )
                .await?,
                prevout_value: tx_input.prev_out_value,
                sequence: tx_input.sequence,
                hash_outputs: Sha256::digest(hash_outputs).into(),
                locktime: request.locktime,
                sighash_flags: SIGHASH_ALL,
            });

            let private_key =
                crate::keystore::secp256k1_get_private_key(hal, &tx_input.keypath).await?;
            // Engage in the Anti-Klepto protocol if the host sends a host nonce commitment.
            let host_nonce: [u8; 32] = match tx_input.host_nonce_commitment {
                Some(pb::AntiKleptoHostNonceCommitment { ref commitment }) => {
                    let signer_commitment = crate::secp256k1::secp256k1_nonce_commit(
                        private_key.as_slice().try_into().unwrap(),
                        &sighash,
                        commitment
                            .as_slice()
                            .try_into()
                            .or(Err(Error::InvalidInput))?,
                    )?;
                    next_response.next.anti_klepto_signer_commitment =
                        Some(pb::AntiKleptoSignerCommitment {
                            commitment: signer_commitment.to_vec(),
                        });

                    get_antiklepto_host_nonce(input_index, &mut next_response)
                        .await?
                        .host_nonce
                        .as_slice()
                        .try_into()
                        .or(Err(Error::InvalidInput))?
                }
                // Return signature directly without the anti-klepto protocol, for backwards compatibility.
                None => [0; 32],
            };

            let sign_result = crate::secp256k1::secp256k1_sign(
                private_key.as_slice().try_into().unwrap(),
                &sighash,
                Some(&host_nonce),
            )?;
            drop(private_key);
            next_response.next.has_signature = true;
            next_response.next.signature = sign_result.signature.to_vec();
        }

        // Update progress.
        if let Some(ref mut c) = progress_component {
            c.set_fraction(input_index + 1, request.num_inputs);
        }
    }

    if inputs_sum_pass1 != inputs_sum_pass2 {
        return Err(Error::InvalidInput);
    }

    next_response.next.r#type = NextType::Done as _;
    Ok(next_response.to_protobuf())
}

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcSignInitRequest,
) -> Result<Response, Error> {
    let result = _process(hal, request).await;
    if let Err(Error::UserAbort) = result {
        hal.ui().status("Transaction\ncanceled", false).await;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bip32::parse_xpub;
    use crate::hal::testing::ui::Screen;
    use crate::hal::{Memory, testing::TestingHal};
    use crate::keystore::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use alloc::boxed::Box;
    use alloc::collections::{BTreeMap, BTreeSet};
    use bitbox_test_vectors::btc_transaction as btc_test_vectors;
    use hex_lit::hex;
    use pb::btc_payment_request_request::{Memo, memo};
    use util::bip32::HARDENED;

    fn extract_next(response: &Response) -> &pb::BtcSignNextResponse {
        match response {
            Response::BtcSignNext(next) => next,
            Response::Btc(pb::BtcResponse {
                response: Some(pb::btc_response::Response::SignNext(next)),
            }) => next,
            _ => panic!("wrong response type"),
        }
    }
    struct TxInput {
        input: pb::BtcSignInputRequest,
        prevtx_version: u32,
        prevtx_inputs: Vec<pb::BtcPrevTxInputRequest>,
        prevtx_outputs: Vec<pb::BtcPrevTxOutputRequest>,
        prevtx_locktime: u32,
        host_nonce: Option<Vec<u8>>,
    }

    struct Transaction {
        coin: pb::BtcCoin,
        // How many dialogs the user has to confirm in the test transaction
        total_confirmations: u32,
        version: u32,
        inputs: Vec<TxInput>,
        outputs: Vec<pb::BtcSignOutputRequest>,
        locktime: u32,
        payment_request: Option<pb::BtcPaymentRequestRequest>,
    }

    impl Transaction {
        /// An arbitrary test transaction with some inputs and outputs.
        fn new(coin: pb::BtcCoin) -> Self {
            let bip44_coin = super::super::params::get(coin).bip44_coin;
            Transaction {
                coin,
                total_confirmations: 6,
                version: 1,
                inputs: vec![
                    TxInput {
                        input: pb::BtcSignInputRequest {
                            prev_out_hash: vec![
                                0x45, 0x17, 0x74, 0x50, 0x1b, 0xaf, 0xdf, 0xf7, 0x46, 0x9, 0xe,
                                0x6, 0x16, 0xd9, 0x5e, 0xd0, 0x80, 0xd7, 0x82, 0x9a, 0xfe, 0xa2,
                                0xbd, 0x97, 0x8a, 0xf8, 0x11, 0xf4, 0x5e, 0x43, 0x81, 0x39,
                            ],
                            prev_out_index: 1,
                            prev_out_value: 1010000000,
                            sequence: 0xffffffff,
                            keypath: vec![84 + HARDENED, bip44_coin, 10 + HARDENED, 0, 5],
                            script_config_index: 0,
                            host_nonce_commitment: None,
                        },
                        prevtx_version: 1,
                        prevtx_inputs: vec![
                            pb::BtcPrevTxInputRequest {
                                prev_out_hash: vec![
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74,
                                ],
                                prev_out_index: 3,
                                signature_script: b"signature script".to_vec(),
                                sequence: 0xffffffff - 2,
                            },
                            pb::BtcPrevTxInputRequest {
                                prev_out_hash: vec![
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75,
                                ],
                                prev_out_index: 23,
                                signature_script: b"signature script 2".to_vec(),
                                sequence: 123456,
                            },
                        ],
                        prevtx_outputs: vec![
                            pb::BtcPrevTxOutputRequest {
                                value: 101000000, // btc 1.01
                                pubkey_script: b"pubkey script".to_vec(),
                            },
                            pb::BtcPrevTxOutputRequest {
                                value: 1010000000, // btc 10.1
                                pubkey_script: b"pubkey script 2".to_vec(),
                            },
                        ],
                        prevtx_locktime: 0,
                        host_nonce: None,
                    },
                    TxInput {
                        input: pb::BtcSignInputRequest {
                            prev_out_hash: vec![
                                0x40, 0x9b, 0x4f, 0x56, 0xca, 0x9f, 0x6, 0xcb, 0x88, 0x28, 0x3,
                                0xad, 0x55, 0x4b, 0xeb, 0x1d, 0x9e, 0xf8, 0x78, 0x7, 0xf0, 0x52,
                                0x29, 0xe7, 0x55, 0x15, 0xe4, 0xb2, 0xaa, 0x87, 0x69, 0x1d,
                            ],
                            prev_out_index: 0,
                            prev_out_value: 1020000000, // btc 10.2, matches prevout tx output at index 0.
                            sequence: 0xffffffff,
                            keypath: vec![84 + HARDENED, bip44_coin, 10 + HARDENED, 0, 7],
                            script_config_index: 0,
                            host_nonce_commitment: None,
                        },
                        prevtx_version: 2,
                        prevtx_inputs: vec![pb::BtcPrevTxInputRequest {
                            prev_out_hash: vec![
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            ],
                            prev_out_index: 3,
                            signature_script: b"signature script".to_vec(),
                            sequence: 0xffffffff - 2,
                        }],
                        prevtx_outputs: vec![pb::BtcPrevTxOutputRequest {
                            value: 1020000000, // btc 10.2
                            pubkey_script: b"pubkey script".to_vec(),
                        }],
                        prevtx_locktime: 87654,
                        host_nonce: None,
                    },
                ],
                outputs: vec![
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2pkh as _,
                        value: 100000000, // btc 1,
                        payload: vec![
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                        ],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2sh as _,
                        value: 1234567890, // btc 12.3456789,
                        payload: vec![
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                        ],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2wpkh as _,
                        value: 6000, // btc .00006
                        payload: vec![
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                        ],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2wsh as _,
                        value: 7000, // btc .00007
                        payload: vec![
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                        ],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        // change
                        ours: true,
                        r#type: 0,
                        value: 690000000, // btc 6.9
                        keypath: vec![84 + HARDENED, bip44_coin, 10 + HARDENED, 1, 3],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        // change #2
                        ours: true,
                        r#type: 0,
                        value: 100,
                        keypath: vec![84 + HARDENED, bip44_coin, 10 + HARDENED, 1, 30],
                        ..Default::default()
                    },
                ],
                locktime: 0,
                payment_request: None,
            }
        }

        /// An arbitrary multisig test transaction with some inputs and outputs.
        fn new_multisig() -> Self {
            let coin = pb::BtcCoin::Tbtc;
            let bip44_coin = super::super::params::get(coin).bip44_coin;
            Transaction {
                coin,
                total_confirmations: 5,
                version: 2,
                inputs: vec![TxInput {
                    input: pb::BtcSignInputRequest {
                        prev_out_hash: vec![
                            0x41, 0x3b, 0x8e, 0x74, 0x05, 0x15, 0x96, 0x6b, 0x20, 0x2b, 0x24, 0xc3,
                            0x19, 0xfc, 0xf3, 0x5f, 0xc5, 0x37, 0x6e, 0xb2, 0x71, 0x95, 0xb8, 0x76,
                            0x62, 0x9a, 0x44, 0x1d, 0x19, 0xaa, 0x6c, 0x0f,
                        ],
                        prev_out_index: 0,
                        prev_out_value: 100000, // btc 0.001
                        sequence: 0xffffffff - 1,
                        keypath: vec![48 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED, 0, 0],
                        script_config_index: 0,
                        host_nonce_commitment: None,
                    },
                    prevtx_version: 1,
                    prevtx_inputs: vec![pb::BtcPrevTxInputRequest {
                        prev_out_hash: vec![
                            0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                        ],
                        prev_out_index: 3,
                        signature_script: b"signature script".to_vec(),
                        sequence: 0xffffffff - 2,
                    }],
                    prevtx_outputs: vec![pb::BtcPrevTxOutputRequest {
                        value: 100000, // btc 0.001
                        pubkey_script: b"pubkey script".to_vec(),
                    }],
                    prevtx_locktime: 0,
                    host_nonce: None,
                }],
                outputs: vec![
                    pb::BtcSignOutputRequest {
                        ours: true,
                        r#type: pb::BtcOutputType::Unknown as _,
                        value: 9825, // btc 0.00009825
                        keypath: vec![48 + HARDENED, bip44_coin, 0 + HARDENED, 2 + HARDENED, 1, 0],
                        ..Default::default()
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2wsh as _,
                        value: 90000, // btc 0.0009
                        payload: vec![
                            0x59, 0x88, 0x02, 0x4d, 0x26, 0x74, 0x2c, 0x74, 0xd1, 0x1c, 0x3b, 0x28,
                            0x83, 0xe7, 0x57, 0x84, 0x67, 0x25, 0xa3, 0xf6, 0x23, 0xae, 0xc2, 0x09,
                            0x76, 0xd3, 0x0e, 0x29, 0xb0, 0xd4, 0xb3, 0x5b,
                        ],
                        ..Default::default()
                    },
                ],
                locktime: 1663289,
                payment_request: None,
            }
        }

        /// An arbitrary policy test transaction with some inputs and outputs.
        fn new_policy() -> Self {
            let mut tx = Self::new_multisig();
            tx.total_confirmations = 9;
            let bip44_coin = super::super::params::get(tx.coin).bip44_coin;
            tx.inputs[0].input.keypath =
                vec![48 + HARDENED, bip44_coin, 0 + HARDENED, 3 + HARDENED, 0, 0];
            tx.outputs[0].keypath =
                vec![48 + HARDENED, bip44_coin, 0 + HARDENED, 3 + HARDENED, 1, 0];
            tx
        }

        fn init_request(&self) -> pb::BtcSignInitRequest {
            pb::BtcSignInitRequest {
                coin: self.coin as _,
                script_configs: vec![pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![
                        84 + HARDENED,
                        super::super::params::get(self.coin).bip44_coin,
                        10 + HARDENED,
                    ],
                }],
                output_script_configs: vec![],
                version: self.version,
                num_inputs: self.inputs.len() as _,
                num_outputs: self.outputs.len() as _,
                locktime: self.locktime,
                format_unit: FormatUnit::Default as _,
                contains_silent_payment_outputs: self
                    .outputs
                    .iter()
                    .any(|output| output.silent_payment.is_some()),
            }
        }

        fn init_request_policy(
            &self,
            policy: pb::btc_script_config::Policy,
            keypath_account: &[u32],
        ) -> pb::BtcSignInitRequest {
            pb::BtcSignInitRequest {
                coin: self.coin as _,
                script_configs: vec![pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::Policy(policy)),
                    }),
                    keypath: keypath_account.to_vec(),
                }],
                output_script_configs: vec![],
                version: self.version,
                num_inputs: self.inputs.len() as _,
                num_outputs: self.outputs.len() as _,
                locktime: self.locktime,
                format_unit: FormatUnit::Default as _,
                contains_silent_payment_outputs: false,
            }
        }

        /// Return the transaction part requested by the device.
        fn make_host_request(&self, response: Response) -> Request {
            let next = extract_next(&response);
            match NextType::try_from(next.r#type).unwrap() {
                NextType::Input => {
                    Request::BtcSignInput(self.inputs[next.index as usize].input.clone())
                }
                NextType::Output => {
                    Request::BtcSignOutput(self.outputs[next.index as usize].clone())
                }
                NextType::PaymentRequest => Request::Btc(pb::BtcRequest {
                    request: Some(pb::btc_request::Request::PaymentRequest(
                        self.payment_request.clone().unwrap(),
                    )),
                }),
                NextType::PrevtxInit => Request::Btc(pb::BtcRequest {
                    request: Some(pb::btc_request::Request::PrevtxInit(
                        pb::BtcPrevTxInitRequest {
                            version: self.inputs[next.index as usize].prevtx_version,
                            num_inputs: self.inputs[next.index as usize].prevtx_inputs.len() as _,
                            num_outputs: self.inputs[next.index as usize].prevtx_outputs.len() as _,
                            locktime: self.inputs[next.index as usize].prevtx_locktime,
                        },
                    )),
                }),
                NextType::PrevtxInput => Request::Btc(pb::BtcRequest {
                    request: Some(pb::btc_request::Request::PrevtxInput(
                        self.inputs[next.index as usize].prevtx_inputs[next.prev_index as usize]
                            .clone(),
                    )),
                }),
                NextType::PrevtxOutput => Request::Btc(pb::BtcRequest {
                    request: Some(pb::btc_request::Request::PrevtxOutput(
                        self.inputs[next.index as usize].prevtx_outputs[next.prev_index as usize]
                            .clone(),
                    )),
                }),
                NextType::HostNonce => Request::Btc(pb::BtcRequest {
                    request: Some(pb::btc_request::Request::AntikleptoSignature(
                        pb::AntiKleptoSignatureRequest {
                            host_nonce: self.inputs[next.index as usize]
                                .host_nonce
                                .as_ref()
                                .expect("need host_nonce")
                                .clone(),
                        },
                    )),
                }),
                _ => panic!("unexpected next response"),
            }
        }
    }

    fn mock_host_responder(tx: alloc::rc::Rc<core::cell::RefCell<Transaction>>) {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() =
            Some(Box::new(move |response: Response| {
                Ok(tx.borrow().make_host_request(response))
            }));
    }

    fn keypath_components(keypath: &bitcoin::bip32::DerivationPath) -> Vec<u32> {
        keypath.as_ref().iter().copied().map(u32::from).collect()
    }

    fn vector_keypath(keypath: &str) -> Vec<u32> {
        keypath_components(&keypath.parse().unwrap())
    }

    fn vector_coin(coin: btc_test_vectors::Coin) -> pb::BtcCoin {
        match coin {
            btc_test_vectors::Coin::Btc => pb::BtcCoin::Btc,
            btc_test_vectors::Coin::Tbtc => pb::BtcCoin::Tbtc,
            btc_test_vectors::Coin::Ltc => pb::BtcCoin::Ltc,
        }
    }

    fn vector_script_config(config: &btc_test_vectors::ScriptConfig) -> pb::BtcScriptConfig {
        use btc_test_vectors::ScriptConfig;

        let config = match config {
            ScriptConfig::Simple { script_type } => {
                let simple_type = match script_type {
                    btc_test_vectors::SimpleType::P2wpkh => SimpleType::P2wpkh,
                    btc_test_vectors::SimpleType::P2wpkhP2sh => SimpleType::P2wpkhP2sh,
                    btc_test_vectors::SimpleType::P2tr => SimpleType::P2tr,
                };
                pb::btc_script_config::Config::SimpleType(simple_type as _)
            }
            ScriptConfig::Multisig {
                threshold,
                xpubs,
                our_xpub_index,
                script_type,
            } => {
                let script_type = match script_type {
                    btc_test_vectors::MultisigScriptType::P2wsh => {
                        pb::btc_script_config::multisig::ScriptType::P2wsh
                    }
                    btc_test_vectors::MultisigScriptType::P2wshP2sh => {
                        pb::btc_script_config::multisig::ScriptType::P2wshP2sh
                    }
                };
                pb::btc_script_config::Config::Multisig(pb::btc_script_config::Multisig {
                    threshold: *threshold,
                    xpubs: xpubs.iter().map(|xpub| parse_xpub(xpub).unwrap()).collect(),
                    our_xpub_index: *our_xpub_index,
                    script_type: script_type as _,
                })
            }
            ScriptConfig::Policy { policy, keys } => {
                pb::btc_script_config::Config::Policy(pb::btc_script_config::Policy {
                    policy: policy.clone(),
                    keys: keys
                        .iter()
                        .map(|key| pb::KeyOriginInfo {
                            root_fingerprint: key
                                .root_fingerprint
                                .as_deref()
                                .map(hex::decode)
                                .transpose()
                                .unwrap()
                                .unwrap_or_default(),
                            keypath: key
                                .keypath
                                .as_deref()
                                .map(vector_keypath)
                                .unwrap_or_default(),
                            xpub: Some(parse_xpub(&key.xpub).unwrap()),
                        })
                        .collect(),
                })
            }
        };
        pb::BtcScriptConfig {
            config: Some(config),
        }
    }

    fn vector_script_config_with_keypath(
        config: &btc_test_vectors::ScriptConfigWithKeypath,
    ) -> pb::BtcScriptConfigWithKeypath {
        pb::BtcScriptConfigWithKeypath {
            script_config: Some(vector_script_config(&config.script_config)),
            keypath: vector_keypath(&config.keypath),
        }
    }

    fn vector_prevtx(
        input: &btc_test_vectors::FirmwareInput,
    ) -> (
        u32,
        Vec<pb::BtcPrevTxInputRequest>,
        Vec<pb::BtcPrevTxOutputRequest>,
        u32,
    ) {
        let Some(prev_tx) = input.prev_tx.as_ref() else {
            return (0, vec![], vec![], 0);
        };
        let inputs = prev_tx
            .input
            .iter()
            .map(|input| pb::BtcPrevTxInputRequest {
                prev_out_hash: input.previous_output.txid.to_byte_array().to_vec(),
                prev_out_index: input.previous_output.vout,
                signature_script: input.script_sig.as_bytes().to_vec(),
                sequence: input.sequence.0,
            })
            .collect();
        let outputs = prev_tx
            .output
            .iter()
            .map(|output| pb::BtcPrevTxOutputRequest {
                value: output.value.to_sat(),
                pubkey_script: output.script_pubkey.as_bytes().to_vec(),
            })
            .collect();
        (
            u32::try_from(prev_tx.version.0).unwrap(),
            inputs,
            outputs,
            prev_tx.lock_time.to_consensus_u32(),
        )
    }

    fn vector_input(
        vector: &btc_test_vectors::TestVector,
        input_index: usize,
        input: &btc_test_vectors::FirmwareInput,
    ) -> TxInput {
        let signature = vector
            .expected_signatures
            .iter()
            .find(|signature| signature.input_index == input_index);
        let host_nonce = signature
            .is_some_and(|signature| signature.kind == btc_test_vectors::SignatureKind::Ecdsa)
            .then(|| vec![u8::try_from(input_index + 1).unwrap(); 32]);
        let host_nonce_commitment =
            host_nonce
                .as_ref()
                .map(|host_nonce| pb::AntiKleptoHostNonceCommitment {
                    commitment: bitbox_secp256k1::ecdsa_anti_exfil_host_commit(
                        SECP256K1, host_nonce,
                    )
                    .unwrap(),
                });
        let (prevtx_version, prevtx_inputs, prevtx_outputs, prevtx_locktime) = vector_prevtx(input);

        TxInput {
            input: pb::BtcSignInputRequest {
                prev_out_hash: input.prev_out_hash.to_byte_array().to_vec(),
                prev_out_index: input.prev_out_index,
                prev_out_value: input.prev_out_value,
                sequence: input.sequence,
                keypath: keypath_components(&input.keypath),
                script_config_index: input.script_config_index,
                host_nonce_commitment,
            },
            prevtx_version,
            prevtx_inputs,
            prevtx_outputs,
            prevtx_locktime,
            host_nonce,
        }
    }

    fn vector_output(output: &btc_test_vectors::FirmwareOutput) -> pb::BtcSignOutputRequest {
        let output_type = match output.output_type {
            btc_test_vectors::OutputType::Unknown => pb::BtcOutputType::Unknown,
            btc_test_vectors::OutputType::P2pkh => pb::BtcOutputType::P2pkh,
            btc_test_vectors::OutputType::P2sh => pb::BtcOutputType::P2sh,
            btc_test_vectors::OutputType::P2wpkh => pb::BtcOutputType::P2wpkh,
            btc_test_vectors::OutputType::P2wsh => pb::BtcOutputType::P2wsh,
            btc_test_vectors::OutputType::P2tr => pb::BtcOutputType::P2tr,
            btc_test_vectors::OutputType::OpReturn => pb::BtcOutputType::OpReturn,
        };
        pb::BtcSignOutputRequest {
            ours: output.ours,
            r#type: output_type as _,
            value: output.value,
            payload: output.payload.clone(),
            keypath: output
                .keypath
                .as_ref()
                .map(keypath_components)
                .unwrap_or_default(),
            script_config_index: output.script_config_index,
            payment_request_index: output.payment_request_index,
            silent_payment: output.silent_payment_address.as_ref().map(|address| {
                pb::btc_sign_output_request::SilentPayment {
                    address: address.clone(),
                }
            }),
            output_script_config_index: output.output_script_config_index,
        }
    }

    fn vector_payment_request(
        request: &btc_test_vectors::PaymentRequest,
    ) -> pb::BtcPaymentRequestRequest {
        pb::BtcPaymentRequestRequest {
            recipient_name: request.recipient_name.clone(),
            memos: request
                .memos
                .iter()
                .map(|memo| match memo {
                    btc_test_vectors::PaymentRequestMemo::Text { note } => Memo {
                        memo: Some(memo::Memo::TextMemo(memo::TextMemo { note: note.clone() })),
                    },
                    btc_test_vectors::PaymentRequestMemo::CoinPurchase {
                        coin_type,
                        amount,
                        address,
                        address_keypath,
                    } => Memo {
                        memo: Some(memo::Memo::CoinPurchaseMemo(memo::CoinPurchaseMemo {
                            coin_type: *coin_type,
                            amount: amount.clone(),
                            address: address.clone(),
                            address_derivation: Some(
                                memo::coin_purchase_memo::AddressDerivation::Eth(
                                    memo::coin_purchase_memo::EthAddressDerivation {
                                        keypath: vector_keypath(address_keypath),
                                    },
                                ),
                            ),
                        })),
                    },
                })
                .collect(),
            nonce: hex::decode(&request.nonce).unwrap(),
            total_amount: request.total_amount,
            signature: hex::decode(&request.signature).unwrap(),
        }
    }

    fn vector_transaction(
        vector: &btc_test_vectors::TestVector,
        request: &btc_test_vectors::FirmwareSignRequest,
    ) -> Transaction {
        Transaction {
            coin: vector_coin(vector.coin),
            total_confirmations: 0,
            version: request.version,
            inputs: request
                .inputs
                .iter()
                .enumerate()
                .map(|(index, input)| vector_input(vector, index, input))
                .collect(),
            outputs: request.outputs.iter().map(vector_output).collect(),
            locktime: request.locktime,
            payment_request: request.payment_requests.first().map(vector_payment_request),
        }
    }

    fn vector_init_request(
        vector: &btc_test_vectors::TestVector,
        request: &btc_test_vectors::FirmwareSignRequest,
    ) -> pb::BtcSignInitRequest {
        pb::BtcSignInitRequest {
            coin: vector_coin(vector.coin) as _,
            script_configs: request
                .script_configs
                .iter()
                .map(vector_script_config_with_keypath)
                .collect(),
            output_script_configs: request
                .output_script_configs
                .iter()
                .map(vector_script_config_with_keypath)
                .collect(),
            version: request.version,
            num_inputs: request.inputs.len() as _,
            num_outputs: request.outputs.len() as _,
            locktime: request.locktime,
            format_unit: match request.format_unit {
                btc_test_vectors::FormatUnit::Default => FormatUnit::Default,
                btc_test_vectors::FormatUnit::Sat => FormatUnit::Sat,
            } as _,
            contains_silent_payment_outputs: request
                .outputs
                .iter()
                .any(|output| output.silent_payment_address.is_some()),
        }
    }

    fn register_vector_configs(hal: &mut TestingHal<'_>, vector: &btc_test_vectors::TestVector) {
        let coin = vector_coin(vector.coin);
        for registration in &vector.registrations {
            let config = vector_script_config(&registration.script_config);
            match config.config.as_ref().unwrap() {
                pb::btc_script_config::Config::Multisig(multisig) => {
                    let keypath = vector_keypath(registration.keypath.as_deref().unwrap());
                    let hash = super::super::multisig::get_hash(
                        coin,
                        multisig,
                        super::super::multisig::SortXpubs::Yes,
                        &keypath,
                    )
                    .unwrap();
                    hal.memory
                        .multisig_set_by_hash(&hash, &registration.name)
                        .unwrap();
                }
                pb::btc_script_config::Config::Policy(policy) => {
                    let hash = super::super::policies::get_hash(coin, policy).unwrap();
                    hal.memory
                        .multisig_set_by_hash(&hash, &registration.name)
                        .unwrap();
                }
                pb::btc_script_config::Config::SimpleType(_) => {
                    panic!("simple script configs cannot be registered")
                }
            }
        }
    }

    #[derive(Default)]
    struct VectorObservations {
        pending_request: Option<(NextType, usize)>,
        prevtx_inputs: BTreeSet<usize>,
        antiklepto_inputs: BTreeSet<usize>,
        signatures: BTreeMap<usize, Vec<u8>>,
        generated_outputs: BTreeMap<usize, Vec<u8>>,
    }

    impl VectorObservations {
        fn observe(&mut self, response: &Response) {
            let next = extract_next(response);
            if let Some((request_type, index)) = self.pending_request.take() {
                if next.has_signature {
                    assert!(matches!(
                        request_type,
                        NextType::Input | NextType::HostNonce
                    ));
                    assert!(
                        self.signatures
                            .insert(index, next.signature.clone())
                            .is_none()
                    );
                }
                if !next.generated_output_pkscript.is_empty() {
                    assert_eq!(request_type, NextType::Output);
                    assert!(!next.silent_payment_dleq_proof.is_empty());
                    assert!(
                        self.generated_outputs
                            .insert(index, next.generated_output_pkscript.clone())
                            .is_none()
                    );
                } else {
                    assert!(next.silent_payment_dleq_proof.is_empty());
                }
            }

            let request_type = NextType::try_from(next.r#type).unwrap();
            match request_type {
                NextType::PrevtxInit => {
                    self.prevtx_inputs.insert(next.index as usize);
                }
                NextType::HostNonce => {
                    assert!(next.anti_klepto_signer_commitment.is_some());
                    self.antiklepto_inputs.insert(next.index as usize);
                }
                _ => assert!(next.anti_klepto_signer_commitment.is_none()),
            }
            if request_type != NextType::Done {
                self.pending_request = Some((request_type, next.index as usize));
            }
        }
    }

    fn current_vector_expectation(
        vector: &btc_test_vectors::TestVector,
    ) -> &btc_test_vectors::VersionExpectation {
        let current = semver::Version::parse(
            crate::version::FIRMWARE_VERSION_SHORT
                .strip_prefix('v')
                .unwrap(),
        )
        .unwrap();
        vector
            .expectations
            .iter()
            .find(|expectation| {
                let after_min = expectation
                    .min_version
                    .as_deref()
                    .is_none_or(|min| current >= semver::Version::parse(min).unwrap());
                let before_max = expectation
                    .max_version_exclusive
                    .as_deref()
                    .is_none_or(|max| current < semver::Version::parse(max).unwrap());
                after_min && before_max
            })
            .unwrap()
    }

    fn observed_vector_screens(screens: &[Screen]) -> Vec<btc_test_vectors::Screen> {
        screens
            .iter()
            .map(|screen| match screen {
                Screen::Confirm {
                    title,
                    body,
                    longtouch,
                } => btc_test_vectors::Screen::Confirm {
                    title: title.clone(),
                    body: body.clone(),
                    longtouch: *longtouch,
                },
                Screen::TotalFee {
                    total,
                    fee,
                    longtouch,
                } => btc_test_vectors::Screen::TransactionFee {
                    amount: total.clone(),
                    fee: fee.clone(),
                    longtouch: *longtouch,
                },
                Screen::Swap { title, from, to } => btc_test_vectors::Screen::Swap {
                    title: title.clone(),
                    from: from.clone(),
                    to: to.clone(),
                },
                Screen::Recipient { recipient, amount } => {
                    btc_test_vectors::Screen::TransactionAddress {
                        amount: amount.clone(),
                        address: recipient.clone(),
                    }
                }
                Screen::Status { title, success } => {
                    let (title, body) = title.split_once('\n').unwrap();
                    assert_eq!(*success, body == "confirmed");
                    btc_test_vectors::Screen::Status {
                        title: title.into(),
                        body: body.into(),
                    }
                }
                _ => panic!("unexpected screen in transaction vector: {screen:?}"),
            })
            .collect()
    }

    fn assert_vector_observations(
        vector: &btc_test_vectors::TestVector,
        request: &btc_test_vectors::FirmwareSignRequest,
        observations: &VectorObservations,
    ) {
        assert!(
            observations.pending_request.is_none(),
            "transaction vector '{}' ended with an unanswered request",
            vector.id
        );

        let expected_prevtx_inputs = if vector.expected_needs_prevtxs {
            (0..request.inputs.len()).collect()
        } else {
            BTreeSet::new()
        };
        assert_eq!(
            observations.prevtx_inputs, expected_prevtx_inputs,
            "unexpected previous-transaction requests for vector '{}'",
            vector.id
        );

        let expected_antiklepto_inputs: BTreeSet<_> = vector
            .expected_signatures
            .iter()
            .filter(|signature| signature.kind == btc_test_vectors::SignatureKind::Ecdsa)
            .map(|signature| signature.input_index)
            .collect();
        assert_eq!(
            observations.antiklepto_inputs, expected_antiklepto_inputs,
            "unexpected anti-klepto requests for vector '{}'",
            vector.id
        );

        let expected_signature_inputs: BTreeSet<_> = vector
            .expected_signatures
            .iter()
            .map(|signature| signature.input_index)
            .collect();
        assert_eq!(
            observations
                .signatures
                .keys()
                .copied()
                .collect::<BTreeSet<_>>(),
            expected_signature_inputs,
            "unexpected signature inputs for vector '{}'",
            vector.id
        );
        for expected in &vector.expected_signatures {
            let signature = &observations.signatures[&expected.input_index];
            match expected.kind {
                btc_test_vectors::SignatureKind::Ecdsa => {
                    bitcoin::secp256k1::ecdsa::Signature::from_compact(signature).unwrap();
                    assert_eq!(expected.sighash, btc_test_vectors::Sighash::All);
                }
                btc_test_vectors::SignatureKind::TaprootKey
                | btc_test_vectors::SignatureKind::TaprootScript => {
                    bitcoin::secp256k1::schnorr::Signature::from_slice(signature).unwrap();
                    assert_eq!(expected.sighash, btc_test_vectors::Sighash::Default);
                }
            }
        }

        let generated_outputs = observations
            .generated_outputs
            .iter()
            .map(|(index, output)| (*index, hex::encode(output)))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            generated_outputs, vector.expected_generated_outputs,
            "unexpected generated outputs for vector '{}'",
            vector.id
        );

        assert_vector_signature_fixture(vector, observations);
    }

    fn assert_vector_signature_fixture(
        vector: &btc_test_vectors::TestVector,
        observations: &VectorObservations,
    ) {
        // Pin exact bytes only for representative signing paths. The assertions above cover the
        // signature kind, input, sighash and anti-klepto exchange for every vector.
        // The ECDSA fixture proves that its deterministic host nonce affected the signature;
        // keystore::tests::test_secp256k1_antiklepto_protocol verifies the host-side check.
        let expected = match vector.id.as_str() {
            "high-fee-rounding" => hex!(
                "fe00a74372509991256c691a98e42792ee8eddb2cd57f34af37fd0f1490716b11e15c53bafa7897912c11749955c5b0b2922dc136a32270637606970cb5bf27e"
            ),
            "policy-tr-keyspend-with-script-tree" => hex!(
                "8ea290ef0b90388bc23380e7e017f4c412f84d106f6f55c38bd4e658932c371da64ea2e75147c3dde8bfbc0f124930ebb681a0bc34c4943d89fdcb9dc5de0a5f"
            ),
            "policy-tr-unspendable-internal-key-complex" => hex!(
                "8e9c801a430d7b5b48cd8c37ae42ff150bca17abb0ee6bb4543cf144e5b96ed9a359b3e30c672d75698b99b36a20a0930c754576242c1f95dbc9fa74d93d5ae8"
            ),
            _ => return,
        };
        assert_eq!(
            observations.signatures[&0].as_slice(),
            expected.as_slice(),
            "signature fixture changed for vector '{}'",
            vector.id
        );
    }

    async fn assert_vector_pubkeys(
        hal: &mut TestingHal<'_>,
        vector: &btc_test_vectors::TestVector,
        request: &btc_test_vectors::FirmwareSignRequest,
    ) {
        for expected in &vector.expected_signatures {
            let input = &request.inputs[expected.input_index];
            let keypath = keypath_components(&input.keypath);
            let xpub = crate::keystore::get_xpub(hal, &keypath, crate::keystore::Compute::Once)
                .await
                .unwrap();
            let pubkey = bitcoin::secp256k1::PublicKey::from_slice(xpub.public_key()).unwrap();
            let actual_pubkey = match expected.kind {
                btc_test_vectors::SignatureKind::Ecdsa => hex::encode(pubkey.serialize()),
                btc_test_vectors::SignatureKind::TaprootKey
                | btc_test_vectors::SignatureKind::TaprootScript => {
                    hex::encode(pubkey.x_only_public_key().0.serialize())
                }
            };
            assert_eq!(
                Some(actual_pubkey.as_str()),
                expected.pubkey.as_deref(),
                "unexpected pubkey for input {} of vector '{}'",
                expected.input_index,
                vector.id
            );

            if let Some(expected_bip352_pubkey) = &input.bip352_pubkey {
                let bip352_pubkey = match expected.kind {
                    btc_test_vectors::SignatureKind::Ecdsa => pubkey.serialize().to_vec(),
                    btc_test_vectors::SignatureKind::TaprootKey
                    | btc_test_vectors::SignatureKind::TaprootScript => {
                        // Taproot silent-payment inputs use the tweaked key-spend private key.
                        pubkey
                            .x_only_public_key()
                            .0
                            .tap_tweak(SECP256K1, None)
                            .0
                            .serialize()
                            .to_vec()
                    }
                };
                assert_eq!(
                    bip352_pubkey.as_slice(),
                    expected_bip352_pubkey.as_slice(),
                    "unexpected BIP352 pubkey for input {} of vector '{}'",
                    expected.input_index,
                    vector.id
                );
            }
        }
    }

    #[async_test::test]
    async fn test_transaction_vectors() {
        let vectors = btc_test_vectors::test_vectors();

        for vector in &vectors.vectors {
            let sign_request = btc_test_vectors::derive_sign_request(vector).unwrap();
            let transaction = alloc::rc::Rc::new(core::cell::RefCell::new(vector_transaction(
                vector,
                &sign_request,
            )));
            let observations =
                alloc::rc::Rc::new(core::cell::RefCell::new(VectorObservations::default()));
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = {
                let transaction = transaction.clone();
                let observations = observations.clone();
                Some(Box::new(move |response: Response| {
                    observations.borrow_mut().observe(&response);
                    Ok(transaction.borrow().make_host_request(response))
                }))
            };

            mock_unlocked_using_mnemonic(&vectors.simulator_seed, "");
            let mut mock_hal = TestingHal::new();
            register_vector_configs(&mut mock_hal, vector);
            let expectation = current_vector_expectation(vector);
            let result = process(&mut mock_hal, &vector_init_request(vector, &sign_request)).await;
            assert_eq!(
                observed_vector_screens(&mock_hal.ui.screens).as_slice(),
                expectation.screens.as_slice(),
                "unexpected screens for transaction vector '{}'",
                vector.id
            );

            match expectation.outcome {
                btc_test_vectors::Outcome::Success => {
                    let response = result.unwrap_or_else(|error| {
                        panic!("transaction vector '{}' failed: {error:?}", vector.id)
                    });
                    observations.borrow_mut().observe(&response);
                    assert_vector_observations(vector, &sign_request, &observations.borrow());
                    assert_vector_pubkeys(&mut mock_hal, vector, &sign_request).await;
                }
                btc_test_vectors::Outcome::InvalidInput => {
                    let error = result.expect_err(&format!(
                        "transaction vector '{}' unexpectedly succeeded",
                        vector.id
                    ));
                    assert_eq!(
                        error,
                        Error::InvalidInput,
                        "unexpected error for transaction vector '{}'",
                        vector.id
                    );
                }
                btc_test_vectors::Outcome::Unsupported => panic!(
                    "transaction vector '{}' is unsupported by this firmware version",
                    vector.id
                ),
            }
        }
    }

    #[async_test::test]
    pub async fn test_sign_init_fail() {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = None;

        let init_req_valid = pb::BtcSignInitRequest {
            coin: pb::BtcCoin::Btc as _,
            script_configs: vec![pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::SimpleType(
                        SimpleType::P2wpkh as _,
                    )),
                }),
                keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
            }],
            output_script_configs: vec![],
            version: 1,
            num_inputs: 1,
            num_outputs: 1,
            locktime: 0,
            format_unit: FormatUnit::Default as _,
            contains_silent_payment_outputs: false,
        };

        {
            // test keystore locked
            crate::keystore::lock();
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_valid,).await,
                Err(Error::InvalidState)
            );
        }

        mock_unlocked();
        {
            // test invalid format unit
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.coin = pb::BtcCoin::Ltc as _;
            init_req_invalid.format_unit = FormatUnit::Sat as _;
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid version
            let mut init_req_invalid = init_req_valid.clone();
            for version in 3..10 {
                init_req_invalid.version = version;
                assert_eq!(
                    process(&mut TestingHal::new(), &init_req_invalid).await,
                    Err(Error::InvalidInput)
                );
            }
        }
        {
            // test invalid locktime
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.locktime = 500000000;
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid inputs
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.num_inputs = 0;
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid outputs
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.num_outputs = 0;
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid coin
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.coin = 4; // BtcCoin is defined from 0 to 3.
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid account keypath
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs[0].keypath[2] = HARDENED + 100;
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // no script configs is invalid
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![];
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // can't mix script configs from different bip44 accounts
            // (mixing input scripts is allowed, but only from the same bip44 account).
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                },
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            SimpleType::P2wpkhP2sh as _,
                        )),
                    }),
                    keypath: vec![49 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                },
            ];
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }

        {
            // can't mix simple type (singlesig) and multisig configs in one tx

            mock_unlocked_using_mnemonic(
                "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
                "",
            );

            let params = super::super::params::get(pb::BtcCoin::Btc);
            let keypath = &[48 + HARDENED, params.bip44_coin, 0 + HARDENED, 2 + HARDENED];
            let multisig = pb::btc_script_config::Multisig {
                threshold: 1,
                xpubs: vec![
                    crate::keystore::get_xpub(
                        &mut TestingHal::new(),
                        keypath,
                        crate::keystore::Compute::Once,
                    )
                    .await
                    .unwrap()
                    .into(),
                    parse_xpub("xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM7wT3KxLKX8Q8keZPd67kRGmrD1WJj").unwrap(),
                ],
                our_xpub_index: 0,
                script_type: pb::btc_script_config::multisig::ScriptType::P2wsh
                    as _,
            };

            let mut mock_hal = TestingHal::new();

            // Register multisig.
            let hash = super::super::multisig::get_hash(
                params.coin,
                &multisig,
                super::super::multisig::SortXpubs::Yes,
                keypath,
            )
            .unwrap();
            mock_hal
                .memory
                .multisig_set_by_hash(&hash, "test name")
                .unwrap();

            assert!(
                super::super::multisig::validate(&mut mock_hal, &multisig, keypath)
                    .await
                    .is_ok()
            );

            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            pb::btc_script_config::SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                },
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::Multisig(multisig)),
                    }),
                    keypath: keypath.to_vec(),
                },
            ];
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
        {
            // no taproot in Litecoin
            assert_eq!(
                process(
                    &mut TestingHal::new(),
                    &pb::BtcSignInitRequest {
                        coin: pb::BtcCoin::Ltc as _,
                        script_configs: vec![pb::BtcScriptConfigWithKeypath {
                            script_config: Some(pb::BtcScriptConfig {
                                config: Some(pb::btc_script_config::Config::SimpleType(
                                    SimpleType::P2tr as _,
                                )),
                            }),
                            keypath: vec![84 + HARDENED, 2 + HARDENED, 10 + HARDENED],
                        }],
                        output_script_configs: vec![],
                        version: 1,
                        num_inputs: 1,
                        num_outputs: 1,
                        locktime: 0,
                        format_unit: FormatUnit::Default as _,
                        contains_silent_payment_outputs: false,
                    }
                )
                .await,
                Err(Error::InvalidInput)
            );
        }
        {
            // invalid output script config (invalid keypath)
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.output_script_configs = vec![pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::SimpleType(
                        SimpleType::P2wpkh as _,
                    )),
                }),
                keypath: vec![],
            }];
            assert_eq!(
                process(&mut TestingHal::new(), &init_req_invalid).await,
                Err(Error::InvalidInput)
            );
        }
    }

    /// Test that receiving an unexpected message from the host results in an invalid state error.
    #[async_test::test]
    pub async fn test_invalid_state() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        mock_unlocked();
        let tx = transaction.clone();
        static mut COUNTER: u32 = 0;
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() =
            Some(Box::new(move |_response: Response| {
                unsafe { COUNTER += 1 }
                // The first input is only expected once, the other times other parts of the
                // transaction are expected.
                Ok(Request::BtcSignInput(tx.borrow().inputs[0].input.clone()))
            }));

        let init_request = transaction.borrow().init_request();
        let result = process(&mut TestingHal::new(), &init_request).await;
        assert_eq!(result, Err(Error::InvalidState));
        assert_eq!(unsafe { COUNTER }, 2);
    }

    /// Test invalid input cases.
    #[async_test::test]
    pub async fn test_invalid_input() {
        enum TestCase {
            // all inputs should be the same coin type.
            WrongCoinInput,
            // all change outputs should be the same coin type.
            WrongCoinChange,
            // all inputs should be from the same account.
            WrongAccountInput,
            // all change outputs should go the same account.
            WrongAccountChange,
            // change num in bip44, should be 1.
            WrongBip44Change(u32),
            // change address index in bip44, should be <10000
            WrongBip44ChangeAddressTooHigh,
            // referenced script config does not exist.
            InvalidInputScriptConfigIndex,
            // referenced script config does not exist.
            InvalidChangeScriptConfigIndex,
            // value 0 is invalid
            WrongOutputValue,
            // input value does not match prevtx output value
            WrongInputValue,
            // input's prevtx hash does not match input's prevOutHash
            WrongPrevoutHash,
            // input's prev_out_index too high
            WrongPrevoutIndex,
            // no inputs in prevtx
            PrevTxNoInputs,
            // no outputs in prevtx
            PrevTxNoOutputs,
        }
        for value in [
            TestCase::WrongCoinInput,
            TestCase::WrongCoinChange,
            TestCase::WrongAccountInput,
            TestCase::WrongAccountChange,
            TestCase::WrongBip44Change(2),
            TestCase::WrongBip44ChangeAddressTooHigh,
            TestCase::InvalidInputScriptConfigIndex,
            TestCase::InvalidChangeScriptConfigIndex,
            TestCase::WrongOutputValue,
            TestCase::WrongInputValue,
            TestCase::WrongPrevoutHash,
            TestCase::WrongPrevoutIndex,
            TestCase::PrevTxNoInputs,
            TestCase::PrevTxNoOutputs,
        ] {
            let transaction =
                alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
            match value {
                TestCase::WrongCoinInput => {
                    transaction.borrow_mut().inputs[0].input.keypath[1] = 1 + HARDENED;
                }
                TestCase::WrongCoinChange => {
                    assert!(transaction.borrow().outputs[4].ours);
                    transaction.borrow_mut().outputs[4].keypath[1] = 1 + HARDENED;
                }
                TestCase::WrongAccountInput => {
                    transaction.borrow_mut().inputs[0].input.keypath[2] += 1;
                }
                TestCase::WrongAccountChange => {
                    assert!(transaction.borrow().outputs[4].ours);
                    transaction.borrow_mut().outputs[4].keypath[2] += 1;
                }
                TestCase::WrongBip44Change(change) => {
                    assert!(transaction.borrow().outputs[4].ours);
                    transaction.borrow_mut().outputs[4].keypath[3] = change;
                }
                TestCase::WrongBip44ChangeAddressTooHigh => {
                    assert!(transaction.borrow().outputs[4].ours);
                    transaction.borrow_mut().outputs[4].keypath[4] = 10000;
                }
                TestCase::InvalidInputScriptConfigIndex => {
                    transaction.borrow_mut().inputs[0].input.script_config_index = 1;
                }
                TestCase::InvalidChangeScriptConfigIndex => {
                    assert!(transaction.borrow().outputs[4].ours);
                    transaction.borrow_mut().outputs[4].script_config_index = 1;
                }
                TestCase::WrongOutputValue => {
                    transaction.borrow_mut().outputs[0].value = 0;
                }
                TestCase::WrongInputValue => {
                    transaction.borrow_mut().inputs[0].input.prev_out_value += 1;
                }
                TestCase::WrongPrevoutHash => {
                    transaction.borrow_mut().inputs[0].input.prev_out_hash[0] += 1;
                }
                TestCase::WrongPrevoutIndex => {
                    let mut tx = transaction.borrow_mut();
                    tx.inputs[0].input.prev_out_index = tx.inputs[0].prevtx_outputs.len() as _;
                }
                TestCase::PrevTxNoInputs => {
                    transaction.borrow_mut().inputs[0].prevtx_inputs.clear();
                }
                TestCase::PrevTxNoOutputs => {
                    transaction.borrow_mut().inputs[0].prevtx_outputs.clear();
                }
            }
            mock_host_responder(transaction.clone());
            mock_unlocked();
            let init_request = transaction.borrow().init_request();
            let result = process(&mut TestingHal::new(), &init_request).await;
            assert_eq!(result, Err(Error::InvalidInput));
        }
    }

    #[async_test::test]
    async fn test_user_aborts() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        mock_host_responder(transaction.clone());
        // We go through all possible user confirmations and abort one of them at a time.
        let total_confirmations = transaction.borrow().total_confirmations;
        for counter in 0..total_confirmations {
            let mut mock_hal = TestingHal::new();
            mock_hal.ui.abort_nth(counter as usize);
            mock_unlocked();
            let init_request = transaction.borrow().init_request();
            assert_eq!(
                process(&mut mock_hal, &init_request).await,
                Err(Error::UserAbort)
            );
        }
    }

    /// The sum of the inputs in the 2nd pass can't be higher than in the first for all inputs.
    #[async_test::test]
    async fn test_input_sum_changes() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        static mut PASS2_INPUT_REQUESTS_COUNTER: u32 = 0;
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = {
            let tx = transaction.clone();
            static mut PASS2: bool = false;
            Some(Box::new(move |response: Response| {
                let tx = tx.borrow();
                let next = extract_next(&response);
                match NextType::try_from(next.r#type).unwrap() {
                    NextType::Output => unsafe { PASS2 = true },
                    NextType::Input if unsafe { PASS2 } => {
                        unsafe { PASS2_INPUT_REQUESTS_COUNTER += 1 }
                        if next.index == 0 {
                            let mut input = tx.inputs[next.index as usize].input.clone();
                            // Amount in first input in pass2 is bigger than the the sum of all
                            // inputs in the first pass, which fails immediately.
                            input.prev_out_value = tx
                                .inputs
                                .iter()
                                .map(|inp| inp.input.prev_out_value)
                                .sum::<u64>()
                                + 1;
                            return Ok(Request::BtcSignInput(input));
                        }
                    }
                    _ => {}
                }
                Ok(tx.make_host_request(response))
            }))
        };
        mock_unlocked();
        let init_request = transaction.borrow().init_request();
        let result = process(&mut TestingHal::new(), &init_request).await;
        assert_eq!(result, Err(Error::InvalidInput));
        // Only one input in the 2nd pass was requested, meaning the process failed after validating
        // the amount in the first input.
        assert_eq!(unsafe { PASS2_INPUT_REQUESTS_COUNTER }, 1);
    }

    /// At the last input, the sum of the inputs in the 2nd pass must be the same as the sum of the
    /// inputs in the first pass.
    #[async_test::test]
    async fn test_input_sum_last_mismatch() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        static mut PASS2_INPUT_REQUESTS_COUNTER: u32 = 0;
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = {
            let tx = transaction.clone();
            static mut PASS2: bool = false;
            Some(Box::new(move |response: Response| {
                let tx = tx.borrow();
                let next = extract_next(&response);
                match NextType::try_from(next.r#type).unwrap() {
                    NextType::Output => unsafe { PASS2 = true },
                    NextType::Input if unsafe { PASS2 } => {
                        unsafe { PASS2_INPUT_REQUESTS_COUNTER += 1 }
                        if next.index == 0 {
                            let mut input = tx.inputs[next.index as usize].input.clone();
                            // errors even if we decrease the amount
                            input.prev_out_value -= 1;
                            return Ok(Request::BtcSignInput(input));
                        }
                    }
                    _ => {}
                }
                Ok(tx.make_host_request(response))
            }))
        };
        mock_unlocked();
        let init_request = transaction.borrow().init_request();
        let result = process(&mut TestingHal::new(), &init_request).await;
        assert_eq!(result, Err(Error::InvalidInput));
        // All inputs were requested, the failure happens when comparing the sums of the two passes
        // at the end.
        assert_eq!(
            unsafe { PASS2_INPUT_REQUESTS_COUNTER },
            transaction.borrow().inputs.len() as u32
        );
    }

    /// Outgoing sum overflows.
    #[async_test::test]
    async fn test_overflow_output_out() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = {
            let tx = transaction.clone();
            Some(Box::new(move |response: Response| {
                let tx = tx.borrow();
                let next = extract_next(&response);
                match NextType::try_from(next.r#type).unwrap() {
                    NextType::Output => {
                        let mut output = tx.outputs[next.index as usize].clone();
                        if next.index == 0 {
                            assert!(!output.ours);
                            output.value = u64::MAX;
                        }
                        Ok(Request::BtcSignOutput(output))
                    }
                    _ => Ok(tx.make_host_request(response)),
                }
            }))
        };
        mock_unlocked();
        let init_request = transaction.borrow().init_request();
        let result = process(&mut TestingHal::new(), &init_request).await;
        assert_eq!(result, Err(Error::InvalidInput));
    }

    /// Outgoing change overflows.
    #[async_test::test]
    async fn test_overflow_output_ours() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = {
            let tx = transaction.clone();
            Some(Box::new(move |response: Response| {
                let tx = tx.borrow();
                let next = extract_next(&response);
                match NextType::try_from(next.r#type).unwrap() {
                    NextType::Output => {
                        let mut output = tx.outputs[next.index as usize].clone();
                        if next.index == 4 {
                            assert!(output.ours);
                            output.value = u64::MAX;
                        }
                        Ok(Request::BtcSignOutput(output))
                    }
                    _ => Ok(tx.make_host_request(response)),
                }
            }))
        };
        mock_unlocked();
        let init_request = transaction.borrow().init_request();
        let result = process(&mut TestingHal::new(), &init_request).await;
        assert_eq!(result, Err(Error::InvalidInput));
    }

    #[cfg(feature = "app-ethereum")]
    #[test]
    pub fn test_validate_swap_source_account() {
        // Swap payment requests are only supported for BTC/LTC source accounts,
        // and only when the selected source config is simple single-sig.
        let keypath = &[84 + HARDENED, 0 + HARDENED, 10 + HARDENED];
        let multisig_pb = pb::btc_script_config::Multisig {
            threshold: 1,
            xpubs: vec![],
            our_xpub_index: 0,
            script_type: pb::btc_script_config::multisig::ScriptType::P2wsh as _,
        };
        let singlesig_account = [ValidatedScriptConfigWithKeypath {
            keypath,
            config: ValidatedScriptConfig::SimpleType(SimpleType::P2wpkh),
        }];
        let mixed_singlesig_accounts = [
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(SimpleType::P2wpkh),
            },
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(SimpleType::P2tr),
            },
        ];
        let multisig_account = [ValidatedScriptConfigWithKeypath {
            keypath,
            config: ValidatedScriptConfig::Multisig {
                name: "test multisig".into(),
                multisig: &multisig_pb,
            },
        }];
        let mixed_accounts = [
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::SimpleType(SimpleType::P2wpkh),
            },
            ValidatedScriptConfigWithKeypath {
                keypath,
                config: ValidatedScriptConfig::Multisig {
                    name: "test multisig".into(),
                    multisig: &multisig_pb,
                },
            },
        ];

        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Btc, &singlesig_account),
            Ok(())
        );
        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Ltc, &singlesig_account),
            Ok(())
        );
        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Btc, &mixed_singlesig_accounts),
            Ok(())
        );
        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Tbtc, &singlesig_account),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Btc, &multisig_account),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_swap_source_account(pb::BtcCoin::Btc, &mixed_accounts),
            Err(Error::InvalidInput)
        );
    }

    #[async_test::test]
    async fn test_op_return_rejects_ours_output() {
        let transaction =
            alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new(pb::BtcCoin::Btc)));

        {
            let mut tx = transaction.borrow_mut();
            let output_index = 5;
            assert!(tx.outputs[output_index].ours);
            tx.outputs[output_index].r#type = pb::BtcOutputType::OpReturn as _;
            tx.outputs[output_index].value = 0;
            tx.outputs[output_index].payload = b"hello world".to_vec();
            tx.outputs[output_index].keypath[3] = 0;
        }

        mock_host_responder(transaction.clone());
        mock_unlocked();
        let init_request = transaction.borrow().init_request();

        assert_eq!(
            process(&mut TestingHal::new(), &init_request).await,
            Err(Error::InvalidInput)
        );
    }
}
