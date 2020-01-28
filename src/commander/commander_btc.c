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

// Returns the string to be used in the confirm title. Returns NULL for an invalid coin.
static const char* _coin_title(BTCCoin coin)
{
    switch (coin) {
    case BTCCoin_BTC:
        return _coin_btc;
    case BTCCoin_TBTC:
        return _coin_tbtc;
    case BTCCoin_LTC:
        return _coin_ltc;
    case BTCCoin_TLTC:
        return _coin_tltc;
    default:
        return NULL;
    }
}

static commander_error_t _btc_pub_xpub(const BTCPubRequest* request, PubResponse* response)
{
    if (!app_btc_xpub(
            request->coin,
            request->output.xpub_type,
            request->keypath,
            request->keypath_count,
            response->pub,
            sizeof(response->pub))) {
        return COMMANDER_ERR_GENERIC;
    }
    if (request->display) {
        const char* coin = _coin_title(request->coin);
        if (coin == NULL) {
            return COMMANDER_ERR_GENERIC;
        }
        char title[100] = {0};
        switch (request->output.xpub_type) {
        case BTCPubRequest_XPubType_TPUB:
        case BTCPubRequest_XPubType_XPUB:
        case BTCPubRequest_XPubType_YPUB:
        case BTCPubRequest_XPubType_ZPUB:
        case BTCPubRequest_XPubType_VPUB:
        case BTCPubRequest_XPubType_UPUB:
        case BTCPubRequest_XPubType_CAPITAL_VPUB:
        case BTCPubRequest_XPubType_CAPITAL_ZPUB:
            snprintf(
                title,
                sizeof(title),
                "%s\naccount #%lu",
                coin,
                (unsigned long)request->keypath[2] - BIP32_INITIAL_HARDENED_CHILD + 1);
            break;
        default:
            return COMMANDER_ERR_GENERIC;
        }
        workflow_verify_pub(title, response->pub);
    }
    return COMMANDER_OK;
}

static commander_error_t _btc_pub_address_simple(
    const BTCPubRequest* request,
    PubResponse* response)
{
    if (!app_btc_address_simple(
            request->coin,
            request->output.script_config.config.simple_type,
            request->keypath,
            request->keypath_count,
            response->pub,
            sizeof(response->pub))) {
        return COMMANDER_ERR_GENERIC;
    }
    if (request->display) {
        const char* coin = _coin_title(request->coin);
        if (coin == NULL) {
            return COMMANDER_ERR_GENERIC;
        }
        char title[100] = {0};
        switch (request->output.script_config.config.simple_type) {
        case BTCScriptConfig_SimpleType_P2WPKH_P2SH:
            snprintf(title, sizeof(title), "%s", coin);
            break;
        case BTCScriptConfig_SimpleType_P2WPKH:
            snprintf(title, sizeof(title), "%s\nbech32", coin);
            break;
        default:
            return COMMANDER_ERR_GENERIC;
        }
        workflow_verify_pub(title, response->pub);
    }
    return COMMANDER_OK;
}

commander_error_t commander_btc_pub(const BTCPubRequest* request, PubResponse* response)
{
    if (!app_btc_enabled(request->coin)) {
        return COMMANDER_ERR_DISABLED;
    }
    switch (request->which_output) {
    case BTCPubRequest_xpub_type_tag:
        return _btc_pub_xpub(request, response);
    case BTCPubRequest_script_config_tag:
        switch (request->output.script_config.which_config) {
        case BTCScriptConfig_simple_type_tag:
            return _btc_pub_address_simple(request, response);
        default:
            return COMMANDER_ERR_INVALID_INPUT;
        }
    default:
        return COMMANDER_ERR_INVALID_INPUT;
    }
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
