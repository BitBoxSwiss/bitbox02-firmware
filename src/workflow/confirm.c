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

#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>

static bool _result = false;
static bool _done = true;
static bool _is_done(void)
{
    return _done;
}

static void _confirm(component_t* component)
{
    (void)component;
    _result = true;
    _done = true;
}

static void _reject(component_t* component)
{
    (void)component;
    _result = false;
    _done = true;
}

static void _on_timeout(void)
{
    _reject(NULL);
}

bool workflow_confirm_with_timeout(
    const char* title,
    const char* body,
    bool accept_only,
    uint32_t timeout)
{
    _result = false;
    _done = false;
    ui_screen_stack_push(confirm_create(title, body, _confirm, accept_only ? NULL : _reject));
    ui_screen_process_with_timeout(_is_done, _on_timeout, timeout);
    ui_screen_stack_pop();
    return _result;
}

bool workflow_confirm(const char* title, const char* body, bool accept_only)
{
    _result = false;
    _done = false;
    ui_screen_stack_push(confirm_create(title, body, _confirm, accept_only ? NULL : _reject));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
    return _result;
}

bool workflow_confirm_scrollable(const char* title, const char* body, bool accept_only)
{
    _result = false;
    _done = false;
    ui_screen_stack_push(
        confirm_create_scrollable(title, body, _confirm, accept_only ? NULL : _reject));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
    return _result;
}
