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

#include "commander_eth.h"

#include <stdio.h>

#include <apps/eth/eth.h>
#include <workflow/verify_pub.h>

#include <wally_bip32.h> // for BIP32_INITIAL_HARDENED_CHILD

static const char* _coin_eth = "Ethereum";
static const char* _coin_ropsten_eth = "Rinkeby";
static const char* _coin_rinkeby_eth = "Ropsten";

static commander_error_t _api_pub(const ETHPubRequest* request, PubResponse* response)
{
    if (!app_eth_address(
            request->coin,
            request->output_type,
            request->keypath,
            request->keypath_count,
            response->pub,
            sizeof(response->pub))) {
        return COMMANDER_ERR_GENERIC;
    }
    if (request->display) {
        const char* coin;
        switch (request->coin) {
        case ETHCoin_ETH:
            coin = _coin_eth;
            break;
        case ETHCoin_RopstenETH:
            coin = _coin_ropsten_eth;
            break;
        case ETHCoin_RinkebyETH:
            coin = _coin_rinkeby_eth;
            break;
        default:
            return false;
        }
        workflow_verify_pub(coin, response->pub);
    }
    return COMMANDER_OK;
}

commander_error_t commander_eth(const ETHRequest* request, ETHResponse* response)
{
    switch (request->which_request) {
    case ETHRequest_pub_tag:
        response->which_response = PubResponse_pub_tag;
        return _api_pub(&(request->request.pub), &response->response.pub);
    default:
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}
