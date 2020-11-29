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
#include "eth_verify.h"
#include <workflow/confirm.h>

#include <keystore.h>
#include <rust/rust.h>
#include <util.h>

#include <hww.pb.h>
#include <secp256k1.h>

static const char* _coin_eth = "Ethereum";
static const char* _coin_ropsten_eth = "Ropsten";
static const char* _coin_rinkeby_eth = "Rinkeby";

app_eth_sign_error_t app_eth_address(
    ETHCoin coin,
    ETHPubRequest_OutputType output_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len,
    bool display,
    const uint8_t* contract_address)
{
    if (coin > _ETHCoin_MAX) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    const app_eth_coin_params_t* params = app_eth_params_get(coin);
    if (params == NULL) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }

    switch (output_type) {
    case ETHPubRequest_OutputType_ADDRESS: {
        if (!rust_ethereum_keypath_is_valid_address(keypath, keypath_len, params->bip44_coin)) {
            return APP_ETH_SIGN_ERR_INVALID_INPUT;
        }
        uint8_t pubkey_uncompressed[65];
        if (!keystore_secp256k1_pubkey(
                KEYSTORE_SECP256K1_PUBKEY_UNCOMPRESSED,
                keypath,
                keypath_len,
                pubkey_uncompressed,
                sizeof(pubkey_uncompressed))) {
            return APP_ETH_SIGN_ERR_INVALID_INPUT;
        }
        rust_ethereum_address_from_pubkey(
            rust_util_bytes(pubkey_uncompressed, sizeof(pubkey_uncompressed)),
            rust_util_cstr_mut(out, out_len));
        break;
    }
    case ETHPubRequest_OutputType_XPUB: {
        if (!rust_ethereum_keypath_is_valid_xpub(keypath, keypath_len, params->bip44_coin)) {
            return APP_ETH_SIGN_ERR_INVALID_INPUT;
        }
        struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
        if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
            return APP_ETH_SIGN_ERR_INVALID_INPUT;
        }
        if (!keystore_encode_xpub(&derived_xpub, XPUB, out, out_len)) {
            return APP_ETH_SIGN_ERR_UNKNOWN;
        }
        break;
    }
    default:
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (display) {
        if (output_type != ETHPubRequest_OutputType_ADDRESS) {
            // Only support displaying the address for now.
            return APP_ETH_SIGN_ERR_INVALID_INPUT;
        }
        const char* coin_name;

        // Check for ERC20-Address
        uint8_t zero[20] = {0};
        if (contract_address != NULL && !MEMEQ(contract_address, zero, sizeof(zero))) {
            const app_eth_erc20_params_t* erc20_params =
                app_eth_erc20_params_get(coin, contract_address);
            if (erc20_params == NULL) {
                return APP_ETH_SIGN_ERR_INVALID_INPUT;
            }
            coin_name = erc20_params->name;
        } else {
            switch (coin) {
            case ETHCoin_ETH:
                coin_name = _coin_eth;
                break;
            case ETHCoin_RopstenETH:
                coin_name = _coin_ropsten_eth;
                break;
            case ETHCoin_RinkebyETH:
                coin_name = _coin_rinkeby_eth;
                break;
            default:
                return APP_ETH_SIGN_ERR_INVALID_INPUT;
            }
        }
        const confirm_params_t confirm_params = {
            .title = coin_name,
            // Some long ERC-20 token names need to be broken into two lines.
            .title_autowrap = true,
            .body = out,
            .scrollable = true,
        };
        if (!workflow_confirm_blocking(&confirm_params)) {
            return APP_ETH_SIGN_ERR_USER_ABORT;
        }
    }
    return APP_ETH_SIGN_OK;
}
