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

#include "confirm.h"

#include "blocking.h"

#include <ui/components/confirm.h>
#include <ui/screen_stack.h>

#include <stddef.h>

static bool _result = false;

static void _confirm(component_t* component)
{
    (void)component;
    _result = true;
    workflow_blocking_unblock();
}

static void _reject(component_t* component)
{
    (void)component;
    _result = false;
    workflow_blocking_unblock();
}

bool workflow_confirm_with_timeout(
    const char* title,
    const char* body,
    bool accept_only,
    uint32_t timeout)
{
    _result = false;
    ui_screen_stack_push(
        confirm_create(title, body, false, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block_with_timeout(timeout);
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}

bool workflow_confirm(const char* title, const char* body, bool longtouch, bool accept_only)
{
    _result = false;
    ui_screen_stack_push(
        confirm_create(title, body, longtouch, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}

bool workflow_confirm_scrollable(const char* title, const char* body, bool accept_only)
{
    _result = false;
    ui_screen_stack_push(
        confirm_create_scrollable(title, body, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}
