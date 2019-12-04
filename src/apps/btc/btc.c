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

#include <hww.pb.h>
#include <keystore.h>

static const uint8_t _xpub_version[4] = {0x04, 0x88, 0xb2, 0x1e};
static const uint8_t _ypub_version[4] = {0x04, 0x9d, 0x7c, 0xb2};
static const uint8_t _zpub_version[4] = {0x04, 0xb2, 0x47, 0x46};
static const uint8_t _tpub_version[4] = {0x04, 0x35, 0x87, 0xcf};
static const uint8_t _vpub_version[4] = {0x04, 0x5f, 0x1c, 0xf6};
static const uint8_t _upub_version[4] = {0x04, 0x4a, 0x52, 0x62};

bool app_btc_address(
    BTCCoin coin,
    BTCPubRequest_OutputType output_type,
    BTCScriptType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    char* out,
    size_t out_len)
{
    const app_btc_coin_params_t* params = app_btc_params_get(coin);
    if (params == NULL) {
        return false;
    }
    if (!btc_common_is_valid_keypath(
            output_type, script_type, keypath, keypath_len, params->bip44_coin)) {
        return false;
    }
    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }
    switch (output_type) {
    case BTCPubRequest_OutputType_TPUB:
        return btc_common_encode_xpub(&derived_xpub, _tpub_version, out, out_len);
    case BTCPubRequest_OutputType_VPUB:
        return btc_common_encode_xpub(&derived_xpub, _vpub_version, out, out_len);
    case BTCPubRequest_OutputType_UPUB:
        return btc_common_encode_xpub(&derived_xpub, _upub_version, out, out_len);
    case BTCPubRequest_OutputType_XPUB:
        return btc_common_encode_xpub(&derived_xpub, _xpub_version, out, out_len);
    case BTCPubRequest_OutputType_YPUB:
        return btc_common_encode_xpub(&derived_xpub, _ypub_version, out, out_len);
    case BTCPubRequest_OutputType_ZPUB:
        return btc_common_encode_xpub(&derived_xpub, _zpub_version, out, out_len);
    case BTCPubRequest_OutputType_ADDRESS: {
        uint8_t hash[32] = {0};
        size_t hash_size_out = 0;
        if (!btc_common_outputhash_from_pubkeyhash(
                script_type, derived_xpub.hash160, hash, &hash_size_out)) {
            return false;
        }
        return btc_common_address_from_outputhash(
            params,
            btc_common_determine_output_type(script_type),
            hash,
            hash_size_out,
            out,
            out_len);
    }
    default:
        return false;
    }
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
