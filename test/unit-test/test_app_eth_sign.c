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
#include <apps/eth/eth_sign.h>

#include <wally_bip32.h>

static uint8_t _recid = 3;

// sig with _recid at the end
static uint8_t _sig[65] =
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55"
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55"
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x03";

static app_eth_sign_error_t _errors[] = {
    APP_ETH_SIGN_ERR_UNKNOWN,
    APP_ETH_SIGN_ERR_INVALID_INPUT,
    APP_ETH_SIGN_ERR_USER_ABORT,
};
static app_eth_sign_error_t _verify_transaction_result = APP_ETH_SIGN_OK;

app_eth_sign_error_t __wrap_app_eth_verify_standard_transaction(const ETHSignRequest* request)
{
    check_expected(request);
    return _verify_transaction_result;
}

app_eth_sign_error_t __wrap_app_eth_verify_erc20_transaction(const ETHSignRequest* request)
{
    check_expected(request);
    return _verify_transaction_result;
}

bool __wrap_keystore_secp256k1_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    memcpy(sig_compact_out, _sig, 64);
    *recid_out = _recid;
    return mock();
}

bool __real_eth_common_is_valid_keypath(ETHCoin coin, const uint32_t* keypath, size_t keypath_len);
bool __wrap_eth_common_is_valid_keypath(ETHCoin coin, const uint32_t* keypath, size_t keypath_len)
{
    assert_int_equal(coin, ETHCoin_ETH);
    check_expected(keypath);
    assert_int_equal(keypath_len, 5);
    return __real_eth_common_is_valid_keypath(coin, keypath, keypath_len);
}

static void _default_request(ETHSignRequest* request)
{
    const ETHSignRequest r = {
        .coin = ETHCoin_ETH,
        .keypath_count = 5,
        .keypath =
            {
                44 + BIP32_INITIAL_HARDENED_CHILD,
                60 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                0,
                0,
            },
        .nonce = {.size = 0, .bytes = ""},
        .gas_price = {.size = 0, .bytes = ""},
        .gas_limit = {.size = 0, .bytes = ""},
        .recipient =
            "\x53\x8c\x7f\x96\xb1\x64\xbf\x1b\x97\xbb\x9f\x4b\xb4\x72\xe8\x9f\x5b"
            "\x14\x84\xf2",
        .value = {.size = 0, .bytes = ""},
        .data = {.size = 0, .bytes = ""},
    };
    memcpy(request, &r, sizeof(r));
}

static void _default_erc20_request(ETHSignRequest* request)
{
    _default_request(request);
    const uint8_t erc20_transfer[4] = {0xa9, 0x05, 0x9c, 0xbb};
    request->data.size = 68;
    memcpy(request->data.bytes, erc20_transfer, sizeof(erc20_transfer));
}

static void _expect_keypath(const ETHSignRequest* request)
{
    expect_memory(
        __wrap_eth_common_is_valid_keypath,
        keypath,
        request->keypath,
        request->keypath_count * sizeof(uint32_t));
}

static void _test_app_eth_sign(void** state)
{
    ETHSignResponse response;

    ETHSignRequest request;
    _default_request(&request);
    expect_memory(__wrap_app_eth_verify_standard_transaction, request, &request, sizeof(request));
    will_return(__wrap_keystore_secp256k1_sign, true);
    _expect_keypath(&request);
    assert_int_equal(APP_ETH_SIGN_OK, app_eth_sign(&request, &response));
    assert_memory_equal(response.signature, _sig, sizeof(_sig));
}

static void _test_app_eth_sign_unhappy(void** state)
{
    ETHSignResponse response;
    ETHSignRequest request;

    { // invalid coin
        _default_request(&request);
        request.coin = _ETHCoin_MAX + 1;
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // invalid keypath
        _default_request(&request);
        request.keypath[0] = 44;
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // leading big endian zeroes: nonce
        _default_request(&request);
        request.nonce.size = 1;
        memcpy(request.nonce.bytes, "\x00", 1);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));

        request.nonce.size = 2;
        memcpy(request.nonce.bytes, "\x00\x01", 2);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // leading big endian zeroes: gas_price
        _default_request(&request);
        request.gas_price.size = 1;
        memcpy(request.gas_price.bytes, "\x00", 1);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));

        request.gas_price.size = 2;
        memcpy(request.gas_price.bytes, "\x00\x01", 2);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // leading big endian zeroes: gas_limit
        _default_request(&request);
        request.gas_limit.size = 1;
        memcpy(request.gas_limit.bytes, "\x00", 1);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));

        request.gas_limit.size = 2;
        memcpy(request.gas_limit.bytes, "\x00\x01", 2);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // recipient can't be zeroes
        _default_request(&request);
        memset(request.recipient, 0, sizeof(request.recipient));
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // leading big endian zeroes: value
        _default_request(&request);
        request.value.size = 1;
        memcpy(request.value.bytes, "\x00", 1);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));

        request.value.size = 2;
        memcpy(request.value.bytes, "\x00\x01", 2);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign(&request, &response));
    }

    { // verify ERC-20 transaction errors
        _default_erc20_request(&request);
        for (size_t i = 0; i < sizeof(_errors) / sizeof(app_eth_sign_error_t); i++) {
            _verify_transaction_result = _errors[i];
            expect_memory(
                __wrap_app_eth_verify_erc20_transaction, request, &request, sizeof(request));
            _expect_keypath(&request);
            assert_int_equal(_errors[i], app_eth_sign(&request, &response));
        }
        _verify_transaction_result = APP_ETH_SIGN_OK;
    }

    { // verify standard transaction errors
        _default_request(&request);
        for (size_t i = 0; i < sizeof(_errors) / sizeof(app_eth_sign_error_t); i++) {
            _verify_transaction_result = _errors[i];
            expect_memory(
                __wrap_app_eth_verify_standard_transaction, request, &request, sizeof(request));
            _expect_keypath(&request);
            assert_int_equal(_errors[i], app_eth_sign(&request, &response));
        }
        _verify_transaction_result = APP_ETH_SIGN_OK;
    }

    { // signing fails
        _default_request(&request);
        expect_memory(
            __wrap_app_eth_verify_standard_transaction, request, &request, sizeof(request));
        will_return(__wrap_keystore_secp256k1_sign, false);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_ERR_UNKNOWN, app_eth_sign(&request, &response));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_eth_sign),
        cmocka_unit_test(_test_app_eth_sign_unhappy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
