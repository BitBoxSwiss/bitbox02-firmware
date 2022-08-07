// Copyright 2019 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
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

#include <platform_config.h>

#include "commander.h"
#include "protobuf.h"

#include <flags.h>
#include <hardfault.h>
#include <keystore.h>
#include <screen.h>
#include <sd.h>
#include <util.h>
#include <version.h>

#include "hww.pb.h"

#include <apps/btc/btc.h>

#define X(a, b, c) b,
static const int32_t _error_code[] = {COMMANDER_ERROR_TABLE};
#undef X

#define X(a, b, c) c,
static const char* const _error_message[] = {COMMANDER_ERROR_TABLE};
#undef X

static void _report_error(Response* response, commander_error_t error_code)
{
    Error* error = &(response->response.error);
    error->code = _error_code[error_code];
    snprintf(error->message, sizeof(error->message), "%s", _error_message[error_code]);
    response->which_response = Response_error_tag;
}

// ------------------------------------ Process ------------------------------------- //

/**
 * Processes the command and forwards it to the requested function.
 */
static commander_error_t _api_process(const Request* request, Response* response)
{
    (void)response;
    switch (request->which_request) {
#if APP_BTC == 1 || APP_LTC == 1
#else
    case Request_btc_pub_tag:
    case Request_btc_sign_init_tag:
    case Request_btc_sign_input_tag:
    case Request_btc_sign_output_tag:
        (void)response;
        return COMMANDER_ERR_DISABLED;
#endif
    default:
        screen_print_debug("command unknown", 1000);
        return COMMANDER_ERR_INVALID_INPUT;
    }
}

/**
 * Receives and processes a command.
 */
void commander(const in_buffer_t* in_buf, buffer_t* out_buf)
{
    Response response = Response_init_zero;

    Request request;
    commander_error_t err =
        protobuf_decode(in_buf, &request) ? COMMANDER_OK : COMMANDER_ERR_INVALID_INPUT;
    if (err == COMMANDER_OK) {
        err = _api_process(&request, &response);
        util_zero(&request, sizeof(request));
    }
    if (err != COMMANDER_OK) {
        _report_error(&response, err);
    }

    protobuf_encode(out_buf, &response);
}
