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
    if (!password_enter("Set password", false, password)) {
        return false;
    }
    if (!password_enter("Repeat password", false, password_repeat)) {
        return false;
    }
    if (!STREQ(password, password_repeat)) {
        workflow_status_create("Passwords\ndo not match", false);
        return false;
    }
    if (strlen(password) < 4) {
        if (!workflow_confirm(
                "WARNING",
                "Your password\n has fewer than\n 4 characters.\nContinue?",
                NULL,
                true,
                false)) {
            return false;
        }
    }
    snprintf(password_out, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", password);
    workflow_status_create("Success", true);
    return true;
}

bool password_check(void)
{
    if (!memory_is_seeded()) {
        Abort("password_check: must be seeded");
    }
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    if (!password_enter("Unlocking device\nrequired", false, password)) {
        return false;
    }
    return workflow_unlock_and_handle_error(password) == KEYSTORE_OK;
}
