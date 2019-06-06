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

#include "btc_params.h"
#include <wally_bip32.h>

app_btc_coin_params_t app_btc_params_btc = {
    .bip44_coin = 0 + BIP32_INITIAL_HARDENED_CHILD,
    .base58_version_p2pkh = 0x00, // starts with 1
    .base58_version_p2sh = 0x05, // starts with 3
    .bech32_hrp = "bc",
    .unit = "BTC",
};

app_btc_coin_params_t app_btc_params_tbtc = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .base58_version_p2pkh = 0x6f, // starts with m or n
    .base58_version_p2sh = 0xc4, // starts with 2
    .bech32_hrp = "tb",
    .unit = "TBTC",
};

app_btc_coin_params_t app_btc_params_ltc = {
    .bip44_coin = 2 + BIP32_INITIAL_HARDENED_CHILD,
    .base58_version_p2pkh = 0x30, // starts with L
    .base58_version_p2sh = 0x32, // starts with M
    .bech32_hrp = "ltc",
    .unit = "LTC",
};

app_btc_coin_params_t app_btc_params_tltc = {
    .bip44_coin = 1 + BIP32_INITIAL_HARDENED_CHILD,
    .base58_version_p2pkh = 0x6f, // starts with m or n
    .base58_version_p2sh = 0xc4, // starts with 2
    .bech32_hrp = "tltc",
    .unit = "TLTC",
};

app_btc_coin_params_t* app_btc_params_get(BTCCoin coin)
{
    switch (coin) {
    case BTCCoin_BTC:
        return &app_btc_params_btc;
    case BTCCoin_TBTC:
        return &app_btc_params_tbtc;
    case BTCCoin_LTC:
        return &app_btc_params_ltc;
    case BTCCoin_TLTC:
        return &app_btc_params_tltc;
    default:
        return NULL;
    }
}
