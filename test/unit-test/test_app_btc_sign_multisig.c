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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <stdio.h>

#include <apps/btc/btc_sign.h>
#include <apps/btc/confirm_locktime_rbf.h>
#include <btc_util.h>
#include <keystore.h>
#include <memory/memory.h>
#include <util.h>
#include <wally_bip32.h>

const char* _multisig_name = "foo";

bool __wrap_memory_multisig_get_by_hash(const uint8_t* hash, char* name_out)
{
    snprintf(name_out, MEMORY_MULTISIG_NAME_MAX_LEN, "%s", _multisig_name);
    return true;
}

bool __wrap_apps_btc_confirm_multisig(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig,
    bool verify_xpubs)
{
    assert_string_equal(title, "Spend from");
    check_expected(coin);
    assert_string_equal(name, _multisig_name);
    check_expected(multisig);
    assert_false(verify_xpubs);
    return true;
}

bool __wrap_workflow_verify_recipient(const char* recipient, const char* amount)
{
    check_expected(recipient);
    return true;
}

bool __wrap_workflow_verify_total(const char* total, const char* fee)
{
    check_expected(total);
    check_expected(fee);
    return true;
}

bool __wrap_apps_btc_confirm_locktime_rbf(uint32_t locktime, enum apps_btc_rbf_flag rbf)
{
    check_expected(locktime);
    check_expected(rbf);
    return true;
}

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

// sudden tenant fault inject concert weather maid people chunk youth stumble grit
static uint8_t _mock_bip39_seed[64] =
    "\x5a\x11\x5b\xcd\xbe\x0f\xe1\x70\x0e\x60\x95\x74\xf3\x57\xb0\x8d\xca\x37\x15\xb0\x35\xe6\xc7"
    "\x76\x77\x0a\xc7\xa0\xab\x2e\x2f\xea\x84\x0b\xa2\x76\x35\x06\xfa\x9c\x39\xde\x4d\xef\x27\xf6"
    "\xf8\xeb\xce\x36\x37\x02\xe9\x83\xe5\x49\xbd\x7d\xef\x14\xa0\x31\xbf\xdd";

typedef struct {
    BTCSignInputRequest input;

    // --- Previous transaction data.
    BTCPrevTxInitRequest prevtx_init;
    // actual count is in prevtx_init.num_inputs
    BTCPrevTxInputRequest prevtx_inputs[10];
    // actual count is in prevtx_init.num_outputs
    BTCPrevTxOutputRequest prevtx_outputs[10];
} _input_t;

static _input_t _inputs[1] = {
    {
        .input =
            {
                .prevOutHash =
                    "\x41\x3b\x8e\x74\x05\x15\x96\x6b\x20\x2b\x24\xc3\x19\xfc\xf3\x5f\xc5"
                    "\x37\x6e\xb2\x71\x95\xb8\x76\x62\x9a\x44\x1d\x19\xaa\x6c\x0f",
                .prevOutIndex = 0,
                .prevOutValue = 100000, // btc 0.001
                .sequence = 4294967294,
                .keypath_count = 6,
                .keypath =
                    {
                        48 + BIP32_INITIAL_HARDENED_CHILD,
                        1 + BIP32_INITIAL_HARDENED_CHILD,
                        0 + BIP32_INITIAL_HARDENED_CHILD,
                        2 + BIP32_INITIAL_HARDENED_CHILD,
                        0,
                        0,
                    },
            },
        .prevtx_init =
            {
                .version = 1,
                .num_inputs = 1,
                .num_outputs = 1,
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
            },
        .prevtx_outputs =
            {
                {.value = 100000, // btc 0.001
                 .pubkey_script =
                     {
                         .bytes = "pubkey script",
                         .size = 13,
                     }},
            },
    },
};

static BTCSignOutputRequest _outputs[2] = {
    {
        .ours = true,
        .value = 9825, // btc 0.00009825
        .keypath_count = 6,
        .keypath =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
                1,
                0,
            },
    },
    {
        .ours = false,
        .type = BTCOutputType_P2WSH,
        .value = 90000, // btc 0.0009
        .hash = {.size = 32,
                 .bytes = "\x59\x88\x02\x4d\x26\x74\x2c\x74\xd1\x1c\x3b\x28\x83\xe7\x57\x84\x67"
                          "\x25\xa3\xf6\x23\xae\xc2\x09\x76\xd3\x0e\x29\xb0\xd4\xb3\x5b"},
    },
};

static void _test_btc_sign_happy(void** state)
{
    BTCSignInitRequest init_req = {
        .coin = BTCCoin_TBTC,
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
        .keypath_account_count = 4,
        .keypath_account =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
            },
        .version = 2,
        .num_inputs = 1,
        .num_outputs = 2,
        .locktime = 1663289,
    };

    // sudden tenant fault inject concert weather maid people chunk youth stumble grit /
    // 48'/1'/0'/2'
    init_req.script_config.config.multisig.xpubs[0] = btc_util_parse_xpub(
        "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYU"
        "CugjeksHSbyZT7rq38VQF");
    // dumb rough room report huge dry sudden hamster wait foot crew obvious / 48'/1'/0'/2'
    init_req.script_config.config.multisig.xpubs[1] = btc_util_parse_xpub(
        "xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM7wT3KxLK"
        "X8Q8keZPd67kRGmrD1WJj");

    mock_state(_mock_seed, _mock_bip39_seed);

    BTCSignNextResponse next = {0};

    expect_value(__wrap_apps_btc_confirm_multisig, coin, init_req.coin);
    expect_memory(
        __wrap_apps_btc_confirm_multisig,
        multisig,
        &init_req.script_config.config.multisig,
        sizeof(BTCScriptConfig_Multisig));
    assert_int_equal(APP_BTC_OK, app_btc_sign_init(&init_req, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 0);

    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&_inputs[0].input, &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_init(&_inputs[0].prevtx_init, &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_input(&_inputs[0].prevtx_inputs[0], &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_output(&_inputs[0].prevtx_outputs[0], &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&_outputs[0], &next));

    expect_string(
        __wrap_workflow_verify_recipient,
        recipient,
        "tb1qtxyqynfxwsk8f5gu8v5g8e6hs3njtglkywhvyztk6v8znvx5kddsmhuve2");
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, init_req.locktime);
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_OFF);
    expect_string(__wrap_workflow_verify_total, total, "0.00090175 TBTC");
    expect_string(__wrap_workflow_verify_total, fee, "0.00000175 TBTC");
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&_outputs[1], &next));

    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&_inputs[0].input, &next));
    assert_true(next.has_signature);
    // d9b92d11eae078d7ae3b589bbbc76f4b514fe7aeb844db397937cded4dc594e450cc7e4bf6ab67fef292800d89da9d108f0767c465534486b235426b192928af
    // TODO: investigate why electrum produces a different but valid sig:
    // 48662590d14217fc78db9075a74b110e92fecec11d4ff527444bfe3539e1009a09dca025b7f1a45d8725b26f96b0ec5661bad94bfd5dbce6f6f133f624820aa4
    const uint8_t* expected_signature = (const uint8_t*)
        "\x5d\x9e\xb3\x32\x2d\x82\x1d\x82\x1d\x1f\xa3\xf7\x26\x95\x96\x43\x2f\x06\x50\x33\xbf\xea\x06\xc4\x5b\x51\xe3\x30\x94\x9f\x32\x6d\x51\x6a\xf5\x9b\xb7\x52\x11\x64\x40\xfc\xd6\x90\x56\x47\xdf\x20\x41\xc2\x76\x74\x3b\xab\x89\x37\x6f\x18\xb2\x46\x87\xa7\x75\x21";
    assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
}

static void _test_btc_sign_large_happy(void** state)
{
    BTCSignInitRequest init_req = {
        .coin = BTCCoin_TBTC,
        .script_config =
            {
                .which_config = BTCScriptConfig_multisig_tag,
                .config =
                    {
                        .multisig =
                            {
                                .threshold = 7,
                                .xpubs_count = 15,
                                .our_xpub_index = 14,
                            },
                    },
            },
        .keypath_account_count = 4,
        .keypath_account =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
            },
        .version = 2,
        .num_inputs = 1,
        .num_outputs = 2,
        .locktime = 1663289,
    };

    // clang-format off

    // sudden tenant fault inject concert weather maid people chunk youth stumble grit /
    // 48'/1'/0'/2'
    XPub* xpubs = init_req.script_config.config.multisig.xpubs;
    xpubs[0] = btc_util_parse_xpub("xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ");
    xpubs[1] = btc_util_parse_xpub("xpub6EQcxF2jFkYGn89AwoQJEEJkYMbRjED9AZgt7bkxQA5BLhZEoaQpUHcADbB5GxcMrTdDSGmjP7M3u462Q9otyE2PPam66P5KFLWitPVfYz9");
    xpubs[2] = btc_util_parse_xpub("xpub6EP4EycVS5dq1PN7ZqsxBtptkYhfLvLGokZjnB3fvPshMiAohh6E5TaJjAafZWoPRjo6uiZxhtDXLgCuk81ooQgwrsnEdfSWSfa4VUtX8nu");
    xpubs[3] = btc_util_parse_xpub("xpub6Eszd4BGGmHShcGtys5gbvV2zrBtW1gaorKf9YuvV4L3bePw7XePyyb2DKswZ5AhFfkcQwjQsiJEUTKhfRstRdHZUjQnJ2RJoQqL8g7FS4b");
    xpubs[4] = btc_util_parse_xpub("xpub6Df3nbvH6P3FTvjgKaZcSuydyEofK545U4Bb15JY8R9MtFkKrhYrc3bpEF6fHtNM7xQ1qHwsVpS56TJWUjbKcmRwPkQr17ovV2RaVSJaBq3");
    xpubs[5] = btc_util_parse_xpub("xpub6FQQ62gUYzS9wnHWHMPLWrpVnzS8xAf8XvfW1xzXEXTkTCtBrfbeww2zNeCgm3PbueMoq8opQvQDzp5Yf9EtiqVd7d1ASDoWSC1m7g1KHza");
    xpubs[6] = btc_util_parse_xpub("xpub6EQNZUUAzJAoFAVVetYUrFVrf7mLyYsnHiQihkA3KPhoRHx7m6SgKBYV4z5Rd9CvUc11ACN8Ap5Wxigt6GYRPUqXGFfm3833ezJpjAmvJKt");
    xpubs[7] = btc_util_parse_xpub("xpub6EGZy7cizYn2zUf9NT4qJ3Kr1ZrxdzPRcv2CwAnB1BTGWw7n9ZgDYvwmzzJXM6V7AgZ6CL3DrARZk5DzM9o8tz2RVTeC7QoHh9SxbW3b7Pw");
    xpubs[8] = btc_util_parse_xpub("xpub6DaV7oCAkm4HJQMoProrrKYq1RvcgpStgYUCzLRaaeJSBSy9WBRFMNnQyAWJUYy9myUFRTvogq1C2f7x4A2yhtYgr7gL6eZXv2eJvzU12pe");
    xpubs[9] = btc_util_parse_xpub("xpub6FFVRbdHt5DgHqR69KuWXRVDp93e1xKxv8rRLwhhCGnWaoF1ecnfdxpg2Nf1pvJTgT1UYg28CVt7YbUXFJL86vi9FaPN9QGtWLeCmf9dA24");
    xpubs[10] = btc_util_parse_xpub("xpub6FNywxebMjvSSginZrk7DfNmAHvPJAy3j6pJ9FmUQCoh4FKPzNymdHnkA1z77Ke4GK7g5GkdrBhpyXfWTbZkH6Yo1t4v524wDwF8SAKny9J");
    xpubs[11] = btc_util_parse_xpub("xpub6F1V9y6gXejomurTy2hN1UDCJidYahVkqtQJSZLYmcPcPDWkxGgWTrrLnCrCkGESSUSq6GpVVQx9kejPV97BEa9F85utABNL9r6xyPZFiDm");
    xpubs[12] = btc_util_parse_xpub("xpub6ECHc4kmTC2tQg2ZoAoazwyag9C4V6yFsZEhjwMJixdVNsUibot6uEvsZY38ZLVqWCtyc9gbzFEwHQLHCT8EiDDKSNNsFAB8NQYRgkiAQwu");
    xpubs[13] = btc_util_parse_xpub("xpub6F7CaxXzBCtvXwpRi61KYyhBRkgT1856ujHV5AbJK6ySCUYoDruBH6Pnsi6eHkDiuKuAJ2tSc9x3emP7aax9Dc3u7nP7RCQXEjLKihQu6w1");
    xpubs[14] = btc_util_parse_xpub("xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF");
    // clang-format on

    mock_state(_mock_seed, _mock_bip39_seed);

    BTCSignNextResponse next = {0};

    expect_value(__wrap_apps_btc_confirm_multisig, coin, init_req.coin);
    expect_memory(
        __wrap_apps_btc_confirm_multisig,
        multisig,
        &init_req.script_config.config.multisig,
        sizeof(BTCScriptConfig_Multisig));
    assert_int_equal(APP_BTC_OK, app_btc_sign_init(&init_req, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 0);

    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&_inputs[0].input, &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_init(&_inputs[0].prevtx_init, &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_input(&_inputs[0].prevtx_inputs[0], &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_prevtx_output(&_inputs[0].prevtx_outputs[0], &next));
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&_outputs[0], &next));

    expect_string(
        __wrap_workflow_verify_recipient,
        recipient,
        "tb1qtxyqynfxwsk8f5gu8v5g8e6hs3njtglkywhvyztk6v8znvx5kddsmhuve2");
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, init_req.locktime);
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_OFF);
    expect_string(__wrap_workflow_verify_total, total, "0.00090175 TBTC");
    expect_string(__wrap_workflow_verify_total, fee, "0.00000175 TBTC");
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&_outputs[1], &next));

    assert_int_equal(APP_BTC_OK, app_btc_sign_input(&_inputs[0].input, &next));
    assert_true(next.has_signature);
    const uint8_t* expected_signature = (const uint8_t*)
        "\xdc\x95\xd6\xe7\x7a\xef\x9b\xc2\x1d\xef\x9b\xa6\xce\x94\xcd\xf0\x4b\xcc\xfc\x50\xbb\x29\x2b\x5b\x44\x33\xf3\x2f\x79\xa0\x09\x06\x19\x3e\xac\x60\xfd\x88\xdb\x4f\xe7\xe6\xd9\xea\xb1\x7c\x21\xd6\xf4\x90\x03\xea\xdd\x8e\x12\x10\xbc\x4a\x85\x2b\xd1\xcd\x31\xe4";
    assert_memory_equal(next.signature, expected_signature, sizeof(next.signature));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_sign_happy),
        cmocka_unit_test(_test_btc_sign_large_happy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
