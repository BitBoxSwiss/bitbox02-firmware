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

#ifndef _APPS_BTC_H
#define _APPS_BTC_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

#include <hww.pb.h>

typedef enum {
    APP_BTC_OK,
    APP_BTC_ERR_USER_ABORT,
    APP_BTC_ERR_INVALID_INPUT,
    APP_BTC_ERR_DUPLICATE,
    APP_BTC_ERR_STATE,
    APP_BTC_ERR_UNKNOWN,
} app_btc_result_t;

/**
 * @return true if coin is enabled
 */
bool app_btc_enabled(BTCCoin coin);

/**
 * Stores a script configuration alongside a user chosen name on the device. If the user aborts,
 * nothing is stored.
 * @param[in] name Name to give to the script config. Must be at most MEMORY_MULTISIG_NAME_MAX_LEN
 * bytes (including null terminator). If it is the empty string, the name is entered on the device.
 * @return OK if the registration was successful, ERR_USER_ABORT if the user aborted, ERR_UNKNOWN
 * for unknown errors.
 */
USE_RESULT app_btc_result_t app_btc_register_script_config(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const char* name,
    BTCRegisterScriptConfigRequest_XPubType xpub_type);

#endif
