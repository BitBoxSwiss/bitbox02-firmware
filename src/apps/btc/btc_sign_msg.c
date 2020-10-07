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
#include "btc_sign_msg.h"
#include "btc_common.h"

#include <hardfault.h>
#include <hww.pb.h>
#include <keystore.h>
#include <rust/rust.h>
#include <workflow/confirm.h>

#include <wally_script.h>

#include <stdio.h>

#define MAX_MESSAGE_SIZE 1024

USE_RESULT app_btc_result_t app_btc_sign_msg(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg,
    size_t msg_size,
    uint8_t* signature_out)
{
    if (coin != BTCCoin_BTC) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    if (script_config->which_config != BTCScriptConfig_simple_type_tag) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    BTCScriptConfig_SimpleType simple_type = script_config->config.simple_type;
    if (simple_type != BTCScriptConfig_SimpleType_P2WPKH_P2SH &&
        simple_type != BTCScriptConfig_SimpleType_P2WPKH) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    if (msg_size > MAX_MESSAGE_SIZE) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    // Keypath and script_config are validated in app_btc_address_simple().
    char address[500] = {0};
    if (!app_btc_address_simple(
            coin, simple_type, keypath, keypath_len, address, sizeof(address))) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    char basic_info[100] = {0};
    int snprintf_result =
        snprintf(basic_info, sizeof(basic_info), "Coin: %s", btc_common_coin_name(coin));
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(basic_info)) {
        Abort("app_btc_sign_message/0");
    }
    const confirm_params_t params_basic = {
        .title = "Sign message",
        .body = basic_info,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm_blocking(&params_basic)) {
        return APP_BTC_ERR_USER_ABORT;
    }
    const confirm_params_t params_address = {
        .title = "Address",
        .body = address,
        .scrollable = true,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm_blocking(&params_address)) {
        return APP_BTC_ERR_USER_ABORT;
    }

    switch (rust_workflow_verify_message(rust_util_bytes(msg, msg_size))) {
    case VerifyMessageResultOk:
        break;
    case VerifyMessageResultInvalidInput:
        return APP_BTC_ERR_INVALID_INPUT;
    case VerifyMessageResultUserAbort:
        return APP_BTC_ERR_USER_ABORT;
    default:
        Abort("unexpected verify message result");
    }

    // See
    // https://github.com/spesmilo/electrum/blob/84dc181b6e7bb20e88ef6b98fb8925c5f645a765/electrum/ecc.py#L355-L358.
    // This is the message format that is widespread for p2pkh addresses.
    // Electrum re-used it for p2wpkh-p2sh and p2wpkh addresses.
    const char msg_header[] =
        "\x18"
        "Bitcoin Signed Message:\n";
    // sizeof(msg_header) includes null terminator
    uint8_t formatted_msg[sizeof(msg_header) - 1 + MAX_MESSAGE_SIZE + MAX_VARINT_SIZE] = {0};

    size_t offset = sizeof(msg_header) - 1;
    memcpy(formatted_msg, msg_header, offset);
    offset += wally_varbuff_to_bytes(msg, msg_size, &formatted_msg[offset]);
    uint8_t hash[32] = {0};
    rust_sha256(formatted_msg, offset, hash);
    rust_sha256(hash, sizeof(hash), hash);
    int recid;
    if (!keystore_secp256k1_sign(keypath, keypath_len, hash, signature_out, &recid)) {
        return APP_BTC_ERR_UNKNOWN;
    }
    if (recid > 0xFF) {
        Abort("unexpected recid");
    }
    signature_out[64] = (uint8_t)recid;

    return APP_BTC_OK;
}
