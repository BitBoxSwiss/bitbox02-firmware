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

#include "entry_screen.h"
#include "knight_rider.h"
#include "label.h"

#include <stdbool.h>
#include <string.h>

#include <hardfault.h>
#include <ui/ui_util.h>
#include <util.h>

typedef struct {
    uint16_t screen_count;
    void (*done_callback)(void);
} entry_screen_data_t;

static void _on_event(const event_t* event, component_t* component)
{
    entry_screen_data_t* data = (entry_screen_data_t*)component->data;
    switch (event->id) {
    case EVENT_BOTTOM_CONTINUOUS_TAP:
    case EVENT_BOTTOM_SLIDE:
        data->done_callback();
        break;
    default:
        break;
    }
}

static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

/**
 * Creates an entry screen.
 * @param[in] done_callback The callback that is called when the user touches to enter.
 */
component_t* entry_screen_create(const char* text, void (*done_callback)(void))
{
    component_t* entry_screen = malloc(sizeof(component_t));
    if (!entry_screen) {
        Abort("Error: malloc entry_screen");
    }
    entry_screen_data_t* data = malloc(sizeof(entry_screen_data_t));
    if (!data) {
        Abort("Error: malloc entry_screen data");
    }
    memset(entry_screen, 0, sizeof(component_t));
    memset(data, 0, sizeof(entry_screen_data_t));

    data->screen_count = 0;
    data->done_callback = done_callback;

    entry_screen->data = data;
    entry_screen->parent = NULL;
    entry_screen->f = &_component_functions;
    entry_screen->dimension.width = SCREEN_WIDTH;
    entry_screen->dimension.height = SCREEN_HEIGHT;
    entry_screen->position.top = 0;
    entry_screen->position.left = 0;

    ui_util_add_sub_component(entry_screen, knight_rider_create(entry_screen, SCREEN_HEIGHT - 1));
    ui_util_add_sub_component(entry_screen, label_create(text, NULL, CENTER, entry_screen));

    return entry_screen;
}
