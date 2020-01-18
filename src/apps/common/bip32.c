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

#include "bip32.h"

bool apps_common_bip32_xpub_from_protobuf(const XPub* xpub_in, struct ext_key* xpub_out)
{
    struct ext_key* tmp = NULL;
    // libwally quirk: bip32_key_init_alloc takes 20 bytes for the parent fingerprint but only uses
    // the first 4 bytes (the real size of the fingerprint).
    uint8_t parent_fingerprint[20] = {0};
    memcpy(parent_fingerprint, xpub_in->parent_fingerprint, sizeof(xpub_in->parent_fingerprint));
    // Another libwally quirk: no function to create an ext_key from params without malloc.
    if (bip32_key_init_alloc(
            BIP32_VER_MAIN_PUBLIC,
            xpub_in->depth[0],
            xpub_in->child_num,
            xpub_in->chain_code,
            sizeof(xpub_in->chain_code),
            xpub_in->public_key,
            sizeof(xpub_in->public_key),
            NULL,
            0,
            NULL,
            0,
            parent_fingerprint,
            sizeof(parent_fingerprint),
            &tmp) != WALLY_OK) {
        return false;
    }
    *xpub_out = *tmp;
    bip32_key_free(tmp);
    return true;
}
