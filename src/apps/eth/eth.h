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

#ifndef _APPS_ETH_H
#define _APPS_ETH_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

#include "eth_verify.h"
#include <hww.pb.h>

/**
 * Retrieves a specified encoding of an ethereum address.
 * @param[in] output_type
 * if ADDRESS, produces a checksummed ethereum address.
 * if XPUB, produces the xpub.
 * @param[out] out will hold the result.
 * @param[in] out_len must be at least APP_ETH_ADDRESS_HEX_LEN for ADDRESS, and at least 80
 * (BIP32_SERIALIZED_LEN+1) for XPUB.
 * @param[in] display Wether to ask user to confirm address
 * @param[in] contract_address pointer to contract address
 */
USE_RESULT app_eth_sign_error_t app_eth_address(
    ETHCoin coin,
    ETHPubRequest_OutputType output_type,
    const uint32_t* keypath,
    size_t keypath_len,
    char* out,
    size_t out_len,
    bool display,
    const uint8_t* contract_address);

#endif
