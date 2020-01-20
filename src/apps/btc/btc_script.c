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

#include "btc_script.h"
#include "btc_common.h"

#include <keystore.h>
#include <wally_bip32.h>

bool btc_script_outputhash_at_keypath(
    BTCScriptType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }
    return btc_common_outputhash_from_pubkeyhash(
        script_type, derived_xpub.hash160, output_hash, output_hash_size);
}
