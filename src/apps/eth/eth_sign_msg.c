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

#include "eth_sign_msg.h"
#include "eth_common.h"
#include "eth_sighash.h"
#include "eth_verify.h"
#include <sha3.h>

#include "eth.h"
#include <hardfault.h>
#include <keystore.h>
#include <rust/rust.h>
#include <util.h>
#include <workflow/confirm.h>

app_eth_sign_error_t app_eth_sign_msg(
    const ETHSignMessageRequest* request,
    ETHSignResponse* response)
{
    // To update this number you need to update the buffer size later.
    if (request->msg.size > 9999) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    // Only support main net for now. Otherwise a user could be tricked into signing something for
    // main net even if they believe they are signing for testnet.
    if (request->coin != ETHCoin_ETH) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    // Let user verify that it is signing for the expected address
    char address[APP_ETH_ADDRESS_HEX_LEN] = {0};
    if (!app_eth_address(
            request->coin,
            ETHPubRequest_OutputType_ADDRESS,
            request->keypath,
            request->keypath_count,
            address,
            sizeof(address))) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    {
        confirm_params_t params = {
            .title = "Sign message\nYour address",
            .body = address,
            .scrollable = true,
            .accept_is_nextarrow = true,
        };
        if (!workflow_confirm_blocking(&params)) {
            return APP_ETH_SIGN_ERR_USER_ABORT;
        }
    }

    const char msg_header[] =
        "\x19"
        "Ethereum Signed Message:\n";
    // sizeof(msg_header) includes null terminator
    // the maximum length of the signed data is 1024, therefore 4 bytes might be needed as length
    // prefix.
    char msg[sizeof(msg_header) - 1 + sizeof(request->msg.bytes) + 4] = {0};

    // payload_offset is also the length of the fixed header + payload size prefix
    size_t payload_offset = snprintf(msg, sizeof(msg), "%s%d", msg_header, request->msg.size);
    memcpy(&msg[payload_offset], request->msg.bytes, request->msg.size);

    // determine if the message is in ASCII
    bool all_ascii =
        rust_util_all_ascii_bytes(rust_util_bytes(request->msg.bytes, request->msg.size));

    char body[sizeof(request->msg.bytes) * 2 + 1] = {0};
    confirm_params_t params = {
        .body = body,
        .scrollable = true,
        .shorten_body = true,
        .longtouch = true,
    };
    if (all_ascii) {
        // If it is all ASCII, copy the bytes over and ensure there is a null terminator
        snprintf(body, sizeof(body), "%.*s", request->msg.size, request->msg.bytes);
        params.title = "Sign message\nData";
    } else {
        // If it is binary, convert to hex
        util_uint8_to_hex(request->msg.bytes, request->msg.size, body);
        params.title = "Sign message\nData (hex)";
        params.display_size = request->msg.size;
    }

    if (!workflow_confirm_blocking(&params)) {
        return APP_ETH_SIGN_ERR_USER_ABORT;
    }

    // Calculate the hash
    uint8_t sighash[sha3_256_hash_size];
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    rhash_sha3_update(&ctx, (const unsigned char*)msg, payload_offset + request->msg.size);
    rhash_keccak_final(&ctx, sighash);

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
