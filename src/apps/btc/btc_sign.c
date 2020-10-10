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
#include "btc_bip143.h"
#include "btc_common.h"
#include "btc_params.h"
#include "btc_sign_validate.h"
#include "confirm_locktime_rbf.h"

#include <rust/rust.h>

#include <hardfault.h>
#include <keystore.h>
#include <ui/components/empty.h>
#include <ui/components/progress.h>
#include <ui/screen_stack.h>
#include <util.h>
#include <workflow/confirm.h>
#include <workflow/verify_recipient.h>
#include <workflow/verify_total.h>

#include <wally_script.h>
#include <wally_transaction.h>

// Singing flow:
//
// init
// for each input:
//    inputs_pass1
//    prevtx init
//    for each prevtx input:
//        prevtx inputs
//    for each prevtx output:
//        prevtx outputs
// for each output:
//    outputs
// for each input:
//    inputs_pass2
//
// The hash_prevout and hash_sequence and total_in are accumulated in inputs_pass1.
//
// For each input in pass1, the input's prevtx is streamed to compute and compare the prevOutHash
// and input amount.
//
// For each output, the recipient is confirmed. At the last output, the total out, fee, locktime/RBF
// are confirmed.
//
// The inputs are signed in inputs_pass2.
//
// IMPORTANT assumptions:
//
// - In the 2nd pass, if the inputs provided by the host are not the same as in the 1st pass,
//   nothing bad will happen because the sighash uses the prevout and sequence hashes from the first
//   pass, and the value from the 2nd pass. The BTC consensus rules will reject the tx if there is a
//   mismatch.
//
// - Only SIGHASH_ALL. Other sighash types must be carefully studied and might not be secure with
//   the above flow or the above assumption.
typedef enum {
    STATE_INIT,
    STATE_PREVTX_INIT,
    STATE_PREVTX_INPUTS,
    STATE_PREVTX_OUTPUTS,
    STATE_INPUTS_PASS1,
    STATE_OUTPUTS,
    STATE_INPUTS_PASS2,
} _signing_state_t;

// Base component on the screen stack during signing, which is shown while the device is waiting for
// the next signing api call. Without this, the 'See the BitBoxApp' waiting screen would flicker in
// between user confirmations.
//
// Pushed with the first output, and, popped in with the last output (and _reset(), which is called
// if there is an error or if the signing finishes normally).
// Rationale: the earliest user input happens in the first output, the latest in the last output.
static component_t* _empty_component = NULL;
// The progress component is shown when streaming inputs and previous transactions.  Pushed in sign
// init, popped the last prevtx output, which is the last part of the inputs streaming of pass1.
//
// It is also shown when streaming the inputs in the 2nd pass, when signing the inputs, pushed with
// the last output.
static component_t* _progress_component = NULL;

static _signing_state_t _state = STATE_INIT;
static const app_btc_coin_params_t* _coin_params = NULL;

// Inputs and changes must be of a type defined in _init_request.script_configs.
// Inputs and changes keypaths must have the prefix as defined in the referenced script_config..
static BTCSignInitRequest _init_request = {0};

// Current input or output being processed.
static uint32_t _index;
static enum apps_btc_rbf_flag _rbf;
static bool _locktime_applies;

// State of previous transaction during STATE_PREVTX_*.
//
// The state valid from STATE_PREVTX_INIT until the input referencing this tx is done in
// STATE_INPUTS_PASS1.
static struct {
    // Set for the duration of STATE_PREVTX_*.
    BTCPrevTxInitRequest init_request;
    // Current index of input/output of the previous transaction, referenced by the current input at
    // `_index`.
    uint32_t index;

    // Hash accumulator for the whole transaction.
    void* tx_hash_ctx;

    // TODO: prev tx hash accumulator to compute the prev tx ID.

    // The input that referenced this prev tx. Will be used to validate the supplied input value.
    BTCSignInputRequest referencing_input;
} _prevtx;

// used during STATE_INPUTS_PASS1. Will contain the sum of all spent output
// values.
static uint64_t _inputs_sum_pass1 = 0;
// used during STATE_INPUTS_PASS2. Can't exceed _inputs_sum_pass1.
static uint64_t _inputs_sum_pass2 = 0;
// used during STATE_OUTPUTS. Will contain the sum of all our output values
// (change or receive to self).
static uint64_t _outputs_sum_ours = 0;
// used during STATE_OUTPUTS. Will contain the sum of all outgoing output values
// (non-change outputs).
static uint64_t _outputs_sum_out = 0;
// number of change outputs. if >1, a warning is shown.
static uint16_t _num_changes = 0;

// used during STATE_INPUTS_PASS1
static void* _hash_prevouts_ctx = NULL;
static void* _hash_sequence_ctx = NULL;
// By the end of STATE_INPUTS_PASS1, will contain the prevouts hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 2.
static uint8_t _hash_prevouts[32] = {0};
// By the end of STATE_INPUTS_PASS1, will contain the sequence hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 3.
static uint8_t _hash_sequence[32] = {0};

// used during STATE_OUTPUTS
static void* _hash_outputs_ctx = NULL;
// By the end of STATE_OUTPUTS, will contain the hashOutputs hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 8.
static uint8_t _hash_outputs[32] = {0};

static void _maybe_pop_empty_screen(void)
{
    if (_empty_component != NULL) {
        if (ui_screen_stack_top() != _empty_component) {
            Abort("btc_sign: mismatched screen push/pop");
        }
        ui_screen_stack_pop_and_clean();
        _empty_component = NULL;
    }
}

static void _maybe_pop_progress_screen(void)
{
    if (_progress_component != NULL) {
        if (ui_screen_stack_top() != _progress_component) {
            Abort("btc_sign: mismatched screen push/pop");
        }
        ui_screen_stack_pop_and_clean();
        _progress_component = NULL;
    }
}

/**
 * Sets the progress of the progress bar depending on the current signing state.
 * During inputs streaming: progress is number of inputs processed, with a subprogress of how many
 * inputs/outputs of the prevtx have been processed.
 */
static void _update_progress(void)
{
    if (_progress_component == NULL) {
        return;
    }
    float progress = 0;
    switch (_state) {
    case STATE_INPUTS_PASS1:
    case STATE_INPUTS_PASS2:
        progress = _index / (float)_init_request.num_inputs;
        break;
    case STATE_OUTPUTS:
        // Once we reached the outputs stage, the progress for loading the inputs is 100%.
        progress = 1.F;
        break;
    case STATE_PREVTX_INPUTS: {
        float step = 1.F / (float)_init_request.num_inputs;
        uint32_t num_inputs_outputs =
            _prevtx.init_request.num_inputs + _prevtx.init_request.num_outputs;
        float subprogress = _prevtx.index / (float)num_inputs_outputs;
        progress = _index * step + subprogress * step;
        break;
    }
    case STATE_PREVTX_OUTPUTS: {
        float step = 1.F / (float)_init_request.num_inputs;
        uint32_t num_inputs_outputs =
            _prevtx.init_request.num_inputs + _prevtx.init_request.num_outputs;
        float subprogress =
            (float)(_prevtx.init_request.num_inputs + _prevtx.index) / (float)num_inputs_outputs;
        progress = _index * step + subprogress * step;
        break;
    }
    default:
        break;
    }
    progress_set(_progress_component, progress);
}

// Must be called in any code path that exits the signing process (error or regular finish).
static void _reset(void)
{
    _state = STATE_INIT;
    _coin_params = NULL;
    util_zero(&_init_request, sizeof(_init_request));
    rust_sha256_free(&_prevtx.tx_hash_ctx);
    util_zero(&_prevtx, sizeof(_prevtx));
    _index = 0;
    _rbf = CONFIRM_LOCKTIME_RBF_OFF;
    _locktime_applies = false;
    _inputs_sum_pass1 = 0;
    _inputs_sum_pass2 = 0;
    _outputs_sum_out = 0;
    _outputs_sum_ours = 0;
    _num_changes = 0;

    rust_sha256_free(&_hash_prevouts_ctx);
    _hash_prevouts_ctx = rust_sha256_new();

    rust_sha256_free(&_hash_sequence_ctx);
    _hash_sequence_ctx = rust_sha256_new();

    rust_sha256_free(&_hash_outputs_ctx);
    _hash_outputs_ctx = rust_sha256_new();

    _maybe_pop_empty_screen();
    _maybe_pop_progress_screen();
}

static app_btc_result_t _error(app_btc_result_t err)
{
    _reset();
    return err;
}

app_btc_result_t app_btc_sign_init(const BTCSignInitRequest* request, BTCSignNextResponse* next_out)
{
    if (_state != STATE_INIT) {
        return _error(APP_BTC_ERR_STATE);
    }
    // Currently we do not support time-based nlocktime
    if (request->locktime >= 500000000) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    // currently only support version 1 or version 2 tx.
    // version 2: https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
    if (request->version != 1 && request->version != 2) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    if (request->num_inputs < 1 || request->num_outputs < 1) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
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
    // Want First input
    _index = 0;
    _state = STATE_INPUTS_PASS1;
    next_out->type = BTCSignNextResponse_Type_INPUT;
    next_out->index = _index;

    _progress_component = progress_create("Loading transaction...");
    ui_screen_stack_push(_progress_component);

    return APP_BTC_OK;
}

static void _hash_varint(void* ctx, uint64_t v)
{
    uint8_t varint[MAX_VARINT_SIZE] = {0};
    size_t size = wally_varint_to_bytes(v, varint);
    rust_sha256_update(ctx, varint, size);
}

app_btc_result_t app_btc_sign_prevtx_init(
    const BTCPrevTxInitRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_PREVTX_INIT) {
        return _error(APP_BTC_ERR_STATE);
    }
    if (request->num_inputs < 1 || request->num_outputs < 1) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    // Hash version
    // assumes little endian environment
    rust_sha256_update(_prevtx.tx_hash_ctx, &request->version, sizeof(request->version));

    _prevtx.init_request = *request;

    // Want first input of prevtx
    _state = STATE_PREVTX_INPUTS;
    // `_index` (main tx input index) unchanged
    _prevtx.index = 0;
    next_out->type = BTCSignNextResponse_Type_PREVTX_INPUT;
    next_out->index = _index;
    next_out->prev_index = _prevtx.index;
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_prevtx_input(
    const BTCPrevTxInputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_PREVTX_INPUTS) {
        return _error(APP_BTC_ERR_STATE);
    }

    if (_prevtx.index == 0) {
        // Hash number of inputs
        _hash_varint(_prevtx.tx_hash_ctx, _prevtx.init_request.num_inputs);
    }

    // Hash prevOutHash
    rust_sha256_update(_prevtx.tx_hash_ctx, request->prev_out_hash, sizeof(request->prev_out_hash));
    // Hash preOutIndex
    // assumes little endian environment
    rust_sha256_update(
        _prevtx.tx_hash_ctx, &request->prev_out_index, sizeof(request->prev_out_index));

    // Hash sig script
    _hash_varint(_prevtx.tx_hash_ctx, request->signature_script.size);
    rust_sha256_update(
        _prevtx.tx_hash_ctx, request->signature_script.bytes, request->signature_script.size);

    // Hash sequence
    // assumes little endian environment
    rust_sha256_update(_prevtx.tx_hash_ctx, &request->sequence, sizeof(request->sequence));

    if (_prevtx.index < _prevtx.init_request.num_inputs - 1) {
        // Want next input of previous transaction
        _prevtx.index++;
        next_out->type = BTCSignNextResponse_Type_PREVTX_INPUT;
        next_out->index = _index;
        next_out->prev_index = _prevtx.index;
    } else {
        // Done with prevtx inputs.
        // Want first output.
        _state = STATE_PREVTX_OUTPUTS;
        _prevtx.index = 0;
        next_out->type = BTCSignNextResponse_Type_PREVTX_OUTPUT;
        next_out->index = _index;
        next_out->prev_index = 0;
    }
    _update_progress();
    return APP_BTC_OK;
}

app_btc_result_t app_btc_sign_prevtx_output(
    const BTCPrevTxOutputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_PREVTX_OUTPUTS) {
        return _error(APP_BTC_ERR_STATE);
    }

    if (_prevtx.index == 0) {
        // Hash number of inputs
        _hash_varint(_prevtx.tx_hash_ctx, _prevtx.init_request.num_outputs);
    }
    if (_prevtx.index == _prevtx.referencing_input.prevOutIndex) {
        if (_prevtx.referencing_input.prevOutValue != request->value) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
    }

    // Hash value
    // assumes little endian environment
    rust_sha256_update(_prevtx.tx_hash_ctx, &request->value, sizeof(request->value));

    // Hash pubkeyScript
    _hash_varint(_prevtx.tx_hash_ctx, request->pubkey_script.size);
    rust_sha256_update(
        _prevtx.tx_hash_ctx, request->pubkey_script.bytes, request->pubkey_script.size);

    bool last = _prevtx.index == _prevtx.init_request.num_outputs - 1;

    if (last) {
        // Hash locktime
        // assumes little endian environment
        rust_sha256_update(
            _prevtx.tx_hash_ctx,
            &_prevtx.init_request.locktime,
            sizeof(_prevtx.init_request.locktime));

        uint8_t txhash[32] = {0};
        rust_sha256_finish(&_prevtx.tx_hash_ctx, txhash);
        // hash again to produce the final double-hash
        rust_sha256(txhash, sizeof(txhash), txhash);

        if (!MEMEQ(txhash, _prevtx.referencing_input.prevOutHash, sizeof(txhash))) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
    }

    if (!last) {
        // Want next output of previous transaction
        _prevtx.index++;
        next_out->type = BTCSignNextResponse_Type_PREVTX_OUTPUT;
        next_out->index = _index;
        next_out->prev_index = _prevtx.index;
    } else {
        // Done with prevtx outputs.
        if (_index < _init_request.num_inputs - 1) {
            // Want the next input of the main tx.
            _state = STATE_INPUTS_PASS1;
            _index++;
            next_out->type = BTCSignNextResponse_Type_INPUT;
            next_out->index = _index;
        } else {
            // Done with all main tx inputs, want first output.
            _state = STATE_OUTPUTS;
            _index = 0;
            next_out->type = BTCSignNextResponse_Type_OUTPUT;
            next_out->index = _index;
        }
        _update_progress();
    }
    return APP_BTC_OK;
}

static app_btc_result_t _sign_input_pass1(
    const BTCSignInputRequest* request,
    BTCSignNextResponse* next_out)
{
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
    if (!safe_uint64_add(&_inputs_sum_pass1, request->prevOutValue)) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    if (_index == _init_request.num_inputs - 1) {
        // Done with inputs pass 1.

        rust_sha256_finish(&_hash_prevouts_ctx, _hash_prevouts);
        // hash hash_prevouts to produce the final double-hash
        rust_sha256(_hash_prevouts, 32, _hash_prevouts);

        rust_sha256_finish(&_hash_sequence_ctx, _hash_sequence);
        // hash hash_sequence to produce the final double-hash
        rust_sha256(_hash_sequence, 32, _hash_sequence);
    }

    // Want the previous tx of this input.
    // Init prevtx state.
    rust_sha256_free(&_prevtx.tx_hash_ctx);
    util_zero(&_prevtx, sizeof(_prevtx));
    _prevtx.referencing_input = *request;
    _prevtx.tx_hash_ctx = rust_sha256_new();

    _update_progress();

    _state = STATE_PREVTX_INIT;
    next_out->type = BTCSignNextResponse_Type_PREVTX_INIT;
    next_out->index = _index;

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
        if (!btc_common_is_valid_keypath_address_multisig_p2wsh(
                keypath, keypath_count, expected_bip44_coin)) {
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

static app_btc_result_t _sign_input_pass2(
    const BTCSignInputRequest* request,
    const BTCScriptConfig* script_config_account,
    BTCSignNextResponse* next_out)
{
    if (!safe_uint64_add(&_inputs_sum_pass2, request->prevOutValue)) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    if (_index == _init_request.num_inputs - 1) {
        // In the last input, the two sums have to match.
        if (_inputs_sum_pass2 != _inputs_sum_pass1) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
    } else if (_inputs_sum_pass2 > _inputs_sum_pass1) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
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
            sighash_script_size =
                wally_varbuff_to_bytes(sighash_script_tmp, sighash_script_size, sighash_script);

            break;
        }
        default:
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
        uint8_t sighash[32] = {0};
        // construct hash to sign
        btc_bip143_sighash(
            _init_request.version,
            _hash_prevouts,
            _hash_sequence,
            request->prevOutHash,
            request->prevOutIndex,
            sighash_script,
            sighash_script_size,
            request->prevOutValue,
            request->sequence,
            _hash_outputs,
            _init_request.locktime,
            WALLY_SIGHASH_ALL,
            sighash);
        uint8_t sig_out[64] = {0};
        if (!keystore_secp256k1_sign(
                request->keypath, request->keypath_count, sighash, sig_out, NULL)) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        // check assumption
        if (sizeof(next_out->signature) != sizeof(sig_out)) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        memcpy(next_out->signature, sig_out, sizeof(sig_out));
        next_out->has_signature = true;
    }

    if (_index < _init_request.num_inputs - 1) {
        _index++;
        // Want next input
        next_out->type = BTCSignNextResponse_Type_INPUT;
        next_out->index = _index;

        _update_progress();
    } else {
        // Done with inputs pass2 -> done completely.
        _reset();
        next_out->type = BTCSignNextResponse_Type_DONE;
    }
    return APP_BTC_OK;
}

// num_changes must be >1.
static bool _warn_changes(uint16_t num_changes)
{
    char body[100] = {0};
    snprintf(body, sizeof(body), "There are %d\nchange outputs.\nProceed?", num_changes);
    const confirm_params_t params = {
        .title = "Warning",
        .body = body,
    };
    return workflow_confirm_blocking(&params);
}

app_btc_result_t app_btc_sign_input(
    const BTCSignInputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_INPUTS_PASS1 && _state != STATE_INPUTS_PASS2) {
        return _error(APP_BTC_ERR_STATE);
    }
    // relative locktime and sequence nummbers < 0xffffffff-2 are not supported
    if (request->sequence < 0xffffffff - 2) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    if (_coin_params->rbf_support) {
        if (request->sequence == 0xffffffff - 2) {
            _rbf = CONFIRM_LOCKTIME_RBF_ON;
        }
    }
    if (request->sequence < 0xffffffff) {
        _locktime_applies = true;
    }
    if (request->prevOutValue == 0) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    if (request->script_config_index >= _init_request.script_configs_count) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
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
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    if (_state == STATE_INPUTS_PASS1) {
        return _sign_input_pass1(request, next_out);
    }
    return _sign_input_pass2(request, &script_config_account->script_config, next_out);
}

app_btc_result_t app_btc_sign_output(
    const BTCSignOutputRequest* request,
    BTCSignNextResponse* next_out)
{
    _maybe_pop_progress_screen();

    if (_state != STATE_OUTPUTS) {
        return _error(APP_BTC_ERR_STATE);
    }
    if (request->script_config_index >= _init_request.script_configs_count) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }

    const BTCScriptConfigWithKeypath* script_config_account =
        &_init_request.script_configs[request->script_config_index];

    // get pubkeyhash or scripthash. If request->ours=true, we compute the hash
    // from the keystore, otherwise it is provided in request->hash.

    uint8_t hash_bytes[sizeof(request->hash.bytes)] = {0};
    size_t hash_size;

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
            if (!btc_common_outputhash_from_pubkeyhash(
                    script_config_account->script_config.config.simple_type,
                    pubkey_hash160,
                    hash_bytes,
                    &hash_size)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }
            output_type = btc_common_determine_output_type(
                script_config_account->script_config.config.simple_type);
            break;
        }
        case BTCScriptConfig_multisig_tag:
            if (!btc_common_outputhash_from_multisig_p2wsh(
                    &script_config_account->script_config.config.multisig,
                    request->keypath[request->keypath_count - 2],
                    request->keypath[request->keypath_count - 1],
                    hash_bytes)) {
                return _error(APP_BTC_ERR_UNKNOWN);
            }
            hash_size = 32;
            output_type = BTCOutputType_P2WSH;
            break;
        default:
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

    } else {
        hash_size = request->hash.size;
        memcpy(hash_bytes, request->hash.bytes, hash_size);
        output_type = request->type;
    }
    if (request->value == 0) {
        return _error(APP_BTC_ERR_INVALID_INPUT);
    }
    if (request->ours) {
        if (!safe_uint64_add(&_outputs_sum_ours, request->value)) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
    } else {
        if (!safe_uint64_add(&_outputs_sum_out, request->value)) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
    }

    if (request->ours) {
        _num_changes++;
    }

    if (!request->ours) {
        char address[100] = {0};
        // assemble address to display, get user confirmation
        if (!btc_common_address_from_outputhash(
                _coin_params, output_type, hash_bytes, hash_size, address, sizeof(address))) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

        // Verify output if it is not a change output.
        char formatted_value[100] = {0};
        if (!btc_common_format_amount(
                request->value, _coin_params->unit, formatted_value, sizeof(formatted_value))) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }

        // This call blocks.
        if (!workflow_verify_recipient(address, formatted_value)) {
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
        if (!btc_common_pkscript_from_outputhash(
                output_type, hash_bytes, hash_size, pk_script, &pk_script_len)) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }

        // assumes little endian environment.
        rust_sha256_update(_hash_outputs_ctx, &request->value, 8);
        uint8_t pk_script_serialized[sizeof(pk_script) + 8] = {0};
        size_t pk_script_serialized_len =
            wally_varbuff_to_bytes(pk_script, pk_script_len, pk_script_serialized);
        rust_sha256_update(_hash_outputs_ctx, pk_script_serialized, pk_script_serialized_len);
    }

    if (_index == 0) {
        // The device shows the default "See the BitBoxApp" screen, then the progress bar while
        // processing the inputs, up until the first output is processed. Afterwards, the base
        // screen is the empty screen to avoid flicker, until the last output is processed.
        _empty_component = empty_create();
        ui_screen_stack_push(_empty_component);
    }
    if (_index == _init_request.num_outputs - 1) {
        _maybe_pop_empty_screen();
        if (_init_request.num_inputs > 2) {
            // Show progress of signing inputs if there are more than 2 inputs. This is an arbitrary
            // cutoff; less or equal to 2 inputs is fast enough so it does not need a progress bar.
            _progress_component = progress_create("Signing transaction...");
            // Popped with the last input of pass2.
            ui_screen_stack_push(_progress_component);
        }
    }

    if (_index < _init_request.num_outputs - 1) {
        _index++;
        // Want next output
        next_out->type = BTCSignNextResponse_Type_OUTPUT;
        next_out->index = _index;
    } else {
        // Done with outputs. Verify locktime, total and fee. Warn if there are multiple change
        // outputs.

        if (_num_changes > 1) {
            if (!_warn_changes(_num_changes)) {
                return _error(APP_BTC_ERR_USER_ABORT);
            }
        }

        // A locktime of 0 will also not be verified, as it's certainly in the past and can't do any
        // harm.
        if (_init_request.locktime > 0) {
            // This is not a security feature, the extra locktime/RBF user confirmation is skipped
            // if the tx is not rbf or has a locktime of 0.
            if (_locktime_applies || _rbf == CONFIRM_LOCKTIME_RBF_ON) {
                // The RBF nsequence bytes are often set in conjunction with a locktime,
                // so verify both simultaneously.
                // There is no RBF in Litecoin, so make sure it is disabled.
                if (!_coin_params->rbf_support) {
                    _rbf = CONFIRM_LOCKTIME_RBF_DISABLED;
                }
                if (!apps_btc_confirm_locktime_rbf(_init_request.locktime, _rbf)) {
                    return _error(APP_BTC_ERR_USER_ABORT);
                }
            }
        }

        // total_out, including fee.
        if (_inputs_sum_pass1 < _outputs_sum_ours) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
        uint64_t total_out = _inputs_sum_pass1 - _outputs_sum_ours;
        if (total_out < _outputs_sum_out) {
            return _error(APP_BTC_ERR_INVALID_INPUT);
        }
        uint64_t fee = total_out - _outputs_sum_out;

        char formatted_total_out[100] = {0};
        if (!btc_common_format_amount(
                total_out, _coin_params->unit, formatted_total_out, sizeof(formatted_total_out))) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        char formatted_fee[100] = {0};
        if (!btc_common_format_amount(
                fee, _coin_params->unit, formatted_fee, sizeof(formatted_fee))) {
            return _error(APP_BTC_ERR_UNKNOWN);
        }
        // This call blocks.
        if (!workflow_verify_total(formatted_total_out, formatted_fee)) {
            return _error(APP_BTC_ERR_USER_ABORT);
        }

        rust_sha256_finish(&_hash_outputs_ctx, _hash_outputs);
        // hash hash_outputs to produce the final double-hash
        rust_sha256(_hash_outputs, 32, _hash_outputs);

        // Want first input of pass2
        _state = STATE_INPUTS_PASS2;
        _index = 0;
        next_out->type = BTCSignNextResponse_Type_INPUT;
        next_out->index = _index;
    }
    return APP_BTC_OK;
}

#ifdef TESTING
void tst_app_btc_reset(void)
{
    _reset();
}
#endif
