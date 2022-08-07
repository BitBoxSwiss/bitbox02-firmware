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

#ifndef _APPS_BTC_CONFIRM_MULTISIG_H_
#define _APPS_BTC_CONFIRM_MULTISIG_H_

#include "btc_params.h"

#include <btc.pb.h>
#include <compiler_util.h>
#include <stdbool.h>

/**
 * Confirms a multisig setup with the user during send/receive.
 * Verified are:
 * - coin
 * - multisig type (m-of-n)
 * - name given by the user
 * @param[in] title the title shown in each confirmation screen
 * @param[in] params Coin params of the coin to be confirmed.
 * @param[in] name User given name of the multisig account.
 * @param[in] multisig multisig details
 */
USE_RESULT bool apps_btc_confirm_multisig_basic(
    const char* title,
    const app_btc_coin_params_t* params,
    const char* name,
    const BTCScriptConfig_Multisig* multisig);

#endif
