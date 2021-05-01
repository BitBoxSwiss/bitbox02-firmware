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

#include <apps/btc/btc.h>
#include <apps/btc/btc_common.h>
#include <keystore.h>
#include <util.h>

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

// We mock all called functions to make sure they are actually called. For some,
// the real function is called as it's easier to check all function input/output
// this way.

bool __wrap_btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    check_expected(script_type);
    check_expected(keypath);
    check_expected(keypath_len);
    return mock();
}

bool __real_btc_common_outputhash_from_pubkeyhash(
    BTCScriptConfig_SimpleType script_type,
    uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size);
bool __wrap_btc_common_outputhash_from_pubkeyhash(
    BTCScriptConfig_SimpleType script_type,
    uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    assert_true(__real_btc_common_outputhash_from_pubkeyhash(
        script_type, pubkey_hash, output_hash, output_hash_size));
    return mock();
}

bool __wrap_keystore_secp256k1_pubkey_hash160(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash160_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    assert_non_null(hash160_out);
    uint8_t hash160[20] =
        "\x7c\x16\x87\x6b\x37\xb1\xb5\xa1\xf9\x0a\x53\xe9\xae\x3c\x25\xa2\x4a\x2a\x80\x8d";
    memcpy(hash160_out, hash160, sizeof(hash160));
    return mock();
}

typedef struct {
    BTCCoin coin;
    BTCPubRequest_XPubType xpub_type;
    const char* out;
} xpub_testcase_t;

typedef struct {
    BTCCoin coin;
    BTCScriptConfig_SimpleType script_type;
    const char* out;
} address_testcase_t;

static address_testcase_t _address_tests[] = {
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
        .out = "3HNUdmEorEcqoknW5A5Wx6GZTkHPm5TeJg",
    },
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH,
        .out = "bc1q0stgw6ehkx66r7g22056u0p95f9z4qydmyx3ja",
    },

    {
        .coin = BTCCoin_TBTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
        .out = "2N8vghWAqTh8C1YR3kHhPa3Fpg6VZZRmwS8",
    },
    {
        .coin = BTCCoin_TBTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH,
        .out = "tb1q0stgw6ehkx66r7g22056u0p95f9z4qyd3zazfw",
    },

    {
        .coin = BTCCoin_LTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
        .out = "MPacweemoMUGcG4QB34rmjWxnSsqj8iQ5f",
    },
    {
        .coin = BTCCoin_LTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH,
        .out = "ltc1q0stgw6ehkx66r7g22056u0p95f9z4qydlcu42d",
    },

    {
        .coin = BTCCoin_TLTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH_P2SH,
        .out = "2N8vghWAqTh8C1YR3kHhPa3Fpg6VZZRmwS8",
    },
    {
        .coin = BTCCoin_TLTC,
        .script_type = BTCScriptConfig_SimpleType_P2WPKH,
        .out = "tltc1q0stgw6ehkx66r7g22056u0p95f9z4qydg2lue8",
    },
};

static void _test_app_btc_address_simple(void** state)
{
    { // invalid coin
        bool result = app_btc_address_simple(
            _BTCCoin_MIN - 1, BTCScriptConfig_SimpleType_P2WPKH, NULL, 0, NULL, 0);
        assert_false(result);
        result = app_btc_address_simple(
            _BTCCoin_MAX + 1, BTCScriptConfig_SimpleType_P2WPKH, NULL, 0, NULL, 0);
        assert_false(result);
    }

    for (int bools = 0; bools < 8; bools++) {
        bool keypath_valid = bools & 1;
        bool get_hash160_success = bools & 2;
        bool encode_success = bools & 4;
        for (size_t test_case_index = 0;
             test_case_index < sizeof(_address_tests) / sizeof(address_testcase_t);
             test_case_index++) {
            const address_testcase_t* test_case = &_address_tests[test_case_index];
            char out[XPUB_ENCODED_LEN] = {0};
            uint32_t expected_keypath[3] = {1, 2, 3};
            expect_value(
                __wrap_btc_common_is_valid_keypath_address_simple,
                script_type,
                test_case->script_type);
            expect_memory(
                __wrap_btc_common_is_valid_keypath_address_simple, keypath, expected_keypath, 3);
            expect_value(
                __wrap_btc_common_is_valid_keypath_address_simple,
                keypath_len,
                sizeof(expected_keypath) / sizeof(uint32_t));
            will_return(__wrap_btc_common_is_valid_keypath_address_simple, keypath_valid);
            if (keypath_valid) {
                expect_memory(
                    __wrap_keystore_secp256k1_pubkey_hash160, keypath, expected_keypath, 3);
                expect_value(
                    __wrap_keystore_secp256k1_pubkey_hash160,
                    keypath_len,
                    sizeof(expected_keypath) / sizeof(uint32_t));
                will_return(__wrap_keystore_secp256k1_pubkey_hash160, get_hash160_success);
            }
            if (keypath_valid && get_hash160_success) {
                will_return(__wrap_btc_common_outputhash_from_pubkeyhash, encode_success);
            }
            bool result = app_btc_address_simple(
                test_case->coin, test_case->script_type, expected_keypath, 3, out, sizeof(out));
            assert_int_equal(result, keypath_valid && get_hash160_success && encode_success);
            if (result) {
                assert_string_equal(out, test_case->out);
            }
        }
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_btc_address_simple),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
