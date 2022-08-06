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

static commander_error_t _result(app_btc_result_t result)
{
    switch (result) {
    case APP_BTC_OK:
        return COMMANDER_OK;
    case APP_BTC_ERR_USER_ABORT:
        return COMMANDER_ERR_USER_ABORT;
    case APP_BTC_ERR_INVALID_INPUT:
        return COMMANDER_ERR_INVALID_INPUT;
    case APP_BTC_ERR_DUPLICATE:
        return COMMANDER_ERR_DUPLICATE;
    case APP_BTC_ERR_STATE:
        return COMMANDER_ERR_INVALID_STATE;
    default:
        return COMMANDER_ERR_GENERIC;
    }
}

static commander_error_t _api_register_script_config(const BTCRegisterScriptConfigRequest* request)
{
    app_btc_result_t result = app_btc_register_script_config(
        request->registration.coin,
        &request->registration.script_config,
        request->registration.keypath,
        request->registration.keypath_count,
        request->name,
        request->xpub_type);
    return _result(result);
}

commander_error_t commander_btc(const BTCRequest* request, BTCResponse* response)
{
    switch (request->which_request) {
    case BTCRequest_register_script_config_tag:
        response->which_response = BTCResponse_success_tag;
        return _api_register_script_config(&(request->request.register_script_config));
    default:
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}
