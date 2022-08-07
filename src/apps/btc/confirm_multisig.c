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
#include "btc_ui.h"

#include <apps/common/bip32.h>
#include <hardfault.h>
#include <keystore.h>
#include <rust/rust.h>

#include <stdio.h>

bool apps_btc_confirm_multisig_basic(
    const char* title,
    const app_btc_coin_params_t* params,
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
        params->name);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(basic_info)) {
        Abort("apps_btc_confirm_multisig/0");
    }
    const confirm_params_t params_basic = {
        .title = title,
        .body = basic_info,
        .accept_is_nextarrow = true,
    };
    if (!app_btc_ui()->confirm(&params_basic)) {
        return false;
    }

    const confirm_params_t params_name = {
        .title = title,
        .body = name,
        .scrollable = true,
        .accept_is_nextarrow = true,
    };
    return app_btc_ui()->confirm(&params_name);
}
