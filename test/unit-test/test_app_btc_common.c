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

#include <apps/btc/btc_common.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

static void _test_btc_common_format_amount_invalid_params(void** state)
{
    char out[100] = {0};

    assert_false(btc_common_format_amount(0, "", NULL, sizeof(out)));
    for (size_t wrong_out_len = 0; wrong_out_len < 30; wrong_out_len++) {
        assert_false(btc_common_format_amount(0, "", out, wrong_out_len));
    }
    assert_true(btc_common_format_amount(0, "", out, 31));
}

typedef struct {
    uint64_t satoshi;
    const char* out;
} btc_format_test_t;

static void _test_btc_common_format_amount(void** state)
{
    const btc_format_test_t tests[] = {
        {0, "0 LOL"},
        {1, "0.00000001 LOL"},
        {2, "0.00000002 LOL"},
        {10, "0.0000001 LOL"},
        {15, "0.00000015 LOL"},
        {20, "0.0000002 LOL"},
        {300, "0.000003 LOL"},
        {370, "0.0000037 LOL"},
        {371, "0.00000371 LOL"},
        {40000000000, "400 LOL"},
        {4000000000, "40 LOL"},
        {400000000, "4 LOL"},
        {40000000, "0.4 LOL"},
        {4000000, "0.04 LOL"},
        {400000, "0.004 LOL"},
        {40000, "0.0004 LOL"},
        {4000, "0.00004 LOL"},
        {400, "0.000004 LOL"},
        {40, "0.0000004 LOL"},
        {4, "0.00000004 LOL"},
        {5432345, "0.05432345 LOL"},
        {54323452, "0.54323452 LOL"},
        {543234527, "5.43234527 LOL"},
        {5432345270, "54.3234527 LOL"},
        {54323452708, "543.23452708 LOL"},
        {100000000, "1 LOL"},
        {1234567800000001, "12345678.00000001 LOL"},
        {0xffffffffffffffff, "184467440737.09551615 LOL"},
        {0xffffffffffffffff - 5, "184467440737.0955161 LOL"},
    };
    for (size_t i = 0; i < sizeof(tests) / sizeof(btc_format_test_t); i++) {
        const btc_format_test_t* test = &tests[i];
        char out[100] = {0};
        assert_true(btc_common_format_amount(test->satoshi, "LOL", out, sizeof(out)));
        assert_string_equal(test->out, out);
    }
}

static void _test_btc_common_is_valid_keypath_xpubs(void** state)
{
    const uint32_t bip44_account = 0 + BIP32_INITIAL_HARDENED_CHILD;
    const uint32_t bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD;

    // only p2wpkh-p2sh and p2wpkh are valid purposes
    const uint32_t valid_purposes[] = {
        49 + BIP32_INITIAL_HARDENED_CHILD,
        84 + BIP32_INITIAL_HARDENED_CHILD,
    };
    const BTCOutputType output_types[] = {
        BTCPubRequest_OutputType_TPUB,
        BTCPubRequest_OutputType_VPUB,
        BTCPubRequest_OutputType_UPUB,
        BTCPubRequest_OutputType_XPUB,
        BTCPubRequest_OutputType_YPUB,
        BTCPubRequest_OutputType_ZPUB,
    };
    for (size_t purpose_idx = 0; purpose_idx < sizeof(valid_purposes) / sizeof(uint32_t);
         purpose_idx++) {
        for (size_t output_type_idx = 0;
             output_type_idx < sizeof(output_types) / sizeof(BTCOutputType);
             output_type_idx++) {
            const uint32_t keypath[5] = {
                valid_purposes[purpose_idx],
                bip44_coin,
                bip44_account,
                0,
                0,
            };
            assert_true(btc_common_is_valid_keypath(
                output_types[output_type_idx],
                BTCScriptType_SCRIPT_UNKNOWN,
                keypath,
                3,
                bip44_coin));

            { // invalid account
                uint32_t invalid_keypath[3] = {
                    valid_purposes[purpose_idx],
                    bip44_coin,
                    BIP32_INITIAL_HARDENED_CHILD - 1,
                };
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    invalid_keypath,
                    3,
                    bip44_coin));
                // max 100 accounts
                invalid_keypath[2] = BIP32_INITIAL_HARDENED_CHILD + 100;
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    invalid_keypath,
                    3,
                    bip44_coin));
            }
            { // expected coin and keypath coin do not match
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    4,
                    2 + BIP32_INITIAL_HARDENED_CHILD));
            }

            { // invalid keypath sizes
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    2,
                    bip44_coin));
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    1,
                    bip44_coin));
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    0,
                    bip44_coin));
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    4,
                    bip44_coin));
                assert_false(btc_common_is_valid_keypath(
                    output_types[output_type_idx],
                    BTCScriptType_SCRIPT_UNKNOWN,
                    keypath,
                    5,
                    bip44_coin));
            }
        }
    }

    { // invalid purposes
        uint32_t keypath[] = {
            44 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_XPUB, BTCScriptType_SCRIPT_UNKNOWN, keypath, 3, bip44_coin));
        keypath[0] = 100 + BIP32_INITIAL_HARDENED_CHILD;
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_XPUB, BTCScriptType_SCRIPT_UNKNOWN, keypath, 3, bip44_coin));
    }
}

static void _test_btc_common_is_valid_keypath_addresses(void** state)
{
    const uint32_t bip44_account = 99 + BIP32_INITIAL_HARDENED_CHILD;
    const uint32_t bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD;
    { // invalid output type
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            _BTCPubRequest_OutputType_MAX + 1,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // invalid script type (legacy)
        const uint32_t keypath[] = {
            44 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS, BTCScriptType_SCRIPT_P2PKH, keypath, 5, bip44_coin));
    }
    { // valid p2wpkh-p2sh; receive
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_true(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // valid p2wpkh-p2sh; receive on high address
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            9999,
        };
        assert_true(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }

    { // invalid p2wpkh-p2sh; receive on too high address
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            10000,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // valid p2wpkh-p2sh; change
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            1,
            0,
        };
        assert_true(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }

    { // valid p2wpkh-p2sh; invalid bip44 change values
        uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            2,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
        keypath[3] = 0 + BIP32_INITIAL_HARDENED_CHILD;
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
        keypath[3] = 1 + BIP32_INITIAL_HARDENED_CHILD;
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // invalid p2wpkh-p2sh; wrong purpose
        const uint32_t keypath[] = {
            84 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // invalid p2wpkh-p2sh; account too high
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            100 + BIP32_INITIAL_HARDENED_CHILD,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // invalid p2wpkh-p2sh; account too low
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            BIP32_INITIAL_HARDENED_CHILD - 1,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin));
    }
    { // invalid p2wpkh-p2sh; expected coin mismatch
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS,
            BTCScriptType_SCRIPT_P2WPKH_P2SH,
            keypath,
            5,
            bip44_coin + 1));
    }
    { // valid p2wpkh
        const uint32_t keypath[] = {
            84 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_true(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS, BTCScriptType_SCRIPT_P2WPKH, keypath, 5, bip44_coin));
    }
    { // invalid p2wpkh; wrong purpose
        const uint32_t keypath[] = {
            49 + BIP32_INITIAL_HARDENED_CHILD,
            bip44_coin,
            bip44_account,
            0,
            0,
        };
        assert_false(btc_common_is_valid_keypath(
            BTCPubRequest_OutputType_ADDRESS, BTCScriptType_SCRIPT_P2WPKH, keypath, 5, bip44_coin));
    }
}

static void _test_btc_common_encode_xpub(void** state)
{
    struct ext_key xpub = {0};
    assert_int_equal(
        bip32_key_from_seed(
            (const unsigned char*)"seedseedseedseed",
            BIP32_ENTROPY_LEN_128,
            BIP32_VER_MAIN_PRIVATE,
            BIP32_FLAG_SKIP_HASH,
            &xpub),
        WALLY_OK);
    assert_int_equal(bip32_key_strip_private_key(&xpub), WALLY_OK);
    char out[113] = {0};
    assert_false(btc_common_encode_xpub(&xpub, (const uint8_t*)"\x04\x88\xb2\x1e", out, 110));
    assert_true(
        btc_common_encode_xpub(&xpub, (const uint8_t*)"\x04\x88\xb2\x1e", out, sizeof(out)));
    assert_string_equal(
        out,
        "xpub661MyMwAqRbcFLG1NSwsGkQxYGaRj3qDsDB6g64CviEc82D3r7Dp4eMnWdarcVkpPbMgwwuLLPPwCXVQFWWomv"
        "yj6QKEuDXWvNbCDF98tgM");
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_common_format_amount_invalid_params),
        cmocka_unit_test(_test_btc_common_format_amount),
        cmocka_unit_test(_test_btc_common_is_valid_keypath_xpubs),
        cmocka_unit_test(_test_btc_common_is_valid_keypath_addresses),
        cmocka_unit_test(_test_btc_common_encode_xpub),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
