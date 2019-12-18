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

#include "async.h"
#include "blocking.h"
#include "hardfault.h"

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
    const UG_FONT* font,
    bool accept_only,
    uint32_t timeout)
{
    _result = false;
    ui_screen_stack_push(
        confirm_create(title, body, font, false, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block_with_timeout(timeout);
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}

static bool _have_result = false;

static void _confirm_async(component_t* component)
{
    (void)component;
    _result = true;
    _have_result = true;
    ui_screen_stack_pop();
}

static void _reject_async(component_t* component)
{
    (void)component;
    _result = false;
    _have_result = true;
    ui_screen_stack_pop();
}

static enum _confirm_state {
    CONFIRM_IDLE,
    CONFIRM_WAIT,
} _confirm_state = CONFIRM_IDLE;

enum workflow_async_ready workflow_confirm_async(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool accept_only,
    bool* result)
{
    switch (_confirm_state) {
    case CONFIRM_IDLE:
        _result = false;
        ui_screen_stack_push(confirm_create(
            title, body, font, false, _confirm_async, accept_only ? NULL : _reject_async));
        _confirm_state = CONFIRM_WAIT;
        /* FALLTHRU */
    case CONFIRM_WAIT:
        if (!_have_result) {
            return WORKFLOW_ASYNC_NOT_READY;
        }
        _have_result = false;
        _confirm_state = CONFIRM_IDLE;
        *result = _result;
        return WORKFLOW_ASYNC_READY;
    default:
        Abort("workflow_confirm: Internal error");
    }
}

bool workflow_confirm(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool longtouch,
    bool accept_only)
{
    _result = false;
    ui_screen_stack_push(
        confirm_create(title, body, font, longtouch, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}

bool workflow_confirm_scrollable(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool accept_only)
{
    _result = false;
    ui_screen_stack_push(confirm_create_scrollable(
        title, body, font, false, _confirm, accept_only ? NULL : _reject));
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!blocking_result) {
        return false;
    }
    return _result;
}

bool workflow_confirm_scrollable_longtouch(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool* cancel_forced_out)
{
    _result = false;
    ui_screen_stack_push(confirm_create_scrollable(title, body, font, true, _confirm, _reject));
    bool blocking_result = workflow_blocking_block();
    ui_screen_stack_pop();
    *cancel_forced_out = !blocking_result;
    if (*cancel_forced_out) {
        return false;
    }
    return _result;
}
