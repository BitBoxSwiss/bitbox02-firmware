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

#include "status.h"

#include "blocking.h"
#include "workflow.h"

#include <ui/components/status.h>
#include <ui/screen_stack.h>
#include <ui/workflow_stack.h>
#include <util.h>

typedef struct {
    char* msg;
    bool status_success;
    bool finished;
    void (*callback)(void*);
    void* callback_param;
} data_t;

/**
 * Invoked when the status expired.
 */
static void _status_cb(void* param)
{
    data_t* data = (data_t*)param;
    data->finished = true;
}

static void _workflow_status_init(workflow_t* self)
{
    data_t* data = (data_t*)self->data;
    ui_screen_stack_push(
        status_create(data->msg, data->status_success, STATUS_DEFAULT_DELAY, _status_cb, data));
}

static void _workflow_status_cleanup(workflow_t* self)
{
    data_t* data = (data_t*)self->data;
    ui_screen_stack_pop();
    free(data->msg);
}

static void _workflow_status_spin(workflow_t* self)
{
    data_t* data = (data_t*)self->data;
    if (data->finished) {
        if (data->callback) {
            data->callback(data->callback_param);
        }
        workflow_stack_stop_workflow();
    }
}

workflow_t* workflow_status(
    const char* msg,
    bool status_success,
    void (*callback)(void*),
    void* cb_param)
{
    workflow_t* result = workflow_allocate(
        _workflow_status_init, _workflow_status_cleanup, _workflow_status_spin, sizeof(data_t));
    data_t* data = (data_t*)result->data;
    data->msg = util_strdup(msg);
    data->status_success = status_success;
    data->callback = callback;
    data->callback_param = cb_param;
    return result;
}

/**
 * Callback used to wrap workflow_status into workflow_status_blocking().
 */
static void _unlock_cb(void* param)
{
    (void)param;
    workflow_blocking_unblock();
}

/**
 * Blocking wrapper for workflow_status().
 */
void workflow_status_blocking(const char* msg, bool status_success)
{
    workflow_stack_start_workflow(workflow_status(msg, status_success, _unlock_cb, NULL));
    workflow_blocking_block();
}
