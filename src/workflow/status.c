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

#include <ui/components/status.h>
#include <ui/screen_stack.h>

void workflow_status_create(const char* msg, bool status_success)
{
    ui_screen_stack_push(
        status_create(msg, status_success, STATUS_DEFAULT_DELAY, workflow_blocking_unblock));
    workflow_blocking_block();
    ui_screen_stack_pop();
}
