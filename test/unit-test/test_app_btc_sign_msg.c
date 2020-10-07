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

#include <apps/btc/btc_sign_msg.h>
#include <keystore.h>
#include <rust/rust.h>
#include <workflow/confirm.h>

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    check_expected(params->body);
    return true;
}

VerifyMessageResult __wrap_rust_workflow_verify_message(Bytes msg)
{
    return VerifyMessageResultOk;
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

static void _test_btc_sign_msg(void** state)
{
    const BTCScriptConfig script_config = {
        .which_config = BTCScriptConfig_simple_type_tag,
        .config =
            {
                .simple_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
            },
    };
    uint32_t keypath[] = {
        49 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };
    const uint8_t msg[] = "sign this message";
    uint8_t signature[65] = {0};

    mock_state(_mock_seed, _mock_bip39_seed);
    expect_string(__wrap_workflow_confirm_blocking, params->body, "Coin: Bitcoin");
    expect_string(
        __wrap_workflow_confirm_blocking, params->body, "32af55mrChF2FUjZo8rSuCbxFy5MEqjCeo");
    assert_int_equal(
        APP_BTC_OK,
        app_btc_sign_msg(
            BTCCoin_BTC,
            &script_config,
            keypath,
            sizeof(keypath) / sizeof(uint32_t),
            msg,
            sizeof(msg) - 1,
            signature));
    const uint8_t expected_signature[65] =
        "\x19\x1b\xcd\x29\x28\x6f\x95\x6a\xf5\x3b\x87\x88\x88\x4c\x91\x1e\xcb\x29\x69\xdb\xcb\x1d"
        "\xa5\x88\xd1\x57\xce\x9b\x6a\x6b\x60\x6b\x3d\xc9\x0a\x12\x74\x7a\x9d\xef\x4d\x7c\xaa\x72"
        "\x8b\xcf\x0c\xb3\x0d\xd2\xdc\x93\xba\xdd\xed\x9e\x55\xbd\x1c\x5e\x80\xe9\x78\x8a\x01";
    assert_memory_equal(signature, expected_signature, sizeof(expected_signature));
}

static void _test_btc_sign_msg_invalid(void** state)
{
    const BTCScriptConfig script_config_p2wpkh_p2sh = {
        .which_config = BTCScriptConfig_simple_type_tag,
        .config =
            {
                .simple_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
            },
    };
    const uint32_t valid_keypath[] = {
        49 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };
    const uint8_t msg[1025] = {0};
    uint8_t signature[65] = {0};

    { // Keypath does not match script config.
        const uint32_t keypath[] = {
            84 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0,
            0,
        };

        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_msg(
                BTCCoin_BTC,
                &script_config_p2wpkh_p2sh,
                keypath,
                sizeof(keypath) / sizeof(uint32_t),
                msg,
                5,
                signature));
    }

    { // Invalid coin
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_msg(
                BTCCoin_TBTC,
                &script_config_p2wpkh_p2sh,
                valid_keypath,
                sizeof(valid_keypath) / sizeof(uint32_t),
                msg,
                5,
                signature));
    }
    { // Invalid script type
        const BTCScriptConfig invalid = {
            .which_config = BTCScriptConfig_simple_type_tag,
            .config = {.simple_type = 2},
        };
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_msg(
                BTCCoin_BTC,
                &invalid,
                valid_keypath,
                sizeof(valid_keypath) / sizeof(uint32_t),
                msg,
                5,
                signature));
    }
    { // Multisig not supported
        const BTCScriptConfig invalid = {
            .which_config = BTCScriptConfig_multisig_tag,
        };
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_msg(
                BTCCoin_BTC,
                &invalid,
                valid_keypath,
                sizeof(valid_keypath) / sizeof(uint32_t),
                msg,
                5,
                signature));
    }
    { // Message too big
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_msg(
                BTCCoin_BTC,
                &script_config_p2wpkh_p2sh,
                valid_keypath,
                sizeof(valid_keypath) / sizeof(uint32_t),
                msg,
                sizeof(msg),
                signature));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_sign_msg),
        cmocka_unit_test(_test_btc_sign_msg_invalid),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
