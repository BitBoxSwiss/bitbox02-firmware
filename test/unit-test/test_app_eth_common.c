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

#include <apps/eth/eth_common.h>

#include <hardfault.h>

#include <stdint.h>

#include <wally_bip32.h>

static void _test_eth_common_is_valid_keypath_invalid(void** state)
{
    uint32_t keypath[6] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        60 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
        0,
    };

    // too short
    assert_false(eth_common_is_valid_keypath(ETHCoin_ETH, keypath, 4));

    // too long
    assert_false(eth_common_is_valid_keypath(ETHCoin_ETH, keypath, 6));

    // tweak keypath elements (except for the last, see `_test_eth_common_is_valid_keypath_accounts`
    // for that)
    for (size_t i = 0; i < 4; i++) {
        {
            keypath[i]++;
            assert_false(eth_common_is_valid_keypath(ETHCoin_ETH, keypath, 5));
            keypath[i]--;
        }
    }

    // wrong purpose for coin
    assert_false(eth_common_is_valid_keypath(ETHCoin_RopstenETH, keypath, 5));
    assert_false(eth_common_is_valid_keypath(ETHCoin_RinkebyETH, keypath, 5));

    // Invalid coin
    assert_false(eth_common_is_valid_keypath(_ETHCoin_MAX + 1, keypath, 5));
}

static void _test_eth_common_is_valid_keypath_accounts(void** state)
{
    uint32_t keypath[5] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        60 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };

    // 100 valid accounts
    for (size_t i = 0; i < 100; i++) {
        keypath[4] = i;
        assert_true(eth_common_is_valid_keypath(ETHCoin_ETH, keypath, 5));
    }
    // invalid account
    keypath[4] = 100;
    assert_false(eth_common_is_valid_keypath(ETHCoin_ETH, keypath, 5));
}

static void _bigendian_to_scalar(const uint8_t* bytes, size_t len, bignum256* out)
{
    if (len > 32) {
        Abort("_bigendian_to_scalar: unexpected size");
    }
    // bn_read_be requires a 32 byte big endian input, so we pad our big endian number to the
    // required size.
    uint8_t buf[32] = {0};
    memcpy(buf + sizeof(buf) - len, bytes, len);
    bn_read_be(buf, out);
}

typedef struct {
    const uint8_t bigendian[32];
    const size_t bigendian_len;
    const unsigned int decimals;
    const char* unit;
    const char* expected_result;
} _format_test_t;

static void _test_eth_common_format_amount(void** state)
{
    _format_test_t tests[] = {
        {
            // 0
            .bigendian = "",
            .bigendian_len = 0,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "0.0 LOL",
        },
        {
            // 1000000
            .bigendian = "\x0f\x42\x40",
            .bigendian_len = 3,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "1.0 LOL",
        },
        {
            // 1100000
            .bigendian = "\x10\xc8\xe0",
            .bigendian_len = 3,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "1.1 LOL",
        },
        {
            // 38723987932742983742983742
            .bigendian = "\x20\x08\x1f\x97\x9a\x5c\x8d\x47\x29\x0e\x3e",
            .bigendian_len = 11,
            .decimals = 18,
            .unit = "LOL",
            .expected_result = "38723987.9327... LOL",
        },
        {
            // 123456
            .bigendian = "\x01\xe2\x40",
            .bigendian_len = 3,
            .decimals = 8,
            .unit = "LOL",
            .expected_result = "0.00123456 LOL",
        },
        {
            // 123456
            .bigendian = "\x01\xe2\x40",
            .bigendian_len = 3,
            .decimals = 8,
            .unit = "LOL",
            .expected_result = "0.00123456 LOL",
        },
        {
            // 124567890123
            .bigendian = "\x1d\x00\xd3\x28\xcb",
            .bigendian_len = 5,
            .decimals = 10,
            .unit = "LOL",
            .expected_result = "12.4567890123 LOL",
        },
        {
            // 1245678901234
            .bigendian = "\x01\x22\x08\x3f\x97\xf2",
            .bigendian_len = 6,
            .decimals = 11,
            .unit = "LOL",
            .expected_result = "12.4567890123... LOL",
        },
    };
    for (size_t i = 0; i < sizeof(tests) / sizeof(_format_test_t); i++) {
        const _format_test_t* test = &tests[i];

        bignum256 bignum;
        _bigendian_to_scalar(test->bigendian, test->bigendian_len, &bignum);
        char out[100];
        eth_common_format_amount(&bignum, test->unit, test->decimals, out, sizeof(out));
        assert_string_equal(out, test->expected_result);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_eth_common_is_valid_keypath_invalid),
        cmocka_unit_test(_test_eth_common_is_valid_keypath_accounts),
        cmocka_unit_test(_test_eth_common_format_amount),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
