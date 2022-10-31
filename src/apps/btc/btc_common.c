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

#include <stdio.h>
#include <stdlib.h>

#include "btc_common.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <util.h>
#include <wally_address.h>

bool btc_common_pkscript_from_multisig(
    const multisig_t* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* script_out,
    size_t* script_out_size)
{
    uint8_t pubkeys[MULTISIG_P2WSH_MAX_SIGNERS * EC_PUBLIC_KEY_LEN];

    for (size_t index = 0; index < multisig->xpubs_count; index++) {
        struct ext_key xpub = {0};
        if (bip32_key_unserialize(multisig->xpubs[index], sizeof(multisig->xpubs[index]), &xpub) !=
            WALLY_OK) {
            return false;
        }
        struct ext_key derived_cosigner_xpub = {0};
        const uint32_t keypath[2] = {keypath_change, keypath_address};
        if (bip32_key_from_parent_path(
                &xpub, keypath, 2, BIP32_FLAG_KEY_PUBLIC, &derived_cosigner_xpub) != WALLY_OK) {
            return false;
        }
        memcpy(
            &pubkeys[index * EC_PUBLIC_KEY_LEN], derived_cosigner_xpub.pub_key, EC_PUBLIC_KEY_LEN);
    }

    size_t written;
    if (wally_scriptpubkey_multisig_from_bytes(
            pubkeys,
            multisig->xpubs_count * EC_PUBLIC_KEY_LEN,
            multisig->threshold,
            WALLY_SCRIPT_MULTISIG_SORTED,
            script_out,
            *script_out_size,
            &written) != WALLY_OK) {
        return false;
    }
    if (written > *script_out_size) {
        // Double check since the function above sets written to script_len if the buffer was too
        // short.
        return false;
    }
    *script_out_size = written;

    return true;
}
