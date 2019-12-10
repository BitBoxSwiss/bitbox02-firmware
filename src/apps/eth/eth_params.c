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
    /* Ropsten */
    {
        .coin = ETHCoin_RopstenETH,
        .unit = "TEST",
        .contract_address =
            "\x2f\x45\xb6\xfb\x2f\x28\xa7\x3f\x11\x04\x00\x38\x6d\xa3\x10\x44\xb2\xe9\x53\xd4",
        .decimals = 18,
    },
    /* Ethereum */
    {
        .coin = ETHCoin_ETH,
        .unit = "USDT",
        .contract_address =
            "\xda\xc1\x7f\x95\x8d\x2e\xe5\x23\xa2\x20\x62\x06\x99\x45\x97\xc1\x3d\x83\x1e\xc7",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "USDC",
        .contract_address =
            "\xa0\xb8\x69\x91\xc6\x21\x8b\x36\xc1\xd1\x9d\x4a\x2e\x9e\xb0\xce\x36\x06\xeb\x48",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "LINK",
        .contract_address =
            "\x51\x49\x10\x77\x1a\xf9\xca\x65\x6a\xf8\x40\xdf\xf8\x3e\x82\x64\xec\xf9\x86\xca",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "BAT",
        .contract_address =
            "\x0d\x87\x75\xf6\x48\x43\x06\x79\xa7\x09\xe9\x8d\x2b\x0c\xb6\x25\x0d\x28\x87\xef",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "MKR",
        .contract_address =
            "\x9f\x8f\x72\xaa\x93\x04\xc8\xb5\x93\xd5\x55\xf1\x2e\xf6\x58\x9c\xc3\xa5\x79\xa2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "ZRX",
        .contract_address =
            "\xe4\x1d\x24\x89\x57\x1d\x32\x21\x89\x24\x6d\xaf\xa5\xeb\xde\x1f\x46\x99\xf4\x98",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "SAI",
        .contract_address =
            "\x89\xd2\x4a\x6b\x4c\xcb\x1b\x6f\xaa\x26\x25\xfe\x56\x2b\xdd\x9a\x23\x26\x03\x59",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .unit = "DAI",
        .contract_address =
            "\x6b\x17\x54\x74\xe8\x90\x94\xc4\x4d\xa9\x8b\x95\x4e\xed\xea\xc4\x95\x27\x1d\x0f",
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
