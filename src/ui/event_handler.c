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

#include "event_handler.h"
#include "screen_stack.h"

static void _handle_event(component_t* component, const event_t* event)
{
    if (!component) {
        return;
    }
    uint8_t num_components = component->sub_components.amount;
    for (int i = 0; i < num_components; i++) {
        _handle_event(component->sub_components.sub_components[i], event);
    }
    if (component->f->on_event) {
        component->f->on_event(event, component);
    }
}

void emit_event(const event_t* event)
{
    _handle_event(ui_screen_stack_top(), event);
}
