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
#include "screen_process.h"
#include "screen_saver.h"
#include "screen_stack.h"

static void _handle_event(component_t* component, const event_t* event)
{
    if (!component || component->disabled) {
        return;
    }
    uint8_t num_components = component->sub_components.amount;
    for (int i = 0; i < num_components; i++) {
        component_t* comp = component->sub_components.sub_components[i];
        _handle_event(comp, event);
    }
    if (component->f->on_event) {
        component->f->on_event(event, component);
    }
}

void emit_event(const event_t* event)
{
    _handle_event(screen_process_get_top_component(), event);

    // Reset the screensaver based on slider touch:
    //
    // If the current component is the screensaver itself, only reset it when releasing the touch.
    // Otherwise, the screensaver would disappear, showing the previous component with the user
    // still touching. This can induce anxiety, as it looks like the touch could affect trigger an
    // unwanted action.
    //
    // If the screensaver is not active, we reset the timer with any touch interaction.
    switch (event->id) {
    case EVENT_TOP_SHORT_TAP:
    case EVENT_BOTTOM_SHORT_TAP:
    case EVENT_BOTTOM_SLIDE_RELEASED:
    case EVENT_TOP_SLIDE_RELEASED:
        screen_saver_reset();
        break;
    case EVENT_TOP_SLIDE:
    case EVENT_BOTTOM_SLIDE:
    case EVENT_TOP_CONTINUOUS_TAP:
    case EVENT_BOTTOM_CONTINUOUS_TAP:
        if (screen_saver_get() == NULL) {
            screen_saver_reset();
        }
        break;
    default:
        break;
    }
}
