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

#include <hardfault.h>
#include <keystore.h>
#include <workflow/verify_recipient.h>

#include <sha3.h>

#include <screen.h>

// https://github.com/ethereum/wiki/wiki/RLP
// If ctx is NULL, we skip the hashing.
// If encoded_len_out is NULL, we skip counting the bytes.
// If encoded_len_out is not NULL, we add to it, so it has to be initialized to 0 before the first
// call.
static void _hash_header(
    sha3_ctx* ctx,
    uint8_t small_tag,
    uint8_t large_tag,
    pb_size_t len,
    uint32_t* encoded_len_out)
{
    if (sizeof(len) != 2) {
        Abort("_hash_header: unexpected size");
    }
    if (len <= 55) {
        if (ctx != NULL) {
            uint8_t byte = small_tag + len;
            rhash_sha3_update(ctx, &byte, 1);
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 1;
        }
    } else if (len <= 0xff) {
        if (ctx != NULL) {
            uint8_t encoding[2] = {large_tag + 1, len};
            rhash_sha3_update(ctx, encoding, sizeof(encoding));
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 2;
        }
    } else {
        if (ctx != NULL) {
            uint8_t byte = large_tag + 2;
            rhash_sha3_update(ctx, &byte, 1);
            rhash_sha3_update(ctx, (const uint8_t*)&len, 2);
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 3;
        }
    }
}

// https://github.com/ethereum/wiki/wiki/RLP
// If ctx is NULL, we skip the hashing.
// If encoded_len_out is NULL, we skip counting the bytes.
// If encoded_len_out is not NULL, we add to it, so it has to be initialized to 0 before the first
// call.
static void _hash_element(
    sha3_ctx* ctx,
    const uint8_t* bytes,
    pb_size_t len,
    uint32_t* encoded_len_out)
{
    if (sizeof(len) != 2) {
        Abort("_hash_element: unexpected size");
    }
    // hash header
    if (len != 1 || bytes[0] > 0x7f) {
        _hash_header(ctx, 0x80, 0xb7, len, encoded_len_out);
    }
    if (ctx != NULL) {
        // hash bytes
        rhash_sha3_update(ctx, bytes, len);
    }
    if (encoded_len_out != NULL) {
        *encoded_len_out += len;
    }
}

static bool _compute_sighash(const ETHSignRequest* request, uint8_t chain_id, uint8_t* sighash_out)
{
    // We hash [nonce, gas price, gas limit, recipient, value, data], RLP encoded.
    // The list length prefix is (0xc0 + length of the encoding of all elements).
    // 1) calculate length
    uint32_t encoded_length = 0;
    _hash_element(NULL, request->nonce.bytes, request->nonce.size, &encoded_length);
    _hash_element(NULL, request->gas_price.bytes, request->gas_price.size, &encoded_length);
    _hash_element(NULL, request->gas_limit.bytes, request->gas_limit.size, &encoded_length);
    _hash_element(NULL, request->recipient, sizeof(request->recipient), &encoded_length);
    _hash_element(NULL, request->value.bytes, request->value.size, &encoded_length);
    _hash_element(NULL, request->data.bytes, request->data.size, &encoded_length);
    encoded_length += 3; // EIP155 part, see below.
    if (encoded_length > 0xffff) {
        // Don't support bigger than this for now.
        return false;
    }
    // 2) hash len and encoded tx elements
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    _hash_header(&ctx, 0xc0, 0xf7, (pb_size_t)encoded_length, NULL);
    _hash_element(&ctx, request->nonce.bytes, request->nonce.size, NULL);
    _hash_element(&ctx, request->gas_price.bytes, request->gas_price.size, NULL);
    _hash_element(&ctx, request->gas_limit.bytes, request->gas_limit.size, NULL);
    _hash_element(&ctx, request->recipient, sizeof(request->recipient), NULL);
    _hash_element(&ctx, request->value.bytes, request->value.size, NULL);
    _hash_element(&ctx, request->data.bytes, request->data.size, NULL);
    { // EIP155
        if (chain_id == 0 || chain_id > 0x7f) {
            Abort("chain id encoding error");
        }
        // encodes <chainID><0><0>
        uint8_t eip155_part[3] = {chain_id, 0x80, 0x80};
        rhash_sha3_update(&ctx, eip155_part, sizeof(eip155_part));
    }
    rhash_keccak_final(&ctx, sighash_out);
    return true;
}

app_eth_sign_error_t app_eth_sign(const ETHSignRequest* request, ETHSignResponse* response)
{
    app_eth_coin_params_t* params = app_eth_params_get(request->coin);
    if (params == NULL) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (!eth_common_is_valid_keypath(request->coin, request->keypath, request->keypath_count)) {
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
    if (request->value.size > 0 && request->value.bytes[0] == 0) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    // Verify the transaction (TODO)

    // Sign the transaction

    uint8_t sighash[32];
    if (!_compute_sighash(request, params->chain_id, sighash)) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    // Sign the hash and return the signature, with last byte set to recid.
    // check assumption
    if (sizeof(response->signature) != 65) {
        Abort("unexpected signature size");
    }
    int recid;
    if (!keystore_secp256k1_sign(
            request->keypath, request->keypath_count, sighash, response->signature, &recid)) {
        return APP_ETH_SIGN_ERR_UNKNOWN;
    }
    if (recid > 0xFF) {
        Abort("unexpected recid");
    }
    response->signature[64] = (uint8_t)recid;

    return APP_ETH_SIGN_OK;
}
