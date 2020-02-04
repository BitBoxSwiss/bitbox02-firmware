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
#include "eth_common.h"
#include <apps/btc/btc_common.h>

#include <keystore.h>

#include <hww.pb.h>
#include <secp256k1.h>
#include <sha3.h>

static bool _address(const uint8_t* pubkey_uncompressed, char* out, size_t out_len)
{
    uint8_t hash[32];
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    rhash_sha3_update(&ctx, pubkey_uncompressed + 1, 64);
    rhash_keccak_final(&ctx, hash);
    uint8_t* last20 = hash + sizeof(hash) - APP_ETH_RECIPIENT_BYTES_LEN;
    return eth_common_hexaddress(last20, out, out_len);
}

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

    switch (output_type) {
    case ETHPubRequest_OutputType_ADDRESS: {
        if (!eth_common_is_valid_keypath_address(coin, keypath, keypath_len)) {
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
        return _address(pubkey_uncompressed, out, out_len);
    }
    case ETHPubRequest_OutputType_XPUB: {
        if (!eth_common_is_valid_keypath_xpub(coin, keypath, keypath_len)) {
            return false;
        }
        struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
        if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
            return false;
        }
        const uint8_t version[4] = {0x04, 0x88, 0xb2, 0x1e}; // xpub
        return btc_common_encode_xpub(&derived_xpub, version, out, out_len);
    }
    default:
        return false;
    }
}
