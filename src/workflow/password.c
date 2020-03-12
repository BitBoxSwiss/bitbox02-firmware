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
#include "status.h"
#include "unlock.h"
#include "workflow.h"
#include "workflow/confirm.h"

#include <hardfault.h>
#include <memory/memory.h>
#include <util.h>

#include <stdio.h>

bool password_set(char* password_out)
{
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    char password_repeat[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password_repeat);
    password_enter_blocking("Set password", false, password);
    password_enter_blocking("Repeat password", false, password_repeat);
    if (!STREQ(password, password_repeat)) {
        workflow_status_blocking("Passwords\ndo not match", false);
        return false;
    }
    if (strlen(password) < 4) {
        const confirm_params_t params = {
            .title = "WARNING",
            .body = "Your password\n has fewer than\n 4 characters.\nContinue?",
            .longtouch = true,
        };
        if (!workflow_confirm_blocking(&params)) {
            return false;
        }
    }
    snprintf(password_out, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", password);
    workflow_status_blocking("Success", true);
    return true;
}

bool password_check(void)
{
    if (!memory_is_seeded()) {
        Abort("password_check: must be seeded");
    }
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    password_enter_blocking("Unlocking device\nrequired", false, password);
    return workflow_unlock_and_handle_error(password) == KEYSTORE_OK;
}
