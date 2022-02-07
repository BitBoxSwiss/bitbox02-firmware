// Copyright 2019 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "btc_sign.h"
#include "btc_common.h"
#include "btc_params.h"
#include "btc_sign_validate.h"
#include "btc_ui.h"

#include <rust/rust.h>

#include <hardfault.h>
#include <keystore.h>
#include <keystore/keystore_antiklepto.h>
#include <ui/screen_stack.h>
#include <util.h>

#include <wally_script.h>
#include <wally_transaction.h>

#include <pb_decode.h>

static const app_btc_coin_params_t* _coin_params = NULL;

// Inputs and changes must be of a type defined in _init_request.script_configs.
// Inputs and changes keypaths must have the prefix as defined in the referenced script_config..
static BTCSignInitRequest _init_request = {0};

// used during the first pass through the inputs
static void* _hash_prevouts_ctx = NULL;
static void* _hash_sequence_ctx = NULL;
// By the end of the first pass through the inputs, will contain the prevouts hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 2.
static uint8_t _hash_prevouts[32] = {0};
// By the end of the first pass through the inputs, will contain the sequence hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 3.
static uint8_t _hash_sequence[32] = {0};

// used during processing of the outputs.
static void* _hash_outputs_ctx = NULL;
// By the end of processing the outputs, will contain the hashOutputs hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 8.
static uint8_t _hash_outputs[32] = {0};

// Must be called in any code path that exits the signing process (error or regular finish).
static void _reset(void)
{
    _coin_params = NULL;
    util_zero(&_init_request, sizeof(_init_request));

    rust_sha256_free(&_hash_prevouts_ctx);
    _hash_prevouts_ctx = rust_sha256_new();

    rust_sha256_free(&_hash_sequence_ctx);
    _hash_sequence_ctx = rust_sha256_new();

    rust_sha256_free(&_hash_outputs_ctx);
    _hash_outputs_ctx = rust_sha256_new();

    keystore_antiklepto_clear();
}

static app_btc_result_t _error(app_btc_result_t err)
{
    _reset();
    return err;
}

app_btc_result_t app_btc_sign_init(const BTCSignInitRequest* request)
{
    app_btc_result_t result = app_btc_sign_validate_init_script_configs(
        request->coin, request->script_configs, request->script_configs_count);
    if (result != APP_BTC_OK) {
        return _error(result);
    }
    _reset();
    const app_btc_coin_params_t* coin_params = app_btc_params_get(request->coin);
    if (coin_params == NULL) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    _coin_params = coin_params;
    _init_request = *request;
    return APP_BTC_OK;
}

static bool _is_valid_keypath(
    const uint32_t* keypath_account,
    size_t keypath_account_count,
    const uint32_t* keypath,
    size_t keypath_count,
    const BTCScriptConfig* script_config,
    uint32_t expected_bip44_coin,
    bool must_be_change)
{
    switch (script_config->which_config) {
    case BTCScriptConfig_simple_type_tag:
        if (!btc_common_is_valid_keypath_address_simple(
                script_config->config.simple_type, keypath, keypath_count, expected_bip44_coin)) {
            return false;
        }
        break;
    case BTCScriptConfig_multisig_tag:
        if (!btc_common_is_valid_keypath_address_multisig(
                script_config->config.multisig.script_type,
                keypath,
                keypath_count,
                expected_bip44_coin)) {
            return false;
        }
        break;
    default:
        return false;
    }

    // check that keypath_account is a prefix to keypath with two elements left (change, address).
    if (keypath_account_count + 2 != keypath_count) {
        return false;
    }
    for (size_t i = 0; i < keypath_account_count; i++) {
        if (keypath_account[i] != keypath[i]) {
            return false;
        }
    }

    const uint32_t change = keypath[keypath_count - 2];
    if (must_be_change && change != 1) {
        return false;
    }
    return true;
}

static app_btc_result_t _validate_input(const BTCSignInputRequest* request)
{
    // relative locktime and sequence nummbers < 0xffffffff-2 are not supported
    if (request->sequence < 0xffffffff - 2) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    if (request->prevOutValue == 0) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (request->script_config_index >= _init_request.script_configs_count) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    const BTCScriptConfigWithKeypath* script_config_account =
        &_init_request.script_configs[request->script_config_index];

    if (!_is_valid_keypath(
            script_config_account->keypath,
            script_config_account->keypath_count,
            request->keypath,
            request->keypath_count,
            &script_config_account->script_config,
            _coin_params->bip44_coin,
            false)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_input_pass1(const BTCSignInputRequest* request, bool last)
{
    app_btc_result_t result = _validate_input(request);
    if (result != APP_BTC_OK) {
        return _error(result);
    }

    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 2: accumulate hashPrevouts
        // ANYONECANPAY not supported.
        rust_sha256_update(_hash_prevouts_ctx, request->prevOutHash, 32);
        // assumes little endian environment.
        rust_sha256_update(_hash_prevouts_ctx, &request->prevOutIndex, 4);
    }
    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 3: accumulate hashSequence
        // only SIGHASH_ALL supported.

        // assumes little endian environment.
        rust_sha256_update(_hash_sequence_ctx, &request->sequence, 4);
    }

    if (last) {
        // Done with inputs pass 1.

        rust_sha256_finish(&_hash_prevouts_ctx, _hash_prevouts);
        // hash hash_prevouts to produce the final double-hash
        rust_sha256(_hash_prevouts, 32, _hash_prevouts);

        rust_sha256_finish(&_hash_sequence_ctx, _hash_sequence);
        // hash hash_sequence to produce the final double-hash
        rust_sha256(_hash_sequence, 32, _hash_sequence);
    }
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_input_pass2(
    const BTCSignInputRequest* request,
    uint8_t* sig_out,
    uint8_t* anti_klepto_signer_commitment_out)
{
    app_btc_result_t result = _validate_input(request);
    if (result != APP_BTC_OK) {
        return _error(result);
    }

    { // Sign input.
        uint8_t pubkey_hash160[HASH160_LEN];
        UTIL_CLEANUP_20(pubkey_hash160);
        if (!keystore_secp256k1_pubkey_hash160(
                request->keypath, request->keypath_count, pubkey_hash160)) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }

        // A little more than the max pk script for the data push varint.
        uint8_t sighash_script[MAX_PK_SCRIPT_SIZE + MAX_VARINT_SIZE] = {0};
        size_t sighash_script_size = sizeof(sighash_script);

        const BTCScriptConfig* script_config_account =
            &_init_request.script_configs[request->script_config_index].script_config;

        switch (script_config_account->which_config) {
        case BTCScriptConfig_simple_type_tag:
            if (!btc_common_sighash_script_from_pubkeyhash(
                    script_config_account->config.simple_type,
                    pubkey_hash160,
                    sighash_script,
                    &sighash_script_size)) {
                return _error(APP_BTC_ERR_INVALID_INPUT);
            }
            break;
        case BTCScriptConfig_multisig_tag: {
            uint8_t sighash_script_tmp[MAX_PK_SCRIPT_SIZE] = {0};
            sighash_script_size = sizeof(sighash_script_tmp);
            if (!btc_common_pkscript_from_multisig(
                    &script_config_account->config.multisig,
                    request->keypath[request->keypath_count - 2],
                    request->keypath[request->keypath_count - 1],
                    sighash_script_tmp,
                    &sighash_script_size)) {
                return _error(APP_BTC_ERR_INVALID_INPUT);
            }
            if (wally_varbuff_to_bytes(
                    sighash_script_tmp,
                    sighash_script_size,
                    sighash_script,
                    sizeof(sighash_script),
                    &sighash_script_size) != WALLY_OK) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }
            break;
        }
        default:
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
        uint8_t sighash[32] = {0};
        // construct hash to sign
        const Bip143Args bip143_args = {
            .version = _init_request.version,
            .hash_prevouts = _hash_prevouts,
            .hash_sequence = _hash_sequence,
            .outpoint_hash = request->prevOutHash,
            .outpoint_index = request->prevOutIndex,
            .sighash_script = rust_util_bytes(sighash_script, sighash_script_size),
            .prevout_value = request->prevOutValue,
            .sequence = request->sequence,
            .hash_outputs = _hash_outputs,
            .locktime = _init_request.locktime,
            .sighash_flags = WALLY_SIGHASH_ALL,
        };
        rust_bitcoin_bip143_sighash(&bip143_args, rust_util_bytes_mut(sighash, sizeof(sighash)));

        // Engage in the Anti-Klepto protocol if the host sends a host nonce commitment.
        if (request->has_host_nonce_commitment) {
            if (!keystore_antiklepto_secp256k1_commit(
                    request->keypath,
                    request->keypath_count,
                    sighash,
                    request->host_nonce_commitment.commitment,
                    anti_klepto_signer_commitment_out)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }

            return APP_BTC_OK;
        }

        // Return signature directly without the anti-klepto protocol, for backwards compatibility.
        uint8_t empty_nonce_contribution[32] = {0}; // no nonce contribution given by host.
        uint8_t signature[64] = {0};
        if (!keystore_secp256k1_sign(
                request->keypath,
                request->keypath_count,
                sighash,
                empty_nonce_contribution,
                signature,
                NULL)) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        memcpy(sig_out, signature, sizeof(signature));
    }
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_output(const BTCSignOutputRequest* request, bool last)
{
    if (request->script_config_index >= _init_request.script_configs_count) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    const BTCScriptConfigWithKeypath* script_config_account =
        &_init_request.script_configs[request->script_config_index];

    // get payload. If request->ours=true, we compute the payload
    // from the keystore, otherwise it is provided in request->payload.

    uint8_t payload_bytes[sizeof(request->payload.bytes)] = {0};
    size_t payload_size;

    BTCOutputType output_type;
    if (request->ours) {
        if (!_is_valid_keypath(
                script_config_account->keypath,
                script_config_account->keypath_count,
                request->keypath,
                request->keypath_count,
                &script_config_account->script_config,
                _coin_params->bip44_coin,
                true)) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

        switch (script_config_account->script_config.which_config) {
        case BTCScriptConfig_simple_type_tag: {
            uint8_t pubkey_hash160[HASH160_LEN];
            UTIL_CLEANUP_20(pubkey_hash160);
            if (!keystore_secp256k1_pubkey_hash160(
                    request->keypath, request->keypath_count, pubkey_hash160)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }

            // construct pkScript
            if (!btc_common_payload_from_pubkeyhash(
                    script_config_account->script_config.config.simple_type,
                    pubkey_hash160,
                    payload_bytes,
                    &payload_size)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }
            output_type = btc_common_determine_output_type(
                script_config_account->script_config.config.simple_type);
            break;
        }
        case BTCScriptConfig_multisig_tag:
            if (!btc_common_payload_from_multisig(
                    &script_config_account->script_config.config.multisig,
                    request->keypath[request->keypath_count - 2],
                    request->keypath[request->keypath_count - 1],
                    payload_bytes,
                    &payload_size)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }
            output_type = btc_common_determine_output_type_multisig(
                &script_config_account->script_config.config.multisig);
            break;
        default:
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

    } else {
        payload_size = request->payload.size;
        memcpy(payload_bytes, request->payload.bytes, payload_size);
        output_type = request->type;
    }
    if (request->value == 0) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    if (!request->ours) {
        char address[100] = {0};
        // assemble address to display, get user confirmation
        if (!btc_common_address_from_payload(
                _coin_params, output_type, payload_bytes, payload_size, address, sizeof(address))) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

        // Verify output if it is not a change output.
        char formatted_value[100] = {0};
        rust_bitcoin_util_format_amount(
            request->value,
            rust_util_cstr(_coin_params->unit),
            rust_util_cstr_mut(formatted_value, sizeof(formatted_value)));

        // This call blocks.
        if (!app_btc_ui()->verify_recipient(address, formatted_value)) {
            return _error(APP_BTC_ERR_USER_ABORT);
        }
    }

    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 8: accumulate hashOutputs
        // only SIGHASH_ALL supported.

        // create pk_script
        uint8_t pk_script[MAX_PK_SCRIPT_SIZE] = {0};
        size_t pk_script_len = sizeof(pk_script);
        if (!btc_common_pkscript_from_payload(
                _coin_params,
                output_type,
                payload_bytes,
                payload_size,
                pk_script,
                &pk_script_len)) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

        // assumes little endian environment.
        rust_sha256_update(_hash_outputs_ctx, &request->value, 8);
        uint8_t pk_script_serialized[sizeof(pk_script) + 8] = {0};
        size_t pk_script_serialized_len;
        if (wally_varbuff_to_bytes(
                pk_script,
                pk_script_len,
                pk_script_serialized,
                sizeof(pk_script_serialized),
                &pk_script_serialized_len) != WALLY_OK) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        rust_sha256_update(_hash_outputs_ctx, pk_script_serialized, pk_script_serialized_len);
    }

    if (last) {
        rust_sha256_finish(&_hash_outputs_ctx, _hash_outputs);
        // hash hash_outputs to produce the final double-hash
        rust_sha256(_hash_outputs, 32, _hash_outputs);
    }
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_antiklepto(
    const AntiKleptoSignatureRequest* request,
    uint8_t* sig_out)
{
    if (!keystore_antiklepto_secp256k1_sign(request->host_nonce, sig_out, NULL)) {
        return APP_BTC_ERR_UNKNOWN;
    }
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_init_wrapper(in_buffer_t request_buf)
{
    pb_istream_t in_stream = pb_istream_from_buffer(request_buf.data, request_buf.len);
    BTCSignInitRequest request = {0};
    if (!pb_decode(&in_stream, BTCSignInitRequest_fields, &request)) {
        return _error(APP_BTC_ERR_UNKNOWN);
    }
    return app_btc_sign_init(&request);
}

app_btc_result_t app_btc_sign_input_pass1_wrapper(in_buffer_t request_buf, bool last)
{
    pb_istream_t in_stream = pb_istream_from_buffer(request_buf.data, request_buf.len);
    BTCSignInputRequest request = {0};
    if (!pb_decode(&in_stream, BTCSignInputRequest_fields, &request)) {
        return _error(APP_BTC_ERR_UNKNOWN);
    }
    return app_btc_sign_input_pass1(&request, last);
}

app_btc_result_t app_btc_sign_output_wrapper(in_buffer_t request_buf, bool last)
{
    pb_istream_t in_stream = pb_istream_from_buffer(request_buf.data, request_buf.len);
    BTCSignOutputRequest request = {0};
    if (!pb_decode(&in_stream, BTCSignOutputRequest_fields, &request)) {
        return _error(APP_BTC_ERR_UNKNOWN);
    }
    return app_btc_sign_output(&request, last);
}

app_btc_result_t app_btc_sign_input_pass2_wrapper(
    in_buffer_t request_buf,
    uint8_t* sig_out,
    uint8_t* anti_klepto_signer_commitment_out)
{
    pb_istream_t in_stream = pb_istream_from_buffer(request_buf.data, request_buf.len);
    BTCSignInputRequest request = {0};
    if (!pb_decode(&in_stream, BTCSignInputRequest_fields, &request)) {
        return _error(APP_BTC_ERR_UNKNOWN);
    }
    return app_btc_sign_input_pass2(&request, sig_out, anti_klepto_signer_commitment_out);
}

app_btc_result_t app_btc_sign_antiklepto_wrapper(in_buffer_t request_buf, uint8_t* sig_out)
{
    pb_istream_t in_stream = pb_istream_from_buffer(request_buf.data, request_buf.len);
    AntiKleptoSignatureRequest request = {0};
    if (!pb_decode(&in_stream, AntiKleptoSignatureRequest_fields, &request)) {
        return _error(APP_BTC_ERR_UNKNOWN);
    }
    return app_btc_sign_antiklepto(&request, sig_out);
}

void app_btc_sign_reset(void)
{
    _reset();
}
