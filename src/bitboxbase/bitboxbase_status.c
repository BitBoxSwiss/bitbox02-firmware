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

#include "bitboxbase_status.h"
#include <hardfault.h>
#include <leds.h>
#include <rust/bitbox02_rust.h>
#include <string.h>
#include <ui/component.h>
#include <ui/components/image.h>
#include <ui/components/label.h>
#include <ui/components/ui_images.h>
#include <ui/event.h>
#include <ui/oled/oled.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>

uint32_t frames = 0;
char msg[250];

static void _render(component_t* component);
static void _on_event(const event_t* event, component_t* component);

static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

void _render(component_t* component)
{
    (void)component;
    if (frames++ > 1000) {
        ui_screen_stack_pop();
    }
    UG_ClearBuffer();
    UG_PutString(10, 10, msg, false);
    UG_SendBuffer();
}

void _on_event(const event_t* event, component_t* component)
{
    (void)event;
    (void)component;
}

static component_t* _create(void)
{
    component_t* status = malloc(sizeof(component_t));
    if (!status) {
        Abort("Error: malloc show_logo");
    }
    memset(status, 0, sizeof(component_t));
    status->f = &_component_functions;
    status->dimension.width = 128;
    status->dimension.height = 64;
    return status;
}

void bitboxbase_status(void)
{
    memset(msg, 0, sizeof(msg));
    frames = 0;
    bitboxbase_status_get(msg, sizeof(msg));
    ui_screen_stack_push(_create());
}
