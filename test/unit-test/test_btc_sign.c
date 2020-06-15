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
#include <wally_bip32.h>
#include <workflow/confirm.h>

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

bool __wrap_btc_common_format_amount(uint64_t satoshi, const char* unit, char* out, size_t out_len)
{
    check_expected(satoshi);
    check_expected(unit);
    snprintf(out, out_len, "%s", (const char*)(mock()));
    return true;
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
    mock_state(mod->seeded ? _mock_seed : NULL, mod->seeded ? _mock_bip39_seed : NULL);

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
            .hash =
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
            .hash =
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
            .hash =
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
            .hash =
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
    expect_value(__wrap_btc_common_format_amount, satoshi, outputs[0].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount0");
    if (!mod->litecoin_rbf_disabled) {
        expect_string(
            __wrap_workflow_verify_recipient, recipient, "12ZEw5Hcv1hTb6YUQJ69y1V7uhcoDz92PH");
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
    expect_value(__wrap_btc_common_format_amount, satoshi, outputs[1].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount1");
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
    expect_value(__wrap_btc_common_format_amount, satoshi, outputs[2].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount2");
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
    expect_value(__wrap_btc_common_format_amount, satoshi, outputs[3].value);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount3");
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
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, 0);
        expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_ON);
        will_return(__wrap_apps_btc_confirm_locktime_rbf, false);

        assert_int_equal(APP_BTC_ERR_USER_ABORT, app_btc_sign_output(&outputs[5], &next));
        // Check the process is really aborted, can't proceed to next stage.
        assert_int_equal(APP_BTC_ERR_STATE, app_btc_sign_input(&inputs[0].input, &next));
        return;
    }
    expect_value(__wrap_btc_common_format_amount, satoshi, total);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount total");
    expect_value(__wrap_btc_common_format_amount, satoshi, fee);
    if (!mod->litecoin_rbf_disabled) {
        expect_string(__wrap_btc_common_format_amount, unit, "BTC");
    } else {
        expect_string(__wrap_btc_common_format_amount, unit, "LTC");
    }
    will_return(__wrap_btc_common_format_amount, "amount fee");
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
            const uint8_t expected_signature[64] =
                "\x41\x6c\x01\x6a\xa0\x6a\xb1\x6b\x1a\x57\x82\x29\x55\x1f\x92\x9d\x0a\x68\x85\x52"
                "\xee\x4b\x58\x59\xc1\x69\x91\x7e\x1e\x5b\x65\x69\x6f\x26\xed\xbc\x16\x0a\x8d\x08"
                "\x66\x64\x01\x8e\x9e\x9b\x14\xba\x12\x00\x55\xd3\xf2\xc3\xf1\xdb\x27\xb9\xf0\xdf"
                "\xc7\x3b\x6b\x33";
            assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            break;
        }
        case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
            const uint8_t expected_signature[64] =
                "\x53\x38\xda\x4a\xa9\x26\x61\x00\x8a\x0c\x84\x3b\x62\x74\x07\xa9\x0d\x83\x17\x21"
                "\xfb\x08\xd7\xc9\x00\x43\x6f\x46\x86\x4b\xbc\xe7\x03\x54\xd0\xbf\xb6\x50\x23\xc9"
                "\x21\x0a\x99\xb0\xd4\xca\x75\xfe\xc9\x5a\x1e\x59\xc6\xfc\xa8\x85\xbd\x81\x12\xca"
                "\xb6\x85\xf7\x23";
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
    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&inputs[1].input, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_DONE);
    assert_true(next.has_signature);
    if (mod->check_sigs) {
        switch (mod->script_type) {
        case BTCScriptConfig_SimpleType_P2WPKH: {
            const uint8_t expected_signature[64] =
                "\xad\x02\x54\x6b\xf3\xc2\x00\xc7\x15\xde\x7c\x07\x06\xd4\xb7\x92\xbb\x42\x3b\x3a"
                "\xcf\x0c\x47\xda\x51\xac\x10\x39\x97\x42\x15\xd6\x29\xe3\xb7\xcb\xee\x81\x3b\x09"
                "\x66\x67\x8c\xea\x8a\x35\xda\xfa\x70\x43\xb4\xcc\x2c\xbf\x37\xd0\xf0\xfa\x13\xa0"
                "\xa2\x91\x6e\xad";
            assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            break;
        }
        case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
            const uint8_t expected_signature[64] =
                "\xdb\x55\x32\x95\x8c\x00\x17\x69\xe9\xe6\xb8\xe0\x41\xa1\x58\x7b\xa2\x09\x7e\x8e"
                "\xc0\x87\x6f\x92\x44\xd8\x53\x71\xe1\xa5\x71\x02\x4c\x16\xd4\xe3\x05\x55\xe6\x60"
                "\x67\x19\xde\x9e\xe6\x07\x58\xc8\x4e\x1c\xd6\x58\x6a\xd3\x53\x61\x4c\x63\xbd\xee"
                "\x4f\x14\x3e\x60";
            assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
            break;
        }
        default:
            assert_false(true);
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
    _modification_t valid = _valid;
    valid.check_sigs = true;
    _sign(&valid);
}
static void _test_seeded(void** state)
{
    _modification_t invalid = _valid;
    invalid.seeded = false;
    _sign(&invalid);
}
static void _test_script_type_p2wpkh_p2sh(void** state)
{
    _modification_t invalid = _valid;
    invalid.script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH;
    invalid.check_sigs = true;
    _sign(&invalid);
}
static void _test_wrong_coin_input(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_coin_input = true;
    _sign(&invalid);
}
static void _test_wrong_coin_change(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_coin_change = true;
    _sign(&invalid);
}
static void _test_wrong_account_input(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_account_input = true;
    _sign(&invalid);
}
static void _test_wrong_account_change(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_account_change = true;
    _sign(&invalid);
}
static void _test_btc_bip44_change(void** state)
{
    _modification_t invalid = _valid;
    invalid.bip44_change = 0;
    _sign(&invalid);
    invalid.bip44_change = 2;
    _sign(&invalid);
}
static void _test_input_sum_changes(void** state)
{
    _modification_t invalid = _valid;
    invalid.input_sum_changes = true;
    _sign(&invalid);
}
static void _test_input_sum_last_mismatch(void** state)
{
    _modification_t invalid = _valid;
    invalid.input_sum_last_mismatch = true;
    _sign(&invalid);
}
static void _test_state_init_after_init(void** state)
{
    _modification_t invalid = _valid;
    invalid.state_init_after_init = true;
    _sign(&invalid);
}
static void _test_state_output_after_init(void** state)
{
    _modification_t invalid = _valid;
    invalid.state_output_after_init = true;
    _sign(&invalid);
}
static void _test_wrong_sequence_number(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_sequence_number = true;
    _sign(&invalid);
}
static void _test_wrong_input_value(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_input_value = true;
    _sign(&invalid);
}
static void _test_wrong_output_value(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_output_value = true;
    _sign(&invalid);
}
static void _test_user_aborts_output(void** state)
{
    _modification_t invalid = _valid;
    invalid.user_aborts_output = true;
    _sign(&invalid);
}
static void _test_litecoin_rbf_disabled(void** state)
{
    _modification_t invalid = _valid;
    invalid.litecoin_rbf_disabled = true;
    _sign(&invalid);
}
static void _test_locktime_applies(void** state)
{
    _modification_t invalid = _valid;
    invalid.locktime_applies = true;
    _sign(&invalid);
}
static void _test_user_aborts_locktime_rbf(void** state)
{
    _modification_t invalid = _valid;
    invalid.user_aborts_locktime_rbf = true;
    _sign(&invalid);
}
static void _test_user_aborts_total(void** state)
{
    _modification_t invalid = _valid;
    invalid.user_aborts_total = true;
    _sign(&invalid);
}
static void _test_user_aborts_multiple_changes(void** state)
{
    _modification_t invalid = _valid;
    invalid.user_aborts_multiple_changes = true;
    _sign(&invalid);
}
static void _test_overflow_input_values_pass1(void** state)
{
    _modification_t invalid = _valid;
    invalid.overflow_input_values_pass1 = true;
    _sign(&invalid);
}
static void _test_overflow_input_values_pass2(void** state)
{
    _modification_t invalid = _valid;
    invalid.overflow_input_values_pass2 = true;
    _sign(&invalid);
}
static void _test_overflow_output_out(void** state)
{
    _modification_t invalid = _valid;
    invalid.overflow_output_out = true;
    _sign(&invalid);
}
static void _test_overflow_output_ours(void** state)
{
    _modification_t invalid = _valid;
    invalid.overflow_output_ours = true;
    _sign(&invalid);
}
static void _test_state_previnit_after_previnit(void** state)
{
    _modification_t invalid = _valid;
    invalid.state_previnit_after_previnit = true;
    _sign(&invalid);
}
static void _test_prevtx_no_inputs(void** state)
{
    _modification_t invalid = _valid;
    invalid.prevtx_no_inputs = true;
    _sign(&invalid);
}
static void _test_prevtx_no_outputs(void** state)
{
    _modification_t invalid = _valid;
    invalid.prevtx_no_outputs = true;
    _sign(&invalid);
}
static void _test_input_wrong_value(void** state)
{
    _modification_t invalid = _valid;
    invalid.input_wrong_value = true;
    _sign(&invalid);
}
static void _test_wrong_prevouthash(void** state)
{
    _modification_t invalid = _valid;
    invalid.wrong_prevouthash = true;
    _sign(&invalid);
}
static void _test_mixed_inputs(void** state)
{
    _modification_t invalid = _valid;
    invalid.mixed_inputs = true;
    _sign(&invalid);
}
static void _test_invalid_input_script_config_index(void** state)
{
    _modification_t invalid = _valid;
    invalid.invalid_input_script_config_index = true;
    _sign(&invalid);
    invalid.mixed_inputs = true;
    _sign(&invalid);
}
static void _test_invalid_change_script_config_index(void** state)
{
    _modification_t invalid = _valid;
    invalid.invalid_change_script_config_index = true;
    _sign(&invalid);
    invalid.mixed_inputs = true;
    _sign(&invalid);
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
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
