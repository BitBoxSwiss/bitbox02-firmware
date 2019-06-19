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

#ifndef _APPS_ETH_PARAMS_H
#define _APPS_ETH_PARAMS_H

#include <generated/hww.pb.h>

typedef struct {
    uint32_t bip44_coin;
    // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md#list-of-chain-ids
    uint8_t chain_id;
    const char* unit;
} app_eth_coin_params_t;

/**
 * @return pointer to static coin params on success. NULL if the coin is
 * unknown.
 */
app_eth_coin_params_t* app_eth_params_get(ETHCoin coin);

#endif
