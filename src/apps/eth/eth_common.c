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

#include "eth_common.h"

#include <wally_bip32.h> // for BIP32_INITIAL_HARDENED_CHILD

#define BIP44_ETH_ACCOUNT_MAX (99) // 100 accounts

bool eth_common_is_valid_keypath(ETHCoin coin, const uint32_t* keypath, size_t keypath_len)
{
    if (keypath_len != 5) {
        return false;
    }
    if (keypath[0] != 44 + BIP32_INITIAL_HARDENED_CHILD) {
        return false;
    }
    uint32_t expected_purpose;
    switch (coin) {
    case ETHCoin_ETH:
        expected_purpose = 60; // mainnet
        break;
    case ETHCoin_RopstenETH:
        expected_purpose = 1; // testnet
        break;
    case ETHCoin_RinkebyETH:
        expected_purpose = 1; // testnet
        break;
    default:
        return false;
    }
    if (keypath[1] != expected_purpose + BIP32_INITIAL_HARDENED_CHILD) {
        return false;
    }
    if (keypath[2] != 0 + BIP32_INITIAL_HARDENED_CHILD) {
        return false;
    }
    if (keypath[3] != 0) {
        return false;
    }
    if (keypath[4] > BIP44_ETH_ACCOUNT_MAX) {
        return false;
    }
    return true;
}
