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

#include <apps/eth/eth.h>
#include <keystore.h>
#include <util.h>

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <wally_bip32.h>

static bool _is_valid_keypath = true;

bool __wrap_rust_ethereum_keypath_is_valid_address(
    ETHCoin coin,
    const uint32_t* keypath,
    size_t keypath_len)
{
    return _is_valid_keypath;
}

bool __wrap_keystore_secp256k1_pubkey_uncompressed(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* pubkey_out)
{
    memcpy(pubkey_out, (const void*)mock(), EC_PUBLIC_KEY_UNCOMPRESSED_LEN);
    return true;
}

typedef struct {
    uint8_t pubkey[65];
    const char* expected_address;
} _test_case_t;

static _test_case_t _test_cases[] = {
    {
        .pubkey =
            {
                0x04, 0xd8, 0xae, 0xa8, 0x0d, 0x2d, 0xbc, 0xeb, 0xbe, 0x10, 0xfd, 0xfa, 0xc2,
                0xd2, 0xdb, 0x19, 0x64, 0x15, 0x5b, 0xa9, 0x9e, 0x0d, 0xd7, 0xbf, 0xd5, 0xcf,
                0xfe, 0xd9, 0x7a, 0x1c, 0xae, 0xf7, 0xd0, 0xb9, 0x07, 0x2d, 0x9c, 0x0f, 0x50,
                0x49, 0x30, 0xef, 0x59, 0xb7, 0x52, 0xd4, 0xfe, 0xa0, 0xcb, 0xde, 0x3e, 0x27,
                0x3e, 0xe9, 0x54, 0xd8, 0xda, 0xc8, 0xee, 0x03, 0x1a, 0x4e, 0xd1, 0x71, 0xfd,
            },
        .expected_address = "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B",
    },
    {
        .pubkey =
            {
                0x04, 0xfe, 0x6a, 0x17, 0xd0, 0xac, 0xc1, 0x30, 0xb3, 0xb4, 0xf2, 0x38, 0x32,
                0x33, 0x22, 0x05, 0x87, 0xe0, 0x54, 0x03, 0x46, 0xf3, 0x37, 0x62, 0x14, 0xe3,
                0x9b, 0xb4, 0x43, 0x06, 0xd3, 0x72, 0xd5, 0x12, 0xe9, 0x68, 0x53, 0x38, 0x64,
                0xc0, 0xad, 0x91, 0x30, 0xfc, 0xef, 0xb0, 0xa3, 0x9c, 0x4b, 0x87, 0xdb, 0xd7,
                0xcc, 0x42, 0xda, 0xc7, 0xcd, 0x8d, 0xa9, 0x93, 0x8b, 0x8c, 0x43, 0xb5, 0xe7,
            },
        .expected_address = "0x937384E07747D517668169764ED3f140B676C6d4",
    },
    {
        .pubkey =
            {
                0x04, 0x18, 0x99, 0x9a, 0xad, 0xc2, 0x59, 0xbf, 0xb6, 0x1c, 0xde, 0xfd, 0x83,
                0x51, 0x41, 0xc5, 0x3d, 0x44, 0x86, 0xe1, 0x5c, 0x11, 0x0c, 0x7a, 0x98, 0x35,
                0x97, 0x7f, 0xe6, 0xad, 0x55, 0xd3, 0xe1, 0xbd, 0x5e, 0x71, 0x82, 0x6c, 0x8d,
                0x65, 0x84, 0x1a, 0xdc, 0x7d, 0xdd, 0xa6, 0x57, 0x1f, 0x8a, 0x2d, 0x7c, 0x8a,
                0xb2, 0xc3, 0xd4, 0x02, 0xed, 0x79, 0x5c, 0x97, 0x8c, 0x11, 0xf0, 0x16, 0xe1,
            },
        .expected_address = "0xE77c290fd9c8000462D6f652cC2fC6e3010fb55d",
    },
    {
        .pubkey =
            {
                0x04, 0x83, 0x0d, 0x5f, 0x1e, 0x41, 0xe8, 0x9d, 0x41, 0x2b, 0xdd, 0x9c, 0x77,
                0x73, 0xb5, 0xe1, 0x3e, 0x77, 0x60, 0x1e, 0xea, 0xa7, 0x21, 0xb2, 0x9d, 0x4a,
                0x44, 0x0c, 0xd4, 0xd3, 0x6e, 0x19, 0xdd, 0x8b, 0xee, 0xbd, 0x62, 0x23, 0xf1,
                0xce, 0x87, 0xce, 0x5d, 0x74, 0x37, 0xe8, 0xbd, 0xb3, 0xd6, 0xf9, 0xe8, 0xb5,
                0xf9, 0x1f, 0x67, 0x3e, 0x80, 0xbb, 0x73, 0x84, 0x64, 0xaf, 0x10, 0x31, 0x3e,
            },
        .expected_address = "0x8814580E4414453a69381A159930598DB5544549",
    },
};

static void _test_app_eth_address_invalid(void** state)
{
    { // invalid coin
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT,
            app_eth_address(
                _ETHCoin_MAX + 1, ETHPubRequest_OutputType_ADDRESS, NULL, 0, NULL, 0, false, NULL));
    }
    { // invalid keypath
        _is_valid_keypath = false;
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT,
            app_eth_address(
                ETHCoin_ETH, ETHPubRequest_OutputType_ADDRESS, NULL, 0, NULL, 0, false, NULL));
        _is_valid_keypath = true;
    }
    { // invalid output type
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT,
            app_eth_address(
                ETHCoin_ETH, _ETHPubRequest_OutputType_MIN - 1, NULL, 0, NULL, 0, false, NULL));
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT,
            app_eth_address(
                ETHCoin_ETH, _ETHPubRequest_OutputType_MAX + 1, NULL, 0, NULL, 0, false, NULL));
    }
}

static void _test_app_eth_address(void** state)
{
    const uint32_t keypath[5] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        60 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };

    for (size_t index = 0; index < sizeof(_test_cases) / sizeof(_test_case_t); index++) {
        const _test_case_t* test_case = &_test_cases[index];
        will_return(__wrap_keystore_secp256k1_pubkey_uncompressed, test_case->pubkey);

        char address[43];
        assert_int_equal(
            APP_ETH_SIGN_OK,
            app_eth_address(
                ETHCoin_ETH,
                ETHPubRequest_OutputType_ADDRESS,
                keypath,
                5,
                address,
                sizeof(address),
                false,
                NULL));
        assert_string_equal(address, test_case->expected_address);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_eth_address_invalid),
        cmocka_unit_test(_test_app_eth_address),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
