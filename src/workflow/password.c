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

#include <string.h>

#include "password.h"

#include "password_enter.h"
#include "unlock.h"
#include "workflow.h"
#include <hardfault.h>
#include <memory.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>

#include <stdio.h>

static char _password_candidate[SET_PASSWORD_MAX_PASSWORD_LENGTH];
// true after the workflow has finished (passwords entered and match or do not
// match).
static bool _done = true;
// whether the passwords match or do not match (undefined if _done is false).
static bool _result = false;
static bool _is_done(void)
{
    return _done;
}
// The callback that is called if the passwords match
static bool (*_callback)(const char* password) = NULL;

static void _match_callback(void)
{
    // Need to pop here before the callback is called, as the callback can add
    // more screens.
    ui_screen_stack_pop();

    _result = _callback(_password_candidate);
    util_zero(_password_candidate, sizeof(_password_candidate));
    _done = true;
}

static void _no_match_callback(void)
{
    util_zero(_password_candidate, sizeof(_password_candidate));
    _result = false;
    ui_screen_stack_pop();
    _done = true;
}

static void _confirm_done(const char* password)
{
    if (STREQ(_password_candidate, password)) {
        component_t* password_match =
            status_create("Success", true, STATUS_DEFAULT_DELAY, _match_callback);
        ui_screen_stack_switch(password_match);
    } else {
        component_t* password_no_match = status_create(
            "Passwords\ndo not match", false, STATUS_DEFAULT_DELAY, _no_match_callback);
        ui_screen_stack_switch(password_no_match);
    }
}

static void _confirm_enter(void)
{
    ui_screen_stack_switch(set_password_create(_confirm_done));
}

static void _set_done(const char* password)
{
    int snprintf_result =
        snprintf(_password_candidate, sizeof(_password_candidate), "%s", password);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_password_candidate)) {
        Abort("length mismatch");
    }
    ui_screen_stack_switch(entry_screen_create("Repeat password", _confirm_enter));
}

static void _set_enter(void)
{
    ui_screen_stack_switch(set_password_create(_set_done));
}

bool password_set(bool (*callback)(const char* password))
{
    if (!_done) {
        return false;
    }
    _done = false;
    _result = false;
    _callback = callback;
    memset(_password_candidate, 0, sizeof(_password_candidate));
    ui_screen_stack_push(entry_screen_create("Set password", _set_enter));
    ui_screen_process(_is_done);
    return _result;
}

bool password_check(void)
{
    if (!memory_is_seeded()) {
        Abort("password_check: must be seeded");
    }
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    password_enter("Unlocking device\nrequired", password);
    keystore_error_t unlock_result = workflow_unlock_and_handle_error(password);
    util_zero(password, sizeof(password));
    return unlock_result == KEYSTORE_OK;
}
