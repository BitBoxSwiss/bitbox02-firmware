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
#include <workflow/confirm.h>

#include <stdio.h>

bool apps_btc_confirm_multisig(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig,
    bool verify_xpubs)
{
    char basic_info[100] = {0};
    int snprintf_result = snprintf(
        basic_info,
        sizeof(basic_info),
        "Coin: %s\nMultisig type: %lu-of-%lu",
        btc_common_coin_name(coin),
        (unsigned long)multisig->threshold,
        (unsigned long)multisig->xpubs_count);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(basic_info)) {
        Abort("apps_btc_confirm_multisig/0");
    }
    const confirm_params_t params_basic = {
        .title = title,
        .body = basic_info,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm(&params_basic)) {
        return false;
    }

    const confirm_params_t params_name = {
        .title = title,
        .body = name,
        .scrollable = true,
        .accept_is_nextarrow = true,
    };
    if (!workflow_confirm(&params_name)) {
        return false;
    }

    if (!verify_xpubs) {
        return true;
    }

    BTCPubRequest_XPubType xpub_type;
    switch (coin) {
    case BTCCoin_BTC:
    case BTCCoin_LTC:
        xpub_type = BTCPubRequest_XPubType_CAPITAL_ZPUB;
        break;
    case BTCCoin_TBTC:
    case BTCCoin_TLTC:
        xpub_type = BTCPubRequest_XPubType_CAPITAL_VPUB;
        break;
    default:
        Abort("confirm multisig: unknown coin");
    }

    size_t num_cosigners = multisig->xpubs_count;
    for (size_t i = 0; i < num_cosigners; i++) {
        const XPub* xpub_in = &multisig->xpubs[i];
        struct ext_key xpub = {0};
        if (!apps_common_bip32_xpub_from_protobuf(xpub_in, &xpub)) {
            return false;
        }
        char xpub_str[XPUB_ENCODED_LEN] = {0};
        if (!btc_common_encode_xpub(&xpub, xpub_type, xpub_str, sizeof(xpub_str))) {
            return false;
        }
        char confirm[XPUB_ENCODED_LEN + 100] = {0};
        if (i == multisig->our_xpub_index) {
            snprintf_result = snprintf(
                confirm,
                sizeof(confirm),
                "Cosigner %lu/%lu (this device): %s",
                (unsigned long)(i + 1),
                (unsigned long)num_cosigners,
                xpub_str);
            if (snprintf_result < 0 || snprintf_result >= (int)sizeof(confirm)) {
                Abort("apps_btc_confirm_multisig/1");
            }
        } else {
            snprintf_result = snprintf(
                confirm,
                sizeof(confirm),
                "Cosigner %lu/%lu: %s",
                (unsigned long)(i + 1),
                (unsigned long)num_cosigners,
                xpub_str);
            if (snprintf_result < 0 || snprintf_result >= (int)sizeof(confirm)) {
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
        if (!workflow_confirm(&params_xpub)) {
            return false;
        }
    }
    return true;
}
