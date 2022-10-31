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

#ifndef _APPS_BTC_COMMON_H
#define _APPS_BTC_COMMON_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>
#include <keystore.h>

#include <wally_bip32.h>
#include <wally_crypto.h>
#include <wally_script.h>

#define MULTISIG_P2WSH_MAX_SIGNERS 15

typedef struct {
    size_t xpubs_count;
    uint8_t xpubs[MULTISIG_P2WSH_MAX_SIGNERS][BIP32_SERIALIZED_LEN];
    uint32_t threshold;
} multisig_t;

// see https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
#define MAX_VARINT_SIZE (9)
// current expected max pk script size is a m-of-15 multisig. 700 is also enough for m-of-20, which
// is technically possible to extend to if needed.
#define MAX_PK_SCRIPT_SIZE (700)

/**
 * Creates a n-of-m multisig script based on OP_CHECKMULTISIG. 0<n<=m<=15.
 * Note that the multisig config and keypaths are *not* validated, this must be done before calling.
 * @param[in] multisig Multisig configuration (threshold, signers). The xpubs are account-level
 * xpubs.
 * @param[in] keypath_change 0 for receive addresses, 1 for change addresses.
 * @param[in] keypath_address receive address index.
 * @param[out] script_out script to be created. Must be at least 517 bytes.
 * @param[out] script_out_size The size of the generated script.
 * @return true on success, false on failure.
 */
USE_RESULT bool btc_common_pkscript_from_multisig(
    const multisig_t* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* script_out,
    size_t* script_out_size);

#endif
