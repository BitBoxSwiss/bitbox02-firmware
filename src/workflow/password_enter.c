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
#include <ui/workflow_stack.h>
#include <util.h>

#include <stdio.h>
#include <stdlib.h>

typedef struct {
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH];
    bool password_available;
    bool special_chars;
    void (*callback)(const char* password, void* param);
    void* callback_param;
    char* title;
} password_enter_data_t;

static void _pw_entered(const char* password, void* param)
{
    password_enter_data_t* data = (password_enter_data_t*)param;
    int snprintf_result = snprintf(data->password, sizeof(data->password), "%s", password);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(data->password)) {
        Abort("length mismatch");
    }
    data->password_available = true;
}

static void _password_init(workflow_t* self)
{
    password_enter_data_t* data = (password_enter_data_t*)self->data;
    ui_screen_stack_push(trinary_input_string_create_password(
        data->title, data->special_chars, _pw_entered, data, NULL, NULL));
}

static void _password_cleanup(workflow_t* self)
{
    password_enter_data_t* data = (password_enter_data_t*)self->data;
    ui_screen_stack_pop();
    free(data->title);
    free(self->data);
    free(self);
}

static void _password_spin(workflow_t* self)
{
    password_enter_data_t* data = (password_enter_data_t*)self->data;
    if (data->password_available) {
        if (data->callback) {
            data->callback(data->password, data->callback_param);
        }
        util_zero(data->password, sizeof(data->password));
        /* We're done. */
        workflow_stack_stop_workflow();
    }
}

workflow_t* password_enter(
    const char* title,
    bool special_chars,
    void (*callback)(const char* password, void* param),
    void* callback_param)
{
    workflow_t* self = workflow_allocate(
        _password_init, _password_cleanup, _password_spin, sizeof(password_enter_data_t));
    password_enter_data_t* data = (password_enter_data_t*)self->data;
    data->callback = callback;
    data->callback_param = callback_param;
    data->special_chars = special_chars;
    data->title = util_strdup(title);
    return self;
}

static void _password_enter_unblock_cb(const char* password, void* param)
{
    char* out_password = (char*)param;
    int snprintf_result = snprintf(out_password, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", password);
    if (snprintf_result < 0 || snprintf_result >= SET_PASSWORD_MAX_PASSWORD_LENGTH) {
        Abort("length mismatch");
    }
    workflow_blocking_unblock();
}

void password_enter_blocking(const char* title, bool special_chars, char* password_out)
{
    workflow_stack_start_workflow(
        password_enter(title, special_chars, _password_enter_unblock_cb, (void*)password_out));
    workflow_blocking_block();
}
