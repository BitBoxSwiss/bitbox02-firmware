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
#include <apps/eth/eth_sign.h>
#include <util.h>
#include <workflow/verify_pub.h>

#include <wally_bip32.h> // for BIP32_INITIAL_HARDENED_CHILD

static const char* _coin_eth = "Ethereum";
static const char* _coin_ropsten_eth = "Ropsten";
static const char* _coin_rinkeby_eth = "Rinkeby";

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
        if (request->output_type != ETHPubRequest_OutputType_ADDRESS) {
            // Only support displaying the address for now.
            return COMMANDER_ERR_GENERIC;
        }
        const char* coin;

        // Check for ERC20-Address
        uint8_t zero[20] = {0};
        if (!MEMEQ(request->contract_address, zero, sizeof(zero))) {
            const app_eth_erc20_params_t* erc20_params =
                app_eth_erc20_params_get(request->coin, request->contract_address);
            if (erc20_params == NULL) {
                return COMMANDER_ERR_GENERIC;
            }
            coin = erc20_params->name;
        } else {
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
                return COMMANDER_ERR_GENERIC;
            }
        }
        workflow_verify_pub(coin, response->pub);
    }
    return COMMANDER_OK;
}

static commander_error_t _api_sign(const ETHSignRequest* request, ETHSignResponse* response)
{
    app_eth_sign_error_t result = app_eth_sign(request, response);
    if (result == APP_ETH_SIGN_ERR_USER_ABORT) {
        return COMMANDER_ERR_USER_ABORT;
    }
    if (result != APP_ETH_SIGN_OK) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

commander_error_t commander_eth(const ETHRequest* request, ETHResponse* response)
{
    switch (request->which_request) {
    case ETHRequest_pub_tag:
        response->which_response = PubResponse_pub_tag;
        return _api_pub(&(request->request.pub), &response->response.pub);
    case ETHRequest_sign_tag:
        response->which_response = ETHResponse_sign_tag;
        return _api_sign(&(request->request.sign), &response->response.sign);
    default:
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}
