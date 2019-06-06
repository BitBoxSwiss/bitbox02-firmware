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

#include "pairing.h"
#include "workflow.h"
#include <base32.h>
#include <hardfault.h>
#include <stdio.h>
#include <ui/component.h>
#include <ui/components/confirm.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>

static bool _done = false;
static bool _result = false;

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

bool workflow_pairing_create(const uint8_t* hash)
{
    _done = false;
    _result = false;

    char base32[60] = {0};
    int count = base32_encode(hash, 32, (uint8_t*)base32, sizeof(base32));
    if (count < 20) {
        Abort("unexpected base32 size");
    }
    char base32_formatted[100];
    snprintf(
        base32_formatted,
        sizeof(base32_formatted),
        "%.5s %.5s\n%.5s %.5s",
        base32,
        base32 + 5,
        base32 + 10,
        base32 + 15);

    ui_screen_stack_push(confirm_create("Pairing code", base32_formatted, _confirm, _reject));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
    return _result;
}
