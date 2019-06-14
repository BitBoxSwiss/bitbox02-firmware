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
#include "workflow.h"
#include <hardfault.h>
#include <keystore.h>
#include <memory.h>
#include <reset.h>
#include <string.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>
#ifndef TESTING
#include <hal_delay.h>
#endif

static bool _done = true;
static bool _is_done(void)
{
    return _done;
}

static void _enter_mnemonic_passphrase(void);

static char _mnemonic_passphrase_unconfirmed[SET_PASSWORD_MAX_PASSWORD_LENGTH];

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
    _done = true;
}

static void _mnemonic_passphrase_confirm_done(const char* passphrase)
{
    bool equal = STREQ(_mnemonic_passphrase_unconfirmed, passphrase);
    util_zero(_mnemonic_passphrase_unconfirmed, sizeof(_mnemonic_passphrase_unconfirmed));
    if (equal) {
        _finish_bip39(passphrase);
    } else {
        ui_screen_stack_switch(status_create(
            "Passphrases\ndo not match", false, STATUS_DEFAULT_DELAY, _enter_mnemonic_passphrase));
    }
}

static void _passphrase_confirm_enter(void)
{
    ui_screen_stack_switch(set_password_create(_mnemonic_passphrase_confirm_done));
}

static void _mnemonic_passphrase_enter_done(const char* passphrase)
{
    int snprintf_result = snprintf(
        _mnemonic_passphrase_unconfirmed,
        sizeof(_mnemonic_passphrase_unconfirmed),
        "%s",
        passphrase);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_mnemonic_passphrase_unconfirmed)) {
        Abort("length mismatch");
    }
    ui_screen_stack_switch(
        entry_screen_create("Confirm\nmnemonic passphrase", _passphrase_confirm_enter));
}

static void _passphrase_enter(void)
{
    ui_screen_stack_switch(set_password_create(_mnemonic_passphrase_enter_done));
}

static void _enter_mnemonic_passphrase(void)
{
    memset(_mnemonic_passphrase_unconfirmed, 0, sizeof(_mnemonic_passphrase_unconfirmed));
    ui_screen_stack_switch(entry_screen_create("Enter\nmnemonic passphrase", _passphrase_enter));
}

static void _enter(void)
{
    ui_screen_stack_switch(set_password_create(workflow_unlock_enter_done));
}

static void _workflow_unlock(void)
{
    // "Enter password"
    ui_screen_stack_push(entry_screen_create("Enter password", _enter));
}

void workflow_unlock_enter_done(const char* password)
{
    uint8_t remaining_attempts = 0;
    keystore_error_t unlock_result = keystore_unlock(password, &remaining_attempts);
    switch (unlock_result) {
    case KEYSTORE_OK:
        if (memory_is_mnemonic_passphrase_enabled()) {
            _enter_mnemonic_passphrase();
        } else {
            // Empty passphrase by default.
            _finish_bip39("");
        }
        break;
    case KEYSTORE_ERR_INCORRECT_PASSWORD: {
        char msg[100] = {0};
        if (remaining_attempts == 1) {
            snprintf(msg, sizeof(msg), "Wrong password\n1 try remains");
        } else {
            snprintf(msg, sizeof(msg), "Wrong password\n%d tries remain", remaining_attempts);
        }
        ui_screen_stack_switch(status_create(msg, false, STATUS_DEFAULT_DELAY, _workflow_unlock));
        break;
    }
    case KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED:
        reset_reset();
        ui_screen_stack_switch(
            status_create("Device reset", false, STATUS_DEFAULT_DELAY, workflow_start));
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
    _done = false;
    ui_screen_stack_pop_all();
    _workflow_unlock();
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
}
