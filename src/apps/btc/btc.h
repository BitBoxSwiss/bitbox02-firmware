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
 * @return true if coin is enabled
 */
bool app_btc_enabled(BTCCoin coin);

#endif
