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

#include "workflow.h"

#include <ui/components/confirm.h>
#include <ui/components/waiting.h>
#include <ui/screen_stack.h>

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
