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

#ifndef _APPS_BTC_SCRIPT_H
#define _APPS_BTC_SCRIPT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>
#include <hww.pb.h>

/**
 * Converts a pubkey at the keypath to a hash used in an output script, e.g. pubkeyhash or script
 * hash.
 * It can then be used to create an address.
 * @param[in] script_type script type defining the hash.
 * @param[in] keypath keypath at which to create the script.
 * @param[in] keypath_len number of keypath elements.
 * @param[out] output_hash will have the resulting hash. Must be of size 32.
 * @param[out] output_hash_size will be 32 for p2wsh scripts, HASH160_LEN for
 * all others.
 * return true on success, false on failure.
 */
USE_RESULT bool btc_script_outputhash_at_keypath(
    BTCScriptType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* output_hash,
    size_t* output_hash_size);

#endif
