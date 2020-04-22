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
 * Returns the xpub at a given keypath.
 * @param[in] coin Coin to generate address for.
 * @param[in] xpub_type the xpub will be serialized using this versin specifier (xpub, ypub, etc.).
 * @param[in] keypath keypath at which to get the xpub.
 * @param[in] keypath_len number of keypath elements.
 * @param[out] out will hold the xub.
 * @param[in] out_len size of out.
 * @return true on success, false on failure.m
 */
USE_RESULT bool app_btc_xpub(
    BTCCoin coin,
    BTCPubRequest_XPubType xpub_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len);

/**
 * Returns the electrum wallet encryption xpub.
 * @param[in] keypath its value currently needs to be m/4541509'/1112098098'
 * @param[in] keypath_len number of keypath elements.
 * @param[out] out will hold the xpub.
 * @param[in] out_len size of out.
 */
bool app_btc_electrum_encryption_key(
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len);

/**
 * Creates an address from a public key at a given keypath.
 * @param[in] coin Coin to generate address for.
 * @param[in] script_config script configuration, which determines the address.
 * @param[in] keypath keypath at which to create the address.
 * @param[in] keypath_len number of keypath elements.
 * @param[out] out will hold the address.
 * @param[in] out_len size of out.
 * @return true on success.
 */
USE_RESULT bool app_btc_address_simple(
    BTCCoin coin,
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len);

/**
 * Creates an address from a public key at a given keypath.
 * @param[in] coin Coin to generate address for.
 * @param[in] script_config script configuration, which determines the address.
 * @param[in] keypath keypath at which to create the address.
 * @param[in] keypath_len number of keypath elements.
 * @param[out] out will hold the address.
 * @param[in] out_len size of out.
 * @param[in] display if true, show the address on the screen.
 * @return see app_btc_result_t.
 */
USE_RESULT app_btc_result_t app_btc_address_multisig_p2wsh(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len,
    bool display);

/**
 * @return true if coin is enabled
 */
bool app_btc_enabled(BTCCoin coin);

/**
 * @param[out] is_registered is true if the script config was previously registered on the device.
 * @return true on success, false on failure.
 */
USE_RESULT bool app_btc_is_script_config_registered(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    bool* is_registered);

/**
 * Stores a script configuration alongside a user chosen name on the device. If the user aborts,
 * nothing is stored.
 * @param[in] name Name to give to the script config. Must be at most MEMORY_MULTISIG_NAME_MAX_LEN
 * bytes (including null terminator).
 * @return OK if the registration was successful, ERR_USER_ABORT if the user aborted, ERR_UNKNOWN
 * for unknown errors.
 */
USE_RESULT app_btc_result_t app_btc_register_script_config(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const char* name);

#endif
