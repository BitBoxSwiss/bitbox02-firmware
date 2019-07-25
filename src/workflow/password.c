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

#include <hardfault.h>
#include <memory.h>
#include <util.h>

#include <stdio.h>

bool password_set(char* password_out)
{
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    char password_repeat[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    password_enter("Set password", password);
    password_enter("Repeat password", password_repeat);
    if (!STREQ(password, password_repeat)) {
        util_zero(password, sizeof(password));
        util_zero(password_repeat, sizeof(password_repeat));
        workflow_status_create("Passwords\ndo not match", false);
        return false;
    }
    util_zero(password_repeat, sizeof(password_repeat));
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
    password_enter("Unlocking device\nrequired", password);
    keystore_error_t unlock_result = workflow_unlock_and_handle_error(password);
    util_zero(password, sizeof(password));
    return unlock_result == KEYSTORE_OK;
}
