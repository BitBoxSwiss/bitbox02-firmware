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
        .name = "TEST",
        .unit = "TEST",
        .contract_address =
            "\x2f\x45\xb6\xfb\x2f\x28\xa7\x3f\x11\x04\x00\x38\x6d\xa3\x10\x44\xb2\xe9\x53\xd4",
        .decimals = 18,
    },
    /* Ethereum */
    {
        .coin = ETHCoin_ETH,
        .name = "1SG",
        .unit = "1SG",
        .contract_address =
            "\x0f\x72\x71\x4b\x35\xa3\x66\x28\x5d\xf8\x58\x86\xa2\xee\x17\x46\x01\x29\x2a\x17",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FirstBlood",
        .unit = "1ST",
        .contract_address =
            "\xaf\x30\xd2\xa7\xe9\x0d\x7d\xc3\x61\xc8\xc4\x58\x5e\x9b\xb7\xd2\xf6\xf1\x5b\xc7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "1World",
        .unit = "1WO",
        .contract_address =
            "\xfd\xbc\x1a\xdc\x26\xf0\xf8\xf8\x60\x6a\x5d\x63\xb7\xd3\xa3\xcd\x21\xc2\x2b\x23",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "999",
        .unit = "999",
        .contract_address =
            "\xbf\x05\x57\x19\x88\xda\xab\x22\xd3\x3c\x28\xbb\xb1\x35\x66\xea\xe9\xde\xe6\x26",
        .decimals = 3,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Airbloc",
        .unit = "ABL",
        .contract_address =
            "\xf8\xb3\x58\xb3\x39\x7a\x8e\xa5\x46\x4f\x8c\xc7\x53\x64\x5d\x42\xe1\x4b\x79\xea",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Arcblock",
        .unit = "ABT",
        .contract_address =
            "\xb9\x8d\x4c\x97\x42\x5d\x99\x08\xe6\x6e\x53\xa6\xfd\xf6\x73\xac\xca\x0b\xe9\x86",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Abyss Token",
        .unit = "ABYSS",
        .contract_address =
            "\x0e\x8d\x6b\x47\x1e\x33\x2f\x14\x0e\x7d\x9d\xbb\x99\xe5\xe3\x82\x2f\x72\x8d\xa6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aladdin",
        .unit = "ADN",
        .contract_address =
            "\x95\xa4\x1f\xb8\x0c\xa7\x03\x06\xe9\xec\xf4\xe5\x1c\xea\x31\xbd\x18\x37\x9c\x18",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "adToken",
        .unit = "ADT",
        .contract_address =
            "\xd0\xd6\xd6\xc5\xfe\x4a\x67\x7d\x34\x3c\xc4\x33\x53\x6b\xb7\x17\xba\xe1\x67\xdd",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AdEx",
        .unit = "ADX",
        .contract_address =
            "\x44\x70\xbb\x87\xd7\x7b\x96\x3a\x01\x3d\xb9\x39\xbe\x33\x2f\x92\x7f\x2b\x99\x2e",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aencoin",
        .unit = "AEN",
        .contract_address =
            "\x0b\xef\x61\x9c\xf3\x8c\xf0\xc2\x29\x67\x28\x9b\x84\x19\x72\x0f\xbd\x1d\xb9\xf7",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aergo",
        .unit = "AERGO",
        .contract_address =
            "\xae\x31\xb8\x5b\xfe\x62\x74\x7d\x08\x36\xb8\x26\x08\xb4\x83\x03\x61\xa3\xd3\x7a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Asian Fintech",
        .unit = "AFIN",
        .contract_address =
            "\xee\x9e\x5e\xff\x40\x1e\xe9\x21\xb1\x38\x49\x0d\x00\xca\x8d\x1f\x13\xf6\x7a\x72",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SingularityNET",
        .unit = "AGI",
        .contract_address =
            "\x8e\xb2\x43\x19\x39\x37\x16\x66\x8d\x76\x8d\xce\xc2\x93\x56\xae\x9c\xff\xe2\x85",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Agrocoin",
        .unit = "AGRO",
        .contract_address =
            "\x1f\xd2\x7f\x0c\xfc\x6f\x27\x3b\x87\xa5\xe0\xf6\xfc\xf0\x63\x42\x2e\x7b\xcd\x6a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AgaveCoin",
        .unit = "AGVC",
        .contract_address =
            "\x8b\x79\x65\x6f\xc3\x8a\x04\x04\x4e\x49\x5e\x22\xfa\xd7\x47\x12\x6c\xa3\x05\xc4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AI Doctor",
        .unit = "AIDOC",
        .contract_address =
            "\x58\x4b\x44\x85\x36\x80\xee\x34\xa0\xf3\x37\xb7\x12\xa8\xf6\x6d\x81\x6d\xf1\x51",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ambrosus",
        .unit = "AMB",
        .contract_address =
            "\x4d\xc3\x64\x3d\xbc\x64\x2b\x72\xc1\x58\xe7\xf3\xd2\xff\x23\x2d\xf6\x1c\xb6\xce",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Amino Network",
        .unit = "AMIO",
        .contract_address =
            "\x2e\x68\xdf\xb3\xf5\x0e\xa3\x02\xc8\x8f\x8d\xb7\x40\x96\xd5\x75\x65\xd9\x97\x0a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AMLT",
        .unit = "AMLT",
        .contract_address =
            "\xca\x0e\x72\x69\x60\x0d\x35\x3f\x70\xb1\x4a\xd1\x18\xa4\x95\x75\x45\x5c\x0f\x2f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AMO Coin",
        .unit = "AMO",
        .contract_address =
            "\x38\xc8\x7a\xa8\x9b\x2b\x8c\xd9\xb9\x5b\x73\x6e\x1f\xa7\xb6\x12\xea\x97\x21\x69",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ampleforth",
        .unit = "AMPL",
        .contract_address =
            "\xd4\x6b\xa6\xd9\x42\x05\x0d\x48\x9d\xbd\x93\x8a\x2c\x90\x9a\x5d\x50\x39\xa1\x61",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Anchor",
        .unit = "ANCT",
        .contract_address =
            "\x54\x56\xbc\x77\xdd\x27\x5c\x45\xc3\xc1\x5f\x0c\xf9\x36\xb7\x63\xcf\x57\xc3\xb5",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ankr",
        .unit = "ANKR",
        .contract_address =
            "\x82\x90\x33\x3c\xef\x9e\x6d\x52\x8d\xd5\x61\x8f\xb9\x7a\x76\xf2\x68\xf3\xed\xd4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aragon",
        .unit = "ANT",
        .contract_address =
            "\x96\x0b\x23\x6a\x07\xcf\x12\x26\x63\xc4\x30\x33\x50\x60\x9a\x66\xa7\xb2\x88\xc0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aurora",
        .unit = "AOA",
        .contract_address =
            "\x9a\xb1\x65\xd7\x95\x01\x9b\x6d\x8b\x3e\x97\x1d\xda\x91\x07\x14\x21\x30\x5e\x5a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AppCoins",
        .unit = "APPC",
        .contract_address =
            "\x1a\x7a\x8b\xd9\x10\x6f\x2b\x8d\x97\x7e\x08\x58\x2d\xc7\xd2\x4c\x72\x3a\xb0\xdb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aeron",
        .unit = "ARN",
        .contract_address =
            "\xba\x5f\x11\xb1\x6b\x15\x57\x92\xcf\x3b\x2e\x68\x80\xe8\x70\x68\x59\xa8\xae\xb6",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ARPA Chain",
        .unit = "ARPA",
        .contract_address =
            "\xba\x50\x93\x3c\x26\x8f\x56\x7b\xdc\x86\xe1\xac\x13\x1b\xe0\x72\xc6\xb0\xb7\x1a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "AirSwap",
        .unit = "AST",
        .contract_address =
            "\x27\x05\x4b\x13\xb1\xb7\x98\xb3\x45\xb5\x91\xa4\xd2\x2e\x65\x62\xd4\x7e\xa7\x5a",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Artfinity",
        .unit = "AT",
        .contract_address =
            "\xe5\x4b\x34\x58\xc4\x7e\x44\xc3\x7a\x26\x7e\x7c\x63\x3a\xfe\xf8\x82\x87\xc2\x94",
        .decimals = 5,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ATC Coin",
        .unit = "ATCC",
        .contract_address =
            "\xdd\xaa\xf4\xa0\x70\x2a\x03\xa4\x50\x5f\x23\x52\xa1\xab\xa0\x01\xff\xc3\x44\xbe",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ATLANT",
        .unit = "ATL",
        .contract_address =
            "\x78\xb7\xfa\xda\x55\xa6\x4d\xd8\x95\xd8\xc8\xc3\x57\x79\xdd\x8b\x67\xfa\x8a\x05",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ATN",
        .unit = "ATN",
        .contract_address =
            "\x46\x17\x33\xc1\x7b\x07\x55\xca\x56\x49\xb6\xdb\x08\xb3\xe2\x13\xfc\xf2\x25\x46",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cube",
        .unit = "AUTO",
        .contract_address =
            "\x62\x2d\xff\xcc\x4e\x83\xc6\x4b\xa9\x59\x53\x0a\x5a\x55\x80\x68\x7a\x57\x58\x1b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "B2BX",
        .unit = "B2B",
        .contract_address =
            "\x5d\x51\xfc\xce\xd3\x11\x4a\x8b\xb5\xe9\x0c\xdd\x0f\x9d\x68\x2b\xcb\xcc\x53\x93",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BaaSid",
        .unit = "BAAS",
        .contract_address =
            "\x3e\x65\xe1\xee\xfd\xe5\xea\x7c\xcf\xc9\xa9\xa1\x63\x4a\xbe\x90\xf3\x22\x62\xf8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Band Protocol",
        .unit = "BAND",
        .contract_address =
            "\xba\x11\xd0\x0c\x5f\x74\x25\x5f\x56\xa5\xe3\x66\xf4\xf7\x7f\x5a\x18\x6d\x7f\x55",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Basic Attention Token",
        .unit = "BAT",
        .contract_address =
            "\x0d\x87\x75\xf6\x48\x43\x06\x79\xa7\x09\xe9\x8d\x2b\x0c\xb6\x25\x0d\x28\x87\xef",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BABB",
        .unit = "BAX",
        .contract_address =
            "\x9a\x02\x42\xb7\xa3\x3d\xac\xbe\x40\xed\xb9\x27\x83\x4f\x96\xeb\x39\xf8\xfb\xcb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Brickblock",
        .unit = "BBK",
        .contract_address =
            "\x4a\x60\x58\x66\x6c\xf1\x05\x7e\xac\x3c\xd3\xa5\xa6\x14\x62\x05\x47\x55\x9f\xc9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blockmason Credit Protocol",
        .unit = "BCPT",
        .contract_address =
            "\x1c\x44\x81\x75\x0d\xaa\x5f\xf5\x21\xa2\xa7\x49\x0d\x99\x81\xed\x46\x46\x5d\xbd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitCapitalVendor",
        .unit = "BCV",
        .contract_address =
            "\x10\x14\x61\x3e\x2b\x3c\xbc\x4d\x57\x50\x54\xd4\x98\x2e\x58\x0d\x9b\x99\xd7\xb1",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Buggyra Coin Zero",
        .unit = "BCZERO",
        .contract_address =
            "\xd4\x52\x47\xc0\x73\x79\xd9\x49\x04\xe0\xa8\x7b\x44\x81\xf0\xa1\xdd\xfa\x0c\x64",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BidiPass",
        .unit = "BDP",
        .contract_address =
            "\x59\x31\x14\xf0\x3a\x0a\x57\x5a\xec\xe9\xed\x67\x5e\x52\xed\x68\xd2\x17\x2b\x8c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DAO.Casino",
        .unit = "BET",
        .contract_address =
            "\x8a\xa3\x3a\x78\x99\xfc\xc8\xea\x5f\xbe\x6a\x60\x8a\x10\x9c\x38\x93\xa1\xb8\xb2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitForex Token",
        .unit = "BF",
        .contract_address =
            "\x5b\x71\xbe\xe9\xd9\x61\xb1\xb8\x48\xf8\x48\x5e\xec\x8d\x87\x87\xf8\x02\x17\xf5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BnkToTheFuture",
        .unit = "BFT",
        .contract_address =
            "\x01\xff\x50\xf8\xb7\xf7\x4e\x4f\x00\x58\x0d\x95\x96\xcd\x3d\x0d\x6d\x6e\x32\x6f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BHEX Token",
        .unit = "BHT",
        .contract_address =
            "\xfc\x29\xb6\xe6\x26\xb6\x77\x76\x67\x5f\xff\x55\xd5\xbc\x04\x52\xd0\x42\xf4\x34",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bibox Token",
        .unit = "BIX",
        .contract_address =
            "\xb3\x10\x4b\x4b\x9d\xa8\x20\x25\xe8\xb9\xf8\xfb\x28\xb3\x55\x3c\xe2\xf6\x70\x69",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bloom",
        .unit = "BLT",
        .contract_address =
            "\x10\x7c\x45\x04\xcd\x79\xc5\xd2\x69\x6e\xa0\x03\x0a\x8d\xd4\xe9\x26\x01\xb8\x2e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bluzelle",
        .unit = "BLZ",
        .contract_address =
            "\x57\x32\x04\x6a\x88\x37\x04\x40\x4f\x28\x4c\xe4\x1f\xfa\xdd\x5b\x00\x7f\xd6\x68",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blackmoon",
        .unit = "BMC",
        .contract_address =
            "\xdf\x6e\xf3\x43\x35\x07\x80\xbf\x8c\x34\x10\xbf\x06\x2e\x0c\x01\x5b\x1d\xd6\x71",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitMart Token",
        .unit = "BMX",
        .contract_address =
            "\x98\x6e\xe2\xb9\x44\xc4\x2d\x01\x7f\x52\xaf\x21\xc4\xc6\x9b\x84\xdb\xea\x35\xd8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Chimpion",
        .unit = "BNANA",
        .contract_address =
            "\x07\xef\x9e\x82\x72\x1a\xc1\x68\x09\xd2\x4d\xaf\xbe\x17\x92\xce\x01\x65\x4d\xb4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bankera",
        .unit = "BNK",
        .contract_address =
            "\xc8\x0c\x5e\x40\x22\x01\x72\xb3\x6a\xde\xe2\xc9\x51\xf2\x6f\x2a\x57\x78\x10\xc5",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bancor",
        .unit = "BNT",
        .contract_address =
            "\x1f\x57\x3d\x6f\xb3\xf1\x3d\x68\x9f\xf8\x44\xb4\xce\x37\x79\x4d\x79\xa7\xff\x1c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BOSAGORA",
        .unit = "BOA",
        .contract_address =
            "\x74\x6d\xda\x2e\xa2\x43\x40\x0d\x5a\x63\xe0\x70\x0f\x19\x0a\xb7\x9f\x06\x48\x9e",
        .decimals = 7,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BOLT",
        .unit = "BOLT",
        .contract_address =
            "\x9f\x23\x5d\x23\x35\x48\x57\xef\xe6\xc5\x41\xdb\x92\xa9\xef\x18\x77\x68\x9b\xcb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BORA",
        .unit = "BORA",
        .contract_address =
            "\x26\xfb\x86\x57\x9e\x37\x1c\x7a\xed\xc4\x61\xb2\xdd\xef\x0a\x86\x28\xc9\x3d\x3b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "botXcoin",
        .unit = "BOTX",
        .contract_address =
            "\xef\x19\xf4\xe4\x88\x30\x09\x3c\xe5\xbc\x8b\x3f\xf7\xf9\x03\xa0\xae\x3e\x9f\xa1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BOX Token",
        .unit = "BOX",
        .contract_address =
            "\xe1\xa1\x78\xb6\x81\xbd\x05\x96\x4d\x3e\x3e\xd3\x3a\xe7\x31\x57\x7d\x9d\x96\xdd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ContentBox",
        .unit = "BOX",
        .contract_address =
            "\x63\xf5\x84\xfa\x56\xe6\x0e\x4d\x0f\xe8\x80\x2b\x27\xc7\xe6\xe3\xb3\x3e\x00\x7f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blockport",
        .unit = "BPT",
        .contract_address =
            "\x32\x76\x82\x77\x9b\xab\x2b\xf4\xd1\x33\x7e\x89\x74\xab\x9d\xe8\x27\x5a\x7c\xa8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bitsdaq",
        .unit = "BQQQ",
        .contract_address =
            "\x1b\x80\xee\xea\xdc\xc5\x90\xf3\x05\x94\x5b\xcc\x25\x8c\xfa\x77\x0b\xbe\x18\x90",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BQT",
        .unit = "BQTX",
        .contract_address =
            "\x9d\x8b\xe9\x4d\x06\x12\x17\x0c\xe5\x33\xac\x4d\x7b\x43\xcc\x3c\xd9\x1e\x5a\x1a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bread",
        .unit = "BRD",
        .contract_address =
            "\x55\x8e\xc3\x15\x2e\x2e\xb2\x17\x49\x05\xcd\x19\xae\xa4\xe3\x4a\x23\xde\x9a\xd6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Breezecoin",
        .unit = "BRZE",
        .contract_address =
            "\x77\xc0\x75\x55\xaf\x5f\xfd\xc9\x46\xfb\x47\xce\x15\xea\x68\x62\x0e\x4e\x71\x70",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitMax Token",
        .unit = "BTMX",
        .contract_address =
            "\x1c\x28\x9a\x12\xa8\x55\x2b\x31\x4d\x0d\x15\x3d\x69\x91\xfd\x27\xa5\x4a\xa6\x40",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bitrue Coin",
        .unit = "BTR",
        .contract_address =
            "\xd4\x33\x13\x8d\x12\xbe\xb9\x92\x9f\xf6\xfd\x58\x3d\xc8\x36\x63\xee\xa6\xaa\xa5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blocktrade Token",
        .unit = "BTT",
        .contract_address =
            "\xfa\x45\x6c\xf5\x52\x50\xa8\x39\x08\x8b\x27\xee\x32\xa4\x24\xd7\xda\xcb\x54\xff",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BTU Protocol",
        .unit = "BTU",
        .contract_address =
            "\xb6\x83\xd8\x3a\x53\x2e\x2c\xb7\xdf\xa5\x27\x5e\xed\x36\x98\x43\x63\x71\xcc\x9f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Binance USD",
        .unit = "BUSD",
        .contract_address =
            "\x4f\xab\xb1\x45\xd6\x46\x52\xa9\x48\xd7\x25\x33\x02\x3f\x6e\x7a\x62\x3c\x7c\x53",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blue Whale EXchange",
        .unit = "BWX",
        .contract_address =
            "\xce\x51\x14\xd7\xfa\x83\x61\xf0\xc0\x88\xee\x26\xfa\x3a\x54\x46\xc4\xa1\xf5\x0b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bitbook Gambling",
        .unit = "BXK",
        .contract_address =
            "\xeb\x69\x85\xac\xd6\xd0\xcb\xff\x60\xb8\x80\x32\xb0\xb2\x9a\xc1\xd9\xd6\x6a\x1b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bit-Z Token",
        .unit = "BZ",
        .contract_address =
            "\x43\x75\xe7\xad\x8a\x01\xb8\xec\x3e\xd0\x41\x39\x9f\x62\xd9\xcd\x12\x0e\x00\x63",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Bezant",
        .unit = "BZNT",
        .contract_address =
            "\xe1\xae\xe9\x84\x95\x36\x5f\xc1\x79\x69\x9c\x1b\xb3\xe7\x61\xfa\x71\x6b\xee\x62",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CRYPTO20",
        .unit = "C20",
        .contract_address =
            "\x26\xe7\x53\x07\xfc\x0c\x02\x14\x72\xfe\xb8\xf7\x27\x83\x95\x31\xf1\x12\xf3\x17",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Change",
        .unit = "CAG",
        .contract_address =
            "\x7d\x4b\x8c\xce\x05\x91\xc9\x04\x4a\x22\xee\x54\x35\x33\xb7\x2e\x97\x6e\x36\xc3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cajutel",
        .unit = "CAJ",
        .contract_address =
            "\x3c\x6a\x7a\xb4\x7b\x5f\x05\x8b\xe0\xe7\xc7\xfe\x1a\x4b\x79\x25\xb8\xac\xa4\x0e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CanYaCoin",
        .unit = "CAN",
        .contract_address =
            "\x1d\x46\x24\x14\xfe\x14\xcf\x48\x9c\x7a\x21\xca\xc7\x85\x09\xf4\xbf\x8c\xd7\xc0",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cashaa",
        .unit = "CAS",
        .contract_address =
            "\xe8\x78\x0b\x48\xbd\xb0\x5f\x92\x86\x97\xa5\xe8\x15\x5f\x67\x2e\xd9\x14\x62\xf7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CashBet Coin",
        .unit = "CBC",
        .contract_address =
            "\x26\xdb\x54\x39\xf6\x51\xca\xf4\x91\xa8\x7d\x48\x79\x9d\xa8\x1f\x19\x1b\xdb\x6b",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CommerceBlock",
        .unit = "CBT",
        .contract_address =
            "\x07\x6c\x97\xe1\xc8\x69\x07\x2e\xe2\x2f\x8c\x91\x97\x8c\x99\xb4\xbc\xb0\x25\x91",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Clipper Coin",
        .unit = "CCC",
        .contract_address =
            "\xbf\x59\xe6\xfe\x2b\xc4\xee\x8d\x30\x3e\x49\x33\x90\xb4\xaa\xca\xb1\x6f\xcc\x91",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Blox",
        .unit = "CDT",
        .contract_address =
            "\x17\x7d\x39\xac\x67\x6e\xd1\xc6\x7a\x2b\x26\x8a\xd7\xf1\xe5\x88\x26\xe5\xb0\xaf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CEEK VR",
        .unit = "CEEK",
        .contract_address =
            "\xb0\x56\xc3\x8f\x6b\x7d\xc4\x06\x43\x67\x40\x3e\x26\x42\x4c\xd2\xc6\x06\x55\xe1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Celer Network",
        .unit = "CELR",
        .contract_address =
            "\x4f\x92\x54\xc8\x3e\xb5\x25\xf9\xfc\xf3\x46\x49\x0b\xbb\x3e\xd2\x8a\x81\xc6\x67",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Centrality",
        .unit = "CENNZ",
        .contract_address =
            "\x11\x22\xb6\xa0\xe0\x0d\xce\x05\x63\x08\x2b\x6e\x29\x53\xf3\xa9\x43\x85\x5c\x1f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CoinEx Token",
        .unit = "CET",
        .contract_address =
            "\x08\x1f\x67\xaf\xa0\xcc\xf8\xc7\xb1\x75\x40\x76\x7b\xbe\x95\xdf\x2b\xa8\xd9\x7f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CoinPoker",
        .unit = "CHP",
        .contract_address =
            "\xf3\xdb\x75\x60\xe8\x20\x83\x46\x58\xb5\x90\xc9\x62\x34\xc3\x33\xcd\x3d\x5e\x5e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Chromia",
        .unit = "CHR",
        .contract_address =
            "\x91\x50\x44\x52\x67\x58\x53\x3d\xfb\x91\x8e\xce\xb6\xe4\x4b\xc2\x16\x32\x06\x0d",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SwissBorg",
        .unit = "CHSB",
        .contract_address =
            "\xba\x9d\x41\x99\xfa\xb4\xf2\x6e\xfe\x35\x51\xd4\x90\xe3\x82\x14\x86\xf1\x35\xba",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Chiliz",
        .unit = "CHZ",
        .contract_address =
            "\x35\x06\x42\x4f\x91\xfd\x33\x08\x44\x66\xf4\x02\xd5\xd9\x7f\x05\xf8\xe3\xb4\xaf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cryptoindex.com 100",
        .unit = "CIX100",
        .contract_address =
            "\x63\x93\xe8\x22\x87\x47\x28\xf8\xaf\xa7\xe1\xc9\x94\x4e\x41\x7d\x37\xca\x58\x78",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Color Platform",
        .unit = "CLR",
        .contract_address =
            "\x23\x96\xfb\xc0\xe2\xe3\xae\x4b\x72\x06\xeb\xdb\x57\x06\xe2\xa5\x92\x03\x49\xcb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cindicator",
        .unit = "CND",
        .contract_address =
            "\xd4\xc4\x35\xf5\xb0\x9f\x85\x5c\x33\x17\xc8\x52\x4c\xb1\xf5\x86\xe4\x27\x95\xfa",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Content Neutrality Network",
        .unit = "CNN",
        .contract_address =
            "\x87\x13\xd2\x66\x37\xcf\x49\xe1\xb6\xb4\xa7\xce\x57\x10\x6a\xab\xc9\x32\x53\x43",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cocos-BCX",
        .unit = "COCOS",
        .contract_address =
            "\x0c\x6f\x5f\x7d\x55\x5e\x75\x18\xf6\x84\x1a\x79\x43\x6b\xd2\xb1\xee\xf0\x33\x81",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CONUN",
        .unit = "CON",
        .contract_address =
            "\x4d\xd6\x72\xe7\x7c\x79\x58\x44\xfe\x3a\x46\x4e\xf8\xef\x0f\xaa\xe6\x17\xc8\xfb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Constant",
        .unit = "CONST",
        .contract_address =
            "\x49\x83\xf7\x67\xb1\xbc\x44\x32\x8e\x43\x47\x29\xdd\xab\xea\x0a\x06\x4c\xa1\xac",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cosmo Coin",
        .unit = "COSM",
        .contract_address =
            "\xd1\xe1\x0c\x37\xa2\x7d\x95\xd9\x57\x20\x29\x1b\x1d\xc6\xf1\x2f\x74\xc7\x14\x43",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Covesting",
        .unit = "COV",
        .contract_address =
            "\xe2\xfb\x65\x29\xef\x56\x6a\x08\x0e\x6d\x23\xde\x0b\xd3\x51\x31\x10\x87\xd5\x67",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cryptopay",
        .unit = "CPAY",
        .contract_address =
            "\x0e\xbb\x61\x42\x04\xe4\x7c\x09\xb6\xc3\xfe\xb9\xaa\xec\xad\x8e\xe0\x60\xe2\x3e",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CPChain",
        .unit = "CPC",
        .contract_address =
            "\xfa\xe4\xee\x59\xcd\xd8\x6e\x3b\xe9\xe8\xb9\x0b\x53\xaa\x86\x63\x27\xd7\xc0\x90",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Contents Protocol",
        .unit = "CPT",
        .contract_address =
            "\x9b\x62\x51\x3c\x8a\x27\x29\x0c\xf6\xa7\xa9\xe2\x93\x86\xe6\x00\x24\x5e\xa8\x19",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cryptaur",
        .unit = "CPT",
        .contract_address =
            "\x88\xd5\x0b\x46\x6b\xe5\x52\x22\x01\x9d\x71\xf9\xe8\xfa\xe1\x7f\x5f\x45\xfc\xa1",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Carry",
        .unit = "CRE",
        .contract_address =
            "\x11\x5e\xc7\x9f\x1d\xe5\x67\xec\x68\xb7\xae\x7e\xda\x50\x1b\x40\x66\x26\x47\x8e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Credo",
        .unit = "CREDO",
        .contract_address =
            "\x4e\x06\x03\xe2\xa2\x7a\x30\x48\x0e\x5e\x3a\x4f\xe5\x48\xe2\x9e\xf1\x2f\x64\xbe",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Crypto.com Coin",
        .unit = "CRO",
        .contract_address =
            "\xa0\xb7\x3e\x1f\xf0\xb8\x09\x14\xab\x6f\xe0\x44\x4e\x65\x84\x8c\x4c\x34\x45\x0b",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Crypterium",
        .unit = "CRPT",
        .contract_address =
            "\x80\xa7\xe0\x48\xf3\x7a\x50\x50\x03\x51\xc2\x04\xcb\x40\x77\x66\xfa\x3b\xae\x7f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Credits",
        .unit = "CS",
        .contract_address =
            "\x46\xb9\xad\x94\x4d\x10\x59\x45\x0d\xa1\x16\x35\x11\x06\x9c\x71\x8f\x69\x9d\x31",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitDice",
        .unit = "CSNO",
        .contract_address =
            "\x29\xd7\x52\x77\xac\x7f\x03\x35\xb2\x16\x5d\x08\x95\xe8\x72\x5c\xbf\x65\x8d\x73",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Caspian",
        .unit = "CSP",
        .contract_address =
            "\xa6\x44\x6d\x65\x5a\x0c\x34\xbc\x4f\x05\x04\x2e\xe8\x81\x70\xd0\x56\xcb\xaf\x45",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cortex",
        .unit = "CTXC",
        .contract_address =
            "\xea\x11\x75\x5a\xe4\x1d\x88\x9c\xee\xc3\x9a\x63\xe6\xff\x75\xa0\x2b\xc1\xc0\x0d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "carVertical",
        .unit = "CV",
        .contract_address =
            "\xda\x6c\xb5\x8a\x0d\x0c\x01\x61\x0a\x29\xc5\xa6\x5c\x30\x3e\x13\xe8\x85\x88\x7c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Civic",
        .unit = "CVC",
        .contract_address =
            "\x41\xe5\x56\x00\x54\x82\x4e\xa6\xb0\x73\x2e\x65\x6e\x3a\xd6\x4e\x20\xe9\x4e\x45",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Content Value Network",
        .unit = "CVNT",
        .contract_address =
            "\x64\x00\xb5\x52\x2f\x8d\x44\x8c\x08\x03\xe6\x24\x54\x36\xdd\x1c\x81\xdf\x09\xce",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CyberVein",
        .unit = "CVT",
        .contract_address =
            "\xbe\x42\x8c\x38\x67\xf0\x5d\xea\x2a\x89\xfc\x76\xa1\x02\xb5\x44\xea\xc7\xf7\x72",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CWV Chain",
        .unit = "CWV",
        .contract_address =
            "\xed\x49\x4c\x9e\x2f\x8e\x34\xe5\x3b\xdd\x0e\xa9\xb4\xd8\x03\x05\xcb\x15\xc5\xc2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CyberMusic",
        .unit = "CYMT",
        .contract_address =
            "\x78\xc2\x92\xd1\x44\x5e\x6b\x95\x58\xbf\x42\xe8\xbc\x36\x92\x71\xde\xd0\x62\xea",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CanonChain",
        .unit = "CZR",
        .contract_address =
            "\x02\x23\xfc\x70\x57\x42\x14\xf6\x58\x13\xfe\x33\x6d\x87\x0a\xc4\x7e\x14\x7f\xae",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Davinci Coin",
        .unit = "DAC",
        .contract_address =
            "\xaa\xd5\x4c\x9f\x27\xb8\x76\xd2\x53\x84\x55\xdd\xa6\x92\x07\x27\x9f\xf6\x73\xa5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DACSEE",
        .unit = "DACS",
        .contract_address =
            "\xa3\x11\x08\xe5\xba\xb5\x49\x45\x60\xdb\x34\xc9\x54\x92\x65\x8a\xf2\x39\x35\x7c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Edge",
        .unit = "DADI",
        .contract_address =
            "\xfb\x2f\x26\xf2\x66\xfb\x28\x05\xa3\x87\x23\x0f\x2a\xa0\xa3\x31\xb4\xd9\x6f\xba",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Constellation",
        .unit = "DAG",
        .contract_address =
            "\xa8\x25\x8a\xbc\x8f\x28\x11\xdd\x48\xec\xcd\x20\x9d\xb6\x8f\x25\xe3\xe3\x46\x67",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Digital Asset Guarantee Token",
        .unit = "DAGT",
        .contract_address =
            "\x56\xd1\xae\x30\xc9\x72\x88\xda\x4b\x58\xbc\x39\xf0\x26\x09\x17\x78\xe4\xe3\x16",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dai Stablecoin",
        .unit = "DAI",
        .contract_address =
            "\x6b\x17\x54\x74\xe8\x90\x94\xc4\x4d\xa9\x8b\x95\x4e\xed\xea\xc4\x95\x27\x1d\x0f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Streamr DATAcoin",
        .unit = "DATA",
        .contract_address =
            "\x0c\xf0\xee\x63\x78\x8a\x08\x49\xfe\x52\x97\xf3\x40\x7f\x70\x1e\x12\x2c\xc0\x23",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dentacoin",
        .unit = "DCN",
        .contract_address =
            "\x08\xd3\x2b\x0d\xa6\x3e\x2c\x3b\xcf\x80\x19\xc9\xc5\xd8\x49\xd7\xa9\xd7\x91\xe6",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Scry.info",
        .unit = "DDD",
        .contract_address =
            "\x9f\x5f\x3c\xfd\x7a\x32\x70\x0c\x93\xf9\x71\x63\x74\x07\xff\x17\xb9\x1c\x73\x42",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Darico Ecosystem Coin",
        .unit = "DEC",
        .contract_address =
            "\x89\xc6\xc8\x56\xa6\xdb\x3e\x46\x10\x71\x63\xd0\xcd\xa7\xa7\xff\x21\x1b\xd6\x55",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dent",
        .unit = "DENT",
        .contract_address =
            "\x35\x97\xbf\xd5\x33\xa9\x9c\x9a\xa0\x83\x58\x7b\x07\x44\x34\xe6\x1e\xb0\xa2\x58",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DEX",
        .unit = "DEX",
        .contract_address =
            "\x49\x7b\xae\xf2\x94\xc1\x1a\x5f\x0f\x5b\xea\x3f\x2a\xdb\x30\x73\xdb\x44\x8b\x56",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DigixDAO",
        .unit = "DGD",
        .contract_address =
            "\xe0\xb7\x92\x7c\x4a\xf2\x37\x65\xcb\x51\x31\x4a\x0e\x05\x21\xa9\x64\x5f\x0e\x2a",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Digitex Futures",
        .unit = "DGTX",
        .contract_address =
            "\x1c\x83\x50\x14\x78\xf1\x32\x09\x77\x04\x70\x08\x49\x6d\xac\xbd\x60\xbb\x15\xef",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Digix Gold Token",
        .unit = "DGX",
        .contract_address =
            "\x4f\x3a\xfe\xc4\xe5\xa3\xf2\xa6\xa1\xa4\x11\xde\xf7\xd7\xdf\xe5\x0e\xe0\x57\xbf",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Etheroll",
        .unit = "DICE",
        .contract_address =
            "\x2e\x07\x1d\x29\x66\xaa\x7d\x8d\xec\xb1\x00\x58\x85\xba\x19\x77\xd6\x03\x8a\x65",
        .decimals = 16,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Agrello",
        .unit = "DLT",
        .contract_address =
            "\x07\xe3\xc7\x06\x53\x54\x8b\x04\xf0\xa7\x59\x70\xc1\xf8\x1b\x4c\xbb\xfb\x60\x6f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DMarket",
        .unit = "DMT",
        .contract_address =
            "\x2c\xcb\xff\x3a\x04\x2c\x68\x71\x6e\xd2\xa2\xcb\x0c\x54\x4a\x9f\x1d\x19\x35\xe1",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "district0x",
        .unit = "DNT",
        .contract_address =
            "\x0a\xbd\xac\xe7\x0d\x37\x90\x23\x5a\xf4\x48\xc8\x85\x47\x60\x3b\x94\x56\x04\xea",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dock",
        .unit = "DOCK",
        .contract_address =
            "\xe5\xda\xda\x80\xaa\x64\x77\xe8\x5d\x09\x74\x7f\x28\x42\xf7\x99\x3d\x0d\xf7\x1c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Diamond Platform Token",
        .unit = "DPT",
        .contract_address =
            "\x10\xc7\x15\x15\x60\x24\x29\xc1\x9d\x53\x01\x1e\xa7\x04\x0b\x87\xa4\x89\x48\x38",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DreamTeam Token",
        .unit = "DREAM",
        .contract_address =
            "\x82\xf4\xde\xd9\xce\xc9\xb5\x75\x0f\xbf\xf5\xc2\x18\x5a\xee\x35\xaf\xc1\x65\x87",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dragon Coins",
        .unit = "DRG",
        .contract_address =
            "\x81\x4f\x67\xfa\x28\x6f\x75\x72\xb0\x41\xd0\x41\xb1\xd9\x9b\x43\x2c\x91\x55\xee",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dragonchain",
        .unit = "DRGN",
        .contract_address =
            "\x41\x9c\x4d\xb4\xb9\xe2\x5d\x6d\xb2\xad\x96\x91\xcc\xb8\x32\xc8\xd9\xfd\xa0\x5e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dropil",
        .unit = "DROP",
        .contract_address =
            "\x46\x72\xba\xd5\x27\x10\x74\x71\xcb\x50\x67\xa8\x87\xf4\x65\x6d\x58\x5a\x8a\x31",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DATA",
        .unit = "DTA",
        .contract_address =
            "\x69\xb1\x48\x39\x5c\xe0\x01\x5c\x13\xe3\x6b\xff\xba\xd6\x3f\x49\xef\x87\x4e\x03",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dynamic Trading Rights",
        .unit = "DTR",
        .contract_address =
            "\xd2\x34\xbf\x24\x10\xa0\x00\x9d\xf9\xc3\xc6\x3b\x61\x0c\x09\x73\x8f\x18\xcc\xd7",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dusk Network",
        .unit = "DUSK",
        .contract_address =
            "\x94\x0a\x2d\xb1\xb7\x00\x8b\x6c\x77\x6d\x4f\xaa\xca\x72\x9d\x6d\x4a\x4a\xa5\x51",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DxChain Token",
        .unit = "DX",
        .contract_address =
            "\x97\x3e\x52\x69\x11\x76\xd3\x64\x53\x86\x8d\x9d\x86\x57\x27\x88\xd2\x70\x41\xa9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "EURBASE",
        .unit = "EBASE",
        .contract_address =
            "\x86\xfa\xdb\x80\xd8\xd2\xcf\xf3\xc3\x68\x08\x19\xe4\xda\x99\xc1\x02\x32\xba\x0f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ecoreal Estate",
        .unit = "ECOREAL",
        .contract_address =
            "\xb0\x52\xf8\xa3\x3d\x8b\xb0\x68\x41\x4e\xad\xe0\x6a\xf6\x95\x51\x99\xf9\xf0\x10",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Eden",
        .unit = "EDN",
        .contract_address =
            "\x05\x86\x0d\x45\x3c\x79\x74\xcb\xf4\x65\x08\xc0\x6c\xba\x14\xe2\x11\xc6\x29\xce",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Eidoo",
        .unit = "EDO",
        .contract_address =
            "\xce\xd4\xe9\x31\x98\x73\x4d\xda\xff\x84\x92\xd5\x25\xbd\x25\x8d\x49\xeb\x38\x8e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Endor Protocol",
        .unit = "EDR",
        .contract_address =
            "\xc5\x28\xc2\x8f\xec\x0a\x90\xc0\x83\x32\x8b\xc4\x5f\x58\x7e\xe2\x15\x76\x0a\x0f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Egretia",
        .unit = "EGT",
        .contract_address =
            "\x8e\x1b\x44\x8e\xc7\xad\xfc\x7f\xa3\x5f\xc2\xe8\x85\x67\x8b\xd3\x23\x17\x6e\x34",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "EDUCare",
        .unit = "EKT",
        .contract_address =
            "\x4e\xcd\xb6\x38\x5f\x3d\xb3\x84\x7f\x9c\x4a\x9b\xf3\xf9\x91\x7b\xb2\x7a\x54\x52",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ELA Coin",
        .unit = "ELAC",
        .contract_address =
            "\xec\x6c\x86\x1c\x2a\x2b\x1e\x5f\xf5\x33\x67\x31\xbc\x80\xc2\x9d\xbf\xf8\x82\x73",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "aelf",
        .unit = "ELF",
        .contract_address =
            "\xbf\x21\x79\x85\x9f\xc6\xd5\xbe\xe9\xbf\x91\x58\x63\x2d\xc5\x16\x78\xa4\x10\x0e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Eminer",
        .unit = "EM",
        .contract_address =
            "\x35\xb0\x87\x22\xaa\x26\xbe\x11\x9c\x16\x08\x02\x9c\xcb\xc9\x76\xac\x5c\x10\x82",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Emirex Token",
        .unit = "EMRX",
        .contract_address =
            "\xbd\xbc\x2a\x5b\x32\xf3\xa5\x14\x1a\xcd\x18\xc3\x98\x83\x06\x6e\x4d\xab\x97\x74",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Enigma",
        .unit = "ENG",
        .contract_address =
            "\xf0\xee\x6b\x27\xb7\x59\xc9\x89\x3c\xe4\xf0\x94\xb4\x9a\xd2\x8f\xd1\x5a\x23\xe4",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Enjin Coin",
        .unit = "ENJ",
        .contract_address =
            "\xf6\x29\xcb\xd9\x4d\x37\x91\xc9\x25\x01\x52\xbd\x8d\xfb\xdf\x38\x0e\x2a\x3b\x9c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Dimension Chain",
        .unit = "EON",
        .contract_address =
            "\x4c\xb1\x0f\x4d\xf4\xbf\x4f\x64\xd4\x79\x7d\x00\xd4\x68\x18\x1e\xf7\x31\xbe\x9a",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "eosDAC",
        .unit = "EOSDAC",
        .contract_address =
            "\x7e\x9e\x43\x1a\x0b\x8c\x4d\x53\x2c\x74\x5b\x10\x43\xc7\xfa\x29\xa4\x8d\x4f\xba",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ERC20",
        .unit = "ERC20",
        .contract_address =
            "\xc3\x76\x1e\xb9\x17\xcd\x79\x0b\x30\xda\xd9\x9f\x6c\xc5\xb4\xff\x93\xc4\xf9\xea",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Elitium",
        .unit = "EUM",
        .contract_address =
            "\x6a\xb4\xa7\xd7\x5b\x0a\x42\xb6\xbc\x83\xe8\x52\xda\xb9\xe1\x21\xf9\xc6\x10\xaa",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "STASIS EURO",
        .unit = "EURS",
        .contract_address =
            "\xdb\x25\xf2\x11\xab\x05\xb1\xc9\x7d\x59\x55\x16\xf4\x57\x94\x52\x8a\x80\x7a\xd8",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Envion",
        .unit = "EVN",
        .contract_address =
            "\xd7\x80\xae\x2b\xf0\x4c\xd9\x6e\x57\x7d\x3d\x01\x47\x62\xf8\x31\xd9\x71\x29\xd0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Everus",
        .unit = "EVR",
        .contract_address =
            "\x31\x37\x61\x97\x05\xb5\xfc\x22\xa3\x04\x89\x89\xf9\x83\x90\x5e\x45\x6b\x59\xab",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Everex",
        .unit = "EVX",
        .contract_address =
            "\xf3\xdb\x5f\xa2\xc6\x6b\x7a\xf3\xeb\x0c\x0b\x78\x25\x10\x81\x6c\xbe\x48\x13\xb8",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "EveryCoin ",
        .unit = "EVY",
        .contract_address =
            "\xee\xd3\xae\x7b\x0f\x8b\x5b\x9b\xb8\xc0\x35\xa9\x94\x13\x82\xb1\x82\x26\x71\xcd",
        .decimals = 12,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "EXMR FDN",
        .unit = "EXMR",
        .contract_address =
            "\x33\x1f\xa6\xc9\x7c\x64\xe4\x74\x75\x16\x4b\x9f\xc8\x14\x3b\x53\x3c\x5e\xf5\x29",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FABRK",
        .unit = "FAB",
        .contract_address =
            "\x12\x68\x3d\xc9\xee\xc9\x5a\x5f\x74\x2d\x40\x20\x6e\x73\x31\x9e\x6b\x9d\x8a\x91",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Fatcoin",
        .unit = "FAT",
        .contract_address =
            "\x2e\xc9\x5b\x8e\xda\x54\x9b\x79\xa1\x24\x83\x35\xa3\x9d\x29\x9d\x00\xed\x31\x4c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Fetch.ai",
        .unit = "FET",
        .contract_address =
            "\x1d\x28\x7c\xc2\x5d\xad\x7c\xca\xf7\x6a\x26\xbc\x66\x0c\x5f\x7c\x8e\x2a\x05\xbd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Flowchain",
        .unit = "FLC",
        .contract_address =
            "\x5b\x53\xf9\x75\x5f\x82\x43\x9c\xba\x66\x00\x7e\xc7\x07\x3c\x59\xe0\xda\x4a\x7d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FLETA",
        .unit = "FLETA",
        .contract_address =
            "\x77\x88\xd7\x59\xf2\x1f\x53\x53\x30\x51\xa9\xae\x65\x7f\xa0\x5a\x1e\x06\x8f\xc6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Folgory Coin",
        .unit = "FLG",
        .contract_address =
            "\x5e\x04\x0a\xc7\x21\x40\xf0\x61\x7b\xc2\x4a\xb7\x13\x4c\x0c\x6e\xca\xe0\xe9\x65",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FNB Protocol",
        .unit = "FNB",
        .contract_address =
            "\x47\xb2\x8f\x36\x5b\xf4\xcb\x38\xdb\x4b\x63\x56\x86\x4b\xde\x7b\xc4\xb3\x51\x29",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FOAM",
        .unit = "FOAM",
        .contract_address =
            "\x49\x46\xfc\xea\x7c\x69\x26\x06\xe8\x90\x80\x02\xe5\x5a\x58\x2a\xf4\x4a\xc1\x21",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "The Force Protocol",
        .unit = "FOR",
        .contract_address =
            "\x1f\xcd\xce\x58\x95\x9f\x53\x66\x21\xd7\x6f\x5b\x7f\xfb\x95\x5b\xaa\x5a\x67\x2f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Fusion",
        .unit = "FSN",
        .contract_address =
            "\xd0\x35\x2a\x01\x9e\x9a\xb9\xd7\x57\x77\x6f\x53\x23\x77\xaa\xeb\xd3\x6f\xd5\x41",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "1irstcoin",
        .unit = "FST",
        .contract_address =
            "\x31\x0c\x93\xdf\xc1\xc5\xe3\x4c\xdf\x51\x67\x81\x03\xf6\x3c\x41\x76\x20\x89\xcd",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Fantom",
        .unit = "FTM",
        .contract_address =
            "\x4e\x15\x36\x1f\xd6\xb4\xbb\x60\x9f\xa6\x3c\x81\xa2\xbe\x19\xd8\x73\x71\x78\x70",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FTX Token",
        .unit = "FTT",
        .contract_address =
            "\x50\xd1\xc9\x77\x19\x02\x47\x60\x76\xec\xfc\x8b\x2a\x83\xad\x6b\x93\x55\xa4\xc9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Etherparty",
        .unit = "FUEL",
        .contract_address =
            "\xea\x38\xea\xa3\xc8\x6c\x8f\x9b\x75\x15\x33\xba\x2e\x56\x2d\xeb\x9a\xcd\xed\x40",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "FunFair",
        .unit = "FUN",
        .contract_address =
            "\x41\x9d\x0d\x8b\xdd\x9a\xf5\xe6\x06\xae\x22\x32\xed\x28\x5a\xff\x19\x0e\x71\x1b",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Function X",
        .unit = "FX",
        .contract_address =
            "\x8c\x15\xef\x5b\x4b\x21\x95\x1d\x50\xe5\x3e\x4f\xbd\xa8\x29\x8f\xfa\xd2\x50\x57",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Flexacoin",
        .unit = "FXC",
        .contract_address =
            "\x4a\x57\xe6\x87\xb9\x12\x64\x35\xa9\xb1\x9e\x4a\x80\x21\x13\xe2\x66\xad\xeb\xde",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "GAPS",
        .unit = "GAP",
        .contract_address =
            "\xcd\x85\x44\xde\xfe\xde\xc7\xc6\xb6\x0b\x5a\x42\x32\x32\x03\x65\xb1\xb2\x1f\xcc",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hashgard",
        .unit = "GARD",
        .contract_address =
            "\x5c\x64\x03\x1c\x62\x06\x18\x65\xe5\xfd\x0f\x53\xd3\xcd\xae\xf8\x0f\x72\xe9\x9d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Gold Bits Coin",
        .unit = "GBC",
        .contract_address =
            "\xc8\x05\x8d\x59\xe2\x08\x39\x9b\x76\xe6\x6d\xa1\xec\x66\x9d\xd6\xb1\xbe\xe2\xea",
        .decimals = 10,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Global Digital Content",
        .unit = "GDC",
        .contract_address =
            "\x30\x1c\x75\x5b\xa0\xfc\xa0\x0b\x19\x23\x76\x8f\xff\xb3\xdf\x7f\x4e\x63\xaf\x31",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DAOstack",
        .unit = "GEN",
        .contract_address =
            "\x54\x3f\xf2\x27\xf6\x4a\xa1\x7e\xa1\x32\xbf\x98\x86\xca\xb5\xdb\x55\xdc\xad\xdf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "GET Protocol",
        .unit = "GET",
        .contract_address =
            "\x8a\x85\x42\x88\xa5\x97\x60\x36\xa7\x25\x87\x91\x64\xca\x3e\x91\xd3\x0c\x6a\x1b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "GoWithMi",
        .unit = "GMAT",
        .contract_address =
            "\xa1\x10\xee\xeb\xc0\x75\x14\x07\xbd\xca\xea\x4c\xd2\x30\xf0\x4a\x2b\x82\xa3\x3a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "GMB",
        .unit = "GMB",
        .contract_address =
            "\x1d\x46\x4a\xc5\xe0\x46\xe5\xfe\x28\x0c\x95\x88\xed\xf8\xeb\x68\x1b\x07\x00\x8f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Gnosis",
        .unit = "GNO",
        .contract_address =
            "\x68\x10\xe7\x76\x88\x0c\x02\x93\x3d\x47\xdb\x1b\x9f\xc0\x59\x08\xe5\x38\x6b\x96",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Golem",
        .unit = "GNT",
        .contract_address =
            "\xa7\x44\x76\x44\x31\x19\xa9\x42\xde\x49\x85\x90\xfe\x1f\x24\x54\xd7\xd4\xac\x0d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Genaro Network",
        .unit = "GNX",
        .contract_address =
            "\x6e\xc8\xa2\x4c\xab\xdc\x33\x9a\x06\xa1\x72\xf8\x22\x3e\xa5\x57\x05\x5a\xda\xa5",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "GNY",
        .unit = "GNY",
        .contract_address =
            "\x24\x75\x51\xf2\xeb\x33\x62\xe2\x22\xc7\x42\xe9\xc7\x88\xb8\x95\x7d\x9b\xc8\x7e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ParkinGo",
        .unit = "GOT",
        .contract_address =
            "\x61\x3f\xa2\xa6\xe6\xda\xa7\x0c\x65\x90\x60\xe8\x6b\xa1\x44\x3d\x26\x79\xc9\xd7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Grid+",
        .unit = "GRID",
        .contract_address =
            "\x12\xb1\x9d\x3e\x2c\xcc\x14\xda\x04\xfa\xe3\x3e\x63\x65\x2c\xe4\x69\xb3\xf2\xfd",
        .decimals = 12,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Global Social Chain",
        .unit = "GSC",
        .contract_address =
            "\x22\x8b\xa5\x14\x30\x9f\xfd\xf0\x3a\x81\xa2\x05\xa6\xd0\x40\xe4\x29\xd6\xe8\x0c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Gatechain Token",
        .unit = "GT",
        .contract_address =
            "\xe6\x67\x47\xa1\x01\xbf\xf2\xdb\xa3\x69\x71\x99\xdc\xce\x5b\x74\x3b\x45\x47\x59",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Game.com",
        .unit = "GTC",
        .contract_address =
            "\xb7\x08\x35\xd7\x82\x2e\xbb\x94\x26\xb5\x65\x43\xe3\x91\x84\x6c\x10\x7b\xd3\x2c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Gifto",
        .unit = "GTO",
        .contract_address =
            "\xc5\xbb\xae\x50\x78\x1b\xe1\x66\x93\x06\xb9\xe0\x01\xef\xf5\x7a\x29\x57\xb0\x9d",
        .decimals = 5,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Gemini Dollar",
        .unit = "GUSD",
        .contract_address =
            "\x05\x6f\xd4\x09\xe1\xd7\xa1\x24\xbd\x70\x17\x45\x9d\xfe\xa2\xf3\x87\xb6\xd5\xcd",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Genesis Vision",
        .unit = "GVT",
        .contract_address =
            "\x10\x3c\x3a\x20\x9d\xa5\x9d\x3e\x7c\x4a\x89\x30\x7e\x66\x52\x1e\x08\x1c\xfd\xf0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hubii Network",
        .unit = "HBT",
        .contract_address =
            "\xdd\x6c\x68\xbb\x32\x46\x2e\x01\x70\x50\x11\xa4\xe2\xad\x1a\x60\x74\x0f\x21\x7f",
        .decimals = 15,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "HedgeTrade",
        .unit = "HEDG",
        .contract_address =
            "\xf1\x29\x04\x73\xe2\x10\xb2\x10\x8a\x85\x23\x7f\xbc\xd7\xb6\xeb\x42\xcc\x65\x4f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Esportbits",
        .unit = "HLT",
        .contract_address =
            "\xa8\x09\xd3\x63\xa6\x6c\x57\x6a\x2a\x81\x4c\xdb\xfe\xfc\x10\x7c\x60\x0a\x55\xf0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hi Mutual Society",
        .unit = "HMC",
        .contract_address =
            "\xaa\x0b\xb1\x0c\xec\x1f\xa3\x72\xeb\x3a\xbc\x17\xc9\x33\xfc\x6b\xa8\x63\xdd\x9e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Humaniq",
        .unit = "HMQ",
        .contract_address =
            "\xcb\xcc\x0f\x03\x6e\xd4\x78\x8f\x63\xfc\x0f\xee\x32\x87\x3d\x6a\x74\x87\xb9\x08",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Holo",
        .unit = "HOT",
        .contract_address =
            "\x6c\x6e\xe5\xe3\x1d\x82\x8d\xe2\x41\x28\x2b\x96\x06\xc8\xe9\x8e\xa4\x85\x26\xe2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hydro Protocol",
        .unit = "HOT",
        .contract_address =
            "\x9a\xf8\x39\x68\x7f\x6c\x94\x54\x2a\xc5\xec\xe2\xe3\x17\xda\xae\x35\x54\x93\xa1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Huobi Pool Token",
        .unit = "HPT",
        .contract_address =
            "\xa6\x6d\xaa\x57\x43\x20\x24\x02\x3d\xb6\x54\x77\xba\x87\xd4\xe7\xf5\xf9\x52\x13",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Huobi Token",
        .unit = "HT",
        .contract_address =
            "\x6f\x25\x96\x37\xdc\xd7\x4c\x76\x77\x81\xe3\x7b\xc6\x13\x3c\xd6\xa6\x8a\xa1\x61",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Humanscape",
        .unit = "HUM",
        .contract_address =
            "\x17\x4a\xfe\x7a\x03\x2b\x5a\x33\xa3\x27\x0a\x9f\x6c\x30\x74\x6e\x25\x70\x85\x32",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hxro",
        .unit = "HXRO",
        .contract_address =
            "\x4b\xd7\x05\x56\xae\x3f\x8a\x6e\xc6\xc4\x08\x0a\x0c\x32\x7b\x24\x32\x54\x38\xf3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hydro",
        .unit = "HYDRO",
        .contract_address =
            "\xeb\xbd\xf3\x02\xc9\x40\xc6\xbf\xd4\x9c\x6b\x16\x5f\x45\x7f\xdb\x32\x46\x49\xbc",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Hyperion",
        .unit = "HYN",
        .contract_address =
            "\xe9\x9a\x89\x4a\x69\xd7\xc2\xe3\xc9\x2e\x61\xb6\x4c\x50\x5a\x6a\x57\xd2\xbc\x07",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IDEX",
        .unit = "IDEX",
        .contract_address =
            "\xb7\x05\x26\x82\x13\xd5\x93\xb8\xfd\x88\xd3\xfd\xef\xf9\x3a\xff\x5c\xbd\xcf\xae",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "indaHash",
        .unit = "IDH",
        .contract_address =
            "\x51\x36\xc9\x8a\x80\x81\x1c\x3f\x46\xbd\xda\x8b\x5c\x45\x55\xcf\xd9\xf8\x12\xf0",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Invictus Hyperion Fund",
        .unit = "IHF",
        .contract_address =
            "\xaf\x12\x50\xfa\x68\xd7\xde\xcd\x34\xfd\x75\xde\x87\x42\xbc\x03\xb2\x9b\xd5\x8e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IHT Real Estate Protocol",
        .unit = "IHT",
        .contract_address =
            "\xed\xa8\xb0\x16\xef\xa8\xb1\x16\x12\x08\xcf\x04\x1c\xd8\x69\x72\xee\xe0\xf3\x1e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "INLOCK",
        .unit = "ILK",
        .contract_address =
            "\xf7\x84\x68\x2c\x82\x52\x6e\x24\x5f\x50\x97\x51\x90\xef\x0f\xff\x4e\x4f\xc0\x77",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Insight Chain",
        .unit = "INB",
        .contract_address =
            "\x17\xaa\x18\xa4\xb6\x4a\x55\xab\xed\x7f\xa5\x43\xf2\xba\x4e\x91\xf2\xdc\xe4\x82",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IntelliShare",
        .unit = "INE",
        .contract_address =
            "\x86\xe6\xa4\xf5\x12\xb1\x29\x0c\x04\x39\x70\xb0\x4e\x0b\x57\x0d\x4f\xc9\x82\x91",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "INO COIN",
        .unit = "INO",
        .contract_address =
            "\xc9\x85\x9f\xcc\xc8\x76\xe6\xb4\xb3\xc7\x49\xc5\xd2\x9e\xa0\x4f\x48\xac\xb7\x4f",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Insolar",
        .unit = "INS",
        .contract_address =
            "\x5b\x2e\x4a\x70\x0d\xfb\xc5\x60\x06\x1e\x95\x7e\xde\xc8\xf6\xee\xeb\x74\xa3\x20",
        .decimals = 10,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Insights Network",
        .unit = "INSTAR",
        .contract_address =
            "\xc7\x2f\xe8\xe3\xdd\x5b\xef\x0f\x9f\x31\xf2\x59\x39\x9f\x30\x12\x72\xef\x2a\x2d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IoTeX",
        .unit = "IOTX",
        .contract_address =
            "\x6f\xb3\xe0\xa2\x17\x40\x7e\xff\xf7\xca\x06\x2d\x46\xc2\x6e\x5d\x60\xa1\x4d\x69",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IQeon",
        .unit = "IQN",
        .contract_address =
            "\x0d\xb8\xd8\xb7\x6b\xc3\x61\xba\xcb\xb7\x2e\x2c\x49\x1e\x06\x08\x5a\x97\xab\x31",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "IoT Chain",
        .unit = "ITC",
        .contract_address =
            "\x5e\x6b\x6d\x9a\xba\xd9\x09\x3f\xdc\x86\x1e\xa1\x60\x0e\xba\x1b\x35\x5c\xd9\x40",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ivy",
        .unit = "IVY",
        .contract_address =
            "\xa4\xea\x68\x7a\x2a\x7f\x29\xcf\x2d\xc6\x6b\x39\xc6\x8e\x44\x11\xc0\xd0\x0c\x49",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Jibrel Network",
        .unit = "JNT",
        .contract_address =
            "\xa5\xfd\x1a\x79\x1c\x4d\xfc\xaa\xcc\x96\x3d\x4f\x73\xc6\xae\x58\x24\x14\x9e\xa7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Jewel",
        .unit = "JWL",
        .contract_address =
            "\x82\x75\xeb\xf5\x21\xdc\x21\x7a\xa7\x9c\x88\x13\x20\x17\xa5\xbc\xef\x00\x1d\xd9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BitKan",
        .unit = "KAN",
        .contract_address =
            "\x14\x10\x43\x4b\x03\x46\xf5\xbe\x67\x8d\x0f\xb5\x54\xe5\xc7\xab\x62\x0f\x8f\x4a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Karatgold Coin",
        .unit = "KBC",
        .contract_address =
            "\xf3\x58\x66\x84\x10\x7c\xe0\x85\x9c\x44\xaa\x2b\x2e\x0f\xb8\xcd\x87\x31\xa1\x5a",
        .decimals = 7,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Kcash",
        .unit = "KCASH",
        .contract_address =
            "\x32\xd7\x48\x96\xf0\x52\x04\xd1\xb6\xae\x7b\x0a\x3c\xeb\xd7\xfc\x0c\xd8\xf9\xc7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "KuCoin Shares",
        .unit = "KCS",
        .contract_address =
            "\x03\x9b\x56\x49\xa5\x99\x67\xe3\xe9\x36\xd7\x47\x1f\x9c\x37\x00\x10\x0e\xe1\xab",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Selfkey",
        .unit = "KEY",
        .contract_address =
            "\x4c\xc1\x93\x56\xf2\xd3\x73\x38\xb9\x80\x2a\xa8\xe8\xfc\x58\xb0\x37\x32\x96\xe7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "KickToken",
        .unit = "KICK",
        .contract_address =
            "\xc1\x2d\x1c\x73\xee\x7d\xc3\x61\x5b\xa4\xe3\x7e\x4a\xbf\xdb\xdd\xfa\x38\x90\x7e",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Kyber Network",
        .unit = "KNC",
        .contract_address =
            "\xdd\x97\x4d\x5c\x2e\x29\x28\xde\xa5\xf7\x1b\x98\x25\xb8\xb6\x46\x68\x6b\xd2\x00",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Krios",
        .unit = "KRI",
        .contract_address =
            "\xde\x12\x89\xe6\x8a\xd9\xe6\x5c\xcf\x50\xd8\x00\xc0\xce\xc2\xd5\x14\xb8\x0a\x40",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Kryll",
        .unit = "KRL",
        .contract_address =
            "\x46\x4e\xbe\x77\xc2\x93\xe4\x73\xb4\x8c\xfe\x96\xdd\xcf\x88\xfc\xf7\xbf\xda\xc0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Kuai Token",
        .unit = "KT",
        .contract_address =
            "\x26\xdd\xf6\xca\xba\xdc\xbf\x4f\x01\x38\x41\xbd\x8d\x91\x48\x30\xbe\xb0\xd9\x84",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LATOKEN",
        .unit = "LA",
        .contract_address =
            "\xe5\x03\x65\xf5\xd6\x79\xcb\x98\xa1\xdd\x62\xd6\xf6\xe5\x8e\x59\x32\x1b\xcd\xdf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Lambda",
        .unit = "LAMB",
        .contract_address =
            "\x89\x71\xf9\xfd\x71\x96\xe5\xce\xe2\xc1\x03\x2b\x50\xf6\x56\x85\x5a\xf7\xdd\x26",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Cred",
        .unit = "LBA",
        .contract_address =
            "\xfe\x5f\x14\x1b\xf9\x4f\xe8\x4b\xc2\x8d\xed\x0a\xb9\x66\xc1\x6b\x17\x49\x06\x57",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Aave",
        .unit = "LEND",
        .contract_address =
            "\x80\xfb\x78\x4b\x7e\xd6\x67\x30\xe8\xb1\xdb\xd9\x82\x0a\xfd\x29\x93\x1a\xab\x03",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "UNUS SED LEO",
        .unit = "LEO",
        .contract_address =
            "\x2a\xf5\xd2\xad\x76\x74\x11\x91\xd1\x5d\xfe\x7b\xf6\xac\x92\xd4\xbd\x91\x2c\xa3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LinkEye",
        .unit = "LET",
        .contract_address =
            "\xfa\x31\x18\xb3\x45\x22\x58\x0c\x35\xae\x27\xf6\xcf\x52\xda\x1d\xbb\x75\x62\x88",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Leverj",
        .unit = "LEV",
        .contract_address =
            "\x0f\x4c\xa9\x26\x60\xef\xad\x97\xa9\xa7\x0c\xb0\xfe\x96\x9c\x75\x54\x39\x77\x2c",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Levolution",
        .unit = "LEVL",
        .contract_address =
            "\x09\x97\x0a\xec\x76\x6b\x6f\x32\x23\xac\xa9\x11\x15\x55\xe9\x9d\xc5\x0f\xf1\x3a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Winding Tree",
        .unit = "LIF",
        .contract_address =
            "\xeb\x99\x51\x02\x16\x98\xb4\x2e\x43\x99\xf9\xcb\xb6\x26\x7a\xa3\x5f\x82\xd5\x9d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LIFE",
        .unit = "LIFE",
        .contract_address =
            "\xff\x18\xdb\xc4\x87\xb4\xc2\xe3\x22\x2d\x11\x59\x52\xba\xbf\xda\x8b\xa5\x2f\x5f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LikeCoin",
        .unit = "LIKE",
        .contract_address =
            "\x02\xf6\x1f\xd2\x66\xda\x6e\x8b\x10\x2d\x41\x21\xf5\xce\x7b\x99\x26\x40\xcf\x98",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LINA",
        .unit = "LINA",
        .contract_address =
            "\xc0\x5d\x14\x44\x2a\x51\x0d\xe4\xd3\xd7\x1a\x3d\x31\x65\x85\xaa\x0c\xe3\x2b\x50",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Chainlink",
        .unit = "LINK",
        .contract_address =
            "\x51\x49\x10\x77\x1a\xf9\xca\x65\x6a\xf8\x40\xdf\xf8\x3e\x82\x64\xec\xf9\x86\xca",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LINKA",
        .unit = "LINKA",
        .contract_address =
            "\x57\x8b\x49\xc4\x59\x61\xf9\x8d\x8d\xf9\x28\x54\xb5\x3f\x16\x41\xaf\x0a\x50\x36",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Linkey",
        .unit = "LKY",
        .contract_address =
            "\x49\xbd\x2d\xa7\x5b\x1f\x7a\xf1\xe4\xdf\xd6\xb1\x12\x5f\xec\xde\x59\xdb\xec\x58",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Lendingblock",
        .unit = "LND",
        .contract_address =
            "\x09\x47\xb0\xe6\xd8\x21\x37\x88\x05\xc9\x59\x82\x91\x38\x5c\xe7\xc7\x91\xa6\xb2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LockTrip",
        .unit = "LOC",
        .contract_address =
            "\x5e\x33\x46\x44\x40\x10\x13\x53\x22\x26\x8a\x46\x30\xd2\xed\x5f\x8d\x09\x44\x6c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Locus Chain",
        .unit = "LOCUS",
        .contract_address =
            "\xc6\x45\x00\xdd\x7b\x0f\x17\x94\x80\x7e\x67\x80\x2f\x8a\xbb\xf5\xf8\xff\xb0\x54",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Loom Network",
        .unit = "LOOM",
        .contract_address =
            "\xa4\xe8\xc3\xec\x45\x61\x07\xea\x67\xd3\x07\x5b\xf9\xe3\xdf\x3a\x75\x82\x3d\xb0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Loopring",
        .unit = "LRC",
        .contract_address =
            "\xbb\xbb\xca\x6a\x90\x1c\x92\x6f\x24\x0b\x89\xea\xcb\x64\x1d\x8a\xec\x7a\xea\xfd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LuckySevenToken",
        .unit = "LST",
        .contract_address =
            "\x6b\x9f\x1f\x09\x2e\x0b\x10\x01\x5a\x43\x91\xa8\x0c\xd3\xe6\xb6\xce\xfd\x17\x28",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "LTO Network",
        .unit = "LTO",
        .contract_address =
            "\x3d\xb6\xba\x6a\xb6\xf9\x5e\xfe\xd1\xa6\xe7\x94\xca\xd4\x92\xfa\xaa\xbf\x29\x4d",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Lunyr",
        .unit = "LUN",
        .contract_address =
            "\xfa\x05\xa7\x3f\xfe\x78\xef\x8f\x1a\x73\x94\x73\xe4\x62\xc5\x4b\xae\x65\x67\xd9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Litex",
        .unit = "LXT",
        .contract_address =
            "\xbc\x46\xd9\x96\x1a\x39\x32\xf7\xd6\xb6\x4a\xbf\xde\xc8\x0c\x18\x16\xc4\xb8\x35",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Lympo",
        .unit = "LYM",
        .contract_address =
            "\xc6\x90\xf7\xc7\xfc\xff\xa6\xa8\x2b\x79\xfa\xb7\x50\x8c\x46\x6f\xef\xdf\xc8\xc5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Decentraland",
        .unit = "MANA",
        .contract_address =
            "\x0f\x5d\x2f\xb2\x9f\xb7\xd3\xcf\xee\x44\x4a\x20\x02\x98\xf4\x68\x90\x8c\xc9\x42",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Matic Network",
        .unit = "MATIC",
        .contract_address =
            "\x7d\x1a\xfa\x7b\x71\x8f\xb8\x93\xdb\x30\xa3\xab\xc0\xcf\xc6\x08\xaa\xcf\xeb\xb0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MineBee",
        .unit = "MB",
        .contract_address =
            "\x8d\x81\x29\x96\x32\x91\x74\x0d\xdd\xd9\x17\xab\x01\xaf\x18\xc7\xae\xd4\xba\x58",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MovieBloc",
        .unit = "MBL",
        .contract_address =
            "\xb8\x79\xda\x8b\x24\xc9\xb8\x68\x5d\xe8\x52\x6c\xf4\x92\xe9\x54\xf1\x65\xd7\x4b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MCO",
        .unit = "MCO",
        .contract_address =
            "\xb6\x3b\x60\x6a\xc8\x10\xa5\x2c\xca\x15\xe4\x4b\xb6\x30\xfd\x42\xd8\xd1\xd8\x3d",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Moeda Loyalty Points",
        .unit = "MDA",
        .contract_address =
            "\x51\xdb\x5a\xd3\x5c\x67\x1a\x87\x20\x7d\x88\xfc\x11\xd5\x93\xac\x0c\x84\x15\xbd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MediShares",
        .unit = "MDS",
        .contract_address =
            "\x66\x18\x60\x08\xc1\x05\x06\x27\xf9\x79\xd4\x64\xea\xbb\x25\x88\x60\x56\x3d\xbe",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Measurable Data Token",
        .unit = "MDT",
        .contract_address =
            "\x81\x4e\x09\x08\xb1\x2a\x99\xfe\xcf\x5b\xc1\x01\xbb\x5d\x0b\x8b\x5c\xdf\x7d\x26",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MediBloc [ERC20]",
        .unit = "MEDX",
        .contract_address =
            "\xfd\x1e\x80\x50\x8f\x24\x3e\x64\xce\x23\x4e\xa8\x8a\x5f\xd2\x82\x7c\x71\xd4\xb7",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Metronome",
        .unit = "MET",
        .contract_address =
            "\xa3\xd5\x8c\x4e\x56\xfe\xdc\xae\x3a\x7c\x43\xa7\x25\xae\xe9\xa7\x1f\x0e\xce\x4e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MEXC Token",
        .unit = "MEXC",
        .contract_address =
            "\x7d\xe2\xd1\x23\x04\x29\x94\x73\x71\x05\x80\x2d\x2a\xbd\x0a\x10\xa7\xbd\xe2\x76",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Mainframe",
        .unit = "MFT",
        .contract_address =
            "\xdf\x2c\x72\x38\x19\x8a\xd8\xb3\x89\x66\x65\x74\xf2\xd8\xbc\x41\x1a\x4b\x74\x28",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MargiX",
        .unit = "MGX",
        .contract_address =
            "\x14\x12\xf6\xaa\x5a\xdc\x77\xc6\x20\x71\x5b\xb2\xa0\x20\xaa\x69\x0b\x85\xf6\x8a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MINDOL",
        .unit = "MIN",
        .contract_address =
            "\x5d\x64\xd8\x50\xc8\x36\x80\x08\xaf\xb3\x92\x24\xe9\x2a\xd0\xdc\xef\xf3\xcf\x38",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Mithril",
        .unit = "MITH",
        .contract_address =
            "\x38\x93\xb9\x42\x2c\xd5\xd7\x0a\x81\xed\xef\xfe\x3d\x5a\x1c\x6a\x97\x83\x10\xbb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Morpheus Labs",
        .unit = "MITX",
        .contract_address =
            "\x4a\x52\x7d\x8f\xc1\x3c\x52\x03\xab\x24\xba\x09\x44\xf4\xcb\x14\x65\x8d\x1d\xb6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Maker",
        .unit = "MKR",
        .contract_address =
            "\x9f\x8f\x72\xaa\x93\x04\xc8\xb5\x93\xd5\x55\xf1\x2e\xf6\x58\x9c\xc3\xa5\x79\xa2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Melon",
        .unit = "MLN",
        .contract_address =
            "\xec\x67\x00\x5c\x4e\x49\x8e\xc7\xf5\x5e\x09\x2b\xd1\xd3\x5c\xbc\x47\xc9\x18\x92",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Moss Coin",
        .unit = "MOC",
        .contract_address =
            "\x86\x5e\xc5\x8b\x06\xbf\x63\x05\xb8\x86\x79\x3a\xa2\x0a\x2d\xa3\x1d\x03\x4e\x68",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Molecular Future",
        .unit = "MOF",
        .contract_address =
            "\x65\x34\x30\x56\x0b\xe8\x43\xc4\xa3\xd1\x43\xd0\x11\x0e\x89\x6c\x2a\xb8\xac\x0d",
        .decimals = 16,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MenaPay",
        .unit = "MPAY",
        .contract_address =
            "\x38\x10\xa4\xdd\xf4\x1e\x58\x6f\xa0\xdb\xa1\x46\x3a\x79\x51\xb7\x48\xce\xcf\xca",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Morpheus.Network",
        .unit = "MRPH",
        .contract_address =
            "\x7b\x0c\x06\x04\x34\x68\x46\x99\x67\xdb\xa2\x2d\x1a\xf3\x3d\x77\xd4\x40\x56\xc8",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "doc.com Token",
        .unit = "MTC",
        .contract_address =
            "\x90\x5e\x33\x7c\x6c\x86\x45\x26\x3d\x35\x21\x20\x5a\xa3\x7b\xf4\xd0\x34\xe7\x45",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Monetha",
        .unit = "MTH",
        .contract_address =
            "\xaf\x4d\xce\x16\xda\x28\x77\xf8\xc9\xe0\x05\x44\xc9\x3b\x62\xac\x40\x63\x1f\x16",
        .decimals = 5,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Metal",
        .unit = "MTL",
        .contract_address =
            "\xf4\x33\x08\x93\x66\x89\x9d\x83\xa9\xf2\x6a\x77\x3d\x59\xec\x7e\xcf\x30\x35\x5e",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MVL",
        .unit = "MVL",
        .contract_address =
            "\xa8\x49\xea\xae\x99\x4f\xb8\x6a\xfa\x73\x38\x2e\x9b\xd8\x8c\x2b\x6b\x18\xdc\x71",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Restart Energy MWAT",
        .unit = "MWAT",
        .contract_address =
            "\x64\x25\xc6\xbe\x90\x2d\x69\x2a\xe2\xdb\x75\x2b\x3c\x26\x8a\xfa\xdb\x09\x9d\x3b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "MX Token",
        .unit = "MX",
        .contract_address =
            "\x11\xee\xf0\x4c\x88\x4e\x24\xd9\xb7\xb4\x76\x0e\x74\x76\xd0\x6d\xdf\x79\x7f\x36",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Machine Xchange Coin",
        .unit = "MXC",
        .contract_address =
            "\x5c\xa3\x81\xbb\xfb\x58\xf0\x09\x2d\xf1\x49\xbd\x3d\x24\x3b\x08\xb9\xa8\x38\x6e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Maximine Coin",
        .unit = "MXM",
        .contract_address =
            "\x8e\x76\x6f\x57\xf7\xd1\x6c\xa5\x0b\x4a\x0b\x90\xb8\x8f\x64\x68\xa0\x9b\x04\x39",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "NeoWorld Cash",
        .unit = "NASH",
        .contract_address =
            "\x4b\x94\xc8\x56\x77\x63\x65\x41\x01\xf6\x90\xcf\x4d\x54\x95\x72\x06\x38\x3b\x75",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Niobium Coin",
        .unit = "NBC",
        .contract_address =
            "\x9f\x19\x56\x17\xfa\x8f\xba\xd9\x54\x0c\x5d\x11\x3a\x99\xa0\xa0\x17\x2a\xae\xdc",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Nucleus Vision",
        .unit = "NCASH",
        .contract_address =
            "\x80\x98\x26\xcc\xea\xb6\x8c\x38\x77\x26\xaf\x96\x27\x13\xb6\x4c\xb5\xcb\x3c\xca",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PolySwarm",
        .unit = "NCT",
        .contract_address =
            "\x9e\x46\xa3\x8f\x5d\xaa\xbe\x86\x83\xe1\x07\x93\xb0\x67\x49\xee\xf7\xd7\x33\xd1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Nectar",
        .unit = "NEC",
        .contract_address =
            "\xcc\x80\xc0\x51\x05\x7b\x77\x4c\xd7\x50\x67\xdc\x48\xf8\x98\x7c\x4e\xb9\x7a\x5e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "NEXT",
        .unit = "NET",
        .contract_address =
            "\xf2\x92\x26\x91\x45\x95\x05\x2a\x04\xf5\xaf\xbe\x64\x10\xd0\xc3\xed\x70\x75\x48",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Neumark",
        .unit = "NEU",
        .contract_address =
            "\xa8\x23\xe6\x72\x20\x06\xaf\xe9\x9e\x91\xc3\x0f\xf5\x29\x50\x52\xfe\x6b\x8e\x32",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Nexo",
        .unit = "NEXO",
        .contract_address =
            "\xb6\x21\x32\xe3\x5a\x6c\x13\xee\x1e\xe0\xf8\x4d\xc5\xd4\x0b\xad\x8d\x81\x52\x06",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "NAGA",
        .unit = "NGC",
        .contract_address =
            "\x72\xdd\x4b\x6b\xd8\x52\xa3\xaa\x17\x2b\xe4\xd6\xc5\xa6\xdb\xec\x58\x8c\xf1\x31",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Numeraire",
        .unit = "NMR",
        .contract_address =
            "\x17\x76\xe1\xf2\x6f\x98\xb1\xa5\xdf\x9c\xd3\x47\x95\x3a\x26\xdd\x3c\xb4\x66\x71",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Noah Coin",
        .unit = "NOAH",
        .contract_address =
            "\x58\xa4\x88\x41\x82\xd9\xe8\x35\x59\x7f\x40\x5e\x5f\x25\x82\x90\xe4\x6a\xe7\xc2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "NaPoleonX",
        .unit = "NPX",
        .contract_address =
            "\x28\xb5\xe1\x2c\xce\x51\xf1\x55\x94\xb0\xb9\x1d\x5b\x5a\xda\xa7\x0f\x68\x4a\x02",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Pundi X",
        .unit = "NPXS",
        .contract_address =
            "\xa1\x5c\x7e\xbe\x1f\x07\xca\xf6\xbf\xf0\x97\xd8\xa5\x89\xfb\x8a\xc4\x9a\xe5\xb3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OAX",
        .unit = "OAX",
        .contract_address =
            "\x70\x1c\x24\x4b\x98\x8a\x51\x3c\x94\x59\x73\xde\xfa\x05\xde\x93\x3b\x23\xfe\x1d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ocean Protocol",
        .unit = "OCEAN",
        .contract_address =
            "\x98\x5d\xd3\xd4\x2d\xe1\xe2\x56\xd0\x9e\x1c\x10\xf1\x12\xbc\xcb\x80\x15\xad\x41",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Odyssey",
        .unit = "OCN",
        .contract_address =
            "\x40\x92\x67\x8e\x4e\x78\x23\x0f\x46\xa1\x53\x4c\x0f\xbc\x8f\xa3\x97\x80\x89\x2b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ODEM",
        .unit = "ODE",
        .contract_address =
            "\xbf\x52\xf2\xab\x39\xe2\x6e\x09\x51\xd2\xa0\x2b\x49\xb7\x70\x2a\xbe\x30\x40\x6a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Origo",
        .unit = "OGO",
        .contract_address =
            "\xff\x0e\x5e\x01\x4c\xf9\x7e\x06\x15\xcb\x50\xf6\xf3\x9d\xa6\x38\x8e\x2f\xae\x6e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OKB",
        .unit = "OKB",
        .contract_address =
            "\x75\x23\x1f\x58\xb4\x32\x40\xc9\x71\x8d\xd5\x8b\x49\x67\xc5\x11\x43\x42\xa8\x6c",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OneLedger",
        .unit = "OLT",
        .contract_address =
            "\x64\xa6\x04\x93\xd8\x88\x72\x8c\xf4\x26\x16\xe0\x34\xa0\xdf\xea\xe3\x8e\xfc\xf0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OmiseGO",
        .unit = "OMG",
        .contract_address =
            "\xd2\x61\x14\xcd\x6e\xe2\x89\xac\xcf\x82\x35\x0c\x8d\x84\x87\xfe\xdb\x8a\x0c\x07",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BigONE Token",
        .unit = "ONE",
        .contract_address =
            "\x03\x96\x34\x0f\x16\xbb\xec\x97\x32\x80\xab\x05\x3e\xfc\x3f\x20\x8f\xa3\x77\x95",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Opacity",
        .unit = "OPQ",
        .contract_address =
            "\x77\x59\x9d\x2c\x6d\xb1\x70\x22\x42\x43\xe2\x55\xe6\x66\x92\x80\xf1\x1f\x14\x73",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Orbs",
        .unit = "ORBS",
        .contract_address =
            "\xff\x56\xcc\x6b\x1e\x6d\xed\x34\x7a\xa0\xb7\x67\x6c\x85\xab\x0b\x3d\x08\xb0\xfa",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Origin Sport",
        .unit = "ORS",
        .contract_address =
            "\xeb\x9a\x4b\x18\x58\x16\xc3\x54\xdb\x92\xdb\x09\xcc\x3b\x50\xbe\x60\xb9\x01\xb6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OST",
        .unit = "OST",
        .contract_address =
            "\x2c\x4e\x8f\x2d\x74\x61\x13\xd0\x69\x6c\xe8\x9b\x35\xf0\xd8\xbf\x88\xe0\xae\xca",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OVCODE",
        .unit = "OVC",
        .contract_address =
            "\x49\xd0\x9c\xda\x1d\xeb\x8a\x16\x80\xf1\x27\x0c\x5e\xd1\x52\x18\xfc\x4b\x18\xf0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Paxos Standard",
        .unit = "PAX",
        .contract_address =
            "\x8e\x87\x0d\x67\xf6\x60\xd9\x5d\x5b\xe5\x30\x38\x0d\x0e\xc0\xbd\x38\x82\x89\xe1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PAX Gold",
        .unit = "PAXG",
        .contract_address =
            "\x45\x80\x48\x80\xde\x22\x91\x3d\xaf\xe0\x9f\x49\x80\x84\x8e\xce\x6e\xcb\xaf\x78",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TenX",
        .unit = "PAY",
        .contract_address =
            "\xb9\x70\x48\x62\x8d\xb6\xb6\x61\xd4\xc2\xaa\x83\x3e\x95\xdb\xe1\xa9\x05\xb2\x80",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Paypex",
        .unit = "PAYX",
        .contract_address =
            "\x62\xa5\x6a\x4a\x2e\xf4\xd3\x55\xd3\x4d\x10\xfb\xf8\x37\xe7\x47\x50\x4d\x38\xd4",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Peculium",
        .unit = "PCL",
        .contract_address =
            "\x0f\x02\xe2\x77\x45\xe3\xb6\xe9\xe1\x31\x0d\x19\x46\x9e\x2b\x5d\x7b\x5e\xc9\x9a",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Perlin",
        .unit = "PERL",
        .contract_address =
            "\xb5\xa7\x3f\x5f\xc8\xbb\xdb\xce\x59\xbf\xd0\x1c\xa8\xd3\x50\x62\xe0\xda\xd8\x01",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PCHAIN",
        .unit = "PI",
        .contract_address =
            "\xb9\xbb\x08\xab\x7e\x9f\xa0\xa1\x35\x6b\xd4\xa3\x9e\xc0\xca\x26\x7e\x03\xb0\xb3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PlayChip",
        .unit = "PLA",
        .contract_address =
            "\x01\x98\xf4\x6f\x52\x0f\x33\xcd\x43\x29\xbd\x4b\xe3\x80\xa2\x5a\x90\x53\x6c\xd5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PLANET",
        .unit = "PLA",
        .contract_address =
            "\x30\x7d\x45\xaf\xbb\x7e\x84\xf8\x2e\xf3\xd2\x51\xa6\xbb\x0f\x00\xed\xf6\x32\xe4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Polybius",
        .unit = "PLBT",
        .contract_address =
            "\x0a\xff\xa0\x6e\x7f\xbe\x5b\xc9\xa7\x64\xc9\x79\xaa\x66\xe8\x25\x6a\x63\x1f\x02",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Pillar",
        .unit = "PLR",
        .contract_address =
            "\xe3\x81\x85\x04\xc1\xb3\x2b\xf1\x55\x7b\x16\xc2\x38\xb2\xe0\x1f\xd3\x14\x9c\x17",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Pluton",
        .unit = "PLU",
        .contract_address =
            "\xd8\x91\x2c\x10\x68\x1d\x8b\x21\xfd\x37\x42\x24\x4f\x44\x65\x8d\xba\x12\x26\x4e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PlayCoin [ERC20]",
        .unit = "PLY",
        .contract_address =
            "\x59\xbe\x93\x7f\x05\xcf\x2c\x40\x6b\x61\xc4\x2c\x6c\x82\xa0\x93\xfa\x54\xed\xfe",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PumaPay",
        .unit = "PMA",
        .contract_address =
            "\x84\x6c\x66\xcf\x71\xc4\x3f\x80\x40\x3b\x51\xfe\x39\x06\xb3\x59\x9d\x63\x33\x6f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Kleros",
        .unit = "PNK",
        .contract_address =
            "\x93\xed\x3f\xbe\x21\x20\x7e\xc2\xe8\xf2\xd3\xc3\xde\x6e\x05\x8c\xb7\x3b\xc0\x4d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Penta",
        .unit = "PNT",
        .contract_address =
            "\x53\x06\x6c\xdd\xbc\x00\x99\xeb\x6c\x96\x78\x5d\x9b\x3d\xf2\xaa\xee\xde\x5d\xa3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Po.et",
        .unit = "POE",
        .contract_address =
            "\x0e\x09\x89\xb1\xf9\xb8\xa3\x89\x83\xc2\xba\x80\x53\x26\x9c\xa6\x2e\xc9\xb1\x95",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Polymath",
        .unit = "POLY",
        .contract_address =
            "\x99\x92\xec\x3c\xf6\xa5\x5b\x00\x97\x8c\xdd\xf2\xb2\x7b\xc6\x88\x2d\x88\xd1\xec",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Power Ledger",
        .unit = "POWR",
        .contract_address =
            "\x59\x58\x32\xf8\xfc\x6b\xf5\x9c\x85\xc5\x27\xfe\xc3\x74\x0a\x1b\x7a\x36\x12\x69",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PayPie",
        .unit = "PPP",
        .contract_address =
            "\xc4\x22\x09\xac\xcc\x14\x02\x9c\x10\x12\xfb\x56\x80\xd9\x5f\xbd\x60\x36\xe2\xa0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Populous",
        .unit = "PPT",
        .contract_address =
            "\xd4\xfa\x14\x60\xf5\x37\xbb\x90\x85\xd2\x2c\x7b\xcc\xb5\xdd\x45\x0e\xf2\x8e\x3a",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ProChain",
        .unit = "PRA",
        .contract_address =
            "\x90\x41\xfe\x5b\x3f\xde\xa0\xf5\xe4\xaf\xdc\x17\xe7\x51\x80\x73\x8d\x87\x7a\x01",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Presearch",
        .unit = "PRE",
        .contract_address =
            "\x88\xa3\xe4\xf3\x5d\x64\xaa\xd4\x1a\x6d\x40\x30\xac\x9a\xfe\x43\x56\xcb\x84\xfa",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Propy",
        .unit = "PRO",
        .contract_address =
            "\x22\x6b\xb5\x99\xa1\x2c\x82\x64\x76\xe3\xa7\x71\x45\x46\x97\xea\x52\xe9\xe2\x20",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Prometeus",
        .unit = "PROM",
        .contract_address =
            "\xfc\x82\xbb\x4b\xa8\x60\x45\xaf\x6f\x32\x73\x23\xa4\x6e\x80\x41\x2b\x91\xb2\x7d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "PressOne",
        .unit = "PRS",
        .contract_address =
            "\xe0\xd9\x55\x30\x82\x0a\xaf\xc5\x1b\x1d\x98\x02\x3a\xa1\xff\x00\x0b\x78\xd8\xb2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Pivot Token",
        .unit = "PVT",
        .contract_address =
            "\x78\x69\xc4\xa1\xa3\xf6\xf8\x68\x4f\xbc\xc4\x22\xa2\x1a\xd7\xab\xe3\x16\x78\x34",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "QASH",
        .unit = "QASH",
        .contract_address =
            "\x61\x8e\x75\xac\x90\xb1\x2c\x60\x49\xba\x3b\x27\xf5\xd5\xf8\x65\x1b\x00\x37\xf6",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Qubitica",
        .unit = "QBIT",
        .contract_address =
            "\x16\x02\xaf\x2c\x78\x2c\xc0\x3f\x92\x41\x99\x2e\x24\x32\x90\xfc\xcf\x73\xbb\x13",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "QuickX Protocol",
        .unit = "QCX",
        .contract_address =
            "\xf9\xe5\xaf\x7b\x42\xd3\x1d\x51\x67\x7c\x75\xbb\xbd\x37\xc1\x98\x6e\xc7\x9a\xee",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "QuarkChain",
        .unit = "QKC",
        .contract_address =
            "\xea\x26\xc4\xac\x16\xd4\xa5\xa1\x06\x82\x0b\xc8\xae\xe8\x5f\xd0\xb7\xb2\xb6\x64",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Quant",
        .unit = "QNT",
        .contract_address =
            "\x4a\x22\x0e\x60\x96\xb2\x5e\xad\xb8\x83\x58\xcb\x44\x06\x8a\x32\x48\x25\x46\x75",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Quanta Utility Token",
        .unit = "QNTU",
        .contract_address =
            "\x42\x34\xf6\x3b\x1d\x20\x2f\x6c\x01\x6c\xa3\xb6\xa0\xd4\x1d\x7d\x85\xf1\x77\x16",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Poseidon Network",
        .unit = "QQQ",
        .contract_address =
            "\x28\x22\xf6\xd1\xb2\xf4\x1f\x93\xf3\x3d\x93\x7b\xc7\xd8\x4a\x8d\xfa\x4f\x4c\x21",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Quantstamp",
        .unit = "QSP",
        .contract_address =
            "\x99\xea\x4d\xb9\xee\x77\xac\xd4\x0b\x11\x9b\xd1\xdc\x4e\x33\xe1\xc0\x70\xb8\x0d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "QunQun",
        .unit = "QUN",
        .contract_address =
            "\x26\x4d\xc2\xde\xdc\xdc\xbb\x89\x75\x61\xa5\x7c\xba\x50\x85\xca\x41\x6f\xb7\xb4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Revain",
        .unit = "R",
        .contract_address =
            "\x48\xf7\x75\xef\xbe\x4f\x5e\xce\x6e\x0d\xf2\xf7\xb5\x93\x2d\xf5\x68\x23\xb9\x90",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Rublix",
        .unit = "RBLX",
        .contract_address =
            "\xfc\x2c\x4d\x8f\x95\x00\x2c\x14\xed\x0a\x7a\xa6\x51\x02\xca\xc9\xe5\x95\x3b\x5e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ripio Credit Network",
        .unit = "RCN",
        .contract_address =
            "\xf9\x70\xb8\xe3\x6e\x23\xf7\xfc\x3f\xd7\x52\xee\xa8\x6f\x8b\xe8\xd8\x33\x75\xa6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Raiden Network Token",
        .unit = "RDN",
        .contract_address =
            "\x25\x5a\xa6\xdf\x07\x54\x0c\xb5\xd3\xd2\x97\xf0\xd0\xd4\xd8\x4c\xb5\x2b\xc8\xe6",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Remme",
        .unit = "REM",
        .contract_address =
            "\x83\x98\x4d\x61\x42\x93\x4b\xb5\x35\x79\x3a\x82\xad\xb0\xa4\x6e\xf0\xf6\x6b\x6d",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ren",
        .unit = "REN",
        .contract_address =
            "\x40\x8e\x41\x87\x6c\xcc\xdc\x0f\x92\x21\x06\x00\xef\x50\x37\x26\x56\x05\x2a\x38",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Augur",
        .unit = "REP",
        .contract_address =
            "\x19\x85\x36\x5e\x9f\x78\x35\x9a\x9b\x6a\xd7\x60\xe3\x24\x12\xf4\xa4\x45\xe8\x62",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Request",
        .unit = "REQ",
        .contract_address =
            "\x8f\x82\x21\xaf\xbb\x33\x99\x8d\x85\x84\xa2\xb0\x57\x49\xba\x73\xc3\x7a\x93\x8a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "RealTract",
        .unit = "RET",
        .contract_address =
            "\xd7\x39\x40\x87\xe1\xdb\xbe\x47\x7f\xe4\xf1\xcf\x37\x3b\x9a\xc9\x45\x95\x65\xff",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Refereum",
        .unit = "RFR",
        .contract_address =
            "\xd0\x92\x9d\x41\x19\x54\xc4\x74\x38\xdc\x1d\x87\x1d\xd6\x08\x1f\x5c\x5e\x14\x9c",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "RChain",
        .unit = "RHOC",
        .contract_address =
            "\x16\x82\x96\xbb\x09\xe2\x4a\x88\x80\x5c\xb9\xc3\x33\x56\x53\x6b\x98\x0d\x3f\xc5",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "iExec RLC",
        .unit = "RLC",
        .contract_address =
            "\x60\x7f\x4c\x5b\xb6\x72\x23\x0e\x86\x72\x08\x55\x32\xf7\xe9\x01\x54\x4a\x73\x75",
        .decimals = 9,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OneRoot Network",
        .unit = "RNT",
        .contract_address =
            "\xff\x60\x3f\x43\x94\x6a\x3a\x28\xdf\x5e\x6a\x73\x17\x25\x55\xd8\xc8\xb0\x23\x86",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ROOBEE",
        .unit = "ROOBEE",
        .contract_address =
            "\xa3\x1b\x17\x67\xe0\x9f\x84\x2e\xcf\xd4\xbc\x47\x1f\xe4\x4f\x83\x0e\x38\x91\xaa",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Robotina",
        .unit = "ROX",
        .contract_address =
            "\x57\x4f\x84\x10\x8a\x98\xc5\x75\x79\x4f\x75\x48\x3d\x80\x1d\x1d\x5d\xc8\x61\xa5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Rocket Pool",
        .unit = "RPL",
        .contract_address =
            "\xb4\xef\xd8\x5c\x19\x99\x9d\x84\x25\x13\x04\xbd\xa9\x9e\x90\xb9\x23\x00\xbd\x93",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Reserve Rights",
        .unit = "RSR",
        .contract_address =
            "\x87\x62\xdb\x10\x6b\x2c\x2a\x0b\xcc\xb3\xa8\x0d\x1e\xd4\x12\x73\x55\x26\x16\xe8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Rotharium",
        .unit = "RTH",
        .contract_address =
            "\x3f\xd8\xf3\x9a\x96\x2e\xfd\xa0\x49\x56\x98\x1c\x31\xab\x89\xfa\xb5\xfb\x8b\xc8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ruff",
        .unit = "RUFF",
        .contract_address =
            "\xf2\x78\xc1\xca\x96\x90\x95\xff\xdd\xde\xd0\x20\x29\x0c\xf8\xb5\xc4\x24\xac\xe2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "S4FE",
        .unit = "S4F",
        .contract_address =
            "\xae\xc7\xd1\x06\x9e\x3a\x91\x4a\x3e\xb5\x0f\x0b\xfb\x17\x96\x75\x1f\x2c\xe4\x8a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Single Collateral DAI ",
        .unit = "SAI",
        .contract_address =
            "\x89\xd2\x4a\x6b\x4c\xcb\x1b\x6f\xaa\x26\x25\xfe\x56\x2b\xdd\x9a\x23\x26\x03\x59",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SALT",
        .unit = "SALT",
        .contract_address =
            "\x41\x56\xd3\x34\x2d\x5c\x38\x5a\x87\xd2\x64\xf9\x06\x53\x73\x35\x92\x00\x05\x81",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Santiment Network Token",
        .unit = "SAN",
        .contract_address =
            "\x7c\x5a\x0c\xe9\x26\x7e\xd1\x9b\x22\xf8\xca\xe6\x53\xf1\x98\xe3\xe8\xda\xf0\x98",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Sealchain",
        .unit = "SEAL",
        .contract_address =
            "\x07\xbf\x5f\x95\x85\x1e\xf2\xb2\x99\x6f\x19\x25\x69\xe4\x06\xa6\xfe\xa2\xa9\x5a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Seele",
        .unit = "SEELE",
        .contract_address =
            "\xb1\xee\xf1\x47\x02\x8e\x9f\x48\x0d\xbc\x5c\xca\xa3\x27\x7d\x41\x7d\x1b\x85\xf0",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Sentinel",
        .unit = "SENT",
        .contract_address =
            "\xa4\x4e\x51\x37\x29\x3e\x85\x5b\x1b\x7b\xc7\xe2\xc6\xf8\xcd\x79\x6f\xfc\xb0\x37",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Skrumble Network",
        .unit = "SKM",
        .contract_address =
            "\xd9\x9b\x8a\x7f\xa4\x8e\x25\xcc\xe8\x3b\x81\x81\x22\x20\xa3\xe0\x3b\xf6\x4e\x5f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Silverway",
        .unit = "SLV",
        .contract_address =
            "\x4c\x1c\x49\x57\xd2\x2d\x8f\x37\x3a\xed\x54\xd0\x85\x3b\x09\x06\x66\xf6\xf9\xde",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SmartMesh",
        .unit = "SMT",
        .contract_address =
            "\x55\xf9\x39\x85\x43\x1f\xc9\x30\x40\x77\x68\x7a\x35\xa1\xba\x10\x3d\xc1\xe0\x81",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SunContract",
        .unit = "SNC",
        .contract_address =
            "\xf4\x13\x41\x46\xaf\x2d\x51\x1d\xd5\xea\x8c\xdb\x1c\x4a\xc8\x8c\x57\xd6\x04\x04",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Snetwork",
        .unit = "SNET",
        .contract_address =
            "\xff\x19\x13\x8b\x03\x9d\x93\x8d\xb4\x6b\xdd\xa0\x06\x7d\xc4\xba\x13\x2e\xc7\x1c",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SingularDTV",
        .unit = "SNGLS",
        .contract_address =
            "\xae\xc2\xe8\x7e\x0a\x23\x52\x66\xd9\xc5\xad\xc9\xde\xb4\xb2\xe2\x9b\x54\xd0\x09",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Sport and Leisure",
        .unit = "SNL",
        .contract_address =
            "\xa8\x06\xb3\xfe\xd6\x89\x11\x36\x94\x0c\xf8\x1c\x40\x85\x66\x15\x00\xaa\x27\x09",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SONM",
        .unit = "SNM",
        .contract_address =
            "\x98\x3f\x6d\x60\xdb\x79\xea\x8c\xa4\xeb\x99\x68\xc6\xaf\xf8\xcf\xa0\x4b\x3c\x63",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Status",
        .unit = "SNT",
        .contract_address =
            "\x74\x4d\x70\xfd\xbe\x2b\xa4\xcf\x95\x13\x16\x26\x61\x4a\x17\x63\xdf\x80\x5b\x9e",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Sentivate",
        .unit = "SNTVT",
        .contract_address =
            "\x78\x65\xaf\x71\xcf\x0b\x28\x8b\x4e\x7f\x65\x4f\x4f\x78\x51\xeb\x46\xa2\xb7\xf8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Synthetix Network Token",
        .unit = "SNX",
        .contract_address =
            "\xc0\x11\xa7\x24\x00\xe5\x8e\xcd\x99\xee\x49\x7c\xf8\x9e\x37\x75\xd4\xbd\x73\x2f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "All Sports",
        .unit = "SOC",
        .contract_address =
            "\x2d\x0e\x95\xbd\x47\x95\xd7\xac\xe0\xda\x3c\x0f\xf7\xb7\x06\xa5\x97\x0e\xb9\xd3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SOLVE",
        .unit = "SOLVE",
        .contract_address =
            "\x44\x6c\x90\x33\xe7\x51\x6d\x82\x0c\xc9\xa2\xce\x2d\x0b\x73\x28\xb5\x79\x40\x6f",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SIRIN LABS Token",
        .unit = "SRN",
        .contract_address =
            "\x68\xd5\x7c\x9a\x1c\x35\xf6\x3e\x2c\x83\xee\x8e\x49\xa6\x4e\x9d\x70\x52\x8d\x25",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "STACS",
        .unit = "STACS",
        .contract_address =
            "\x28\x67\x08\xf0\x69\x22\x59\x05\x19\x46\x73\x75\x5f\x12\x35\x9e\x6a\xff\x6f\xe1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Storj",
        .unit = "STORJ",
        .contract_address =
            "\xb6\x4e\xf5\x1c\x88\x89\x72\xc9\x08\xcf\xac\xf5\x9b\x47\xc1\xaf\xbc\x0a\xb8\xac",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Storm",
        .unit = "STORM",
        .contract_address =
            "\xd0\xa4\xb8\x94\x6c\xb5\x2f\x06\x61\x27\x3b\xfb\xc6\xfd\x0e\x0c\x75\xfc\x64\x33",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "STPT",
        .unit = "STPT",
        .contract_address =
            "\xde\x7d\x85\x15\x7d\x97\x14\xea\xdf\x59\x50\x45\xcc\x12\xca\x4a\x5f\x3e\x2a\xdb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Substratum",
        .unit = "SUB",
        .contract_address =
            "\x8d\x75\x95\x9f\x1e\x61\xec\x25\x71\xaa\x72\x79\x82\x37\x10\x1f\x08\x4d\xe6\x3a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "sUSD",
        .unit = "SUSD",
        .contract_address =
            "\x57\xab\x1e\x02\xfe\xe2\x37\x74\x58\x0c\x11\x97\x40\x12\x9e\xac\x70\x81\xe9\xd3",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Swace",
        .unit = "SWACE",
        .contract_address =
            "\x03\xb1\x55\xaf\x3f\x44\x59\x19\x3a\x27\x63\x95\xdd\x76\xe3\x57\xbb\x47\x2d\xa1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "SwftCoin",
        .unit = "SWFTC",
        .contract_address =
            "\x0b\xb2\x17\xe4\x0f\x8a\x5c\xb7\x9a\xdf\x04\xe1\xaa\xb6\x0e\x5a\xbd\x0d\xfc\x1e",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Swarm",
        .unit = "SWM",
        .contract_address =
            "\x35\x05\xf4\x94\xc3\xf0\xfe\xd0\xb5\x94\xe0\x1f\xa4\x1d\xd3\x96\x76\x45\xca\x39",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Spectre.ai Dividend Token",
        .unit = "SXDT",
        .contract_address =
            "\x12\xb3\x06\xfa\x98\xf4\xcb\xb8\xd4\x45\x7f\xdf\xf3\xa0\xa0\xa5\x6f\x07\xcc\xdf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Swipe",
        .unit = "SXP",
        .contract_address =
            "\x8c\xe9\x13\x7d\x39\x32\x6a\xd0\xcd\x64\x91\xfb\x5c\xc0\xcb\xa0\xe0\x89\xb6\xa9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Spectre.ai Utility Token",
        .unit = "SXUT",
        .contract_address =
            "\x2c\x82\xc7\x3d\x5b\x34\xaa\x01\x59\x89\x46\x2b\x29\x48\xcd\x61\x6a\x37\x64\x1f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TaaS",
        .unit = "TAAS",
        .contract_address =
            "\xe7\x77\x5a\x6e\x9b\xcf\x90\x4e\xb3\x9d\xa2\xb6\x8c\x5e\xfb\x4f\x93\x60\xe0\x8c",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Traceability Chain",
        .unit = "TAC",
        .contract_address =
            "\xca\x69\x4e\xb7\x9e\xf3\x55\xea\x09\x99\x48\x5d\x21\x1e\x68\xf3\x9a\xe9\x84\x93",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TAGZ5",
        .unit = "TAGZ5",
        .contract_address =
            "\x4d\x04\x25\xe4\x7e\xe2\xd1\x6b\x94\xc0\x36\x71\x5d\xfc\xb5\x2a\x0c\xeb\xc4\xdc",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Lamden",
        .unit = "TAU",
        .contract_address =
            "\xc2\x7a\x2f\x05\xfa\x57\x7a\x83\xba\x0f\xdb\x4c\x38\x44\x3c\x07\x18\x35\x65\x01",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TokenClub",
        .unit = "TCT",
        .contract_address =
            "\x48\x24\xa7\xb6\x4e\x39\x66\xb0\x13\x3f\x4f\x4f\xfb\x1b\x9d\x6b\xeb\x75\xff\xf7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Telcoin",
        .unit = "TEL",
        .contract_address =
            "\x85\xe0\x76\x36\x1c\xc8\x13\xa9\x08\xff\x67\x2f\x9b\xad\x15\x41\x47\x44\x02\xb2",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TEMCO",
        .unit = "TEMCO",
        .contract_address =
            "\x2f\xc2\x46\xaa\x66\xf0\xda\x5b\xb1\x36\x8f\x68\x85\x48\xec\xbb\xe9\xbd\xee\x5d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tokenomy",
        .unit = "TEN",
        .contract_address =
            "\xdd\x16\xec\x0f\x66\xe5\x4d\x45\x3e\x67\x56\x71\x3e\x53\x33\x55\x98\x90\x40\xe4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TE-FOOD",
        .unit = "TFD",
        .contract_address =
            "\xe5\xf1\x66\xc0\xd8\x87\x2b\x68\x79\x00\x61\x31\x7b\xb6\xcc\xa0\x45\x82\xc9\x12",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TrueFlip",
        .unit = "TFL",
        .contract_address =
            "\xa7\xf9\x76\xc3\x60\xeb\xbe\xd4\x46\x5c\x28\x55\x68\x4d\x1a\xae\x52\x71\xef\xa9",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ThoreCoin",
        .unit = "THR",
        .contract_address =
            "\x1c\xb3\x20\x9d\x45\xb2\xa6\x0b\x7f\xbc\xa1\xcc\xdb\xf8\x7f\x67\x42\x37\xa4\xaa",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ThoreNext",
        .unit = "THX",
        .contract_address =
            "\xf0\x8c\x68\xbd\x5f\x41\x94\xd9\x94\xfd\x70\x72\x67\x46\xbf\x52\x9e\xe5\xa6\x17",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Trade Token X",
        .unit = "TIOX",
        .contract_address =
            "\xd9\x47\xb0\xce\xab\x2a\x88\x85\x86\x6b\x9a\x04\xa0\x6a\xe9\x9d\xe8\x52\xa3\xd4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Monolith",
        .unit = "TKN",
        .contract_address =
            "\xaa\xaf\x91\xd9\xb9\x0d\xf8\x00\xdf\x4f\x55\xc2\x05\xfd\x69\x89\xc9\x77\xe7\x3a",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Time New Bank",
        .unit = "TNB",
        .contract_address =
            "\xf7\x92\x0b\x07\x68\xec\xb2\x0a\x12\x3f\xac\x32\x31\x1d\x07\xd1\x93\x38\x1d\x6f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tierion",
        .unit = "TNT",
        .contract_address =
            "\x08\xf5\xa9\x23\x5b\x08\x17\x3b\x75\x69\xf8\x36\x45\xd2\xc7\xfb\x55\xe8\xcc\xd8",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TOP",
        .unit = "TOP",
        .contract_address =
            "\xdc\xd8\x59\x14\xb8\xae\x28\xc1\xe6\x2f\x1c\x48\x8e\x1d\x96\x8d\x5a\xaf\xfe\x2b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "T.OS",
        .unit = "TOSC",
        .contract_address =
            "\xd5\x06\x49\xaa\xb1\xd3\x9d\x68\xbc\x96\x5e\x0f\x6d\x1c\xfe\x00\x10\xe4\x90\x8b",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "OriginTrail",
        .unit = "TRAC",
        .contract_address =
            "\xaa\x7a\x9c\xa8\x7d\x36\x94\xb5\x75\x5f\x21\x3b\x5d\x04\x09\x4b\x8d\x0f\x0a\x6f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tratin",
        .unit = "TRAT",
        .contract_address =
            "\x3e\xb5\x5d\x5b\x22\xee\x0f\x9b\x03\xd5\x9b\x49\x94\xc5\xae\x7f\xe8\x11\xbe\x92",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tellor",
        .unit = "TRB",
        .contract_address =
            "\x0b\xa4\x5a\x8b\x5d\x55\x75\x93\x5b\x81\x58\xa8\x8c\x63\x1e\x9f\x9c\x95\xa2\xe5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tripio",
        .unit = "TRIO",
        .contract_address =
            "\x8b\x40\x76\x11\x42\xb9\xaa\x6d\xc8\x96\x4e\x61\xd0\x58\x59\x95\x42\x5c\x3d\x94",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TrustVerse",
        .unit = "TRV",
        .contract_address =
            "\x72\x95\x5e\xcf\xf7\x6e\x48\xf2\xc8\xab\xcc\xe1\x1d\x54\xe5\x73\x4d\x6f\x36\x57",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "TrueUSD",
        .unit = "TUSD",
        .contract_address =
            "\x00\x00\x00\x00\x00\x08\x5d\x47\x80\xb7\x31\x19\xb6\x44\xae\x5e\xcd\x22\xb3\x76",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ubex",
        .unit = "UBEX",
        .contract_address =
            "\x67\x04\xb6\x73\xc7\x0d\xe9\xbf\x74\xc8\xfb\xa4\xb4\xbd\x74\x8f\x0e\x21\x90\xe1",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Unibright",
        .unit = "UBT",
        .contract_address =
            "\x84\x00\xd9\x4a\x5c\xb0\xfa\x0d\x04\x1a\x37\x88\xe3\x95\x28\x5d\x61\xc9\xee\x5e",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "UNIVERSAL CASH",
        .unit = "UCASH",
        .contract_address =
            "\x92\xe5\x2a\x1a\x23\x5d\x9a\x10\x3d\x97\x09\x01\x06\x6c\xe9\x10\xaa\xce\xfd\x37",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "UGAS",
        .unit = "UGAS",
        .contract_address =
            "\x87\x16\xfc\x5d\xa0\x09\xd3\xa2\x08\xf0\x17\x8b\x63\x7a\x50\xf4\xef\x42\x40\x0f",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "UnlimitedIP",
        .unit = "UIP",
        .contract_address =
            "\x42\x90\x56\x3c\x2d\x7c\x25\x5b\x5e\xec\x87\xf2\xd3\xbd\x10\x38\x9f\x99\x1d\x68",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Unikoin Gold",
        .unit = "UKG",
        .contract_address =
            "\x24\x69\x27\x91\xbc\x44\x4c\x5c\xd0\xb8\x1e\x3c\xbc\xab\xa4\xb0\x4a\xcd\x1f\x3b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ultiledger",
        .unit = "ULT",
        .contract_address =
            "\xe8\x84\xcc\x27\x95\xb9\xc4\x5b\xee\xac\x06\x07\xda\x95\x39\xfd\x57\x1c\xcf\x85",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "UNI COIN",
        .unit = "UNI",
        .contract_address =
            "\xe6\x87\x7e\xa9\xc2\x8f\xbd\xec\x63\x1f\xfb\xc0\x87\x95\x6d\x00\x23\xa7\x6b\xf2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Ultra",
        .unit = "UOS",
        .contract_address =
            "\xd1\x3c\x73\x42\xe1\xef\x68\x7c\x5a\xd2\x1b\x27\xc2\xb6\x5d\x77\x2c\xab\x5c\x8c",
        .decimals = 4,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Sentinel Protocol",
        .unit = "UPP",
        .contract_address =
            "\xc8\x6d\x05\x48\x09\x62\x34\x32\x21\x0c\x10\x7a\xf2\xe3\xf6\x19\xdc\xfb\xf6\x52",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Uquid Coin",
        .unit = "UQC",
        .contract_address =
            "\xd0\x1d\xb7\x3e\x04\x78\x55\xef\xb4\x14\xe6\x20\x20\x98\xc4\xbe\x4c\xd2\x42\x3b",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "USD Coin",
        .unit = "USDC",
        .contract_address =
            "\xa0\xb8\x69\x91\xc6\x21\x8b\x36\xc1\xd1\x9d\x4a\x2e\x9e\xb0\xce\x36\x06\xeb\x48",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "USDK",
        .unit = "USDK",
        .contract_address =
            "\x1c\x48\xf8\x6a\xe5\x72\x91\xf7\x68\x63\x49\xf1\x26\x01\x91\x0b\xd8\xd4\x70\xbb",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "USDQ",
        .unit = "USDQ",
        .contract_address =
            "\x49\x54\xdb\x63\x91\xf4\xfe\xb5\x46\x8b\x6b\x94\x3d\x49\x35\x35\x35\x96\xae\xc9",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tether USD",
        .unit = "USDT",
        .contract_address =
            "\xda\xc1\x7f\x95\x8d\x2e\xe5\x23\xa2\x20\x62\x06\x99\x45\x97\xc1\x3d\x83\x1e\xc7",
        .decimals = 6,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Utrust",
        .unit = "UTK",
        .contract_address =
            "\x70\xa7\x28\x33\xd6\xbf\x7f\x50\x8c\x82\x24\xce\x59\xea\x1e\xf3\xd0\xea\x3a\x38",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Universa",
        .unit = "UTNP",
        .contract_address =
            "\x9e\x33\x19\x63\x6e\x21\x26\xe3\xc0\xbc\x9e\x31\x34\xae\xc5\xe1\x50\x8a\x46\xc7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "United Traders Token",
        .unit = "UTT",
        .contract_address =
            "\x16\xf8\x12\xbe\x7f\xff\x02\xca\xf6\x62\xb8\x5d\x5d\x58\xa5\xda\x65\x72\xd4\xdf",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "U Network",
        .unit = "UUU",
        .contract_address =
            "\x35\x43\x63\x8e\xd4\xa9\x00\x6e\x48\x40\xb1\x05\x94\x42\x71\xbc\xea\x15\x60\x5d",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Valor Token",
        .unit = "VALOR",
        .contract_address =
            "\x29\x7e\x4e\x5e\x59\xad\x72\xb1\xb0\xa2\xfd\x44\x69\x29\xe7\x61\x17\xbe\x0e\x0a",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VeriDocGlobal",
        .unit = "VDG",
        .contract_address =
            "\x57\xc7\x5e\xcc\xc8\x55\x71\x36\xd3\x26\x19\xa1\x91\xfb\xcd\xc8\x85\x60\xd7\x11",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "BLOCKv",
        .unit = "VEE",
        .contract_address =
            "\x34\x0d\x2b\xde\x5e\xb2\x8c\x1e\xed\x91\xb2\xf7\x90\x72\x3e\x3b\x16\x06\x13\xb7",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Veritaseum",
        .unit = "VERI",
        .contract_address =
            "\x8f\x34\x70\xa7\x38\x8c\x05\xee\x4e\x7a\xf3\xd0\x1d\x8c\x72\x2b\x0f\xf5\x23\x74",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VestChain",
        .unit = "VEST",
        .contract_address =
            "\x37\xf0\x4d\x2c\x3a\xe0\x75\xfa\xd5\x48\x3b\xb9\x18\x49\x1f\x65\x6b\x12\xbd\xb6",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Voyager Token",
        .unit = "VGX",
        .contract_address =
            "\x5a\xf2\xbe\x19\x3a\x6a\xbc\xa9\xc8\x81\x70\x01\xf4\x57\x44\x77\x7d\xb3\x07\x56",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Viberate",
        .unit = "VIB",
        .contract_address =
            "\x2c\x97\x4b\x2d\x0b\xa1\x71\x6e\x64\x4c\x1f\xc5\x99\x82\xa8\x9d\xdd\x2f\xf7\x24",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VIBE",
        .unit = "VIBE",
        .contract_address =
            "\xe8\xff\x5c\x9c\x75\xde\xb3\x46\xac\xac\x49\x3c\x46\x3c\x89\x50\xbe\x03\xdf\xba",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VideoCoin",
        .unit = "VID",
        .contract_address =
            "\x2c\x90\x23\xbb\xc5\x72\xff\x8d\xc1\x22\x8c\x78\x58\xa2\x80\x04\x6e\xa8\xc9\xe5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "V-ID",
        .unit = "VIDT",
        .contract_address =
            "\x44\x5f\x51\x29\x9e\xf3\x30\x7d\xbd\x75\x03\x6d\xd8\x96\x56\x5f\x5b\x4b\xf7\xa5",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VIDY",
        .unit = "VIDY",
        .contract_address =
            "\xc7\x7b\x23\x0f\x31\xb5\x17\xf1\xef\x36\x2e\x59\xc1\x73\xc2\xbe\x65\x40\xb5\xe8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VNDC",
        .unit = "VNDC",
        .contract_address =
            "\x1f\x3f\x67\x7e\xcc\x58\xf6\xa1\xf9\xe2\xcf\x41\x0d\xf4\x77\x6a\x85\x46\xb5\xde",
        .decimals = 0,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "VNT Chain",
        .unit = "VNT",
        .contract_address =
            "\x69\xd2\x77\x95\x33\xa4\xd2\xc7\x80\x63\x97\x13\x55\x8b\x2c\xc9\x8c\x46\xa9\xb7",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Tael",
        .unit = "WABI",
        .contract_address =
            "\x28\x6b\xda\x14\x13\xa2\xdf\x81\x73\x1d\x49\x30\xce\x2f\x86\x2a\x35\xa6\x09\xfe",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Wrapped Bitcoin",
        .unit = "WBTC",
        .contract_address =
            "\x22\x60\xfa\xc5\xe5\x54\x2a\x77\x3a\xa4\x4f\xbc\xfe\xdf\x7c\x19\x3b\xc2\xc5\x99",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Winco",
        .unit = "WCO",
        .contract_address =
            "\xd4\x4b\xb6\x66\x39\x36\xca\xb1\x31\x05\x84\xa2\x77\xf7\xda\xa6\x94\x3d\x49\x04",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Wings",
        .unit = "WINGS",
        .contract_address =
            "\x66\x70\x88\xb2\x12\xce\x3d\x06\xa1\xb5\x53\xa7\x22\x1e\x1f\xd1\x90\x00\xd9\xaf",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Wixlar",
        .unit = "WIX",
        .contract_address =
            "\x7b\xa1\x9b\x7f\x7d\x10\x6a\x9a\x1e\x09\x85\x39\x7b\x94\xf3\x8e\xee\x0b\x55\x5e",
        .decimals = 2,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "WePower",
        .unit = "WPR",
        .contract_address =
            "\x4c\xf4\x88\x38\x7f\x03\x5f\xf0\x8c\x37\x15\x15\x56\x2c\xba\x71\x2f\x90\x15\xd4",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Waltonchain",
        .unit = "WTC",
        .contract_address =
            "\xb7\xcb\x1c\x96\xdb\x6b\x22\xb0\xd3\xd9\x53\x6e\x01\x08\xd0\x62\xbd\x48\x8f\x74",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "General Attention Currency",
        .unit = "XAC",
        .contract_address =
            "\xde\x4c\x5a\x79\x19\x13\x83\x80\x27\xa2\x18\x57\x09\xe9\x8c\x5c\x60\x27\xea\x63",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Xaurum",
        .unit = "XAUR",
        .contract_address =
            "\x4d\xf8\x12\xf6\x06\x4d\xef\x1e\x5e\x02\x9f\x1c\xa8\x58\x77\x7c\xc9\x8d\x2d\x81",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "CryptoFranc",
        .unit = "XCHF",
        .contract_address =
            "\xb4\x27\x20\x71\xec\xad\xd6\x9d\x93\x3a\xdc\xd1\x9c\xa9\x9f\xe8\x06\x64\xfc\x08",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "DigitalBits",
        .unit = "XDB",
        .contract_address =
            "\xb9\xee\xfc\x4b\x0d\x47\x2a\x44\xbe\x93\x97\x02\x54\xdf\x4f\x40\x16\x56\x9d\x27",
        .decimals = 7,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "XinFin Network",
        .unit = "XDCE",
        .contract_address =
            "\x41\xab\x1b\x6f\xcb\xb2\xfa\x9d\xce\xd8\x1a\xcb\xde\xc1\x3e\xa6\x31\x5f\x2b\xf2",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ETERNAL TOKEN",
        .unit = "XET",
        .contract_address =
            "\x05\x4c\x64\x74\x1d\xba\xfd\xc1\x97\x84\x50\x54\x94\x02\x98\x23\xd8\x9c\x3b\x13",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Mixin",
        .unit = "XIN",
        .contract_address =
            "\xa9\x74\xc7\x09\xcf\xb4\x56\x66\x86\x55\x3a\x20\x79\x06\x85\xa4\x7a\xce\xaa\x33",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "XcelToken Plus",
        .unit = "XLAB",
        .contract_address =
            "\x8c\x4e\x7f\x81\x4d\x40\xf8\x92\x9f\x91\x12\xc5\xd0\x90\x16\xf9\x23\xd3\x44\x72",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "XMax",
        .unit = "XMX",
        .contract_address =
            "\x0f\x8c\x45\xb8\x96\x78\x4a\x1e\x40\x85\x26\xb9\x30\x05\x19\xef\x86\x60\x20\x9c",
        .decimals = 8,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Xensor",
        .unit = "XSR",
        .contract_address =
            "\x6b\xc1\xf3\xa1\xae\x56\x23\x1d\xbb\x64\xd3\xe8\x2e\x07\x08\x57\xea\xe8\x60\x45",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "XYO",
        .unit = "XYO",
        .contract_address =
            "\x55\x29\x6f\x69\xf4\x0e\xa6\xd2\x0e\x47\x85\x33\xc1\x5a\x6b\x08\xb6\x54\xe7\x58",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "YEE",
        .unit = "YEE",
        .contract_address =
            "\x92\x21\x05\xfa\xd8\x15\x3f\x51\x6b\xcf\xb8\x29\xf5\x6d\xc0\x97\xa0\xe1\xd7\x05",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "YGGDRASH",
        .unit = "YEED",
        .contract_address =
            "\xca\x27\x96\xf9\xf6\x1d\xc7\xb2\x38\xaa\xb0\x43\x97\x1e\x49\xc6\x16\x4d\xf3\x75",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "YOU COIN",
        .unit = "YOU",
        .contract_address =
            "\x34\x36\x4b\xee\x11\x60\x7b\x19\x63\xd6\x6b\xca\x66\x5f\xde\x93\xfc\xa6\x66\xa8",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ZB Token",
        .unit = "ZB",
        .contract_address =
            "\xbd\x07\x93\x33\x2e\x9f\xb8\x44\xa5\x2a\x20\x5a\x23\x3e\xf2\x7a\x5b\x34\xb9\x27",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ZEON",
        .unit = "ZEON",
        .contract_address =
            "\xe5\xb8\x26\xca\x2c\xa0\x2f\x09\xc1\x72\x5e\x9b\xd9\x8d\x9a\x88\x74\xc3\x05\x32",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "Zipper",
        .unit = "ZIP",
        .contract_address =
            "\xa9\xd2\x92\x7d\x3a\x04\x30\x9e\x00\x8b\x6a\xf6\xe2\xe2\x82\xae\x29\x52\xe7\xfd",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "0x",
        .unit = "ZRX",
        .contract_address =
            "\xe4\x1d\x24\x89\x57\x1d\x32\x21\x89\x24\x6d\xaf\xa5\xeb\xde\x1f\x46\x99\xf4\x98",
        .decimals = 18,
    },
    {
        .coin = ETHCoin_ETH,
        .name = "ZTCoin",
        .unit = "ZT",
        .contract_address =
            "\xfe\x39\xe6\xa3\x2a\xcd\x2a\xf7\x95\x5c\xb3\xd4\x06\xba\x2b\x55\xc9\x01\xf2\x47",
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
