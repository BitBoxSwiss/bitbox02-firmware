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

#include "label.h"
#include "knight_rider.h"

#include <hardfault.h>
#include <screen.h>
#include <string.h>
#include <touch/gestures.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>
#include <util.h>

typedef struct {
    // +3 for '...' if truncated, +1 for null terminator.
    char text[MAX_LABEL_SIZE + 3 + 1];
    const UG_FONT* font;
    bool upside_down;
    enum screen_position_t position;
    bool scrollable;
    bool slider_is_touched;
    bool slider_was_touched;
    uint16_t slider_position;
    // float slider_position_diff;
    int16_t text_position;
    int16_t text_position_acc;
    // int16_t text_render_counter;
    //  int16_t text_position_last;
    // int16_t text_inertia;
    int16_t text_velocity;
    uint16_t text_velocity_counter;
    uint8_t xoffset;
    uint8_t yoffset;
} data_t;

static void _measure_label_dimensions(component_t* label);

void label_update(component_t* component, const char* text)
{
    data_t* data = (data_t*)component->data;
    int snprintf_result = snprintf(data->text, MAX_LABEL_SIZE + 1, "%s", text);
    if (snprintf_result >= MAX_LABEL_SIZE + 1) {
        // text has been truncated, add '...'
        snprintf(&data->text[MAX_LABEL_SIZE], 4, "...");
    }
    _measure_label_dimensions(component);
    if (component->parent == NULL) {
        return;
    }
    component_t* parent = component->parent;
    switch (data->position) {
    case CENTER:
        ui_util_position_center(parent, component);
        break;
    case CENTER_TOP:
        ui_util_position_center_top(parent, component);
        break;
    case CENTER_BOTTOM:
        ui_util_position_center_bottom(parent, component);
        break;
    case LEFT_TOP:
        ui_util_position_left_top(parent, component);
        break;
    case LEFT_BOTTOM:
        ui_util_position_left_bottom(parent, component);
        break;
    case LEFT_CENTER:
        ui_util_position_left_center(parent, component);
        break;
    case RIGHT_CENTER:
        ui_util_position_right_center(parent, component);
        break;
    case RIGHT_TOP:
        ui_util_position_right_top(parent, component);
        break;
    case RIGHT_BOTTOM:
        ui_util_position_right_bottom(parent, component);
        break;
    default:
        Abort("position undefined or currently not implemented");
    }
    component->position.top += data->yoffset;
    component->position.left += data->xoffset;
}

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    // Slider indicators
    if (data->scrollable) {
        int x = data->slider_position / 2;
        int y = SCREEN_HEIGHT - 1;
        if (!data->slider_was_touched) {
            data->text_position = component->dimension.width / 2 + SCREEN_WIDTH / 6;
            // data->text_position_last = data->text_position;
            ui_util_component_render_subcomponents(component);
        } else if (data->slider_is_touched) {
            UG_DrawLine(MIN(SCREEN_WIDTH, x + 3), y, MAX(0, x - 3), y, screen_front_color);
        }
        // The input sensor has twice the resolution of the screen
        // text_position_acc is used to accumulate movement until there is enough to trigger a
        // repositioning of the text
        if (abs(data->text_position_acc) > 1) {
            data->text_position += data->text_position_acc / 2;
            data->text_position_acc = 0;
        }
        static int32_t render_counter = 0;
        if (!data->slider_is_touched && render_counter++ % 2 == 0 && data->text_velocity != 0) {
            if (data->text_velocity > 0) {
                data->text_velocity = MAX(data->text_velocity - 3, 0);
            } else {
                data->text_velocity = MIN(data->text_velocity + 3, 0);
            }
            data->text_position_acc += data->text_velocity / 10;
        }
        // Stop label from moving out of screen
        int16_t margin = SCREEN_WIDTH / 5;
        data->text_position =
            MIN(component->dimension.width / 2 + margin,
                MAX(-margin - component->dimension.width / 2 + SCREEN_WIDTH, data->text_position));
    }
    // Label
    UG_FontSetVSpace(2);
    UG_FontSelect(data->font);
    if (data->scrollable) {
        UG_PutStringNoBreak(
            data->text_position - component->dimension.width / 2,
            component->position.top,
            data->text,
            data->upside_down);
    } else if (
        data->position == CENTER || data->position == CENTER_TOP ||
        data->position == CENTER_BOTTOM) {
        UG_PutStringCentered(
            component->position.left + data->text_position,
            component->position.top,
            component->dimension.width,
            component->dimension.height,
            data->text,
            data->upside_down);
    } else {
        UG_PutString(
            component->position.left + data->text_position,
            component->position.top,
            data->text,
            data->upside_down);
    }
    UG_FontSetVSpace(0);
}

static void _on_event(const event_t* event, component_t* component)
{
    data_t* data = (data_t*)component->data;
    if (event->data.source != bottom_slider) {
        return;
    }
    if (data->scrollable) {
        switch (event->id) {
        case EVENT_SLIDE: {
            if (event->data.position != data->slider_position) {
                int16_t movement = event->data.position - data->slider_position;
                data->slider_position = event->data.position;
                data->text_position_acc += movement;
                data->text_velocity = event->data.velocity;
                // data->text_inertia = movement * 5;
            }
            data->slider_is_touched = true;
            data->slider_was_touched = true;
            break;
        }
        case EVENT_SHORT_TAP:
        case EVENT_LONG_TAP:
        case EVENT_SLIDE_RELEASED:
            data->slider_is_touched = false;
            break;

        case EVENT_CONTINUOUS_TAP:
            data->slider_position = event->data.position;
            data->slider_is_touched = true;
            data->slider_was_touched = true;
            break;
        default:
            break;
        }
    }
}

static void _cleanup(component_t* component)
{
    data_t* data = (data_t*)component->data;
    // Just in case something sensitive is shown in a label.
    util_zero(data->text, sizeof(data->text));
    ui_util_component_cleanup(component);
}

void _measure_label_dimensions(component_t* label)
{
    data_t* data = (data_t*)label->data;

    UG_FontSetVSpace(2);
    UG_FontSelect(data->font);
    if (data->scrollable) {
        UG_MeasureStringNoBreak(&(label->dimension.width), &(label->dimension.height), data->text);
        if (label->dimension.width < SCREEN_WIDTH) {
            // Do not scroll if text already fits in the screen
            data->scrollable = false;
        }
    } else if (
        data->position == CENTER || data->position == CENTER_TOP ||
        data->position == CENTER_BOTTOM) {
        UG_MeasureStringCentered(&(label->dimension.width), &(label->dimension.height), data->text);
    } else {
        UG_MeasureString(&(label->dimension.width), &(label->dimension.height), data->text);
    }
    UG_FontSetVSpace(0);
}

/********************************** Label Functions **********************************/

static const component_functions_t _component_functions = {
    .cleanup = _cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

static component_t* _label_create(
    const char* text,
    const bool upside_down,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t xoffset,
    uint8_t yoffset,
    bool scrollable,
    component_t* parent)
{
    component_t* label = malloc(sizeof(component_t));
    if (!label) {
        Abort("Error: malloc label");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc label data");
    }
    memset(data, 0, sizeof(data_t));
    memset(label, 0, sizeof(component_t));

    data->font = font != NULL ? font : &font_font_a_11X10;
    data->upside_down = upside_down;
    data->scrollable = scrollable;
    data->position = position;
    data->xoffset = xoffset;
    data->yoffset = yoffset;
    label->data = data;
    label->parent = parent;
    label->f = &_component_functions;

    if (data->scrollable) {
        ui_util_add_sub_component(label, knight_rider_create(label, SCREEN_HEIGHT - 1));
    }

    label_update(label, text);
    return label;
}

component_t* label_create(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent)
{
    return _label_create(text, false, font, position, 0, 0, false, parent);
}

component_t* label_create_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t xoffset,
    uint8_t yoffset,
    component_t* parent)
{
    return _label_create(text, false, font, position, xoffset, yoffset, false, parent);
}

component_t* label_create_scrollable(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent)
{
    return _label_create(text, false, font, position, 0, 0, true, parent);
}

component_t* label_create_scrollable_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t xoffset,
    uint8_t yoffset,
    component_t* parent)
{
    return _label_create(text, false, font, position, xoffset, yoffset, true, parent);
}
