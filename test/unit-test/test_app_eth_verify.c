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

#include <apps/eth/eth_verify.h>

#include <wally_bip32.h>

#define ERC20_METHOD "\xa9\x05\x9c\xbb"
#define ERC20_METHOD_SIZE 4
#define RECIPIENT "\x53\x8c\x7f\x96\xb1\x64\xbf\x1b\x97\xbb\x9f\x4b\xb4\x72\xe8\x9f\x5b\x14\x84\xf2"

#define VALUE "\x08\xdc\x29\x73\x4e\xd0\x00\x00"
static const char* _recipient = "0x538C7f96B164Bf1b97bB9F4BB472E89F5B1484F2";
static const char* _formatted_amount = "0.6384308224 ETH";
static const char* _formatted_amount_erc20 = "0.6384308224 TEST";
static const char* _formatted_total = "0.6384938224 ETH"; // amount + fee
static const char* _formatted_fee = "0.000063 ETH"; // gas price * gas limit

bool __wrap_workflow_verify_recipient(const char* recipient, const char* amount)
{
    check_expected(recipient);
    check_expected(amount);
    return mock();
}

bool __wrap_workflow_verify_total(const char* total, const char* fee)
{
    check_expected(total);
    check_expected(fee);
    return mock();
}

static const app_eth_erc20_params_t _erc20_params = {
    .coin = ETHCoin_RinkebyETH,
    .unit = "TEST",
    .contract_address =
        "\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55\x55",
    .decimals = 18,
};

const app_eth_erc20_params_t* __wrap_app_eth_erc20_params_get(
    ETHCoin coin,
    const uint8_t* contract_address)
{
    return (const app_eth_erc20_params_t*)mock();
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
        .gas_price = {.size = 4, .bytes = "\xb2\xd0\x5e\x00"}, // 3000000000 (3 gwei)
        .gas_limit = {.size = 2, .bytes = "\x52\x08"}, // 21000
        .recipient = RECIPIENT,
        .value = {.size = 8, .bytes = VALUE},
        .data = {.size = 0, .bytes = ""},
    };
    memcpy(request, &r, sizeof(r));
}

static void _default_erc20_request(ETHSignRequest* request)
{
    _default_request(request);
// <0xa9059cbb><32 bytes recipient><32 bytes value>
#define RECIPIENT_PADDED "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00" RECIPIENT
#define VALUE_PADDED                                                                               \
    "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00" \
    "\x00" VALUE
    const uint8_t erc20_transfer[68] = ERC20_METHOD RECIPIENT_PADDED VALUE_PADDED;
    request->data.size = sizeof(erc20_transfer);
    memcpy(request->data.bytes, erc20_transfer, sizeof(erc20_transfer));
}

static void _test_app_eth_verify_erc20_transaction(void** state)
{
    ETHSignRequest request;
    { // happy
        _default_erc20_request(&request);
        will_return(__wrap_app_eth_erc20_params_get, &_erc20_params);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount_erc20);
        will_return(__wrap_workflow_verify_recipient, true);
        expect_string(__wrap_workflow_verify_total, total, _formatted_amount_erc20);
        expect_string(__wrap_workflow_verify_total, fee, _formatted_fee);
        will_return(__wrap_workflow_verify_total, true);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_verify_erc20_transaction(&request));
    }
    { // invalid coin
        _default_erc20_request(&request);
        request.coin = _ETHCoin_MAX + 1;
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_erc20_transaction(&request));
    }
    { // invalid erc20 coin
        _default_erc20_request(&request);
        will_return(__wrap_app_eth_erc20_params_get, NULL);
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_erc20_transaction(&request));
    }
    { // can't have a zero value
        _default_erc20_request(&request);
        const size_t recipient_padded_size = 32;
        // set value part to zero.
        memset(request.data.bytes + ERC20_METHOD_SIZE + recipient_padded_size, 0, 32);
        will_return(__wrap_app_eth_erc20_params_get, &_erc20_params);
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_erc20_transaction(&request));
    }
    { // recipient not zero padded
        for (size_t i = 0; i < 12; i++) {
            _default_erc20_request(&request);
            request.data.bytes[ERC20_METHOD_SIZE + i] = 1;
            will_return(__wrap_app_eth_erc20_params_get, &_erc20_params);
            assert_int_equal(
                APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_erc20_transaction(&request));
        }
    }
    { // rejected recipient
        _default_erc20_request(&request);
        will_return(__wrap_app_eth_erc20_params_get, &_erc20_params);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount_erc20);
        will_return(__wrap_workflow_verify_recipient, false);
        assert_int_equal(APP_ETH_SIGN_ERR_USER_ABORT, app_eth_verify_erc20_transaction(&request));
    }
    { // rejected total/fee
        _default_erc20_request(&request);
        will_return(__wrap_app_eth_erc20_params_get, &_erc20_params);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount_erc20);
        will_return(__wrap_workflow_verify_recipient, true);
        expect_string(__wrap_workflow_verify_total, total, _formatted_amount_erc20);
        expect_string(__wrap_workflow_verify_total, fee, _formatted_fee);
        will_return(__wrap_workflow_verify_total, false);
        assert_int_equal(APP_ETH_SIGN_ERR_USER_ABORT, app_eth_verify_erc20_transaction(&request));
    }
}

static void _test_app_eth_verify_standard_transaction(void** state)
{
    ETHSignRequest request;
    { // happy
        _default_request(&request);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount);
        will_return(__wrap_workflow_verify_recipient, true);
        expect_string(__wrap_workflow_verify_total, total, _formatted_total);
        expect_string(__wrap_workflow_verify_total, fee, _formatted_fee);
        will_return(__wrap_workflow_verify_total, true);
        assert_int_equal(APP_ETH_SIGN_OK, app_eth_verify_standard_transaction(&request));
    }
    { // invalid coin
        _default_request(&request);
        request.coin = _ETHCoin_MAX + 1;
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_standard_transaction(&request));
    }
    { // can't have a zero value and no data
        _default_request(&request);
        request.value.size = 0;
        request.data.size = 0;
        assert_int_equal(
            APP_ETH_SIGN_ERR_INVALID_INPUT, app_eth_verify_standard_transaction(&request));
    }
    { // rejected recipient
        _default_request(&request);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount);
        will_return(__wrap_workflow_verify_recipient, false);
        assert_int_equal(
            APP_ETH_SIGN_ERR_USER_ABORT, app_eth_verify_standard_transaction(&request));
    }
    { // rejected total/fee
        _default_request(&request);
        expect_string(__wrap_workflow_verify_recipient, recipient, _recipient);
        expect_string(__wrap_workflow_verify_recipient, amount, _formatted_amount);
        will_return(__wrap_workflow_verify_recipient, true);
        expect_string(__wrap_workflow_verify_total, total, _formatted_total);
        expect_string(__wrap_workflow_verify_total, fee, _formatted_fee);
        will_return(__wrap_workflow_verify_total, false);
        assert_int_equal(
            APP_ETH_SIGN_ERR_USER_ABORT, app_eth_verify_standard_transaction(&request));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_eth_verify_erc20_transaction),
        cmocka_unit_test(_test_app_eth_verify_standard_transaction),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
