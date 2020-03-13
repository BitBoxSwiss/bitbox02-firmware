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
#include <workflow/status.h>
#include <workflow/verify_pub.h>

#define ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE (4541509 + BIP32_INITIAL_HARDENED_CHILD)
#define ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO (1112098098 + BIP32_INITIAL_HARDENED_CHILD)

bool app_btc_xpub(
    BTCCoin coin,
    BTCPubRequest_XPubType xpub_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    char* out,
    size_t out_len)
{
    const app_btc_coin_params_t* params = app_btc_params_get(coin);
    if (params == NULL) {
        return false;
    }
    if (!btc_common_is_valid_keypath_xpub(xpub_type, keypath, keypath_len, params->bip44_coin)) {
        return false;
    }

    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }
    return btc_common_encode_xpub(&derived_xpub, xpub_type, out, out_len);
}

bool app_btc_electrum_encryption_key(
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len)
{
    if (keypath_len != 2) {
        return false;
    }
    if (keypath[0] != ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_ONE ||
        keypath[1] != ELECTRUM_WALLET_ENCRYPTION_KEYPATH_LEVEL_TWO) {
        return false;
    }

    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }
    return btc_common_encode_xpub(&derived_xpub, BTCPubRequest_XPubType_XPUB, out, out_len);
}

bool app_btc_address_simple(
    BTCCoin coin,
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len)
{
    const app_btc_coin_params_t* params = app_btc_params_get(coin);
    if (params == NULL) {
        return false;
    }
    if (!btc_common_is_valid_keypath_address_simple(
            script_type, keypath, keypath_len, params->bip44_coin)) {
        return false;
    }
    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }

    uint8_t hash[32] = {0};
    size_t hash_size_out = 0;
    if (!btc_common_outputhash_from_pubkeyhash(
            script_type, derived_xpub.hash160, hash, &hash_size_out)) {
        return false;
    }
    return btc_common_address_from_outputhash(
        params, btc_common_determine_output_type(script_type), hash, hash_size_out, out, out_len);
}

app_btc_result_t app_btc_address_multisig_p2wsh(
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
    if (!btc_common_is_valid_keypath_address_multisig_p2wsh(
            keypath, keypath_len, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!btc_common_multisig_is_valid(multisig, keypath, keypath_len - 2, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    // Confirm previously registered multisig.
    uint8_t multisig_hash[SHA256_LEN] = {0};
    if (!btc_common_multisig_hash(coin, multisig, keypath, keypath_len - 2, multisig_hash)) {
        return APP_BTC_ERR_UNKNOWN;
    }
    char multisig_registered_name[MEMORY_MULTISIG_NAME_MAX_LEN] = {0};
    if (!memory_multisig_get_by_hash(multisig_hash, multisig_registered_name)) {
        // Not previously registered -> fail.
        return APP_BTC_ERR_INVALID_INPUT;
    }

    const char* title = "Receive to";
    if (!apps_btc_confirm_multisig(title, coin, multisig_registered_name, multisig, false)) {
        return APP_BTC_ERR_USER_ABORT;
    }

    uint8_t hash[SHA256_LEN] = {0};
    if (!btc_common_outputhash_from_multisig_p2wsh(multisig, keypath[4], keypath[5], hash)) {
        return APP_BTC_ERR_UNKNOWN;
    }

    if (!btc_common_address_from_outputhash(
            params, BTCOutputType_P2WSH, hash, sizeof(hash), out, out_len)) {
        return APP_BTC_ERR_UNKNOWN;
    }

    if (display) {
        if (!workflow_verify_pub(title, out)) {
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

bool app_btc_is_script_config_registered(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    bool* is_registered)
{
    // Only multisig registration supported for now.
    if (script_config->which_config != BTCScriptConfig_multisig_tag) {
        return false;
    }

    uint8_t hash[SHA256_LEN] = {0};
    if (!btc_common_multisig_hash(
            coin, &script_config->config.multisig, keypath, keypath_len, hash)) {
        return false;
    }

    *is_registered = memory_multisig_get_by_hash(hash, NULL);

    return true;
}

app_btc_result_t app_btc_register_script_config(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const char* name)
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
    if (!rust_util_validate_name(rust_util_cstr(name), MEMORY_MULTISIG_NAME_MAX_LEN - 1)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!btc_common_multisig_is_valid(multisig, keypath, keypath_len, params->bip44_coin)) {
        return APP_BTC_ERR_INVALID_INPUT;
    }

    if (!apps_btc_confirm_multisig("Register", coin, name, multisig, true)) {
        return APP_BTC_ERR_USER_ABORT;
    }

    uint8_t hash[SHA256_LEN] = {0};
    if (!btc_common_multisig_hash(coin, multisig, keypath, keypath_len, hash)) {
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
