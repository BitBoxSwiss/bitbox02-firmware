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

#include <stddef.h>

#include "verify_pub.h"

#include "blocking.h"

#include <ui/components/confirm.h>
#include <ui/screen_stack.h>

static void _dismiss(component_t* component)
{
    (void)component;
    workflow_blocking_unblock();
}

void workflow_verify_pub(const char* title, const char* pub)
{
    ui_screen_stack_push(confirm_create_scrollable(title, pub, NULL, false, _dismiss, NULL));
    bool result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!result) {
        // No meaningful error to return here.
    }
}
