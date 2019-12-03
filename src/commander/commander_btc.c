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

#include "commander_btc.h"
#include "commander_states.h"

#include <stdio.h>

#include <apps/btc/btc.h>
#include <apps/btc/btc_sign.h>
#include <workflow/verify_pub.h>

#include <wally_bip32.h> // for BIP32_INITIAL_HARDENED_CHILD

static const char* _coin_btc = "Bitcoin";
static const char* _coin_tbtc = "BTC Testnet";
static const char* _coin_ltc = "Litecoin";
static const char* _coin_tltc = "LTC Testnet";

commander_error_t commander_btc_pub(const BTCPubRequest* request, PubResponse* response)
{
    if (!app_btc_enabled(request->coin)) {
        return COMMANDER_ERR_DISABLED;
    }
    if (!app_btc_address(
            request->coin,
            request->output_type,
            request->script_type,
            request->keypath,
            request->keypath_count,
            response->pub,
            sizeof(response->pub))) {
        return COMMANDER_ERR_GENERIC;
    }
    if (request->display) {
        const char* coin;
        switch (request->coin) {
        case BTCCoin_BTC:
            coin = _coin_btc;
            break;
        case BTCCoin_TBTC:
            coin = _coin_tbtc;
            break;
        case BTCCoin_LTC:
            coin = _coin_ltc;
            break;
        case BTCCoin_TLTC:
            coin = _coin_tltc;
            break;
        default:
            return COMMANDER_ERR_GENERIC;
        }
        char title[100] = {0};
        switch (request->output_type) {
        case BTCPubRequest_OutputType_TPUB:
        case BTCPubRequest_OutputType_VPUB:
        case BTCPubRequest_OutputType_UPUB:
        case BTCPubRequest_OutputType_XPUB:
        case BTCPubRequest_OutputType_YPUB:
        case BTCPubRequest_OutputType_ZPUB:
            snprintf(
                title,
                sizeof(title),
                "%s\naccount #%lu",
                coin,
                (unsigned long)request->keypath[2] - BIP32_INITIAL_HARDENED_CHILD + 1);
            break;
        case BTCPubRequest_OutputType_ADDRESS: {
            switch (request->script_type) {
            case BTCScriptType_SCRIPT_P2PKH:
                snprintf(title, sizeof(title), "%s\nLegacy", coin);
                break;
            case BTCScriptType_SCRIPT_P2WPKH_P2SH:
                snprintf(title, sizeof(title), "%s", coin);
                break;
            case BTCScriptType_SCRIPT_P2WPKH:
                snprintf(title, sizeof(title), "%s\nbech32", coin);
                break;
            default:
                return COMMANDER_ERR_GENERIC;
            }
            break;
        }
        default:
            return COMMANDER_ERR_GENERIC;
        }
        workflow_verify_pub(title, response->pub);
    }
    return COMMANDER_OK;
}

commander_error_t commander_btc_sign(const Request* request, Response* response)
{
    response->which_response = Response_btc_sign_next_tag;
    app_btc_sign_error_t result;
    switch (request->which_request) {
    case Request_btc_sign_init_tag:
        if (!app_btc_enabled(request->request.btc_sign_init.coin)) {
            return COMMANDER_ERR_DISABLED;
        }
        result =
            app_btc_sign_init(&(request->request.btc_sign_init), &response->response.btc_sign_next);
        break;
    case Request_btc_sign_input_tag:
        result = app_btc_sign_input(
            &(request->request.btc_sign_input), &response->response.btc_sign_next);
        break;
    case Request_btc_sign_output_tag:
        result = app_btc_sign_output(
            &(request->request.btc_sign_output), &response->response.btc_sign_next);
        break;
    default:
        return COMMANDER_ERR_GENERIC;
    }
    if (result == APP_BTC_SIGN_ERR_USER_ABORT) {
        return COMMANDER_ERR_USER_ABORT;
    }
    if (result != APP_BTC_SIGN_OK) {
        return COMMANDER_ERR_GENERIC;
    }
    switch (response->response.btc_sign_next.type) {
    case BTCSignNextResponse_Type_INPUT:
        commander_states_force_next(Request_btc_sign_input_tag);
        break;
    case BTCSignNextResponse_Type_OUTPUT:
        commander_states_force_next(Request_btc_sign_output_tag);
        break;
    default:
        break;
    }
    return COMMANDER_OK;
}
