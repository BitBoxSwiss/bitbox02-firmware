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

#include "verify_recipient.h"
#include <stdio.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>

static bool _done = false;
static bool _result = false;
static bool _is_done(void)
{
    return _done;
}

static void _finish(void)
{
    _done = true;
}

static void _confirm(component_t* component)
{
    (void)component;
    _result = true;
    _finish();
}

static void _reject(component_t* component)
{
    (void)component;
    _result = false;
    ui_screen_stack_switch(
        status_create("Transaction\ncanceled", _result, STATUS_DEFAULT_DELAY, _finish));
}

bool workflow_verify_recipient(const char* recipient, const char* amount)
{
    _done = false;
    _result = false;
    ui_screen_stack_push(confirm_transaction_address_create(amount, recipient, _confirm, _reject));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
    return _result;
}
