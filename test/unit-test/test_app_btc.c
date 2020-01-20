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
#include <keystore.h>
#include <util.h>

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#include <wally_bip32.h>

// We mock all called functions to make sure they are actually called. For some,
// the real function is called as it's easier to check all function input/output
// this way.

bool __real_btc_common_encode_xpub(const struct ext_key*, const uint8_t*, char*, size_t);
bool __wrap_btc_common_encode_xpub(
    const struct ext_key* derived_xpub,
    const uint8_t* version,
    char* out,
    size_t out_len)
{
    check_expected(out_len);
    assert_true(__real_btc_common_encode_xpub(derived_xpub, version, out, out_len));
    return mock();
}
bool __wrap_btc_common_is_valid_keypath_xpub(
    BTCPubRequest_XPubType xpub_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    check_expected(xpub_type);
    check_expected(keypath);
    check_expected(keypath_len);
    return mock();
}
bool __wrap_btc_common_is_valid_keypath_address(
    BTCScriptType script_type,
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
    BTCScriptType script_type,
    uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size);
bool __wrap_btc_common_outputhash_from_pubkeyhash(
    BTCScriptType script_type,
    uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    assert_true(__real_btc_common_outputhash_from_pubkeyhash(
        script_type, pubkey_hash, output_hash, output_hash_size));
    return mock();
}

bool __wrap_keystore_get_xpub(const uint32_t* keypath, size_t keypath_len, struct ext_key* xpub_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    assert_non_null(xpub_out);

    // Constant mock xpub. keypath not used, as we are not unit testing
    // derivation here, but address generation.
    uint8_t seed[32] = {0};
    memset(seed, 0x45, sizeof(seed));
    if (bip32_key_from_seed(seed, BIP32_ENTROPY_LEN_256, BIP32_VER_MAIN_PRIVATE, 0, xpub_out) !=
        WALLY_OK) {
        return false;
    }
    // neuter
    xpub_out->priv_key[0] = BIP32_FLAG_KEY_PUBLIC;
    util_zero(xpub_out->priv_key + 1, sizeof(xpub_out->priv_key) - 1);

    return mock();
}

typedef struct {
    BTCCoin coin;
    BTCPubRequest_XPubType xpub_type;
    const char* out;
} xpub_testcase_t;

typedef struct {
    BTCCoin coin;
    BTCScriptType script_type;
    const char* out;
} address_testcase_t;

static xpub_testcase_t _xpub_tests[] = {
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_TPUB,
        .out = "tpubD6NzVbkrYhZ4Y2oG1D7odp1qL1DBqrbzFQvTUv9pYVZmTwhiTLQmcNYM7KkioXEs7A7t2H9nSU4BrFQ"
               "2uWgsH1N3bzWnnqwe7z6EBNnJ3Hx",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_VPUB,
        .out = "vpub5SLqN2bLY4WeZeEAtJZU1iVTewpdyE7vsZHiZaJuSa47cTQYsoEZDoZEpskmHCynVyMMukSnz3X3PVg"
               "J5G1bo6YYoiNdwVeRzaNXeC1Tqgo",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_UPUB,
        .out = "upub57Wa4MvRPNyAiM343wmqodPxUygC2c8RxSmVnBR24ZgEZMbKd94zbju6ofoBHJKs6LEZAGrEXPAVWD4"
               "jMZbazrrwwNgDMapwirJtFbjQ8Nj",
    },

    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_XPUB,
        .out = "xpub661MyMwAqRbcGEcQZ28iRtgTzt7XrU6vhnLA8N6gCaosif31P7ZgTvsWsHfwH2HdKFayQhduuNE9A4u"
               "RWeqdPZukYPmV7KHQY2VpRNV7PiJ",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_YPUB,
        .out = "ypub6QqdH2c5z7967XoXPNvLdymyArFyo66RctrNukzZabBkmkrEdmjF5zXetVdXGvwYithnABEUN2ah3MW"
               "zEMFeBobMQjTuhE6tokZTouiD6jm",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_ZPUB,
        .out = "zpub6jftahH18ngZxpzeDjhxr4sULpQRji5vY1Nbh9tSxbZdprfTtRtoi4Bnuhb7GqbU8Xpaueq2pgwEve8"
               "Yx3fez3GxH5ALH8vP5Ud7CUbyUKz",
    },
};

static address_testcase_t _address_tests[] = {
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptType_SCRIPT_P2PKH,
        .out = "1CK7n8RMJ66oMac58cbriBRbdxjDnzvvXv",
    },
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH_P2SH,
        .out = "3HNUdmEorEcqoknW5A5Wx6GZTkHPm5TeJg",
    },
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH,
        .out = "bc1q0stgw6ehkx66r7g22056u0p95f9z4qydmyx3ja",
    },

    {
        .coin = BTCCoin_TBTC,
        .script_type = BTCScriptType_SCRIPT_P2PKH,
        .out = "mrq55BWL77Y48h5grBaEY6dvVxKvicEaa7",
    },
    {
        .coin = BTCCoin_TBTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH_P2SH,
        .out = "2N8vghWAqTh8C1YR3kHhPa3Fpg6VZZRmwS8",
    },
    {
        .coin = BTCCoin_TBTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH,
        .out = "tb1q0stgw6ehkx66r7g22056u0p95f9z4qyd3zazfw",
    },

    {
        .coin = BTCCoin_LTC,
        .script_type = BTCScriptType_SCRIPT_P2PKH,
        .out = "LWY53LjBNkLrcPJEJkb9zCVMrB6VvkGCzp",
    },
    {
        .coin = BTCCoin_LTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH_P2SH,
        .out = "MPacweemoMUGcG4QB34rmjWxnSsqj8iQ5f",
    },
    {
        .coin = BTCCoin_LTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH,
        .out = "ltc1q0stgw6ehkx66r7g22056u0p95f9z4qydlcu42d",
    },

    {
        .coin = BTCCoin_TLTC,
        .script_type = BTCScriptType_SCRIPT_P2PKH,
        .out = "mrq55BWL77Y48h5grBaEY6dvVxKvicEaa7",
    },
    {
        .coin = BTCCoin_TLTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH_P2SH,
        .out = "2N8vghWAqTh8C1YR3kHhPa3Fpg6VZZRmwS8",
    },
    {
        .coin = BTCCoin_TLTC,
        .script_type = BTCScriptType_SCRIPT_P2WPKH,
        .out = "tltc1q0stgw6ehkx66r7g22056u0p95f9z4qydg2lue8",
    },
};

static void _test_app_btc_xpub(void** state)
{
    { // invalid coin
        bool result = app_btc_xpub(_BTCCoin_MIN - 1, BTCPubRequest_XPubType_XPUB, NULL, 0, NULL, 0);
        assert_false(result);
        result = app_btc_xpub(_BTCCoin_MAX + 1, BTCPubRequest_XPubType_XPUB, NULL, 0, NULL, 0);
        assert_false(result);
    }

    for (int bools = 0; bools < 8; bools++) {
        bool keypath_valid = bools & 1;
        bool get_xpub_success = bools & 2;
        bool encode_success = bools & 4;
        for (size_t test_case_index = 0;
             test_case_index < sizeof(_xpub_tests) / sizeof(xpub_testcase_t);
             test_case_index++) {
            const xpub_testcase_t* test_case = &_xpub_tests[test_case_index];
            char out[112] = {0};
            uint32_t expected_keypath[3] = {1, 2, 3};
            expect_value(__wrap_btc_common_is_valid_keypath_xpub, xpub_type, test_case->xpub_type);
            expect_memory(__wrap_btc_common_is_valid_keypath_xpub, keypath, expected_keypath, 3);
            expect_value(
                __wrap_btc_common_is_valid_keypath_xpub,
                keypath_len,
                sizeof(expected_keypath) / sizeof(uint32_t));
            will_return(__wrap_btc_common_is_valid_keypath_xpub, keypath_valid);
            if (keypath_valid) {
                expect_memory(__wrap_keystore_get_xpub, keypath, expected_keypath, 3);
                expect_value(
                    __wrap_keystore_get_xpub,
                    keypath_len,
                    sizeof(expected_keypath) / sizeof(uint32_t));
                will_return(__wrap_keystore_get_xpub, get_xpub_success);
            }
            if (keypath_valid && get_xpub_success) {
                expect_value(__wrap_btc_common_encode_xpub, out_len, sizeof(out));
                will_return(__wrap_btc_common_encode_xpub, encode_success);
            }
            bool result = app_btc_xpub(
                test_case->coin, test_case->xpub_type, expected_keypath, 3, out, sizeof(out));
            assert_int_equal(result, keypath_valid && get_xpub_success && encode_success);
            if (result) {
                assert_string_equal(out, test_case->out);
            }
        }
    }
}

static void _test_app_btc_address(void** state)
{
    { // invalid coin
        bool result =
            app_btc_address(_BTCCoin_MIN - 1, BTCScriptType_SCRIPT_UNKNOWN, NULL, 0, NULL, 0);
        assert_false(result);
        result = app_btc_address(_BTCCoin_MAX + 1, BTCScriptType_SCRIPT_UNKNOWN, NULL, 0, NULL, 0);
        assert_false(result);
    }

    for (int bools = 0; bools < 8; bools++) {
        bool keypath_valid = bools & 1;
        bool get_xpub_success = bools & 2;
        bool encode_success = bools & 4;
        for (size_t test_case_index = 0;
             test_case_index < sizeof(_address_tests) / sizeof(address_testcase_t);
             test_case_index++) {
            const address_testcase_t* test_case = &_address_tests[test_case_index];
            char out[112] = {0};
            uint32_t expected_keypath[3] = {1, 2, 3};
            expect_value(
                __wrap_btc_common_is_valid_keypath_address, script_type, test_case->script_type);
            expect_memory(__wrap_btc_common_is_valid_keypath_address, keypath, expected_keypath, 3);
            expect_value(
                __wrap_btc_common_is_valid_keypath_address,
                keypath_len,
                sizeof(expected_keypath) / sizeof(uint32_t));
            will_return(__wrap_btc_common_is_valid_keypath_address, keypath_valid);
            if (keypath_valid) {
                expect_memory(__wrap_keystore_get_xpub, keypath, expected_keypath, 3);
                expect_value(
                    __wrap_keystore_get_xpub,
                    keypath_len,
                    sizeof(expected_keypath) / sizeof(uint32_t));
                will_return(__wrap_keystore_get_xpub, get_xpub_success);
            }
            if (keypath_valid && get_xpub_success) {
                will_return(__wrap_btc_common_outputhash_from_pubkeyhash, encode_success);
            }
            bool result = app_btc_address(
                test_case->coin, test_case->script_type, expected_keypath, 3, out, sizeof(out));
            assert_int_equal(result, keypath_valid && get_xpub_success && encode_success);
            if (result) {
                assert_string_equal(out, test_case->out);
            }
        }
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_btc_xpub),
        cmocka_unit_test(_test_app_btc_address),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
