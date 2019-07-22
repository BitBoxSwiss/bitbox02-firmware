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

#ifndef _APPS_ETH_SIGHASH_H
#define _APPS_ETH_SIGHASH_H

#include <compiler_util.h>
#include <generated/hww.pb.h>

#include <stdbool.h>
#include <stdint.h>

/**
 * Computes the sighash of an Ethereum transaction, using the chain_id as described in EIP155.
 * @param[in] request transaction data. coin and keypath are ignored. nonce, gas_price, gas_limit,
 * an value are big endian and are not allowed to have leading zeros (unchecked).
 * @param[in] chain_id EIP155 chain id.
 * @param[out] sighash_out 32 bytes hash to be signed.
 */
USE_RESULT bool app_eth_sighash(
    const ETHSignRequest* request,
    uint8_t chain_id,
    uint8_t* sighash_out);

#endif
