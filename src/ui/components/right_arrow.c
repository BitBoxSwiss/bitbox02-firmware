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

#include "right_arrow.h"

#include <hardfault.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/ui_util.h>
#include <util.h>

#include "../event.h"
#include "../event_handler.h"
#include "ui_images.h"

#include <stdbool.h>
#include <string.h>

#define HEIGHT (5u)

typedef struct {
    uint8_t location;
    bool active; // Marker is 'active', i.e., touched
} right_arrow_data_t;

/**
 * Renders a right arrow.
 * @param[IN] component The right arrow.
 */
static void _render(component_t* component)
{
    right_arrow_data_t* data = (right_arrow_data_t*)component->data;
    const uint8_t scale = 4; // Divide active_count by scale to slow down motion
    static uint16_t active_count = scale - 1; // Start at an offset to allow movement on first touch
    uint8_t j_start = SCREEN_WIDTH / 9 * 8;

    if (data->active) {
        active_count = MIN(4 * scale, active_count + 1);
    } else {
        active_count = MAX(scale - 1, active_count - scale);
    }
    j_start += active_count / scale;
    image_arrow(j_start, component->position.top, HEIGHT, ARROW_RIGHT);
}

static void _on_event(const event_t* event, component_t* component)
{
    right_arrow_data_t* data = (right_arrow_data_t*)component->data;
    gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
    switch (event->id) {
    case EVENT_TOP_SHORT_TAP:
    case EVENT_BOTTOM_SHORT_TAP:
        if (data->location == top_slider && event->id == EVENT_BOTTOM_SHORT_TAP) {
            break;
        }
        if (data->location == bottom_slider && event->id == EVENT_TOP_SHORT_TAP) {
            break;
        }
        if (slider_data->position > SLIDER_POSITION_TWO_THIRD &&
            slider_data->position <= MAX_SLIDER_POS) {
            data->active = false;
            event_t e;
            e.id = EVENT_FORWARD;
            emit_event(&e);
            break;
        }
        /* FALLTHROUGH */
    case EVENT_TOP_CONTINUOUS_TAP:
    case EVENT_BOTTOM_CONTINUOUS_TAP:
        if (data->location == top_slider && event->id == EVENT_BOTTOM_CONTINUOUS_TAP) {
            break;
        }
        if (data->location == bottom_slider && event->id == EVENT_TOP_CONTINUOUS_TAP) {
            break;
        }

        if (slider_data->position > SLIDER_POSITION_TWO_THIRD &&
            slider_data->position <= MAX_SLIDER_POS) {
            data->active = true;
            break;
        }
        /* FALLTHROUGH */
    default:
        data->active = false;
        break;
    }
}

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

/**
 * Creates a right arrow component.
 * @param[in] location whether the arrow should be rendered on top or bottom (top/bottom slider)
 * @param[in] parent The parent component.
 */
component_t* right_arrow_create(slider_location_t location, component_t* parent)
{
    right_arrow_data_t* data = malloc(sizeof(right_arrow_data_t));
    if (!data) {
        Abort("Error: malloc right_arrow data");
    }
    memset(data, 0, sizeof(right_arrow_data_t));
    data->location = location;

    component_t* right_arrow = malloc(sizeof(component_t));
    if (!right_arrow) {
        Abort("Error: malloc right_arrow");
    }
    memset(right_arrow, 0, sizeof(component_t));
    right_arrow->data = data;
    right_arrow->f = &_component_functions;
    right_arrow->parent = parent;
    right_arrow->dimension.height = HEIGHT * 2 - 1;
    right_arrow->dimension.width = HEIGHT;
    if (location == top_slider) {
        ui_util_position_right_top(parent, right_arrow);
    } else {
        ui_util_position_right_bottom(parent, right_arrow);
    }

    return right_arrow;
}
