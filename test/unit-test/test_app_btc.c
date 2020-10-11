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

#include <wally_bip32.h>

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

// sudden tenant fault inject concert weather maid people chunk youth stumble grit
static uint8_t _mock_bip39_seed[64] =
    "\x5a\x11\x5b\xcd\xbe\x0f\xe1\x70\x0e\x60\x95\x74\xf3\x57\xb0\x8d\xca\x37\x15\xb0\x35\xe6\xc7"
    "\x76\x77\x0a\xc7\xa0\xab\x2e\x2f\xea\x84\x0b\xa2\x76\x35\x06\xfa\x9c\x39\xde\x4d\xef\x27\xf6"
    "\xf8\xeb\xce\x36\x37\x02\xe9\x83\xe5\x49\xbd\x7d\xef\x14\xa0\x31\xbf\xdd";

// We mock all called functions to make sure they are actually called. For some,
// the real function is called as it's easier to check all function input/output
// this way.

bool __real_keystore_encode_xpub_at_keypath(const uint32_t*, size_t, const uint8_t*, char*, size_t);
bool __wrap_keystore_encode_xpub_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* version,
    char* out,
    size_t out_len)
{
    check_expected(out_len);
    return __real_keystore_encode_xpub_at_keypath(keypath, keypath_len, version, out, out_len);
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
    BTCScriptConfig_SimpleType script_type;
    const char* out;
} address_testcase_t;

static xpub_testcase_t _xpub_tests[] = {
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_TPUB,
        .out = "tpubDCYSHq3Y2yqZYw2yxYFczWpbbr9yqLXK5V9hADr7czfhvSbVBZ2Up9ouUeJU4ibNvVuHBZywbVtt4Xw"
               "37wAgwWhy5LQsVk5w441qoaEytzZ",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_VPUB,
        .out = "vpub5YWHAFt22Lo9aYTtqdhHNRJDvnmRxi3FhdWxEt1CX5A44xJKc1rGRapoCCJWYQLJKK8m53Gx95MjbnD"
               "JHgVRTbtUH4GiePniveJ9GMSBipu",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_UPUB,
        .out = "upub5Dg1rbD6sfFfjFGn1GufALCikpcz263knWzjTV7K94nB1rV6MMghoXAfAzLvYVgNug1xKZgPgR1BiVb"
               "jZz5QfNCsQiaJ4UyEevEVsqHSu6Q",
    },

    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_XPUB,
        .out = "xpub6CAombDrKht7H8r8WMGXnbVEGj4Kqx2FXrZPofnyH5upB9vn7LBPfi95EcDgYDe98bNNZzU54Q4qNMS"
               "Rj5KT45Fg1jfZpDRhU6RS3WjHkqg",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_YPUB,
        .out = "ypub6X155FtmUPRb8S3FLi49zgajShCmna1kSy5cb4grf6HhEFk1MzLxHmoDFpBGY8J4YEVBKU4dX4RPFe3"
               "zSmjTrJwGt5MzQ8FBjpV5S9fvhh3",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_ZPUB,
        .out = "zpub6qqLNvZgd4y4yjENB4qnCmgEcfMDjC1FN5bqNTak36faHMZEceWWuqTMH28rY2wywsbz4wfByimw8vf"
               "ZAU9UeYcskR4Qz34g1YYipg8DatS",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_CAPITAL_VPUB,
        .out = "Vpub5jQNHVcTbJMX17dGnJAGCVe2eaohB4ir1uAdA9GjtqzTh8sENREYWhgizuFz6qZCYnCjwdH52HkEiwq"
               "4aueNc6197XP83oFipNa1rGqkiFZ",
    },
    {
        .coin = BTCCoin_BTC,
        .xpub_type = BTCPubRequest_XPubType_CAPITAL_ZPUB,
        .out = "Zpub72jRWAJ8C2XSQJPk7jJm2r23LTPUwYgqgMFWHirHQsVyuY89P3tnzxKH5j6L6UAtBLfxwXfJrwASG6H"
               "KThJRo2jYatApPSXfuGpbQf7Q5V3",
    },
};

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

static void _test_app_btc_xpub(void** state)
{
    { // invalid coin
        bool result = app_btc_xpub(_BTCCoin_MIN - 1, BTCPubRequest_XPubType_XPUB, NULL, 0, NULL, 0);
        assert_false(result);
        result = app_btc_xpub(_BTCCoin_MAX + 1, BTCPubRequest_XPubType_XPUB, NULL, 0, NULL, 0);
        assert_false(result);
    }

    for (int bools = 0; bools < 4; bools++) {
        bool keypath_valid = bools & 1;
        bool encode_success = bools & 2;
        for (size_t test_case_index = 0;
             test_case_index < sizeof(_xpub_tests) / sizeof(xpub_testcase_t);
             test_case_index++) {
            const xpub_testcase_t* test_case = &_xpub_tests[test_case_index];

            if (encode_success) {
                mock_state(_mock_seed, _mock_bip39_seed);
            } else {
                mock_state(NULL, NULL);
            }

            char out[XPUB_ENCODED_LEN] = {0};
            uint32_t expected_keypath[3] = {1, 2, 3};
            expect_value(__wrap_btc_common_is_valid_keypath_xpub, xpub_type, test_case->xpub_type);
            expect_memory(__wrap_btc_common_is_valid_keypath_xpub, keypath, expected_keypath, 3);
            expect_value(
                __wrap_btc_common_is_valid_keypath_xpub,
                keypath_len,
                sizeof(expected_keypath) / sizeof(uint32_t));
            will_return(__wrap_btc_common_is_valid_keypath_xpub, keypath_valid);
            if (keypath_valid) {
                expect_value(__wrap_keystore_encode_xpub_at_keypath, out_len, sizeof(out));
            }
            bool result = app_btc_xpub(
                test_case->coin, test_case->xpub_type, expected_keypath, 3, out, sizeof(out));
            assert_int_equal(result, keypath_valid && encode_success);
            if (result) {
                assert_string_equal(out, test_case->out);
            }
        }
    }
}

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
        bool get_xpub_success = bools & 2;
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
            bool result = app_btc_address_simple(
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
        cmocka_unit_test(_test_app_btc_address_simple),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
