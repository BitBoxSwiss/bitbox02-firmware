// Copyright 2023 Shift Crypto AG
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

#include "bip32.h"

#include <string.h>
#include <wally_bip32.h>

bool bip32_derive_xpub(
    const uint8_t* xpub78,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* xpub78_out)
{
    if (keypath_len == 0) {
        memcpy(xpub78_out, xpub78, BIP32_SERIALIZED_LEN);
        return true;
    }

    struct ext_key xpub = {0};
    if (bip32_key_unserialize(xpub78, BIP32_SERIALIZED_LEN, &xpub) != WALLY_OK) {
        return false;
    }
    struct ext_key derived_xpub = {0};
    if (bip32_key_from_parent_path(
            &xpub, keypath, keypath_len, BIP32_FLAG_KEY_PUBLIC, &derived_xpub) != WALLY_OK) {
        return false;
    }
    return bip32_key_serialize(
               &derived_xpub, BIP32_FLAG_KEY_PUBLIC, xpub78_out, BIP32_SERIALIZED_LEN) == WALLY_OK;
}
