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

#include <stdlib.h>
#include <string.h>

#include "orientation_screen.h"
#include "platform_config.h"
#include "unlock.h"
#include "workflow.h"

#include <hardfault.h>
#include <platform_config.h>
#include <ui/components/confirm.h>
#include <ui/components/waiting.h>
#include <ui/screen_stack.h>
#include <ui/workflow_stack.h>
#include <util.h>

static void _confirm_dismiss(bool result, void* param)
{
    (void)result;
    (void)param;
    ui_screen_stack_switch(waiting_create());
}

void workflow_confirm_dismiss(const char* title, const char* body)
{
    const confirm_params_t params = {
        .title = title,
        .body = body,
        .accept_only = true,
    };
    ui_screen_stack_switch(confirm_create(&params, _confirm_dismiss, NULL));
}

workflow_t* workflow_allocate(
    workflow_method init,
    workflow_method cleanup,
    workflow_method spin,
    size_t data_size)
{
    workflow_t* result = (workflow_t*)malloc(sizeof(*result));
    if (!result) {
        Abort("malloc failed in workflow_allocate");
    }
    result->init = init;
    result->cleanup = cleanup;
    result->spin = spin;
    result->data_size = data_size;
    if (!data_size) {
        result->data = NULL;
    } else {
        result->data = calloc(1, data_size);
        if (!result->data) {
            Abort("Workflow data malloc failed.");
        }
    }
    return result;
}

void workflow_destroy(workflow_t* workflow)
{
    if (workflow->cleanup) {
        workflow->cleanup(workflow);
    }
    util_zero(workflow->data, workflow->data_size);
    free(workflow->data);
    util_zero(workflow, sizeof(*workflow));
    free(workflow);
}
