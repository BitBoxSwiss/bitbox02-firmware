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

#include "password_enter.h"
#include "blocking.h"

#include <hardfault.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>

#include <stdio.h>

static char _password[SET_PASSWORD_MAX_PASSWORD_LENGTH];

static void _pw_entered(const char* password, void* param)
{
    (void)param;
    int snprintf_result = snprintf(_password, sizeof(_password), "%s", password);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_password)) {
        Abort("length mismatch");
    }
    workflow_blocking_unblock();
}

bool password_enter(const char* title, bool special_chars, char* password_out)
{
    ui_screen_stack_push(
        trinary_input_string_create_password(title, special_chars, _pw_entered, NULL, NULL, NULL));
    workflow_blocking_block();
    ui_screen_stack_pop();
    snprintf(password_out, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", _password);
    util_zero(_password, sizeof(_password));
    return true;
}
