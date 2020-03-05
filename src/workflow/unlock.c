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

#include "unlock.h"
#include "password_enter.h"
#include "status.h"
#include "unlock_bip39.h"
#include "workflow.h"
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <screen.h>
#include <string.h>
#include <ui/screen_stack.h>
#include <util.h>
#include <workflow/get_mnemonic_passphrase.h>

#include <stdio.h>

keystore_error_t workflow_unlock_and_handle_error(const char* password)
{
    uint8_t remaining_attempts = 0;
    keystore_error_t unlock_result = keystore_unlock(password, &remaining_attempts);
    switch (unlock_result) {
    case KEYSTORE_OK:
    case KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED:
        break;
    case KEYSTORE_ERR_INCORRECT_PASSWORD: {
        char msg[100] = {0};
        if (remaining_attempts == 1) {
            snprintf(msg, sizeof(msg), "Wrong password\n1 try remains");
        } else {
            snprintf(msg, sizeof(msg), "Wrong password\n%d tries remain", remaining_attempts);
        }
        workflow_status_blocking(msg, false);
        break;
    }
    default:
        Abort("keystore unlock failed");
    }
    return unlock_result;
}

bool workflow_unlock(void)
{
    if (!memory_is_initialized()) {
        return false;
    }
    if (!keystore_is_locked()) {
        return true;
    }

    ui_screen_stack_pop_all();

    // Repeat attempting to unlock until success or device reset.
    while (true) {
        char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
        UTIL_CLEANUP_STR(password);
        password_enter_blocking("Enter password", false, password);

        keystore_error_t unlock_result = workflow_unlock_and_handle_error(password);
        if (unlock_result == KEYSTORE_OK) {
            // Keystore unlocked, now unlock bip39 seed.
            workflow_unlock_bip39_blocking();
            break;
        }
        if (unlock_result == KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED) {
            // Device reset
            break;
        }
    }
    return true;
}
