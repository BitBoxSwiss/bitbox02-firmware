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

#include <stdio.h>

#include "btc.h"
#include "btc_common.h"
#include "btc_params.h"
#include "confirm_multisig.h"

#include <hww.pb.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <util.h>
#include <workflow/confirm.h>
#include <workflow/status.h>

app_btc_result_t app_btc_address_multisig(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len,
    bool display)
{
    const app_btc_coin_params_t* params = app_btc_params_get(coin);
    if (params == NULL) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    if (!btc_common_is_valid_keypath_address_multisig(
            multisig->script_type, keypath, keypath_len, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!btc_common_multisig_is_valid(multisig, keypath, keypath_len - 2, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    // Confirm previously registered multisig.
    char multisig_registered_name[MEMORY_MULTISIG_NAME_MAX_LEN] = {0};
    if (!btc_common_multisig_name(
            coin, multisig, keypath, keypath_len - 2, multisig_registered_name)) {
        // Not previously registered -> fail.
        return APP_BTC_ERR_INVALID_INPUT;
    }

    const char* title = "Receive to";

    if (!apps_btc_confirm_multisig_basic(title, params, multisig_registered_name, multisig)) {
        return APP_BTC_ERR_USER_ABORT;
    }

    uint8_t payload[SHA256_LEN] = {0};
    size_t written = 0;

    multisig_t ms = {0};
    if (!btc_common_convert_multisig(multisig, &ms)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!btc_common_payload_from_multisig(
            &ms,
            multisig->script_type,
            keypath[keypath_len - 2],
            keypath[keypath_len - 1],
            payload,
            &written)) {
        return APP_BTC_ERR_UNKNOWN;
    }

    if (!rust_bitcoin_address_from_payload(
            params->coin,
            btc_common_determine_output_type_multisig(multisig),
            rust_util_bytes(payload, written),
            rust_util_cstr_mut(out, out_len))) {
        return APP_BTC_ERR_UNKNOWN;
    }

    if (display) {
        const confirm_params_t confirm_params = {
            .title = title,
            .body = out,
            .scrollable = true,
        };
        if (!workflow_confirm_blocking(&confirm_params)) {
            return APP_BTC_ERR_USER_ABORT;
        }
    }
    return APP_BTC_OK;
}

bool app_btc_enabled(BTCCoin coin)
{
    switch (coin) {
#if APP_BTC == 1
    case BTCCoin_BTC:
        /* PASSTHRU */
    case BTCCoin_TBTC:
        return true;
#endif
#if APP_LTC == 1
    case BTCCoin_LTC:
        /* PASSTHRU */
    case BTCCoin_TLTC:
        return true;
#endif
    default:
        return false;
    }
}

app_btc_result_t app_btc_register_script_config(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const char* name,
    BTCRegisterScriptConfigRequest_XPubType xpub_type)
{
    const app_btc_coin_params_t* params = app_btc_params_get(coin);
    if (params == NULL) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    // Only multisig registration supported for now.
    if (script_config->which_config != BTCScriptConfig_multisig_tag) {
        return APP_BTC_ERR_INVALID_INPUT;
    }
    const BTCScriptConfig_Multisig* multisig = &script_config->config.multisig;

    // Name as entered by the user.
    char entered_name[MEMORY_MULTISIG_NAME_MAX_LEN] = {0};
    if (*name == '\0') {
        const confirm_params_t confirm_params = {
            .title = "Register",
            .body = "Please name this\nmultisig account",
            .accept_is_nextarrow = true,
        };
        if (!workflow_confirm_blocking(&confirm_params)) {
            return APP_BTC_ERR_USER_ABORT;
        }

        // Empty name means we prompt the user to enter the name on the device.
        if (!rust_workflow_trinary_input_name(
                rust_util_cstr_mut(entered_name, sizeof(entered_name)))) {
            return APP_BTC_ERR_USER_ABORT;
        }
        name = entered_name;
    }
    if (!rust_util_validate_name(rust_util_cstr(name), MEMORY_MULTISIG_NAME_MAX_LEN - 1)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!btc_common_multisig_is_valid(multisig, keypath, keypath_len, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!apps_btc_confirm_multisig_extended(
            "Register", params, name, multisig, xpub_type, keypath, keypath_len)) {
        return APP_BTC_ERR_USER_ABORT;
    }

    uint8_t hash[SHA256_LEN] = {0};
    if (!btc_common_multisig_hash_sorted(coin, multisig, keypath, keypath_len, hash)) {
        return APP_BTC_ERR_UNKNOWN;
    }
    // This will rename the multisig config if it already exists.
    memory_result_t result = memory_multisig_set_by_hash(hash, name);
    switch (result) {
    case MEMORY_OK:
        workflow_status_blocking("Multisig account\nregistered", true);
        return APP_BTC_OK;
    case MEMORY_ERR_DUPLICATE_NAME:
        return APP_BTC_ERR_DUPLICATE;
    default:
        return APP_BTC_ERR_UNKNOWN;
    }
}
