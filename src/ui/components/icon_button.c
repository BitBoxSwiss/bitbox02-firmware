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

#include "icon_button.h"
#include "../event.h"
#include "../event_handler.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/ui_util.h>
#include <util.h>

#include <stdbool.h>
#include <string.h>

#define SCALE 4 // Divide active_count by scale to slow down motion

typedef struct {
    uint8_t location;
    bool active; // Marker is 'active', i.e., touched
    uint16_t active_count;
    icon_button_type_t type;
    void (*callback)(component_t* component);
} data_t;

/**
 * Renders an icon_button.
 * @param[in] component The left arrow.
 */
static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    uint16_t y;

    if (data->location == top_slider) {
        y = data->active_count / SCALE;
    } else {
        y = SCREEN_HEIGHT - data->active_count / SCALE - IMAGE_DEFAULT_ARROW_HEIGHT - 1;
    }

    // Explicit upcast to signed int, avoids underflow (happens automatically,
    // but brittle as it depends on the signedness of SCALE).
    data->active_count = data->active ? MIN(4 * SCALE, (int32_t)data->active_count + 1)
                                      : MAX(SCALE - 1, (int32_t)data->active_count - SCALE);

    switch (data->type) {
    case ICON_BUTTON_CHECK:
        image_checkmark(SCREEN_WIDTH / 6 * 5, y, IMAGE_DEFAULT_CHECKMARK_HEIGHT);
        break;
    case ICON_BUTTON_CROSS:
        image_cross(SCREEN_WIDTH / 6, y, IMAGE_DEFAULT_CROSS_HEIGHT);
        break;
    default:
        break;
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    data_t* data = (data_t*)component->data;

    // Return if the slider event is on the wrong slider
    switch (event->id) {
    case EVENT_TOP_SHORT_TAP:
    case EVENT_TOP_CONTINUOUS_TAP:
        if (data->location != top_slider) {
            data->active = false;
            return;
        }
        break;
    case EVENT_BOTTOM_SHORT_TAP:
    case EVENT_BOTTOM_CONTINUOUS_TAP:
        if (data->location != bottom_slider) {
            data->active = false;
            return;
        }
        break;
    default:
        data->active = false;
        return;
    }

    // Return if the slider position is away from the button
    // Only slider events reach here, so ok to typescast event->data
    gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
    switch (data->type) {
    case ICON_BUTTON_CHECK:
        if (slider_data->position < SLIDER_POSITION_TWO_THIRD) {
            data->active = false;
            return;
        }
        break;
    case ICON_BUTTON_CROSS:
        if (slider_data->position >= SLIDER_POSITION_ONE_THIRD) {
            data->active = false;
            return;
        }
        break;
    default:
        data->active = false;
        return;
    }

    data->active = true;

    // Call the callback on short tap
    switch (event->id) {
    case EVENT_TOP_SHORT_TAP:
    case EVENT_BOTTOM_SHORT_TAP:
        if (data->callback) {
            data->callback(component);
        }
        break;
    default:
        break;
    }
}

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

component_t* icon_button_create(
    slider_location_t location,
    icon_button_type_t type,
    void (*callback)(component_t* component))
{
    component_t* icon_button = malloc(sizeof(component_t));
    if (!icon_button) {
        Abort("Error: malloc icon_button");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc icon_button data");
    }
    memset(icon_button, 0, sizeof(component_t));
    memset(data, 0, sizeof(data_t));
    data->location = location;
    data->active = false;
    data->active_count = SCALE - 1; // Start at an offset to allow movement on first touch
    data->type = type;
    data->callback = callback;
    icon_button->data = data;
    icon_button->f = &_component_functions;
    icon_button->parent = NULL; // Gets set by ui_util_add_sub_component() if `icon_button` is
                                // passed as a sub-component
    return icon_button;
}
