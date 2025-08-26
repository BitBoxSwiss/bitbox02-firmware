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

#include "trinary_choice.h"

#include "button.h"
#include "label.h"

#include <hardfault.h>
#include <ui/ui_util.h>

#include <string.h>

typedef struct {
    void (*chosen_cb)(trinary_choice_t, void* param);
    void* chosen_cb_param;
} data_t;

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop,
};

static void _left_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    data->chosen_cb(TRINARY_CHOICE_LEFT, data->chosen_cb_param);
}

static void _middle_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    data->chosen_cb(TRINARY_CHOICE_MIDDLE, data->chosen_cb_param);
}

static void _right_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    data->chosen_cb(TRINARY_CHOICE_RIGHT, data->chosen_cb_param);
}

/********************************** Create Instance **********************************/

component_t* trinary_choice_create(
    const char* message,
    const char* label_left,
    const char* label_middle,
    const char* label_right,
    void (*chosen_cb)(trinary_choice_t, void*),
    void* chosen_cb_param,
    component_t* parent)
{
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc trinary choice data");
    }
    memset(data, 0, sizeof(data_t));

    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc trinary choice");
    }
    memset(component, 0, sizeof(component_t));
    component->data = data;
    component->parent = parent;
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    data->chosen_cb = chosen_cb;
    data->chosen_cb_param = chosen_cb_param;

    if (message != NULL) {
        ui_util_add_sub_component(component, label_create(message, NULL, CENTER, component));
    }

    component_t* button_left =
        button_create(label_left, bottom_slider, 0, _left_selected, component);
    ui_util_add_sub_component(component, button_left);

    component_t* button_middle =
        button_create(label_middle, bottom_slider, 0, _middle_selected, component);
    ui_util_add_sub_component(component, button_middle);

    component_t* button_right =
        button_create(label_right, bottom_slider, 0, _right_selected, component);
    ui_util_add_sub_component(component, button_right);

    ui_util_position_left_bottom_offset(component, button_left, 0, 0);
    ui_util_position_left_bottom_offset(
        component, button_middle, SCREEN_WIDTH / 2 - button_middle->dimension.width / 2, 0);
    ui_util_position_left_bottom_offset(
        component, button_right, SCREEN_WIDTH - button_right->dimension.width, 0);

    return component;
}
