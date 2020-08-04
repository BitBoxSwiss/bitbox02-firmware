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

#include "status.h"
#include <hardfault.h>
#include <keystore.h>
#include <string.h>
#include <ui/workflow_stack.h>

#include <stdio.h>

keystore_error_t workflow_unlock_and_handle_error(const char* password)
{
    uint8_t remaining_attempts = 0;
    keystore_error_t unlock_result = keystore_unlock(password, &remaining_attempts);
    switch (unlock_result) {
    case KEYSTORE_OK:
    case KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED:
        /*
         * The MCU resets before entering this branch.
         * Exit cleanly for testing purposes.
         */
        break;
    case KEYSTORE_ERR_INCORRECT_PASSWORD: {
        char msg[100] = {0};
        if (remaining_attempts == 1) {
            snprintf(msg, sizeof(msg), "Wrong password\n1 try remains");
        } else {
            snprintf(msg, sizeof(msg), "Wrong password\n%d tries remain", remaining_attempts);
        }
        workflow_stack_start_workflow(workflow_status(msg, false, NULL, NULL));
        break;
    }
    default:
        Abort("keystore unlock failed");
    }
    return unlock_result;
}
