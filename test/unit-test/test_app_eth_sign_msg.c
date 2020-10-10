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
#include <apps/eth/eth_sign_msg.h>
#include <keystore.h>
#include <rust/rust.h>
#include <ui/components/confirm.h>

#include <wally_bip32.h>

static uint8_t _recid = 3;

// sig with _recid at the end
static uint8_t _sig[65] =
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55"
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55"
    "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x03";

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    return true;
}

VerifyMessageResult __wrap_rust_workflow_verify_message(Bytes msg)
{
    return VerifyMessageResultOk;
}

bool __wrap_keystore_secp256k1_pubkey_uncompressed(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* pubkey_out)
{
    return true;
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

bool __real_rust_ethereum_keypath_is_valid_address(
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);
bool __wrap_rust_ethereum_keypath_is_valid_address(
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin)
{
    check_expected(keypath);
    assert_int_equal(keypath_len, 5);
    check_expected(expected_coin);
    return __real_rust_ethereum_keypath_is_valid_address(keypath, keypath_len, expected_coin);
}

static void _default_request(ETHSignMessageRequest* request)
{
    ETHSignMessageRequest r = {
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
    };
    r.msg.size = sizeof(request->msg.bytes);
    memset(r.msg.bytes, 'a', sizeof(request->msg.bytes));
    memcpy(request, &r, sizeof(r));
}

static void _expect_keypath(const ETHSignMessageRequest* request)
{
    expect_memory(
        __wrap_rust_ethereum_keypath_is_valid_address,
        keypath,
        request->keypath,
        request->keypath_count * sizeof(uint32_t));
    expect_value(
        __wrap_rust_ethereum_keypath_is_valid_address,
        expected_coin,
        60 + BIP32_INITIAL_HARDENED_CHILD);
}

static void _test_app_eth_sign_msg(void** state)
{
    ETHSignResponse response;

    {
        // Test a long string message
        ETHSignMessageRequest request;
        _default_request(&request);
        will_return(__wrap_keystore_secp256k1_sign, true);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_sign_msg(&request, &response));
        assert_memory_equal(response.signature, _sig, sizeof(_sig));
    }
    {
        // Test a long binary message
        ETHSignMessageRequest request;
        _default_request(&request);
        request.msg.size = 64;
        memset(request.msg.bytes, '\01', 64);
        will_return(__wrap_keystore_secp256k1_sign, true);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_sign_msg(&request, &response));
        assert_memory_equal(response.signature, _sig, sizeof(_sig));
    }
    {
        // test a short string message
        ETHSignMessageRequest request;
        _default_request(&request);
        request.msg.size = 64;
        will_return(__wrap_keystore_secp256k1_sign, true);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_sign_msg(&request, &response));
        assert_memory_equal(response.signature, _sig, sizeof(_sig));
    }
    {
        // test a short binary message
        ETHSignMessageRequest request;
        _default_request(&request);
        request.msg.size = 16;
        memset(request.msg.bytes, '\01', 16);
        will_return(__wrap_keystore_secp256k1_sign, true);
        _expect_keypath(&request);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_sign_msg(&request, &response));
        assert_memory_equal(response.signature, _sig, sizeof(_sig));
    }
}

static void _test_app_eth_sign_msg_unhappy(void** state)
{
    ETHSignResponse response;

    {
        ETHSignMessageRequest request;
        _default_request(&request);
        request.msg.size = 10000;
        assert_int_equal(APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_sign_msg(&request, &response));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_eth_sign_msg),
        cmocka_unit_test(_test_app_eth_sign_msg_unhappy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
