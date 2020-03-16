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

#include "eth.h"
#include "eth_params.h"
#include <apps/btc/btc_common.h>

#include <keystore.h>
#include <rust/rust.h>

#include <hww.pb.h>
#include <secp256k1.h>

bool app_eth_address(
    ETHCoin coin,
    ETHPubRequest_OutputType output_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len)
{
    if (coin > _ETHCoin_MAX) {
        return false;
    }

    const app_eth_coin_params_t* params = app_eth_params_get(coin);
    if (params == NULL) {
        return false;
    }

    switch (output_type) {
    case ETHPubRequest_OutputType_ADDRESS: {
        if (!rust_ethereum_keypath_is_valid_address(keypath, keypath_len, params->bip44_coin)) {
            return false;
        }
        uint8_t pubkey_uncompressed[65];
        if (!keystore_secp256k1_pubkey(
                KEYSTORE_SECP256K1_PUBKEY_UNCOMPRESSED,
                keypath,
                keypath_len,
                pubkey_uncompressed,
                sizeof(pubkey_uncompressed))) {
            return false;
        }
        rust_ethereum_address_from_pubkey(
            rust_util_bytes(pubkey_uncompressed, sizeof(pubkey_uncompressed)),
            rust_util_cstr_mut(out, out_len));
        return true;
    }
    case ETHPubRequest_OutputType_XPUB: {
        if (!rust_ethereum_keypath_is_valid_xpub(keypath, keypath_len, params->bip44_coin)) {
            return false;
        }
        struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
        if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
            return false;
        }
        return btc_common_encode_xpub(&derived_xpub, BTCPubRequest_XPubType_XPUB, out, out_len);
    }
    default:
        return false;
    }
}
