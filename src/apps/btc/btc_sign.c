// Copyright 2019 Shift Cryptosecurity AG
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
#include "btc_script.h"
#include "confirm_locktime_rbf.h"

#include <crypto/sha2/sha256.h>
#include <keystore.h>
#include <util.h>
#include <workflow/verify_recipient.h>
#include <workflow/verify_total.h>

#include <wally_script.h>
#include <wally_transaction.h>

typedef enum {
    STATE_INIT,
    STATE_INPUTS_PASS1,
    STATE_OUTPUTS,
    STATE_INPUTS_PASS2,
} _signing_state_t;

static _signing_state_t _state = STATE_INIT;
static const app_btc_coin_params_t* _coin_params = NULL;

// Inputs and changes must be of the type defined in _init_request.script_type.
// Inputs and changes keypaths must have account _init_request.bip44_account
static BTCSignInitRequest _init_request = {0};

static uint32_t _index;
static enum apps_btc_rbf_flag _rbf;
static bool _locktime_applies;

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

// used during STATE_INPUTS_PASS1
static sha256_context_t _hash_prevouts_ctx, _hash_sequence_ctx;
// By the end of STATE_INPUTS_PASS1, will contain the prevouts hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 2.
static uint8_t _hash_prevouts[32] = {0};
// By the end of STATE_INPUTS_PASS1, will contain the sequence hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 3.
static uint8_t _hash_sequence[32] = {0};

// used during STATE_OUTPUTS
static sha256_context_t _hash_outputs_ctx;
// By the end of STATE_OUTPUTS, will contain the hashOutputs hash.
// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki step 8.
static uint8_t _hash_outputs[32] = {0};

// hashes bytes, puts result into out. bytes and out can overlap.
static void _sha256(const uint8_t* bytes, size_t bytes_len, uint8_t* out)
{
    sha256_context_t ctx;
    sha256_reset(&ctx);
    noise_sha256_update(&ctx, bytes, bytes_len);
    sha256_finish(&ctx, out);
}

static void _reset(void)
{
    _state = STATE_INIT;
    _coin_params = NULL;
    util_zero(&_init_request, sizeof(_init_request));
    _index = 0;
    _rbf = CONFIRM_LOCKTIME_RBF_OFF;
    _locktime_applies = false;
    _inputs_sum_pass1 = 0;
    _inputs_sum_pass2 = 0;
    _outputs_sum_out = 0;
    _outputs_sum_ours = 0;
    sha256_reset(&_hash_prevouts_ctx);
    sha256_reset(&_hash_sequence_ctx);
    sha256_reset(&_hash_outputs_ctx);
}

static app_btc_sign_error_t _error(app_btc_sign_error_t err)
{
    _reset();
    return err;
}

app_btc_sign_error_t app_btc_sign_init(
    const BTCSignInitRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_INIT) {
        return _error(APP_BTC_SIGN_ERR_STATE);
    }
    // Currently we do not support time-based nlocktime
    if (request->locktime >= 500000000) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    // currently only support version 1 or version 2 tx.
    // version 2: https://github.com/bitcoin/bips/blob/master/bip-0068.mediawiki
    if (request->version != 1 && request->version != 2) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    if (request->num_inputs < 1 || request->num_outputs < 1) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    const app_btc_coin_params_t* coin_params = app_btc_params_get(request->coin);
    if (coin_params == NULL) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    _reset();
    _coin_params = coin_params;
    _init_request = *request;
    // Want input #0
    _state = STATE_INPUTS_PASS1;
    next_out->type = BTCSignNextResponse_Type_INPUT;
    next_out->index = _index;
    return APP_BTC_SIGN_OK;
}

static app_btc_sign_error_t _sign_input_pass1(
    const BTCSignInputRequest* request,
    BTCSignNextResponse* next_out)
{
    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 2: accumulate hashPrevouts
        // ANYONECANPAY not supported.
        noise_sha256_update(&_hash_prevouts_ctx, request->prevOutHash, 32);
        // assumes little endian environment.
        noise_sha256_update(&_hash_prevouts_ctx, &request->prevOutIndex, 4);
    }
    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 3: accumulate hashSequence
        // only SIGHASH_ALL supported.

        // assumes little endian environment.
        noise_sha256_update(&_hash_sequence_ctx, &request->sequence, 4);
    }
    if (!safe_uint64_add(&_inputs_sum_pass1, request->prevOutValue)) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }

    if (_index < _init_request.num_inputs - 1) {
        _index++;
        // Want next input
        next_out->type = BTCSignNextResponse_Type_INPUT;
        next_out->index = _index;
    } else {
        // Done with inputs pass 1.

        sha256_finish(&_hash_prevouts_ctx, _hash_prevouts);
        // hash hash_prevouts to produce the final double-hash
        _sha256(_hash_prevouts, 32, _hash_prevouts);

        sha256_finish(&_hash_sequence_ctx, _hash_sequence);
        // hash hash_sequence to produce the final double-hash
        _sha256(_hash_sequence, 32, _hash_sequence);

        // Want first output
        _state = STATE_OUTPUTS;
        _index = 0;
        next_out->type = BTCSignNextResponse_Type_OUTPUT;
        next_out->index = _index;
    }
    return APP_BTC_SIGN_OK;
}

static bool _is_valid_keypath(
    const uint32_t* keypath,
    size_t keypath_count,
    BTCScriptType script_type,
    uint32_t expected_bip44_coin,
    uint32_t expected_bip44_account,
    bool must_be_change)
{
    if (!btc_common_is_valid_keypath_address(
            script_type, keypath, keypath_count, expected_bip44_coin)) {
        return false;
    }
    const uint32_t account = keypath[2];
    const uint32_t change = keypath[3];
    if (account != expected_bip44_account || (must_be_change && change != 1)) {
        return false;
    }
    return true;
}

static app_btc_sign_error_t _sign_input_pass2(
    const BTCSignInputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (!safe_uint64_add(&_inputs_sum_pass2, request->prevOutValue)) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    if (_index == _init_request.num_inputs - 1) {
        // In the last input, the two sums have to match.
        if (_inputs_sum_pass2 != _inputs_sum_pass1) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
    } else if (_inputs_sum_pass2 > _inputs_sum_pass1) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }

    { // Sign input.
        uint8_t pubkey_hash160[20];
        UTIL_CLEANUP_20(pubkey_hash160);
        if (!keystore_secp256k1_pubkey(
                KEYSTORE_SECP256K1_PUBKEY_HASH160,
                request->keypath,
                request->keypath_count,
                pubkey_hash160,
                sizeof(pubkey_hash160))) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }

        uint8_t sighash_script[MAX_SIGHASH_SCRIPT_SIZE] = {0};
        size_t sighash_script_size = sizeof(sighash_script);
        if (!btc_common_sighash_script_from_pubkeyhash(
                _init_request.script_type, pubkey_hash160, sighash_script, &sighash_script_size)) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
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
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }
        // check assumption
        if (sizeof(next_out->signature) != sizeof(sig_out)) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }
        memcpy(next_out->signature, sig_out, sizeof(sig_out));
        next_out->has_signature = true;
    }

    if (_index < _init_request.num_inputs - 1) {
        _index++;
        // Want next input
        next_out->type = BTCSignNextResponse_Type_INPUT;
        next_out->index = _index;
    } else {
        // Done with inputs pass2 -> done completely.
        _reset();
        next_out->type = BTCSignNextResponse_Type_DONE;
    }
    return APP_BTC_SIGN_OK;
}

app_btc_sign_error_t app_btc_sign_input(
    const BTCSignInputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_INPUTS_PASS1 && _state != STATE_INPUTS_PASS2) {
        return _error(APP_BTC_SIGN_ERR_STATE);
    }
    // relative locktime and sequence nummbers < 0xffffffff-2 are not supported
    if (request->sequence < 0xffffffff - 2) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
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
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    if (!_is_valid_keypath(
            request->keypath,
            request->keypath_count,
            _init_request.script_type,
            _coin_params->bip44_coin,
            _init_request.bip44_account,
            false)) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    if (_state == STATE_INPUTS_PASS1) {
        return _sign_input_pass1(request, next_out);
    }
    return _sign_input_pass2(request, next_out);
}

app_btc_sign_error_t app_btc_sign_output(
    const BTCSignOutputRequest* request,
    BTCSignNextResponse* next_out)
{
    if (_state != STATE_OUTPUTS) {
        return _error(APP_BTC_SIGN_ERR_STATE);
    }

    // get pubkeyhash or scripthash. If request->ours=true, we compute the hash
    // from the keystore, otherwise it is provided in request->hash.

    BTCSignOutputRequest_hash_t hash = {0};
    BTCOutputType output_type;
    if (request->ours) {
        if (!_is_valid_keypath(
                request->keypath,
                request->keypath_count,
                _init_request.script_type,
                _coin_params->bip44_coin,
                _init_request.bip44_account,
                true)) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
        size_t out_size = 0;
        if (!btc_script_outputhash_at_keypath(
                _init_request.script_type,
                request->keypath,
                request->keypath_count,
                hash.bytes,
                &out_size)) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }
        hash.size = (pb_size_t)out_size;
        output_type = btc_common_determine_output_type(_init_request.script_type);
    } else {
        hash = request->hash;
        output_type = request->type;
    }
    if (request->value == 0) {
        return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
    }
    if (request->ours) {
        if (!safe_uint64_add(&_outputs_sum_ours, request->value)) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
    } else {
        if (!safe_uint64_add(&_outputs_sum_out, request->value)) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
    }

    if (!request->ours) {
        char address[100] = {0};
        // assemble address to display, get user confirmation
        if (!btc_common_address_from_outputhash(
                _coin_params, output_type, hash.bytes, hash.size, address, sizeof(address))) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }

        // Verify output if it is not a change output.
        char formatted_value[100] = {0};
        if (!btc_common_format_amount(
                request->value, _coin_params->unit, formatted_value, sizeof(formatted_value))) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }

        // This call blocks.
        if (!workflow_verify_recipient(address, formatted_value)) {
            return _error(APP_BTC_SIGN_ERR_USER_ABORT);
        }
    }

    {
        // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
        // point 8: accumulate hashOutputs
        // only SIGHASH_ALL supported.

        // create pk_script
        // current expected output script size is 83 for OP_RETURN
        uint8_t pk_script[100] = {0};
        size_t pk_script_len = sizeof(pk_script);
        if (!btc_common_pkscript_from_outputhash(
                output_type, hash.bytes, hash.size, pk_script, &pk_script_len)) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }

        // assumes little endian environment.
        noise_sha256_update(&_hash_outputs_ctx, &request->value, 8);
        uint8_t pk_script_serialized[sizeof(pk_script) + 8] = {0};
        size_t pk_script_serialized_len =
            wally_varbuff_to_bytes(pk_script, pk_script_len, pk_script_serialized);
        noise_sha256_update(&_hash_outputs_ctx, pk_script_serialized, pk_script_serialized_len);
    }

    if (_index < _init_request.num_outputs - 1) {
        _index++;
        // Want next output
        next_out->type = BTCSignNextResponse_Type_OUTPUT;
        next_out->index = _index;
    } else {
        // Done with outputs. Verify locktime, total and fee.
        //
        // This is not a security feature, a transaction that is not rbf
        // and has a locktime of 0 will not be verified.
        if (_locktime_applies || _rbf == CONFIRM_LOCKTIME_RBF_ON) {
            // The RBF nsequence bytes are often set in conjunction with a locktime,
            // so verify both simultaneously.
            // There is no RBF in Litecoin, so make sure it is disabled.
            if (!_coin_params->rbf_support) {
                _rbf = CONFIRM_LOCKTIME_RBF_DISABLED;
            }
            if (!apps_btc_confirm_locktime_rbf(_init_request.locktime, _rbf)) {
                return _error(APP_BTC_SIGN_ERR_USER_ABORT);
            }
        }

        // total_out, including fee.
        if (_inputs_sum_pass1 < _outputs_sum_ours) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
        uint64_t total_out = _inputs_sum_pass1 - _outputs_sum_ours;
        if (total_out < _outputs_sum_out) {
            return _error(APP_BTC_SIGN_ERR_INVALID_INPUT);
        }
        uint64_t fee = total_out - _outputs_sum_out;

        char formatted_total_out[100] = {0};
        if (!btc_common_format_amount(
                total_out, _coin_params->unit, formatted_total_out, sizeof(formatted_total_out))) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }
        char formatted_fee[100] = {0};
        if (!btc_common_format_amount(
                fee, _coin_params->unit, formatted_fee, sizeof(formatted_fee))) {
            return _error(APP_BTC_SIGN_ERR_UNKNOWN);
        }
        // This call blocks.
        if (!workflow_verify_total(formatted_total_out, formatted_fee)) {
            return _error(APP_BTC_SIGN_ERR_USER_ABORT);
        }

        sha256_finish(&_hash_outputs_ctx, _hash_outputs);
        // hash hash_outputs to produce the final double-hash
        _sha256(_hash_outputs, 32, _hash_outputs);

        // Want first input of pass2
        _state = STATE_INPUTS_PASS2;
        _index = 0;
        next_out->type = BTCSignNextResponse_Type_INPUT;
        next_out->index = _index;
    }
    return APP_BTC_SIGN_OK;
}

#ifdef TESTING
void tst_app_btc_reset(void)
{
    _reset();
}
#endif
