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
#if APP_BTC == 1 || APP_LTC == 1
#include "commander/commander_btc.h"
#endif
#if APP_ETH == 1
#include "commander/commander_eth.h"
#endif
#if PRODUCT_BITBOX_BASE == 1
#include "rust/rust.h"
#endif

#include <flags.h>
#include <hardfault.h>
#include <keystore.h>
#include <random.h>
#include <screen.h>
#include <sd.h>
#include <util.h>
#include <version.h>

#include <workflow/reboot.h>
#include <workflow/restore.h>
#include <workflow/restore_from_mnemonic.h>
#include <workflow/workflow.h>

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

// ------------------------------------ API ------------------------------------- //

#if PLATFORM_BITBOX02 == 1
/**
 * Retrieves a random number, displays it and encodes it into the passed stream.
 * Returns 0 if the encoding failed and the message length if the encoding was
 * successful.
 */
static void _api_process_random(RandomNumberResponse* response)
{
    uint8_t number[RANDOM_NUM_SIZE];
    random_32_bytes(number);

    static char number_hex[BB_HEX_SIZE(number)]; // TODO cleanup
    util_uint8_to_hex(number, sizeof(number), number_hex);

    char number_hex_formatted[BB_HEX_SIZE(number) + 3];
    snprintf(
        number_hex_formatted,
        sizeof(number_hex_formatted),
        "%.16s\n%.16s\n%.16s\n%.16s",
        number_hex,
        number_hex + 16,
        number_hex + 32,
        number_hex + 48);

    workflow_confirm_dismiss("Random", number_hex_formatted);

    memcpy(&response->number, number, sizeof(number));
}

static commander_error_t _api_list_backups(ListBackupsResponse* response)
{
    if (!workflow_list_backups(response)) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

static commander_error_t _api_restore_backup(const RestoreBackupRequest* request)
{
    if (!workflow_restore_backup(request)) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

static commander_error_t _api_get_root_fingerprint(RootFingerprintResponse* response)
{
    bool success = keystore_get_root_fingerprint(response->fingerprint);
    if (!success) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

static commander_error_t _api_restore_from_mnemonic(const RestoreFromMnemonicRequest* request)
{
    if (!workflow_restore_from_mnemonic(request)) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}
#endif

static commander_error_t _api_reboot(void)
{
    if (!workflow_reboot()) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

// ------------------------------------ Process ------------------------------------- //

/**
 * Processes the command and forwards it to the requested function.
 */
static commander_error_t _api_process(const Request* request, Response* response)
{
    switch (request->which_request) {
#if PLATFORM_BITBOX02 == 1
    case Request_random_number_tag:
        response->which_response = Response_random_number_tag;
        _api_process_random(&(response->response.random_number));
        return COMMANDER_OK;
#if APP_BTC == 1 || APP_LTC == 1
    case Request_btc_pub_tag:
        response->which_response = Response_pub_tag;
        return commander_btc_pub(&(request->request.btc_pub), &(response->response.pub));
    case Request_btc_sign_init_tag:
    case Request_btc_sign_input_tag:
    case Request_btc_sign_output_tag:
        return commander_btc_sign(request, response);
    case Request_btc_tag:
        response->which_response = Response_btc_tag;
        return commander_btc(&(request->request.btc), &(response->response.btc));
#else
    case Request_btc_pub_tag:
    case Request_btc_sign_init_tag:
    case Request_btc_sign_input_tag:
    case Request_btc_sign_output_tag:
        return COMMANDER_ERR_DISABLED;
#endif
    case Request_fingerprint_tag:
        response->which_response = Response_fingerprint_tag;
        return _api_get_root_fingerprint(&(response->response.fingerprint));
    case Request_list_backups_tag:
        response->which_response = Response_list_backups_tag;
        return _api_list_backups(&(response->response.list_backups));
    case Request_restore_backup_tag:
        response->which_response = Response_success_tag;
        return _api_restore_backup(&(request->request.restore_backup));
#if APP_ETH == 1
    case Request_eth_tag:
        response->which_response = Response_eth_tag;
        return commander_eth(&(request->request.eth), &(response->response.eth));
#else
    case Request_eth_tag:
        return COMMANDER_ERR_DISABLED;
#endif
    case Request_restore_from_mnemonic_tag:
        response->which_response = Response_success_tag;
        return _api_restore_from_mnemonic(&(request->request.restore_from_mnemonic));
#endif
#if PRODUCT_BITBOX_BASE == 1
    case Request_bitboxbase_tag:
        response->which_response = Response_success_tag;
        return commander_bitboxbase(&(request->request.bitboxbase));
#endif
    case Request_reboot_tag:
        response->which_response = Response_success_tag;
        return _api_reboot();
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
