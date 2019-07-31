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

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_eth_common_is_valid_keypath_invalid),
        cmocka_unit_test(_test_eth_common_is_valid_keypath_accounts),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
