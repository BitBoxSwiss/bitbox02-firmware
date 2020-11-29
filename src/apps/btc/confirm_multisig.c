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

#include "confirm_multisig.h"
#include "btc_common.h"

#include <apps/common/bip32.h>
#include <hardfault.h>
#include <keystore.h>
#include <rust/rust.h>
#include <workflow/confirm.h>

#include <stdio.h>

bool apps_btc_confirm_multisig_basic(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig)
{
    char basic_info[100] = {0};
    int snprintf_result = snprintf(
        basic_info,
        sizeof(basic_info),
        "%lu-of-%lu\n%s multisig",
        (unsigned long)multisig->threshold,
        (unsigned long)multisig->xpubs_count,
        btc_common_coin_name(coin));
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(basic_info)) {
        Abort("apps_btc_confirm_multisig/0");
    }
    const confirm_params_t params_basic = {
        .title = title,
        .body = basic_info,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm_blocking(&params_basic)) {
        return false;
    }

    const confirm_params_t params_name = {
        .title = title,
        .body = name,
        .scrollable = true,
        .accept_is_nextarrow = true,
    };
    return workflow_confirm_blocking(&params_name);
}

bool apps_btc_confirm_multisig_extended(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig,
    BTCRegisterScriptConfigRequest_XPubType xpub_type,
    const uint32_t* keypath,
    size_t keypath_len)
{
    const char* script_type_string;
    switch (multisig->script_type) {
    case BTCScriptConfig_Multisig_ScriptType_P2WSH:
        script_type_string = "p2wsh";
        break;
    case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH:
        script_type_string = "p2wsh-p2sh";
        break;
    default:
        return false;
    }
    char keypath_string[100] = {0};
    rust_bip32_to_string(
        keypath, keypath_len, rust_util_cstr_mut(keypath_string, sizeof(keypath_string)));

    if (!apps_btc_confirm_multisig_basic(title, coin, name, multisig)) {
        return false;
    }

    char info[200] = {0};
    int snprintf_result =
        snprintf(info, sizeof(info), "%s\nat\n%s", script_type_string, keypath_string);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(info)) {
        Abort("apps_btc_confirm_multisig/0");
    }
    const confirm_params_t params = {
        .title = title,
        .body = info,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm_blocking(&params)) {
        return false;
    }

    xpub_type_t output_xpub_type;
    switch (xpub_type) {
    case BTCRegisterScriptConfigRequest_XPubType_AUTO_ELECTRUM:
        switch (coin) {
        case BTCCoin_BTC:
        case BTCCoin_LTC:
            switch (multisig->script_type) {
            case BTCScriptConfig_Multisig_ScriptType_P2WSH:
                output_xpub_type = CAPITAL_ZPUB;
                break;
            case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH:
                output_xpub_type = CAPITAL_YPUB;
                break;
            default:
                return false;
            }
            break;
        case BTCCoin_TBTC:
        case BTCCoin_TLTC:
            switch (multisig->script_type) {
            case BTCScriptConfig_Multisig_ScriptType_P2WSH:
                output_xpub_type = CAPITAL_VPUB;
                break;
            case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH:
                output_xpub_type = CAPITAL_UPUB;
                break;
            default:
                return false;
            }
            break;
        default:
            Abort("confirm multisig: unknown coin");
        }
        break;
    case BTCRegisterScriptConfigRequest_XPubType_AUTO_XPUB_TPUB:
        switch (coin) {
        case BTCCoin_BTC:
        case BTCCoin_LTC:
            output_xpub_type = XPUB;
            break;
        case BTCCoin_TBTC:
        case BTCCoin_TLTC:
            output_xpub_type = TPUB;
            break;
        default:
            Abort("confirm multisig: unknown coin");
        }
        break;
    default:
        Abort("confirm multisig: unknown xpub_type");
    }

    size_t num_cosigners = multisig->xpubs_count;
    for (size_t i = 0; i < num_cosigners; i++) {
        const XPub* xpub_in = &multisig->xpubs[i];
        struct ext_key xpub = {0};
        if (!apps_common_bip32_xpub_from_protobuf(xpub_in, &xpub)) {
            return false;
        }
        char xpub_str[XPUB_ENCODED_LEN] = {0};
        if (!keystore_encode_xpub(&xpub, output_xpub_type, xpub_str, sizeof(xpub_str))) {
            return false;
        }
        char confirm[XPUB_ENCODED_LEN + 100] = {0};
        if (i == multisig->our_xpub_index) {
            int result = snprintf(
                confirm,
                sizeof(confirm),
                "Cosigner %lu/%lu (this device): %s",
                (unsigned long)(i + 1),
                (unsigned long)num_cosigners,
                xpub_str);
            if (result < 0 || result >= (int)sizeof(confirm)) {
                Abort("apps_btc_confirm_multisig/1");
            }
        } else {
            int result = snprintf(
                confirm,
                sizeof(confirm),
                "Cosigner %lu/%lu: %s",
                (unsigned long)(i + 1),
                (unsigned long)num_cosigners,
                xpub_str);
            if (result < 0 || result >= (int)sizeof(confirm)) {
                Abort("apps_btc_confirm_multisig/2");
            }
        }
        const confirm_params_t params_xpub = {
            .title = title,
            .body = confirm,
            .scrollable = true,
            .longtouch = i == num_cosigners - 1,
            .accept_is_nextarrow = true,
        };
        if (!workflow_confirm_blocking(&params_xpub)) {
            return false;
        }
    }
    return true;
}
