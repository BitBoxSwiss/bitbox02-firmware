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

#include <util.h>

#include <wally_bip32.h>

static const app_eth_coin_params_t _params_eth = {
    .bip44_coin = 60 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 1,
    .unit = "ETH",
};

static const app_eth_coin_params_t _params_ropsten_eth = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 3,
    .unit = "TETH",
};

static const app_eth_coin_params_t _params_rinkeby_eth = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .chain_id = 4,
    .unit = "TETH",
};

const app_eth_coin_params_t* app_eth_params_get(ETHCoin coin)
{
    switch (coin) {
    case ETHCoin_ETH:
        return &_params_eth;
    case ETHCoin_RopstenETH:
        return &_params_ropsten_eth;
    case ETHCoin_RinkebyETH:
        return &_params_rinkeby_eth;
    default:
        return NULL;
    }
}

static const app_eth_erc20_params_t _erc20_params[] = {
    {
        .coin = ETHCoin_RopstenETH,
        .unit = "TEST",
        .contract_address =
            {
                0x2f, 0x45, 0xb6, 0xfb, 0x2f, 0x28, 0xa7, 0x3f, 0x11, 0x04,
                0x00, 0x38, 0x6d, 0xa3, 0x10, 0x44, 0xb2, 0xe9, 0x53, 0xd4,
            },
        .decimals = 18,
    },
};

const app_eth_erc20_params_t* app_eth_erc20_params_get(
    ETHCoin coin,
    const uint8_t* contract_address)
{
    for (size_t index = 0; index < sizeof(_erc20_params) / sizeof(app_eth_erc20_params_t);
         index++) {
        const app_eth_erc20_params_t* params = &_erc20_params[index];
        if (params->coin == coin &&
            MEMEQ(contract_address, params->contract_address, sizeof(params->contract_address))) {
            return params;
        }
    }
    return NULL;
}
