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
#include <touch/gestures.h>
#include <ui/screen_stack.h>

typedef struct {
    void (*callback)(bool inserted, void* user_data);
    void* user_data;
    // TODO: use a TIMER interrupt to get a more accurate timer.
    // 250 is ~0.5 sec. Unit: rendering rate.
    int check_interval;
    int count;
} data_t;

static void _insert_poll_callback(component_t* component)
{
    data_t* data = (data_t*)component->data;
    if (data->callback && sd_card_inserted()) {
        data->callback(true, data->user_data);
        data->callback = NULL;
        return;
    }
}

static void _render(component_t* component)
{
    image_sdcard(screen_is_upside_down());
    ui_util_component_render_subcomponents(component);

    data_t* data = (data_t*)component->data;
    if (data->count == data->check_interval) {
        data->count = 0;
        _insert_poll_callback(component);
    }
    data->count++;
}

/********************************** Component Functions **********************************/

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = NULL,
};

/********************************** Create Instance **********************************/

static void _cancel_callback(void* user_data)
{
    component_t* self = (component_t*)user_data;
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(false, data->user_data);
        data->callback = NULL;
    }
}

component_t* sdcard_create(void (*callback)(bool inserted, void* user_data), void* user_data)
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

    data->callback = callback;
    data->user_data = user_data;
    data->check_interval = 250;
    data->count = 0;
    component->data = data;
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(
        component,
        label_create(
            "Insert SD card\nto continue",
            NULL,
            screen_is_upside_down() ? RIGHT_CENTER : LEFT_CENTER,
            component));
    ui_util_add_sub_component(
        component, icon_button_create(top_slider, ICON_BUTTON_CROSS, _cancel_callback, component));
    return component;
}
