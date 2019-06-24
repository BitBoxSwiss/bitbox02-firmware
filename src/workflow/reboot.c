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

#include "reboot.h"
#include <memory.h>
#include <screen.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#ifndef TESTING
#include <driver_init.h>
#endif

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

    auto_enter_t auto_enter = {
        .value = sectrue_u8,
    };
    upside_down_t upside_down = {
        .value = screen_is_upside_down(),
    };
    if (!memory_bootloader_set_flags(auto_enter, upside_down)) {
        // If this failed, we might not be able to reboot into the bootloader.
        // We will try anyway, no point in aborting here.
    }
#ifndef TESTING
    _reset_mcu();
#endif
}

static void _reject(component_t* component)
{
    (void)component;
    _result = false;
    _done = true;
}

bool workflow_reboot(void)
{
    _done = false;
    _result = false;
    ui_screen_stack_push(confirm_create("", "Proceed to upgrade?", _confirm, _reject));
    ui_screen_process(_is_done);
    ui_screen_stack_pop();
    return _result;
}
