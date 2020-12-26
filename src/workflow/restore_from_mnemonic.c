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

#include "restore_from_mnemonic.h"

#include "confirm.h"
#include "password.h"
#include "status.h"
#include "unlock_bip39.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <securechip/securechip.h>
#include <util.h>
#include <workflow/confirm_time.h>

#include <stdio.h>
#include <string.h>

/// Size of the longest BIP39 English word (8 chars, without null terminator).
#define BIP39_MAX_WORD_LEN 8U

#define WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS 24

bool workflow_restore_from_mnemonic(const RestoreFromMnemonicRequest* request)
{
    // same as: MAX_WORD_LENGTH * MAX_WORDS + (MAX_WORDS - 1) + 1
    // (chars per word without null terminator) * (max words) + (spaces between words) + (null
    // terminator)
    char mnemonic[(BIP39_MAX_WORD_LEN + 1) * WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS] = {0};
    UTIL_CLEANUP_STR(mnemonic);

    if (!rust_workflow_mnemonic_get(rust_util_cstr_mut(mnemonic, sizeof(mnemonic)))) {
        return false;
    }
    uint8_t seed[32];
    UTIL_CLEANUP_32(seed);
    size_t seed_len = 0;
    if (!keystore_bip39_mnemonic_to_seed(mnemonic, seed, &seed_len)) {
        workflow_status_blocking("Recovery words\ninvalid", false);
        return false;
    }

    workflow_status_blocking("Recovery words\nvalid", true);

    char password[INPUT_STRING_MAX_SIZE] = {0};
    UTIL_CLEANUP_STR(password);
    // If entering password fails (repeat password does not match the first), we don't want to abort
    // the process immediately. We break out only if the user confirms.
    while (true) {
        if (!password_set(password)) {
            const confirm_params_t params = {
                .title = "",
                .body = "Passwords\ndo not match.\nTry again?",
            };

            if (!workflow_confirm_blocking(&params)) {
                return false;
            }
            continue;
        }
        break;
    }
    if (!keystore_encrypt_and_store_seed(seed, seed_len, password)) {
        workflow_status_blocking("Could not\nrestore backup", false);
        return false;
    }
#if APP_U2F == 1
    if (!workflow_confirm_time(request->timestamp, request->timezone_offset, false)) {
        return false;
    }
    if (!securechip_u2f_counter_set(request->timestamp)) {
        // ignore error
    }
#else
    (void)request;
#endif
    if (!memory_set_initialized()) {
        return false;
    }
    uint8_t remaining_attempts;
    if (keystore_unlock(password, &remaining_attempts) != KEYSTORE_OK) {
        // This should/can never happen, but let's check anyway.
        Abort("workflow_restore_from_mnemonic: unlock failed");
    }
    workflow_unlock_bip39_blocking();
    return true;
}
