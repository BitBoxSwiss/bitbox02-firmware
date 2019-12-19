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
    // Max size of the text shown in the label. Increase if necessary.
    char text[200];
    const UG_FONT* font;
    bool upside_down;
    enum screen_position_t position;
    bool scrollable;
    bool slider_is_touched;
    bool slider_was_touched;
    uint16_t slider_position;
    float slider_position_diff;
    int16_t text_position;
    int16_t text_position_last;
} data_t;

static void _measure_label_dimensions(component_t* label);

void label_update(component_t* component, const char* text)
{
    data_t* data = (data_t*)component->data;
    snprintf(data->text, sizeof(data->text), "%s", text);
    if (component->parent == NULL) {
        return;
    }
    _measure_label_dimensions(component);
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
    case CUSTOM_OFFSET:
        // ui_util_position_left_center_offset(parent, component, data->offset);
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
}

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    // Slider indicators
    if (data->scrollable) {
        int x = data->slider_position * (SCREEN_WIDTH - 1) / MAX_SLIDER_POS;
        int y = SCREEN_HEIGHT - 1;
        if (!data->slider_was_touched) {
            data->text_position = component->dimension.width / 2 + SCREEN_WIDTH * 1 / 6;
            data->text_position_last = data->text_position;
            ui_util_component_render_subcomponents(component);
        } else if (data->slider_is_touched) {
            UG_DrawLine(MIN(SCREEN_WIDTH, x + 3), y, MAX(0, x - 3), y, screen_front_color);
        }
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
    if (data->scrollable) {
        switch (event->id) {
        case EVENT_BOTTOM_SLIDE: {
            gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
            // Variable scroll speed
            int16_t margin = SCREEN_WIDTH / 5;
            data->slider_position_diff += SIGMOID(slider_data->velocity);
            data->text_position = data->text_position_last + (int16_t)data->slider_position_diff;
            data->text_position = MIN(
                component->dimension.width / 2 + margin,
                MAX(-margin - component->dimension.width / 2 + SCREEN_WIDTH, data->text_position));
            data->slider_position = slider_data->position;
            data->slider_is_touched = true;
            data->slider_was_touched = true;
            break;
        }

        case EVENT_BOTTOM_SLIDE_RELEASED:
            data->text_position_last = data->text_position;
            data->slider_position_diff = 0;
            data->slider_is_touched = false;
            break;

        case EVENT_BOTTOM_CONTINUOUS_TAP: {
            gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
            data->slider_position = slider_data->position;
            data->slider_is_touched = true;
            data->slider_was_touched = true;
            break;
        }
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
    uint8_t offset,
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

    snprintf(data->text, sizeof(data->text), "%s", text);
    data->font = font != NULL ? font : &font_font_a_9X9;
    data->upside_down = upside_down;
    data->scrollable = scrollable;
    data->position = position;
    label->data = data;
    label->parent = parent;
    label->f = &_component_functions;

    _measure_label_dimensions(label);

    if (data->scrollable) {
        ui_util_add_sub_component(label, knight_rider_create(label, SCREEN_HEIGHT - 1));
    }

    switch (position) {
    case CENTER:
        ui_util_position_center(parent, label);
        break;
    case CENTER_TOP:
        ui_util_position_center_top(parent, label);
        break;
    case CENTER_BOTTOM:
        ui_util_position_center_bottom(parent, label);
        break;
    case LEFT_TOP:
        ui_util_position_left_top(parent, label);
        break;
    case LEFT_BOTTOM:
        ui_util_position_left_bottom(parent, label);
        break;
    case LEFT_CENTER:
        ui_util_position_left_center(parent, label);
        break;
    case CUSTOM_OFFSET:
        ui_util_position_left_center_offset(parent, label, offset);
        break;
    case RIGHT_CENTER:
        ui_util_position_right_center(parent, label);
        break;
    case RIGHT_TOP:
        ui_util_position_right_top(parent, label);
        break;
    case RIGHT_BOTTOM:
        ui_util_position_right_bottom(parent, label);
        break;
    default:
        Abort("position undefined or currently not implemented");
    }
    return label;
}

component_t* label_create(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent)
{
    return _label_create(text, false, font, position, 0, 0, parent);
}

component_t* label_create_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t offset,
    component_t* parent)
{
    return _label_create(text, false, font, position, offset, 0, parent);
}

component_t* label_create_scrollable(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent)
{
    return _label_create(text, false, font, position, 0, true, parent);
}
