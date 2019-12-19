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
#include "confirm.h"
#include "password_enter.h"
#include "status.h"
#include "workflow.h"
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <screen.h>
#include <string.h>
#include <ui/components/ui_images.h>
#include <ui/screen_stack.h>
#include <ui/ugui/ugui.h>
#include <util.h>
#ifndef TESTING
#include <hal_delay.h>
#endif

#include <stdio.h>

/**
 * @return true if passphrase consists only of spaces, or is empty.
 */
static bool _is_only_spaces(const char* passphrase)
{
    for (const char* c = passphrase; *c != '\0'; c++) {
        if (*c != ' ') {
            return false;
        }
    }
    return true;
}

/**
 * @return true if the passphrase starts or ends with a space or contains consecutive spaces.
 */
static bool _has_dangerous_spaces(const char* passphrase)
{
    size_t len = strlen(passphrase);
    if (len == 0) {
        return false;
    }
    if (passphrase[0] == ' ' || passphrase[len - 1] == ' ') {
        return true;
    }
    // Check for consecutive spaces.
    for (size_t i = 0; i < len - 1; i++) {
        if (passphrase[i] == ' ' && passphrase[i + 1] == ' ') {
            return true;
        }
    }
    return false;
}

static bool _get_mnemonic_passphrase(char* passphrase_out)
{
    if (passphrase_out == NULL) {
        Abort("_get_mnemonic_passphrase");
    }
    while (true) {
        if (!password_enter("Enter\noptional passphrase", true, passphrase_out)) {
            return false;
        }
        if (strlen(passphrase_out) == 0) {
            // No need to confirm the empty passphrase.
            break;
        }
        if (_is_only_spaces(passphrase_out)) {
            workflow_status_create("Invalid passphrase\nPlease try again", false);
            continue;
        }
        if (!workflow_confirm(
                "",
                "You will be asked to\nvisually confirm your\npassphrase now.",
                NULL,
                false,
                true)) {
            return false;
        }
        if (_has_dangerous_spaces(passphrase_out)) {
            if (!workflow_confirm_scrollable(
                    "Danger",
                    "Your passphrase starts or ends with a space, or contains consecutive spaces.",
                    NULL,
                    true)) {
                return false;
            }
        }
        bool cancel_forced = false;
        if (workflow_confirm_scrollable_longtouch(
                "Confirm", passphrase_out, NULL, &cancel_forced)) {
            break;
        }
        if (cancel_forced) {
            return false;
        }
        workflow_status_create("Please try again", false);
    }
    return true;
}

bool workflow_unlock_bip39(void)
{
    // Empty passphrase by default.
    char mnemonic_passphrase[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(mnemonic_passphrase);
    if (memory_is_mnemonic_passphrase_enabled()) {
        if (!_get_mnemonic_passphrase(mnemonic_passphrase)) {
            return false;
        }
    }

    { // animation
        // Cannot render screens during unlocking (unlocking blocks)
        // Therefore hardcode a status screen
        UG_ClearBuffer();
        image_lock(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 1, IMAGE_DEFAULT_LOCK_RADIUS);
        UG_SendBuffer();
#ifndef TESTING
        delay_ms(1200);
#endif
        UG_ClearBuffer();
        image_unlock(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2 - 1, IMAGE_DEFAULT_LOCK_RADIUS);
        UG_SendBuffer();
    }

    if (!keystore_unlock_bip39(mnemonic_passphrase)) {
        Abort("bip39 unlock failed");
    }
    return true;
}

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
        workflow_status_create(msg, false);
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
        if (!password_enter("Enter password", false, password)) {
            return false;
        }

        keystore_error_t unlock_result = workflow_unlock_and_handle_error(password);
        if (unlock_result == KEYSTORE_OK) {
            // Keystore unlocked, now unlock bip39 seed.
            if (!workflow_unlock_bip39()) {
                return false;
            }
            break;
        }
        if (unlock_result == KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED) {
            // Device reset
            break;
        }
    }
    return true;
}
