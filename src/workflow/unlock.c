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
#include "workflow.h"
#include <hardfault.h>
#include <keystore.h>
#include <memory.h>
#include <string.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>
#ifndef TESTING
#include <hal_delay.h>
#endif

static void _finish_bip39(const char* passphrase)
{
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
    if (!keystore_unlock_bip39(passphrase)) {
        Abort("bip39 unlock failed");
    }
}

static void _workflow_unlock(void)
{
    ui_screen_stack_pop_all();
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    password_enter("Enter password", password);
    workflow_unlock_enter_done(password);
    util_zero(password, sizeof(password));
}

static void _get_mnemonic_passphrase(char* passphrase_out)
{
    char mnemonic_passphrase[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    char mnemonic_passphrase_repeat[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    bool equal = false;
    while (!equal) {
        password_enter("Enter\nmnemonic passphrase", mnemonic_passphrase);
        password_enter("Confirm\nmnemonic passphrase", mnemonic_passphrase_repeat);
        equal = STREQ(mnemonic_passphrase, mnemonic_passphrase_repeat);
        if (equal) {
            snprintf(passphrase_out, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", mnemonic_passphrase);
            break;
        }
        workflow_status_create("Passphrases\ndo not match");
    }
    util_zero(mnemonic_passphrase, sizeof(mnemonic_passphrase));
    util_zero(mnemonic_passphrase_repeat, sizeof(mnemonic_passphrase_repeat));
}

void workflow_unlock_enter_done(const char* password)
{
    uint8_t remaining_attempts = 0;
    keystore_error_t unlock_result = keystore_unlock(password, &remaining_attempts);
    switch (unlock_result) {
    case KEYSTORE_OK: {
        // Empty passphrase by default.
        char mnemonic_passphrase[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
        if (memory_is_mnemonic_passphrase_enabled()) {
            _get_mnemonic_passphrase(mnemonic_passphrase);
        }
        _finish_bip39(mnemonic_passphrase);
        util_zero(mnemonic_passphrase, sizeof(mnemonic_passphrase));
        break;
    }
    case KEYSTORE_ERR_INCORRECT_PASSWORD: {
        char msg[100] = {0};
        if (remaining_attempts == 1) {
            snprintf(msg, sizeof(msg), "Wrong password\n1 try remains");
        } else {
            snprintf(msg, sizeof(msg), "Wrong password\n%d tries remain", remaining_attempts);
        }
        workflow_status_create(msg);
        _workflow_unlock();
        break;
    }
    case KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED:
        workflow_status_create("Device reset");
        workflow_start();
        break;
    default:
        Abort("keystore unlock failed");
        break;
    }
}

void workflow_unlock(void)
{
    if (!memory_is_initialized() || !keystore_is_locked()) {
        return;
    }
    _workflow_unlock();
}
