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

#include "button.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ui_util.h>
#include <util.h>

#include <string.h>

static const uint8_t MIN_BUTTON_WIDTH = 32; // 0:SCREEN_WIDTH

/**
 * Component data.
 */
typedef struct {
    const char* text;
    slider_location_t location;
    bool span_over_slider;
    bool upside_down;
    void (*callback)(component_t*);
} button_data_t;

/**
 * Renders a button.
 * @param[in] component The button to be rendered.
 */
static void _render(component_t* component)
{
    button_data_t* data = (button_data_t*)component->data;
    UG_FontSelect(&font_font_a_9X9);
    UG_FontSetHSpace(0);
    UG_PutStringCentered(
        component->position.left,
        component->position.top,
        component->dimension.width,
        component->dimension.height,
        data->text,
        data->upside_down);
    UG_FontSetHSpace(1);
}

static void _on_event(const event_t* event, component_t* component)
{
    button_data_t* data = (button_data_t*)component->data;
    gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
    if (data->span_over_slider) {
        if (event->id ==
            ((data->location == top_slider) ? EVENT_TOP_SHORT_TAP : EVENT_BOTTOM_SHORT_TAP)) {
            data->callback(component);
        }
    } else {
        if (event->id ==
                ((data->location == top_slider) ? EVENT_TOP_SHORT_TAP : EVENT_BOTTOM_SHORT_TAP) &&
            slider_data->position >= component->position.left * MAX_SLIDER_POS / SCREEN_WIDTH &&
            slider_data->position <= (component->position.left + component->dimension.width) *
                                         MAX_SLIDER_POS / SCREEN_WIDTH) {
            data->callback(component);
        }
    }
}

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/
static component_t* _button_create(
    const char* text,
    const slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent,
    bool upside_down)
{
    button_data_t* data = malloc(sizeof(button_data_t));
    if (!data) {
        Abort("Error: malloc button data");
    }
    memset(data, 0, sizeof(button_data_t));
    data->location = location;
    data->upside_down = upside_down;
    data->span_over_slider = false;

    component_t* button = malloc(sizeof(component_t));
    if (!button) {
        Abort("Error: malloc button");
    }
    memset(button, 0, sizeof(component_t));
    button->data = data;
    button->parent = parent;
    button->f = &_component_functions;

    button_update(button, text, callback);

    return button;
}

static component_t* _button_create_wide(
    const char* text,
    const slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent,
    bool upside_down)
{
    component_t* button = _button_create(text, location, callback, parent, upside_down);

    button_data_t* data = (button_data_t*)button->data;
    data->span_over_slider = true;
    if (location == top_slider) {
        ui_util_position_center_top(parent, button);
    } else {
        ui_util_position_center_bottom(parent, button);
    }
    return button;
}

static component_t* _button_create_at_position(
    const char* text,
    const slider_location_t location,
    const uint8_t screen_position,
    void (*callback)(component_t*),
    component_t* parent,
    bool upside_down)
{
    component_t* button = _button_create(text, location, callback, parent, upside_down);

    int16_t pos = screen_position - button->dimension.width / 2;
    if (pos < 0) {
        pos = 0;
    } else if (pos + button->dimension.width >= SCREEN_WIDTH) {
        pos = SCREEN_WIDTH - button->dimension.width;
    }
    button->position.left = pos;
    if (location == bottom_slider) {
        ui_util_position_left_bottom_offset(parent, button, pos, 0);
    } else {
        ui_util_position_left_top_offset(parent, button, pos, 0);
    }

    return button;
}

component_t* button_create(
    const char* text,
    const slider_location_t location,
    const uint8_t screen_position,
    void (*callback)(component_t*),
    component_t* parent)
{
    return _button_create_at_position(text, location, screen_position, callback, parent, false);
}

component_t* button_create_wide(
    const char* text,
    const slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent)
{
    return _button_create_wide(text, location, callback, parent, false);
}

component_t* button_create_upside_down(
    const char* text,
    const slider_location_t location,
    const uint8_t screen_position,
    void (*callback)(component_t*),
    component_t* parent)
{
    return _button_create_at_position(text, location, screen_position, callback, parent, true);
}

component_t* button_create_wide_upside_down(
    const char* text,
    const slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent)
{
    return _button_create_wide(text, location, callback, parent, true);
}

void button_update(component_t* button, const char* text, void (*callback)(component_t*))
{
    button_data_t* data = (button_data_t*)button->data;
    data->callback = callback;
    data->text = text;
    UG_FontSelect(&font_font_a_9X9);
    UG_FontSetHSpace(0);
    UG_MeasureString(&(button->dimension.width), &(button->dimension.height), text);
    if (button->dimension.width < MIN_BUTTON_WIDTH) {
        button->dimension.width = MIN_BUTTON_WIDTH;
    }
    UG_FontSetHSpace(1);
}
