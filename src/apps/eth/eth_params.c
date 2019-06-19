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

#include "eth_params.h"
#include <wally_bip32.h>

app_eth_coin_params_t app_eth_params_eth = {
    .bip44_coin = 60 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 1,
    .unit = "ETH",
};

app_eth_coin_params_t app_eth_params_ropsten_eth = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 3,
    .unit = "TETH",
};

app_eth_coin_params_t app_eth_params_rinkeby_eth = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 4,
    .unit = "TETH",
};

app_eth_coin_params_t* app_eth_params_get(ETHCoin coin)
{
    switch (coin) {
    case ETHCoin_ETH:
        return &app_eth_params_eth;
    case ETHCoin_RopstenETH:
        return &app_eth_params_ropsten_eth;
    case ETHCoin_RinkebyETH:
        return &app_eth_params_rinkeby_eth;
    default:
        return NULL;
    }
}
