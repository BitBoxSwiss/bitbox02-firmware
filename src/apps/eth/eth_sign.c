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
#include "eth_sign.h"
#include "eth_common.h"
#include "eth_params.h"
#include "eth_sighash.h"
#include "eth_verify.h"

#include <hardfault.h>
#include <keystore.h>
#include <rust/rust.h>
#include <util.h>

app_eth_sign_error_t app_eth_sign(const ETHSignRequest* request, ETHSignResponse* response)
{
    const app_eth_coin_params_t* params = app_eth_params_get(request->coin);
    if (params == NULL) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    if (!rust_ethereum_keypath_is_valid_address(
            request->keypath, request->keypath_count, params->bip44_coin)) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    // No zero prefix in the big endian numbers.
    if (request->nonce.size > 0 && request->nonce.bytes[0] == 0) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->gas_price.size > 0 && request->gas_price.bytes[0] == 0) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->gas_limit.size > 0 && request->gas_limit.bytes[0] == 0) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    uint8_t empty[20] = {0};
    if (MEMEQ(request->recipient, empty, sizeof(empty))) {
        // Reserved for contract creation.
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->value.size > 0 && request->value.bytes[0] == 0) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    // User verification
    const uint8_t erc20_transfer[4] = {0xa9, 0x05, 0x9c, 0xbb};
    bool is_erc20_transfer = request->value.size == 0 && request->data.size == 68 &&
                             MEMEQ(request->data.bytes, erc20_transfer, sizeof(erc20_transfer));
    if (is_erc20_transfer) {
        app_eth_sign_error_t result = app_eth_verify_erc20_transaction(request);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
    } else {
        app_eth_sign_error_t result = app_eth_verify_standard_transaction(request);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
    }

    // Sign the transaction

    uint8_t sighash[32];
    eth_sighash_params_t sighash_params = {
        .nonce =
            {
                .data = request->nonce.bytes,
                .len = request->nonce.size,
            },
        .gas_price =
            {
                .data = request->gas_price.bytes,
                .len = request->gas_price.size,
            },
        .gas_limit =
            {
                .data = request->gas_limit.bytes,
                .len = request->gas_limit.size,
            },
        .recipient =
            {
                .data = request->recipient,
                .len = sizeof(request->recipient),
            },
        .value =
            {
                .data = request->value.bytes,
                .len = request->value.size,
            },
        .data =
            {
                .data = request->data.bytes,
                .len = request->data.size,
            },
        .chain_id = params->chain_id,
    };
    if (!app_eth_sighash(sighash_params, sighash)) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    // Sign the hash and return the signature, with last byte set to recid.
    // check assumption
    if (sizeof(response->signature) != 65) {
        Abort("unexpected signature size");
    }
    int recid;
    uint8_t host_nonce[32] = {0}; // TODO: get nonce contribution from host.
    if (!keystore_secp256k1_sign(
            request->keypath,
            request->keypath_count,
            sighash,
            host_nonce,
            response->signature,
            &recid)) {
        return APP_ETH_SIGN_ERR_UNKNOWN;
    }
    if (recid > 0xFF) {
        Abort("unexpected recid");
    }
    response->signature[64] = (uint8_t)recid;

    return APP_ETH_SIGN_OK;
}
