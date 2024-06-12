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

#include "sdcard.h"
#include "icon_button.h"
#include "label.h"
#include "ui_images.h"
#include <hardfault.h>
#include <screen.h>
#include <sd.h>
#include <string.h>
#include <touch/gestures.h>
#include <ui/screen_stack.h>

typedef struct {
    // if true, the callback won't be called until the sd card is inserted.
    // the insert/remove label changes depending on this flag.
    bool insert;
    void (*callback)(bool, void*);
    void* callback_param;
} data_t;

static void _render(component_t* component)
{
    image_sdcard(screen_is_upside_down());
    ui_util_component_render_subcomponents(component);
}

/********************************** Component Functions **********************************/

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

static void _continue_callback(component_t* component)
{
    data_t* data = (data_t*)component->parent->data;
    if (!data->insert || sd_card_inserted()) {
        if (data->callback) {
            data->callback(true, data->callback_param);
            data->callback = NULL;
        }
    }
}

static void _cancel_callback(component_t* component)
{
    data_t* data = (data_t*)component->parent->data;
    if (data->callback) {
        data->callback(false, data->callback_param);
        data->callback = NULL;
    }
}

component_t* sdcard_create(bool insert, void (*callback)(bool, void*), void* callback_param)
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc sdcard");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc sdcard data");
    }
    memset(data, 0, sizeof(data_t));
    memset(component, 0, sizeof(component_t));

    data->insert = insert;
    data->callback = callback;
    data->callback_param = callback_param;
    component->data = data;
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(
        component,
        label_create(
            insert ? "Insert SD card\nto continue" : "Remove SD card\nto continue",
            NULL,
            screen_is_upside_down() ? RIGHT_CENTER : LEFT_CENTER,
            component));
    ui_util_add_sub_component(
        component, icon_button_create(top_slider, ICON_BUTTON_CHECK, _continue_callback));
    if (insert) {
        ui_util_add_sub_component(
            component, icon_button_create(top_slider, ICON_BUTTON_CROSS, _cancel_callback));
    }
    return component;
}
