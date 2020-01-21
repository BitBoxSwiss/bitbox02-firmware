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

static void _test_btc_sign_happy(void** state)
{
    // establish valid request to modify
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

    BTCSignInputRequest inputs[1] = {
        {
            .prevOutHash = "\x1c\xbb\xc4\x00\x90\x9a\x19\x3e\xfe\xfb\x44\x80\xff\x9a\x1e\xdc\x64"
                           "\x74\x6d\x01\x11\xcb\xd7\xdc\x2d\x19\x93\x0e\x16\xd5\x7b\x4e",
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
    };

    BTCSignOutputRequest outputs[2] = {
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

    mock_state(_mock_seed, _mock_bip39_seed);

    BTCSignNextResponse next = {0};

    expect_value(__wrap_apps_btc_confirm_multisig, coin, init_req.coin);
    expect_memory(
        __wrap_apps_btc_confirm_multisig,
        multisig,
        &init_req.script_config.config.multisig,
        sizeof(BTCScriptConfig_Multisig));
    assert_int_equal(APP_BTC_SIGN_OK, app_btc_sign_init(&init_req, &next));
    assert_int_equal(next.type, BTCSignNextResponse_Type_INPUT);
    assert_int_equal(next.index, 0);

    assert_int_equal(APP_BTC_SIGN_OK, app_btc_sign_input(&inputs[0], &next));
    assert_int_equal(APP_BTC_SIGN_OK, app_btc_sign_output(&outputs[0], &next));

    expect_string(
        __wrap_workflow_verify_recipient,
        recipient,
        "tb1qtxyqynfxwsk8f5gu8v5g8e6hs3njtglkywhvyztk6v8znvx5kddsmhuve2");
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, locktime, init_req.locktime);
    expect_value(__wrap_apps_btc_confirm_locktime_rbf, rbf, CONFIRM_LOCKTIME_RBF_OFF);
    expect_string(__wrap_workflow_verify_total, total, "0.00090175 TBTC");
    expect_string(__wrap_workflow_verify_total, fee, "0.00000175 TBTC");
    assert_int_equal(APP_BTC_SIGN_OK, app_btc_sign_output(&outputs[1], &next));

    assert_int_equal(APP_BTC_SIGN_OK, app_btc_sign_input(&inputs[0], &next));
    assert_true(next.has_signature);
    // d9b92d11eae078d7ae3b589bbbc76f4b514fe7aeb844db397937cded4dc594e450cc7e4bf6ab67fef292800d89da9d108f0767c465534486b235426b192928af
    // TODO: investigate why electrum produces a different but valid sig:
    // 48662590d14217fc78db9075a74b110e92fecec11d4ff527444bfe3539e1009a09dca025b7f1a45d8725b26f96b0ec5661bad94bfd5dbce6f6f133f624820aa4
    const uint8_t* expected_signature = (const uint8_t*)
        "\xd9\xb9\x2d\x11\xea\xe0\x78\xd7\xae\x3b\x58\x9b\xbb\xc7\x6f\x4b\x51\x4f\xe7\xae\xb8\x44\xdb\x39\x79\x37\xcd\xed\x4d\xc5\x94\xe4\x50\xcc\x7e\x4b\xf6\xab\x67\xfe\xf2\x92\x80\x0d\x89\xda\x9d\x10\x8f\x07\x67\xc4\x65\x53\x44\x86\xb2\x35\x42\x6b\x19\x29\x28\xaf";
    assert_memory_equal(next.signature, expected_signature, sizeof(expected_signature));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_sign_happy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
