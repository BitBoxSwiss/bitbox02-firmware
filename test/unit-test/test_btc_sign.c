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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <stdio.h>

#include <apps/btc/btc_sign.h>
#include <apps/btc/confirm_locktime_rbf.h>
#include <keystore.h>
#include <rust/rust.h>
#include <secp256k1_ecdsa_s2c.h>
#include <wally_bip32.h>
#include <wally_crypto.h>
#include <workflow/confirm.h>

void __wrap_workflow_status_blocking(const char* msg, bool status_success) {}

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    check_expected(params->body);
    return mock();
}

bool __wrap_workflow_verify_recipient(const char* recipient, const char* amount)
{
    check_expected(recipient);
    check_expected(amount);
    return mock();
}

bool __wrap_workflow_verify_total(const char* total, const char* fee)
{
    check_expected(total);
    check_expected(fee);
    return mock();
}

bool __wrap_apps_btc_confirm_locktime_rbf(uint32_t locktime, enum apps_btc_rbf_flag rbf)
{
    check_expected(locktime);
    check_expected(rbf);
    return mock();
}

void __wrap_rust_bitcoin_util_format_amount(uint64_t satoshi, CStr unit, CStrMut out)
{
    check_expected(satoshi);
    check_expected(unit.buf);
    snprintf(out.buf, out.cap, "%s", (const char*)(mock()));
}

bool __real_btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);
bool __wrap_btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    check_expected(script_type);
    check_expected(keypath);
    assert_int_equal(keypath_len, 5);
    return __real_btc_common_is_valid_keypath_address_simple(
        script_type, keypath, keypath_len, expected_coin);
}

bool __real_keystore_antiklepto_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out);
bool __wrap_keystore_antiklepto_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    check_expected(msg32);
    return __real_keystore_antiklepto_secp256k1_commit(
        keypath, keypath_len, msg32, host_commitment, signer_commitment_out);
}

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static uint8_t _mock_bip39_seed[64] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static void _test_btc_sign_init(void** state)
{
    // establish valid request to modify
    const BTCSignInitRequest init_req_valid = {
        .coin = BTCCoin_BTC,
        .script_configs_count = 1,
        .script_configs =
            {
                {
                    .script_config =
                        {
                            .which_config = BTCScriptConfig_simple_type_tag,
                            .config =
                                {
                                    .simple_type = BTCScriptConfig_SimpleType_P2WPKH,
                                },
                        },
                    .keypath_count = 3,
                    .keypath =
                        {
                            84 + BIP32_INITIAL_HARDENED_CHILD,
                            0 + BIP32_INITIAL_HARDENED_CHILD,
                            0 + BIP32_INITIAL_HARDENED_CHILD,
                        },
                },
            },
        .version = 1,
        .num_inputs = 1,
        .num_outputs = 1,
        .locktime = 0,
    };
    BTCSignNextResponse next = {0};
    { // test valid
        assert_int_equal(APP_BTC_OK, app_btc_sign_init(&init_req_valid, &next));
        assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
        assert_int_equal(next.index, 0);
        assert_int_equal(next.prev_index, 0); // arbitrary
        assert_false(next.has_signature);
    }
    { // test invalid version
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        for (invalid.version = 0; invalid.version < 10; invalid.version++) {
            if (invalid.version == 1 || invalid.version == 2) {
                continue;
            }
            assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
        }
    }
    { // test invalid locktime
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.locktime = 500000000;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // test invalid inputs
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.num_inputs = 0;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // test invalid outputs
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.num_outputs = 0;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // test invalid coin
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.coin = _BTCCoin_MIN - 1;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
        invalid.coin = _BTCCoin_MAX + 1;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // test invalid account keypath
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.script_configs[0].keypath[2] = BIP32_INITIAL_HARDENED_CHILD + 100;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // no script configs is invalid
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.script_configs_count = 0;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // can't mix script configs from different bip44 accounts
        // (mixing input scripts is allowed, but only from the same bip44 account).
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.script_configs_count = 2;
        BTCScriptConfigWithKeypath sc = {
            .script_config =
                {
                    .which_config = BTCScriptConfig_simple_type_tag,
                    .config =
                        {
                            .simple_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
                        },
                },
            .keypath_count = 3,
            .keypath =
                {
                    49 + BIP32_INITIAL_HARDENED_CHILD,
                    0 + BIP32_INITIAL_HARDENED_CHILD,
                    0 + BIP32_INITIAL_HARDENED_CHILD,
                },
        };
        invalid.script_configs[1] = sc;

        assert_int_equal(APP_BTC_OK, app_btc_sign_init(&invalid, &next));
        tst_app_btc_reset();
        invalid.script_configs[0].keypath[2]++;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
    { // can't mix simple type (singlesig) and multisig configs in one tx
        tst_app_btc_reset();
        BTCSignInitRequest invalid = init_req_valid;
        invalid.script_configs_count = 2;
        BTCScriptConfigWithKeypath sc = {
            .script_config =
                {
                    .which_config = BTCScriptConfig_multisig_tag,
                    .config =
                        {
                            .multisig =
                                {
                                    .threshold = 1,
                                    .xpubs_count = 2,
                                },
                        },
                },
            .keypath_count = 4,
            .keypath =
                {
                    48 + BIP32_INITIAL_HARDENED_CHILD,
                    1 + BIP32_INITIAL_HARDENED_CHILD,
                    0 + BIP32_INITIAL_HARDENED_CHILD,
                    2 + BIP32_INITIAL_HARDENED_CHILD,
                },
        };
        invalid.script_configs[1] = sc;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_init(&invalid, &next));
    }
}

typedef struct {
    // true if the sigs should be checked against fixtures.
    bool check_sigs;
    // keystore seeded?
    bool seeded;
    BTCScriptConfig_SimpleType script_type;
    // all inputs should be the same coin type.
    bool wrong_coin_input;
    // all change outputs should be the same coin type.
    bool wrong_coin_change;
    // all inputs should be from the same account.
    bool wrong_account_input;
    // all change outputs should go the same account.
    bool wrong_account_change;
    // change num in bip44, should be 1.
    uint32_t bip44_change;
    // the sum of the inputs in the 2nd pass can't be higher than in the first
    // for all inputs.
    bool input_sum_changes;
    // at the last input, the sum of the inputs in the 2nd pass must be the same
    // as the sum of the inputs in the first pass
    bool input_sum_last_mismatch;
    // can't init twice in a row -> first input expected
    bool state_init_after_init;
    // wrong state transition
    bool state_output_after_init;
    // sequence number below 0xffffffff - 2
    bool wrong_sequence_number;
    // value 0 is invalid
    bool wrong_input_value;
    bool wrong_output_value;
    // when a user aborts on an output verification
    bool user_aborts_output;
    // rbf disabled on Litecoin
    bool litecoin_rbf_disabled;
    // check workflow when a locktime applies
    bool locktime_applies;
    // when a user aborts on a locktime verification
    bool user_aborts_locktime_rbf;
    // rbf enabled but 0 locktime: no user verification.
    bool locktime_zero_with_rbf;
    // when a user aborts on total/fee verification.
    bool user_aborts_total;
    // when a user aborts the warning about multiple change outputs being present.
    bool user_aborts_multiple_changes;
    // if value addition in inputs would overflow
    bool overflow_input_values_pass1;
    bool overflow_input_values_pass2;
    // if outgoing sum overflows
    bool overflow_output_out;
    // if change overflows
    bool overflow_output_ours;
    // can't init prevtx twice in a row -> first prevtx input expected
    bool state_previnit_after_previnit;
    // no inputs in prevtx
    bool prevtx_no_inputs;
    // no outputs in prevtx
    bool prevtx_no_outputs;
    // input value does not match prevtx output value
    bool input_wrong_value;
    // input's prevtx hash does not match input's prevOutHash
    bool wrong_prevouthash;
    // test tx with mixed input types
    bool mixed_inputs;
    // referenced script config does not exist.
    bool invalid_input_script_config_index;
    // referenced script config does not exist.
    bool invalid_change_script_config_index;
    // exercise the antiklepto protocol
    bool antikepto;
    // make one output a P2TR output to exercise P2TR address generation and sighash.
    bool p2tr_output;
} _modification_t;

typedef struct {
    BTCSignInputRequest input;

    // --- Previous transaction data.
    BTCPrevTxInitRequest prevtx_init;
    // actual count is in prevtx_init.num_inputs
    BTCPrevTxInputRequest prevtx_inputs[10];
    // actual count is in prevtx_init.num_outputs
    BTCPrevTxOutputRequest prevtx_outputs[10];
} _input_t;

// Called from `_sign()` to stream and test an input's previous tx.
static bool _stream_prevtx(
    const _modification_t* mod,
    size_t input_index,
    const _input_t* input,
    BTCSignNextResponse* next)
{
    if (mod->prevtx_no_inputs) {
        BTCPrevTxInitRequest invalid = input->prevtx_init;
        invalid.num_inputs = 0;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_prevtx_init(&invalid, next));
        return false;
    }
    if (mod->prevtx_no_outputs) {
        BTCPrevTxInitRequest invalid = input->prevtx_init;
        invalid.num_outputs = 0;
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_prevtx_init(&invalid, next));
        return false;
    }

    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_init(&input->prevtx_init, next));
    assert_int_equal(next->type, BTCSignNextResponse_Type_PREVTX_INPUT);
    assert_int_equal(next->index, input_index);
    assert_int_equal(next->prev_index, 0);

    if (mod->state_previnit_after_previnit) {
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_prevtx_init(&input->prevtx_init, next));
        return false;
    }

    if (mod->state_previnit_after_previnit) {
        assert_int_equal(
            APP_BTC_ERR_STATE, app_btc_sign_prevtx_output(&input->prevtx_outputs[0], next));
        return false;
    }

    for (size_t i = 0; i < input->prevtx_init.num_inputs; i++) {
        assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_input(&input->prevtx_inputs[i], next));
        bool last = i == input->prevtx_init.num_inputs - 1;
        if (last) {
            assert_int_equal(next->type, BTCSignNextResponse_Type_PREVTX_OUTPUT);
            assert_int_equal(next->index, input_index);
            assert_int_equal(next->prev_index, 0);
        } else {
            assert_int_equal(next->type, BTCSignNextResponse_Type_PREVTX_INPUT);
            assert_int_equal(next->index, input_index);
            assert_int_equal(next->prev_index, i + 1);
        }
    }
    for (size_t i = 0; i < input->prevtx_init.num_outputs; i++) {
        bool last = i == input->prevtx_init.num_outputs - 1;
        if (last && (mod->input_wrong_value || mod->wrong_prevouthash)) {
            assert_int_equal(
                APP_BTC_ERR_INVALID_INPUT,
                app_btc_sign_prevtx_output(&input->prevtx_outputs[i], next));
            return false;
        }
        assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_output(&input->prevtx_outputs[i], next));
        if (!last) {
            assert_int_equal(next->type, BTCSignNextResponse_Type_PREVTX_OUTPUT);
            assert_int_equal(next->index, input_index);
            assert_int_equal(next->prev_index, i + 1);
        }
    }
    return true;
}

// _sign goes through the whole sign process of an example tx, successfully.
// The passed params malleate the behavior to induce expected failures.
static void _sign(const _modification_t* mod)
{
    // Need keystore to derive change and input scripts
    keystore_mock_unlocked(
        mod->seeded ? _mock_seed : NULL, sizeof(_mock_seed), mod->seeded ? _mock_bip39_seed : NULL);

    uint32_t purpose;
    switch (mod->script_type) {
    case BTCScriptConfig_SimpleType_P2WPKH:
        purpose = 84 + BIP32_INITIAL_HARDENED_CHILD;
        break;
    case BTCScriptConfig_SimpleType_P2WPKH_P2SH:
        purpose = 49 + BIP32_INITIAL_HARDENED_CHILD;
        break;
    default:
        assert_true(false);
        return;
    }
    BTCSignInitRequest init_req = {
        .coin = BTCCoin_BTC,
        .script_configs_count = 1,
        .script_configs =
            {
                {
                    // First script config varies per test (testing all possible types)
                    .script_config =
                        {
                            .which_config = BTCScriptConfig_simple_type_tag,
                            .config =
                                {
                                    .simple_type = mod->script_type,
                                },
                        },
                    .keypath_count = 3,
                    .keypath =
                        {
                            purpose,
                            0 + BIP32_INITIAL_HARDENED_CHILD,
                            10 + BIP32_INITIAL_HARDENED_CHILD,
                        },
                },
            },
        .version = 1,
        .num_inputs = 2,
        .num_outputs = 6,
        .locktime = 0,
    };

    _input_t inputs[2] = {
        {
            .input =
                {
                    .prevOutHash =
                        "\x45\x17\x74\x50\x1b\xaf\xdf\xf7\x46\x09\x0e\x06\x16\xd9\x5e\xd0\x80\xd7"
                        "\x82\x9a\xfe\xa2\xbd\x97\x8a\xf8\x11\xf4\x5e\x43\x81\x39",
                    .prevOutIndex = 1,
                    .prevOutValue = 1010000000, // btc 10.1, matches prevout tx output at index 1.
                    .sequence = 0xffffffff,
                    .keypath_count = 5,
                    .keypath =
                        {
                            init_req.script_configs[0].keypath[0],
                            init_req.script_configs[0].keypath[1],
                            init_req.script_configs[0].keypath[2],
                            0,
                            5,
                        },
                },
            .prevtx_init =
                {
                    .version = 1,
                    .num_inputs = 2,
                    .num_outputs = 2,
                    .locktime = 0,
                },
            .prevtx_inputs =
                {
                    {
                        .prev_out_hash =
                            {
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            },
                        .prev_out_index = 3,
                        .signature_script =
                            {
                                .bytes = "signature script",
                                .size = 16,
                            },
                        .sequence = 0xffffffff - 2,
                    },
                    {
                        .prev_out_hash =
                            {
                                0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                            },
                        .prev_out_index = 23,
                        .signature_script =
                            {
                                .bytes = "signature script 2",
                                .size = 18,
                            },
                        .sequence = 123456,
                    },
                },
            .prevtx_outputs =
                {
                    {.value = 101000000, // btc 1.01
                     .pubkey_script =
                         {
                             .bytes = "pubkey script",
                             .size = 13,
                         }},
                    {.value = 1010000000, // btc 10.1
                     .pubkey_script =
                         {
                             .bytes = "pubkey script 2",
                             .size = 15,
                         }},
                },
        },
        {
            .input =
                {
                    .prevOutHash =
                        "\x40\x9b\x4f\x56\xca\x9f\x06\xcb\x88\x28\x03\xad\x55\x4b\xeb\x1d\x9e\xf8"
                        "\x78\x07\xf0\x52\x29\xe7\x55\x15\xe4\xb2\xaa\x87\x69\x1d",
                    .prevOutIndex = 0,
                    .prevOutValue = 1020000000, // btc 10.2, matches prevout tx output at index 0.
                    .sequence = 0xffffffff,
                    .keypath_count = 5,
                    .keypath =
                        {
                            init_req.script_configs[0].keypath[0],
                            init_req.script_configs[0].keypath[1],
                            init_req.script_configs[0].keypath[2],
                            0,
                            7,
                        },
                },
            .prevtx_init =
                {
                    .version = 2,
                    .num_inputs = 1,
                    .num_outputs = 1,
                    .locktime = 87654,
                },
            .prevtx_inputs =
                {
                    {
                        .prev_out_hash =
                            {
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            },
                        .prev_out_index = 3,
                        .signature_script =
                            {
                                .bytes = "signature script",
                                .size = 16,
                            },
                        .sequence = 0xffffffff - 2,
                    },
                },
            .prevtx_outputs =
                {
                    {.value = 1020000000, // btc 10.2
                     .pubkey_script =
                         {
                             .bytes = "pubkey script",
                             .size = 13,
                         }},
                },
        },
    };

    if (mod->mixed_inputs) {
        init_req.script_configs_count = 2;
        BTCScriptConfigWithKeypath sc = {
            // Second script config fixed, so in some tests it will be different than the
            // first, testing that mixed inputs are allowed.
            .script_config =
                {
                    .which_config = BTCScriptConfig_simple_type_tag,
                    .config =
                        {
                            .simple_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
                        },
                },
            .keypath_count = 3,
            .keypath =
                {
                    49 + BIP32_INITIAL_HARDENED_CHILD,
                    0 + BIP32_INITIAL_HARDENED_CHILD,
                    10 + BIP32_INITIAL_HARDENED_CHILD,
                },
        };
        init_req.script_configs[1] = sc;
        inputs[0].input.script_config_index = 1;
        // Fix input keypath prefix to match the account script config.
        for (size_t i = 0; i < 3; i++) {
            inputs[0].input.keypath[i] = sc.keypath[i];
        }
    };
    if (mod->invalid_input_script_config_index) {
        inputs[0].input.script_config_index = init_req.script_configs_count;
    }
    if (mod->wrong_account_input) {
        inputs[0].input.keypath[2] = inputs[0].input.keypath[2] + 1;
    }
    if (mod->wrong_coin_input) {
        inputs[0].input.keypath[1] = 1 + BIP32_INITIAL_HARDENED_CHILD;
    }
    if (mod->wrong_sequence_number) {
        inputs[0].input.sequence = 0;
    }
    if (mod->locktime_applies) {
        init_req.locktime = 1;
        inputs[0].input.sequence = 0xffffffff - 1;
    }
    if (mod->user_aborts_locktime_rbf) {
        init_req.locktime = 1;
        inputs[0].input.sequence = 0xffffffff - 2;
    }
    if (mod->locktime_zero_with_rbf) {
        init_req.locktime = 0;
        inputs[0].input.sequence = 0xffffffff - 2;
    }
    if (mod->wrong_input_value) {
        inputs[0].input.prevOutValue = 0;
    }
    if (mod->overflow_input_values_pass1) {
        inputs[1].input.prevOutValue = ULLONG_MAX - inputs[0].input.prevOutValue + 1;
    }
    if (mod->input_wrong_value) {
        inputs[0].input.prevOutValue += 1;
    }
    if (mod->wrong_prevouthash) {
        inputs[0].input.prevOutHash[0] += 1;
    }

    BTCSignOutputRequest outputs[6] = {
        {
            .ours = false,
            .type = BTCOutputType_P2PKH,
            .value = 100000000, // btc 1
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2SH,
            .value = 1234567890, // btc 12.3456789
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2WPKH,
            .value = 6000, // btc .00006
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2WSH,
            .value = 7000, // btc .00007
            .payload =
                {
                    .size = 32,
                    .bytes =
                        {
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                        },
                },
        },
        {
            // change
            .ours = true,
            .value = 690000000, // btc 6.9
            .keypath_count = 5,
            .keypath =
                {
                    init_req.script_configs[0].keypath[0],
                    init_req.script_configs[0].keypath[1],
                    init_req.script_configs[0].keypath[2],
                    mod->bip44_change,
                    3,
                },
        },
        {
            // change #2
            .ours = true,
            .value = 100,
            .keypath_count = 5,
            .keypath =
                {
                    init_req.script_configs[0].keypath[0],
                    init_req.script_configs[0].keypath[1],
                    init_req.script_configs[0].keypath[2],
                    mod->bip44_change,
                    30,
                },
        },
    };
    const uint64_t total = 1339999900; // sum of all non-change outputs + fee
    const uint64_t fee = 5419010; // sum of all inputs - sum of all outputs

    if (mod->invalid_change_script_config_index) {
        outputs[4].script_config_index = init_req.script_configs_count;
    }
    if (mod->wrong_account_change) {
        outputs[4].keypath[2] = outputs[4].keypath[2] + 1;
    }
    if (mod->wrong_coin_change) {
        outputs[4].keypath[1] = 1 + BIP32_INITIAL_HARDENED_CHILD;
    }
    if (mod->wrong_output_value) {
        outputs[0].value = 0;
    }
    if (mod->overflow_output_out) {
        outputs[0].value = ULLONG_MAX;
    }
    if (mod->overflow_output_ours) {
        outputs[4].value = ULLONG_MAX;
    }
    if (mod->litecoin_rbf_disabled) {
        init_req.coin = BTCCoin_LTC;
        init_req.locktime = 1;
        for (size_t i = 0; i < init_req.script_configs_count; i++) {
            init_req.script_configs[i].keypath[1] = 2 + BIP32_INITIAL_HARDENED_CHILD;
        }
        inputs[0].input.sequence = 0xffffffff - 2;
        inputs[0].input.keypath[1] = 2 + BIP32_INITIAL_HARDENED_CHILD;
        inputs[1].input.keypath[1] = 2 + BIP32_INITIAL_HARDENED_CHILD;
        outputs[4].keypath[1] = 2 + BIP32_INITIAL_HARDENED_CHILD;
        outputs[5].keypath[1] = 2 + BIP32_INITIAL_HARDENED_CHILD;
    }
    if (mod->p2tr_output) {
        outputs[0].type = BTCOutputType_P2TR;
        outputs[0].payload.size = 32;
        memcpy(
            outputs[0].payload.bytes,
            "\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4"
            "\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c",
            32);
    }

    BTCSignNextResponse next = {0};
    assert_int_equal(APP_BTC_OK, app_btc_sign_init(&init_req, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 0);
    assert_int_equal(next.prev_index, 0); // arbitrary
    assert_false(next.has_signature);

    if (mod->state_init_after_init) {
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_init(&init_req, &next));
        return;
    }
    if (mod->state_output_after_init) {
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_output(&outputs[0], &next));
        return;
    }

    // === Inputs Pass 1

    // First input, pass1.
    if (!mod->wrong_sequence_number && !mod->wrong_input_value &&
        !mod->invalid_input_script_config_index) {
        expect_value(
            __wrap_btc_common_is_valid_keypath_address_simple,
            script_type,
            init_req.script_configs[inputs[0].input.script_config_index]
                .script_config.config.simple_type);
        expect_memory(
            __wrap_btc_common_is_valid_keypath_address_simple,
            keypath,
            inputs[0].input.keypath,
            inputs[0].input.keypath_count * sizeof(uint32_t));
    }
    if (mod->wrong_coin_input || mod->wrong_account_input || mod->wrong_sequence_number ||
        mod->wrong_input_value || mod->invalid_input_script_config_index) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input(&inputs[0].input, &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[0].input, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_PREVTX_INIT);
    assert_int_equal(next.index, 0);
    assert_false(next.has_signature);

    // First input, prev tx.
    {
        size_t input_index = 0;
        if (!_stream_prevtx(mod, input_index, &inputs[input_index], &next)) {
            return;
        }
        assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
        assert_int_equal(next.index, 1);
    }

    // Second input, pass1.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[1].input.keypath,
        inputs[1].input.keypath_count * sizeof(uint32_t));
    if (mod->overflow_input_values_pass1) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input(&inputs[1].input, &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[1].input, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_PREVTX_INIT);
    assert_int_equal(next.index, 1);
    assert_false(next.has_signature);

    // Second input, prev tx.
    {
        size_t input_index = 1;
        if (!_stream_prevtx(mod, input_index, &inputs[input_index], &next)) {
            return;
        }
        assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
        assert_int_equal(next.index, 0);
    }

    // === Outputs

    // First output
    if (mod->wrong_output_value) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[0], &next));
        return;
    }
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[0].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount0");
    if (!mod->litecoin_rbf_disabled) {
        if (mod->p2tr_output) {
            expect_string(
                __wrap_workflow_verify_recipient,
                recipient,
                "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr");
        } else {
            expect_string(
                __wrap_workflow_verify_recipient, recipient, "12ZEw5Hcv1hTb6YUQJ69y1V7uhcoDz92PH");
        }
    } else {
        expect_string(
            __wrap_workflow_verify_recipient, recipient, "LLnCCHbSzfwWquEdaS5TF2Yt7uz5Qb1SZ1");
    }
    expect_string(__wrap_workflow_verify_recipient, amount, "amount0");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[0], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
    assert_int_equal(next.index, 1);
    assert_false(next.has_signature);

    // Second output
    if (mod->overflow_output_out) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[1], &next));
        return;
    }
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[1].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount1");
    if (!mod->litecoin_rbf_disabled) {
        expect_string(
            __wrap_workflow_verify_recipient, recipient, "34oVnh4gNviJGMnNvgquMeLAxvXJuaRVMZ");
    } else {
        expect_string(
            __wrap_workflow_verify_recipient, recipient, "MB1e6aUeL3Zj4s4H2ZqFBHaaHd7kvvzTco");
    }
    expect_string(__wrap_workflow_verify_recipient, amount, "amount1");
    will_return(__wrap_workflow_verify_recipient, !mod->user_aborts_output);
    if (mod->user_aborts_output) {
        assert_int_equal(APP_BTC_ERR_USER_ABORT, app_btc_sign_output(&outputs[1], &next));
        // Check the process is really aborted, can't proceed to next expected output.
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_output(&outputs[2], &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[1], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
    assert_int_equal(next.index, 2);
    assert_false(next.has_signature);

    // Third output
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[2].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount2");
    if (!mod->litecoin_rbf_disabled) {
        expect_string(
            __wrap_workflow_verify_recipient,
            recipient,
            "bc1qxvenxvenxvenxvenxvenxvenxvenxven2ymjt8");
    } else {
        expect_string(
            __wrap_workflow_verify_recipient,
            recipient,
            "ltc1qxvenxvenxvenxvenxvenxvenxvenxvenwcpknh");
    }
    expect_string(__wrap_workflow_verify_recipient, amount, "amount2");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[2], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
    assert_int_equal(next.index, 3);
    assert_false(next.has_signature);

    // Fourth output
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[3].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount3");
    if (!mod->litecoin_rbf_disabled) {
        expect_string(
            __wrap_workflow_verify_recipient,
            recipient,
            "bc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqd8sxw4");
    } else {
        expect_string(
            __wrap_workflow_verify_recipient,
            recipient,
            "ltc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqwr7k5s");
    }
    expect_string(__wrap_workflow_verify_recipient, amount, "amount3");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[3], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
    assert_int_equal(next.index, 4);
    assert_false(next.has_signature);

    // Fifth output, change. Last output also invokes verification of total and
    // fee.
    if (!mod->invalid_change_script_config_index) {
        expect_value(
            __wrap_btc_common_is_valid_keypath_address_simple,
            script_type,
            init_req.script_configs[0].script_config.config.simple_type);
        expect_memory(
            __wrap_btc_common_is_valid_keypath_address_simple,
            keypath,
            outputs[4].keypath,
            outputs[4].keypath_count * sizeof(uint32_t));
    }
    if (!mod->seeded) {
        assert_int_equal(APP_BTC_ERR_UNKNOWN, app_btc_sign_output(&outputs[4], &next));
        return;
    }
    if (mod->wrong_coin_change || mod->wrong_account_change || mod->bip44_change != 1 ||
        mod->invalid_change_script_config_index) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[4], &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[4], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_OUTPUT);
    assert_int_equal(next.index, 5);
    assert_false(next.has_signature);

    // Sixth output, change. Last output also invokes verification of total and fee.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        outputs[5].keypath,
        outputs[5].keypath_count * sizeof(uint32_t));
    if (mod->overflow_output_ours) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[5], &next));
        return;
    }

    expect_string(
        __wrap_workflow_confirm_blocking, params->body, "There are 2\nchange outputs.\nProceed?");
    will_return(__wrap_workflow_confirm_blocking, !mod->user_aborts_multiple_changes);
    if (mod->user_aborts_multiple_changes) {
        assert_int_equal(APP_BTC_ERR_USER_ABORT, app_btc_sign_output(&outputs[5], &next));
        return;
    }

    if (mod->litecoin_rbf_disabled) {
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, 1);
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_DISABLED);
        will_return(__wrap_apps_btc_confirm_locktime_rbf, true);
    }

    if (mod->locktime_applies) {
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, 1);
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_OFF);
        will_return(__wrap_apps_btc_confirm_locktime_rbf, true);
    }

    if (mod->user_aborts_locktime_rbf) {
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, 1);
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_ON);
        will_return(__wrap_apps_btc_confirm_locktime_rbf, false);

        assert_int_equal(APP_BTC_ERR_USER_ABORT, app_btc_sign_output(&outputs[5], &next));
        // Check the process is really aborted, can't proceed to next stage.
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_input(&inputs[0].input, &next));
        return;
    }
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, total);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount total");
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, fee);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    } else {
        expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "LTC");
    }
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount fee");
    expect_string(__wrap_workflow_verify_total, total, "amount total");
    expect_string(__wrap_workflow_verify_total, fee, "amount fee");
    will_return(__wrap_workflow_verify_total, !mod->user_aborts_total);
    if (mod->user_aborts_total) {
        assert_int_equal(APP_BTC_ERR_USER_ABORT, app_btc_sign_output(&outputs[5], &next));
        // Check the process is really aborted, can't proceed to next stage.
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_input(&inputs[0].input, &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[5], &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 0);
    assert_false(next.has_signature);

    // === Inputs Pass 2

    if (mod->input_sum_changes) {
        inputs[0].input.prevOutValue += inputs[1].input.prevOutValue + 1;
    }
    if (mod->input_sum_last_mismatch) {
        inputs[0].input.prevOutValue -= 1; // errors even if we decrease the amount
    }
    if (mod->overflow_input_values_pass2) {
        inputs[1].input.prevOutValue = ULLONG_MAX - inputs[0].input.prevOutValue + 1;
    }

    // First input, pass2.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[inputs[0].input.script_config_index]
            .script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[0].input.keypath,
        inputs[0].input.keypath_count * sizeof(uint32_t));
    if (mod->input_sum_changes) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input(&inputs[0].input, &next));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[0].input, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 1);
    assert_true(next.has_signature);
    if (mod->check_sigs) {
        switch (mod->script_type) {
        case BTCScriptConfig_SimpleType_P2WPKH: {
            if (mod->p2tr_output) {
                const uint8_t expected_signature[64] =
                    "\x48\x21\x2e\x9e\xd9\xc3\xb5\x3a\xaa\xe9\xba\x0d\xf8\x74\xd8\xcd\x7c\x2c\x0c"
                    "\x1f\xc0\xcd\x59\xdb\x40\xef\x41\xd8\xe8\x86\xe5\xcb\x10\x25\xb7\xf5\x92\x95"
                    "\x84\x34\x2b\xc1\x2f\xb6\x31\xc2\xee\xe7\xf5\x58\x09\x22\x15\x2e\x3d\x18\x43"
                    "\x9e\x64\x01\x93\xdb\x3e\x2e";
                assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            } else {
                const uint8_t expected_signature[64] =
                    "\x64\xbd\xe8\xf2\xa1\xbf\xaf\x54\x0a\x76\x01\x76\x89\xe9\x51\xa2\xf7\xa5\x7d"
                    "\x8a\xa7\xd7\x52\xb7\xd8\x82\xc9\x78\x48\x79\xa4\x3c\x7f\x48\xe5\x67\xf2\xbd"
                    "\x3b\xea\x6b\xf3\x1c\xa9\x9b\xa9\x97\x7c\x0e\x72\xbe\xc0\xe3\x43\x24\x18\x22"
                    "\xe6\xc9\xd8\xaf\x52\x17\xc8";
                assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            }
            break;
        }
        case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
            const uint8_t expected_signature[64] =
                "\x93\x06\xb9\x1b\x52\xa9\x9c\x2a\xee\xee\x92\x84\x76\x25\xa1\xa7\xac\x77\x32\x74"
                "\xbd\xe5\xb3\x28\x01\xb6\xd6\x2e\x4a\xaa\xf3\xe6\x27\x05\x12\xe9\x02\xe1\x51\xdf"
                "\x8d\x7b\x7d\x52\xfb\xa5\xbd\xb8\xb9\x82\x9f\x81\x17\x66\xbd\xe3\xc9\x6c\x50\xfc"
                "\x61\xb1\x6d\x76";
            assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            break;
        }
        default:
            assert_true(false);
        }
    }

    // Second input, pass2.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[1].input.keypath,
        inputs[1].input.keypath_count * sizeof(uint32_t));
    if (mod->input_sum_last_mismatch || mod->overflow_input_values_pass2) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input(&inputs[1].input, &next));
        return;
    }

    if (mod->antikepto) {
        uint8_t host_nonce[32] = {0};
        memset(host_nonce, 0xAB, sizeof(host_nonce));

        inputs[1].input.has_host_nonce_commitment = true;
        // Make host commitment from host_nonce.
        assert_true(secp256k1_ecdsa_anti_exfil_host_commit(
            wally_get_secp_context(),
            inputs[1].input.host_nonce_commitment.commitment,
            host_nonce));

        uint8_t expected_sighash[32] =
            "\xf0\xf8\xa2\x35\xc5\xbb\x47\x12\xa6\xa7\x22\x0b\x50\xd3\x71\x59\xe6\x98\x39\x22\x1a"
            "\xfc\x22\x98\x42\x9f\x7a\x63\x37\x87\x91\x66";
        expect_memory(
            __wrap_keystore_antiklepto_secp256k1_commit,
            msg32,
            expected_sighash,
            sizeof(expected_sighash));

        assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[1].input, &next));
        assert_int_equal(next.type, BTCSignNextResponse_Type_HOST_NONCE);

        AntiKleptoSignatureRequest antiklepto_sig_req = {0};
        memcpy(antiklepto_sig_req.host_nonce, host_nonce, sizeof(host_nonce));
        assert_int_equal(APP_BTC_OK, app_btc_sign_antiklepto(&antiklepto_sig_req, &next));
        assert_int_equal(next.type, BTCSignNextResponse_Type_DONE);
        assert_true(next.has_signature);
        assert_true(next.has_anti_klepto_signer_commitment);

        { // Verify antiklepto nonce
            secp256k1_ecdsa_signature parsed_signature;
            assert_true(secp256k1_ecdsa_signature_parse_compact(
                wally_get_secp_context(), &parsed_signature, next.signature));
            uint8_t pubkey[EC_PUBLIC_KEY_UNCOMPRESSED_LEN];
            assert_true(keystore_secp256k1_pubkey_uncompressed(
                inputs[1].input.keypath, inputs[1].input.keypath_count, pubkey));
            secp256k1_pubkey parsed_pubkey;
            assert_true(secp256k1_ec_pubkey_parse(
                wally_get_secp_context(), &parsed_pubkey, pubkey, sizeof(pubkey)));
            secp256k1_ecdsa_s2c_opening opening;
            assert_true(secp256k1_ecdsa_s2c_opening_parse(
                wally_get_secp_context(), &opening, next.anti_klepto_signer_commitment.commitment));
            assert_true(secp256k1_anti_exfil_host_verify(
                wally_get_secp_context(),
                &parsed_signature,
                expected_sighash,
                &parsed_pubkey,
                host_nonce,
                &opening));
        }
    } else {
        assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[1].input, &next));
        assert_int_equal(next.type, BTCSignNextResponse_Type_DONE);
        assert_true(next.has_signature);
        if (mod->check_sigs) {
            switch (mod->script_type) {
            case BTCScriptConfig_SimpleType_P2WPKH: {
                if (mod->p2tr_output) {
                    const uint8_t expected_signature[64] =
                        "\x2d\x1c\x2b\x99\x13\x4d\x0b\xe5\x39\x2a\x57\x3b\x78\x46\xbc\x87\x53\x62"
                        "\x96\x9e\x0b\xf8\x30\x8c\x6c\x2e\x9a\xc6\xce\x76\xa5\xcb\x31\xaf\xc0\x40"
                        "\xef\xe7\x3e\x6b\x75\x7f\x34\x12\x70\x96\xa0\x5d\x0e\x1c\x0c\x9c\xdd\x4a"
                        "\xa7\xce\x4b\xba\xd5\x10\x64\x1b\x39\xcb";
                    assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
                } else {
                    const uint8_t expected_signature[64] =
                        "\x3f\x1b\xa2\xd9\x74\x14\x98\xcc\xa5\xe4\xb0\x52\xb4\xdf\xaf\xc2\x8d\x49"
                        "\x46\x9e\xae\x6e\x64\x3c\x02\xa4\x44\xfb\x20\xf2\xb2\x0e\x2e\xef\xd3\x12"
                        "\x48\xb3\xa9\xde\x3c\x28\x26\x49\x06\xfa\x90\x1c\xfa\xc1\xa8\xcb\x54\x71"
                        "\xaa\x86\xfc\xcc\x6e\x24\xdb\x53\xc1\x00";
                    assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
                }
                break;
            }
            case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
                const uint8_t expected_signature[64] =
                    "\x13\x51\xc4\x46\x15\xae\x0a\x2d\x04\x94\xbf\x9c\xa7\xe8\xe5\x70\xff\x11\x7a"
                    "\x92\x22\x8f\x3c\x4b\xa3\x85\x80\x02\xb9\x4e\x0a\x97\x75\x83\x61\xd5\x26\x3b"
                    "\x26\x09\x0c\x5b\x00\x42\x1a\xd5\x78\xc8\xb0\xa0\xd8\x9e\x59\x57\xf4\x31\x79"
                    "\xe0\x79\x07\xef\xce\x1f\xc9";
                assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
                break;
            }
            default:
                assert_false(true);
            }
        }
    }
}

static const _modification_t _valid = {
    .script_type = BTCScriptConfig_SimpleType_P2WPKH,
    .seeded = true,
    .bip44_change = 1,
};

static void _test_btc_sign(void** state)
{
    _modification_t modified = _valid;
    modified.check_sigs = true;
    _sign(&modified);
}
static void _test_seeded(void** state)
{
    _modification_t modified = _valid;
    modified.seeded = false;
    _sign(&modified);
}
static void _test_script_type_p2wpkh_p2sh(void** state)
{
    _modification_t modified = _valid;
    modified.script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH;
    modified.check_sigs = true;
    _sign(&modified);
}
static void _test_wrong_coin_input(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_coin_input = true;
    _sign(&modified);
}
static void _test_wrong_coin_change(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_coin_change = true;
    _sign(&modified);
}
static void _test_wrong_account_input(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_account_input = true;
    _sign(&modified);
}
static void _test_wrong_account_change(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_account_change = true;
    _sign(&modified);
}
static void _test_btc_bip44_change(void** state)
{
    _modification_t modified = _valid;
    modified.bip44_change = 0;
    _sign(&modified);
    modified.bip44_change = 2;
    _sign(&modified);
}
static void _test_input_sum_changes(void** state)
{
    _modification_t modified = _valid;
    modified.input_sum_changes = true;
    _sign(&modified);
}
static void _test_input_sum_last_mismatch(void** state)
{
    _modification_t modified = _valid;
    modified.input_sum_last_mismatch = true;
    _sign(&modified);
}
static void _test_state_init_after_init(void** state)
{
    _modification_t modified = _valid;
    modified.state_init_after_init = true;
    _sign(&modified);
}
static void _test_state_output_after_init(void** state)
{
    _modification_t modified = _valid;
    modified.state_output_after_init = true;
    _sign(&modified);
}
static void _test_wrong_sequence_number(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_sequence_number = true;
    _sign(&modified);
}
static void _test_wrong_input_value(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_input_value = true;
    _sign(&modified);
}
static void _test_wrong_output_value(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_output_value = true;
    _sign(&modified);
}
static void _test_user_aborts_output(void** state)
{
    _modification_t modified = _valid;
    modified.user_aborts_output = true;
    _sign(&modified);
}
static void _test_litecoin_rbf_disabled(void** state)
{
    _modification_t modified = _valid;
    modified.litecoin_rbf_disabled = true;
    _sign(&modified);
}
static void _test_locktime_applies(void** state)
{
    _modification_t modified = _valid;
    modified.locktime_applies = true;
    _sign(&modified);
}
static void _test_user_aborts_locktime_rbf(void** state)
{
    _modification_t modified = _valid;
    modified.user_aborts_locktime_rbf = true;
    _sign(&modified);
}
static void _test_locktime_zero_with_rbf(void** state)
{
    _modification_t modified = _valid;
    modified.locktime_zero_with_rbf = true;
    _sign(&modified);
}
static void _test_user_aborts_total(void** state)
{
    _modification_t modified = _valid;
    modified.user_aborts_total = true;
    _sign(&modified);
}
static void _test_user_aborts_multiple_changes(void** state)
{
    _modification_t modified = _valid;
    modified.user_aborts_multiple_changes = true;
    _sign(&modified);
}
static void _test_overflow_input_values_pass1(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_input_values_pass1 = true;
    _sign(&modified);
}
static void _test_overflow_input_values_pass2(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_input_values_pass2 = true;
    _sign(&modified);
}
static void _test_overflow_output_out(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_output_out = true;
    _sign(&modified);
}
static void _test_overflow_output_ours(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_output_ours = true;
    _sign(&modified);
}
static void _test_state_previnit_after_previnit(void** state)
{
    _modification_t modified = _valid;
    modified.state_previnit_after_previnit = true;
    _sign(&modified);
}
static void _test_prevtx_no_inputs(void** state)
{
    _modification_t modified = _valid;
    modified.prevtx_no_inputs = true;
    _sign(&modified);
}
static void _test_prevtx_no_outputs(void** state)
{
    _modification_t modified = _valid;
    modified.prevtx_no_outputs = true;
    _sign(&modified);
}
static void _test_input_wrong_value(void** state)
{
    _modification_t modified = _valid;
    modified.input_wrong_value = true;
    _sign(&modified);
}
static void _test_wrong_prevouthash(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_prevouthash = true;
    _sign(&modified);
}
static void _test_mixed_inputs(void** state)
{
    _modification_t modified = _valid;
    modified.mixed_inputs = true;
    _sign(&modified);
}
static void _test_invalid_input_script_config_index(void** state)
{
    _modification_t modified = _valid;
    modified.invalid_input_script_config_index = true;
    _sign(&modified);
    modified.mixed_inputs = true;
    _sign(&modified);
}
static void _test_invalid_change_script_config_index(void** state)
{
    _modification_t modified = _valid;
    modified.invalid_change_script_config_index = true;
    _sign(&modified);
    modified.mixed_inputs = true;
    _sign(&modified);
}
static void _test_antiklepto(void** state)
{
    _modification_t modified = _valid;
    modified.antikepto = true;
    _sign(&modified);
}
static void _test_p2tr_output(void** state)
{
    _modification_t modified = _valid;
    modified.p2tr_output = true;
    modified.check_sigs = true;
    _sign(&modified);
}
int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_sign_init),

        cmocka_unit_test(_test_btc_sign),
        cmocka_unit_test(_test_seeded),
        cmocka_unit_test(_test_script_type_p2wpkh_p2sh),
        cmocka_unit_test(_test_wrong_coin_input),
        cmocka_unit_test(_test_wrong_coin_change),
        cmocka_unit_test(_test_wrong_account_input),
        cmocka_unit_test(_test_wrong_account_change),
        cmocka_unit_test(_test_btc_bip44_change),
        cmocka_unit_test(_test_input_sum_changes),
        cmocka_unit_test(_test_input_sum_last_mismatch),
        cmocka_unit_test(_test_state_init_after_init),
        cmocka_unit_test(_test_state_output_after_init),
        cmocka_unit_test(_test_wrong_sequence_number),
        cmocka_unit_test(_test_wrong_input_value),
        cmocka_unit_test(_test_wrong_output_value),
        cmocka_unit_test(_test_user_aborts_output),
        cmocka_unit_test(_test_litecoin_rbf_disabled),
        cmocka_unit_test(_test_locktime_applies),
        cmocka_unit_test(_test_user_aborts_locktime_rbf),
        cmocka_unit_test(_test_locktime_zero_with_rbf),
        cmocka_unit_test(_test_user_aborts_total),
        cmocka_unit_test(_test_user_aborts_multiple_changes),
        cmocka_unit_test(_test_overflow_input_values_pass1),
        cmocka_unit_test(_test_overflow_input_values_pass2),
        cmocka_unit_test(_test_overflow_output_out),
        cmocka_unit_test(_test_overflow_output_ours),
        cmocka_unit_test(_test_state_previnit_after_previnit),
        cmocka_unit_test(_test_prevtx_no_inputs),
        cmocka_unit_test(_test_prevtx_no_outputs),
        cmocka_unit_test(_test_input_wrong_value),
        cmocka_unit_test(_test_wrong_prevouthash),
        cmocka_unit_test(_test_mixed_inputs),
        cmocka_unit_test(_test_invalid_input_script_config_index),
        cmocka_unit_test(_test_invalid_change_script_config_index),
        cmocka_unit_test(_test_antiklepto),
        cmocka_unit_test(_test_p2tr_output),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
