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

#include "keyboard_switch.h"
#include "../event.h"
#include "../event_handler.h"

#include <hardfault.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ui_util.h>

#include <stdbool.h>
#include <string.h>

/**
 * Data that is required for the keyboard switch.
 */
typedef struct {
    keyboard_mode_t mode;
    slider_location_t location;
    bool active; // Marker is 'active', i.e., touched
    // if true, the special chars keyboard mode is available.
    bool special_chars;
} keyboard_switch_data_t;

/**
 * Renders the keyboard switch button.
 * @param[IN] component The keyboard switch component.
 */
static void _render(component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    UG_FontSelect(&font_font_a_9X9);
    UG_S16 w = 0, h = 0;
    switch (ks_data->mode) {
    case LOWER_CASE:
        UG_MeasureString(&w, &h, "abc");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 2, 1, "abc", false);
        break;
    case UPPER_CASE:
        UG_MeasureString(&w, &h, "ABC");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "ABC", false);
        break;
    case DIGITS:
        UG_MeasureString(&w, &h, "123");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "123", false);
        break;
    case SPECIAL_CHARS:
        UG_MeasureString(&w, &h, "&?+");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "&?+", false);
        break;
    default:
        Abort("Keyboard mode unrecognized");
        break;
    }
    if (ks_data->active) {
        UG_DrawLine(
            (SCREEN_WIDTH - w) / 2 + 1,
            h + 2,
            (SCREEN_WIDTH + w) / 2 - 1,
            h + 2,
            screen_front_color);
    }
}

/**
 * Switches the keyboard mode from digits to lower case to upper case.
 * @param[IN] component The keyboard switch component.
 */
static void _on_event(const event_t* event, component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
    switch (event->id) {
    case EVENT_TOGGLE_ALPHANUMERIC:
        switch (ks_data->mode) {
        case LOWER_CASE:
            ks_data->mode = UPPER_CASE;
            break;
        case UPPER_CASE:
            ks_data->mode = DIGITS;
            break;
        case DIGITS:
            ks_data->mode = ks_data->special_chars ? SPECIAL_CHARS : LOWER_CASE;
            break;
        case SPECIAL_CHARS:
            ks_data->mode = LOWER_CASE;
            break;
        default:
            Abort("Keyboard mode unrecognized");
            break;
        }
        break;

    case EVENT_UPDATE_ALPHANUMERIC:
        ks_data->mode = *(keyboard_mode_t*)event->data;
        break;
    case EVENT_TOP_CONTINUOUS_TAP:
        if (ks_data->location == top_slider && slider_data->position > SLIDER_POSITION_ONE_THIRD &&
            slider_data->position <= SLIDER_POSITION_TWO_THIRD) {
            ks_data->active = true;
            break;
        }
        /* FALLTHROUGH */
    case EVENT_TOP_SHORT_TAP:
        if (ks_data->location == top_slider && slider_data->position > SLIDER_POSITION_ONE_THIRD &&
            slider_data->position <= SLIDER_POSITION_TWO_THIRD) {
            ks_data->active = false;
            event_t e;
            e.id = EVENT_TOGGLE_ALPHANUMERIC;
            emit_event(&e);
            break;
        }
        /* FALLTHROUGH */
    default:
        ks_data->active = false;
        break;
    }
}

/********************************** Variable-length Input Functions *************************/

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

component_t* keyboard_switch_create(
    slider_location_t location,
    bool special_chars,
    component_t* parent)
{
    component_t* keyboard_switch = malloc(sizeof(component_t));
    if (!keyboard_switch) {
        Abort("Error: malloc keyboard_switch");
    }
    memset(keyboard_switch, 0, sizeof(component_t));

    keyboard_switch_data_t* ks_data = malloc(sizeof(keyboard_switch_data_t));
    if (!ks_data) {
        Abort("Error: malloc keyboard_switch data");
    }
    memset(ks_data, 0, sizeof(keyboard_switch_data_t));

    ks_data->location = location;
    ks_data->mode = LOWER_CASE;
    ks_data->active = false;
    ks_data->special_chars = special_chars;

    keyboard_switch->data = ks_data;
    keyboard_switch->f = &_component_functions;
    keyboard_switch->parent = parent;

    return keyboard_switch;
}

keyboard_mode_t keyboard_current_mode(const component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    return ks_data->mode;
}
