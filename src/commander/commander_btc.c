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
#include <apps/btc/btc_sign_msg.h>

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

static commander_error_t _btc_pub_address_multisig(
    const BTCPubRequest* request,
    PubResponse* response)
{
    const BTCScriptConfig_Multisig* multisig = &request->output.script_config.config.multisig;
    app_btc_result_t result = app_btc_address_multisig(
        request->coin,
        multisig,
        request->keypath,
        request->keypath_count,
        response->pub,
        sizeof(response->pub),
        request->display);
    return _result(result);
}

commander_error_t commander_btc_pub(const BTCPubRequest* request, PubResponse* response)
{
    if (!app_btc_enabled(request->coin)) {
        return COMMANDER_ERR_DISABLED;
    }
    switch (request->which_output) {
    case BTCPubRequest_xpub_type_tag:
        // Handled in Rust.
        return COMMANDER_ERR_INVALID_INPUT;
    case BTCPubRequest_script_config_tag:
        switch (request->output.script_config.which_config) {
        case BTCScriptConfig_simple_type_tag:
            // Handled in Rust.
            return COMMANDER_ERR_INVALID_INPUT;
        case BTCScriptConfig_multisig_tag:
            return _btc_pub_address_multisig(request, response);
        default:
            return COMMANDER_ERR_INVALID_INPUT;
        }
    default:
        return COMMANDER_ERR_INVALID_INPUT;
    }
}

// like commander_states, but for requests nested in BTCRequest.
static commander_states_endpoint_id _force_next = 0;

static void _handle_sign_next(const BTCSignNextResponse* next)
{
    switch (next->type) {
    case BTCSignNextResponse_Type_INPUT:
        commander_states_force_next(Request_btc_sign_input_tag);
        break;
    case BTCSignNextResponse_Type_OUTPUT:
        commander_states_force_next(Request_btc_sign_output_tag);
        break;
    case BTCSignNextResponse_Type_PREVTX_INIT:
        commander_states_force_next(Request_btc_tag);
        _force_next = BTCRequest_prevtx_init_tag;
        break;
    case BTCSignNextResponse_Type_PREVTX_INPUT:
        commander_states_force_next(Request_btc_tag);
        _force_next = BTCRequest_prevtx_input_tag;
        break;
    case BTCSignNextResponse_Type_PREVTX_OUTPUT:
        commander_states_force_next(Request_btc_tag);
        _force_next = BTCRequest_prevtx_output_tag;
        break;
    default:
        break;
    }
}

commander_error_t commander_btc_sign(const Request* request, Response* response)
{
    response->which_response = Response_btc_sign_next_tag;
    app_btc_result_t result;
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
    if (result == APP_BTC_OK) {
        _handle_sign_next(&response->response.btc_sign_next);
    }
    return _result(result);
}

static commander_error_t _api_is_script_config_registered(
    const BTCIsScriptConfigRegisteredRequest* request,
    BTCIsScriptConfigRegisteredResponse* response)
{
    const BTCScriptConfigRegistration* reg = &request->registration;
    if (!app_btc_is_script_config_registered(
            reg->coin,
            &reg->script_config,
            reg->keypath,
            reg->keypath_count,
            &response->is_registered)) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
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
    bool can_call = _force_next == 0 || _force_next == request->which_request;
    if (!can_call) {
        return COMMANDER_ERR_INVALID_STATE;
    }
    _force_next = 0;

    switch (request->which_request) {
    case BTCRequest_is_script_config_registered_tag:
        response->which_response = BTCResponse_is_script_config_registered_tag;
        return _api_is_script_config_registered(
            &(request->request.is_script_config_registered),
            &response->response.is_script_config_registered);
    case BTCRequest_register_script_config_tag:
        response->which_response = BTCResponse_success_tag;
        return _api_register_script_config(&(request->request.register_script_config));
    case BTCRequest_prevtx_init_tag: {
        response->which_response = BTCResponse_sign_next_tag;
        app_btc_result_t result = app_btc_sign_prevtx_init(
            &(request->request.prevtx_init), &response->response.sign_next);
        if (result == APP_BTC_OK) {
            _handle_sign_next(&response->response.sign_next);
        }
        return _result(result);
    }
    case BTCRequest_prevtx_input_tag: {
        response->which_response = BTCResponse_sign_next_tag;
        app_btc_result_t result = app_btc_sign_prevtx_input(
            &(request->request.prevtx_input), &response->response.sign_next);
        if (result == APP_BTC_OK) {
            _handle_sign_next(&response->response.sign_next);
        }
        return _result(result);
    }
    case BTCRequest_prevtx_output_tag: {
        response->which_response = BTCResponse_sign_next_tag;
        app_btc_result_t result = app_btc_sign_prevtx_output(
            &(request->request.prevtx_output), &response->response.sign_next);
        if (result == APP_BTC_OK) {
            _handle_sign_next(&response->response.sign_next);
        }
        return _result(result);
    }
    case BTCRequest_sign_message_tag: {
        response->which_response = BTCResponse_sign_message_tag;
        const BTCSignMessageRequest* sign_request = &request->request.sign_message;
        app_btc_result_t result = app_btc_sign_msg(
            sign_request->coin,
            &sign_request->script_config.script_config,
            sign_request->script_config.keypath,
            sign_request->script_config.keypath_count,
            sign_request->msg.bytes,
            sign_request->msg.size,
            response->response.sign_message.signature);
        return _result(result);
    }
    default:
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}
