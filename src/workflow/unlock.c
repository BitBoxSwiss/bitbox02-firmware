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

#include "blocking.h"
#include "password_enter.h"
#include "status.h"
#include "unlock_bip39.h"
#include "workflow.h"
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <screen.h>
#include <string.h>
#include <ui/screen_stack.h>
#include <ui/workflow_stack.h>
#include <util.h>

#include <stdio.h>

typedef enum {
    UNLOCK_STATUS_PW_REQUEST,
    UNLOCK_STATUS_PW_AVAILABLE,
    UNLOCK_STATUS_FINISHED
} unlock_state_t;

typedef struct {
    void (*callback)(bool result, void* param);
    void* callback_param;
    unlock_state_t state;
    char* password;
    bool result;
} unlock_data_t;

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

static void _unlock_cleanup_password(unlock_data_t* data)
{
    if (data->password) {
        util_zero(data->password, sizeof(data->password));
        free(data->password);
        data->password = NULL;
    }
}

/**
 * Callback invoked when the password entry completes.
 */
static void _unlock_password_completed(const char* password, void* param)
{
    unlock_data_t* data = (unlock_data_t*)param;
    _unlock_cleanup_password(data);
    data->password = util_strdup(password);
    data->state = UNLOCK_STATUS_PW_AVAILABLE;
}

static void _unlock_init(workflow_t* self)
{
    unlock_data_t* data = (unlock_data_t*)self->data;
    if (!memory_is_initialized()) {
        data->state = UNLOCK_STATUS_FINISHED;
        data->result = false;
        return;
    }
    if (!keystore_is_locked()) {
        data->state = UNLOCK_STATUS_FINISHED;
        data->result = true;
        return;
    }

    /*
     * FUTURE: maybe remove?
     * It might be a "security" feature/hack, but it's not
     * clear whether it's useful or not.
     */
    ui_screen_stack_pop_all();
    data->state = UNLOCK_STATUS_PW_REQUEST;
}

static void _unlock_cleanup(workflow_t* self)
{
    unlock_data_t* data = (unlock_data_t*)self->data;
    _unlock_cleanup_password(data);
    free(data);
}

/**
 * Try to unlock the device with the provided password.
 * Request the password again on failure. If the maximum number of
 * attempts have been exceeded, abort immediately as the device is going
 * to be reset.
 */
static void _unlock_handle_password_entry(unlock_data_t* data)
{
    keystore_error_t unlock_result = workflow_unlock_and_handle_error(data->password);
    if (unlock_result == KEYSTORE_OK) {
        // Keystore unlocked, now unlock bip39 seed.
        workflow_stack_start_workflow(workflow_unlock_bip39(NULL, NULL));
        data->result = true;
        data->state = UNLOCK_STATUS_FINISHED;
    } else if (unlock_result == KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED) {
        /*
         * The MCU resets before entering this branch.
         * Exit without doing anything for testing purposes.
         */
    } else {
        /* Wrong password. Go back to the password entry. */
        data->state = UNLOCK_STATUS_PW_REQUEST;
    }
}

static void _unlock_spin(workflow_t* self)
{
    unlock_data_t* data = (unlock_data_t*)self->data;
    switch (data->state) {
    case UNLOCK_STATUS_PW_REQUEST:
        _unlock_cleanup_password(data);
        workflow_stack_start_workflow(
            password_enter("Enter password", false, _unlock_password_completed, (void*)data));
        break;
    case UNLOCK_STATUS_PW_AVAILABLE:
        _unlock_handle_password_entry(data);
        _unlock_cleanup_password(data);
        break;
    case UNLOCK_STATUS_FINISHED:
        /* TODO merge this with UNLOCK_STATUS_PW_AVAILABLE to preserve atomicity if the operation is
         * aborted. */
        if (data->callback) {
            data->callback(data->result, data->callback_param);
            workflow_stack_stop_workflow();
        }
        break;
    default:
        Abort("Unknown _unlock_spin status.");
    }
}

workflow_t* workflow_unlock(void (*callback)(bool result, void* param), void* callback_param)
{
    workflow_t* self =
        workflow_allocate(_unlock_init, _unlock_cleanup, _unlock_spin, sizeof(unlock_data_t));
    unlock_data_t* data = (unlock_data_t*)self->data;
    data->callback = callback;
    data->callback_param = callback_param;
    return self;
}

bool workflow_unlock_blocking(void)
{
    return rust_workflow_unlock_blocking();
}
