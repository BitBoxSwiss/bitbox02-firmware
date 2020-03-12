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

#include "status.h"
#include "label.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
    bool status;
    int counter;
    int delay;
    void (*callback)(void*);
    void* callback_param;
} status_data_t;

static void _render(component_t* component)
{
    status_data_t* data = (status_data_t*)component->data;
    uint8_t height = 10;
    if (data->status) {
        image_checkmark(SCREEN_WIDTH / 6 * 5, SCREEN_HEIGHT / 2 - height / 2, height);
    } else {
        image_cross(SCREEN_WIDTH / 6 * 5, SCREEN_HEIGHT / 2 - height / 2, height);
    }
    if (data->callback != NULL) {
        if (data->counter == data->delay) {
            data->callback(data->callback_param);
            data->callback = NULL;
            data->counter = 0;
        }
        data->counter++;
    }
    ui_util_component_render_subcomponents(component);
}

/********************************** Component Functions **********************************/

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

component_t* status_create(
    const char* text,
    bool status_success,
    int delay,
    void (*callback)(void*),
    void* callback_param)
{
    component_t* status = malloc(sizeof(component_t));
    if (!status) {
        Abort("Error: malloc status");
    }
    status_data_t* data = malloc(sizeof(status_data_t));
    if (!data) {
        Abort("Error: malloc status data");
    }
    memset(data, 0, sizeof(status_data_t));
    memset(status, 0, sizeof(component_t));

    data->status = status_success;
    data->delay = delay;
    data->callback = callback;
    data->callback_param = callback_param;
    status->data = data;
    status->f = &_component_functions;
    status->dimension.width = SCREEN_WIDTH;
    status->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(status, label_create(text, NULL, LEFT_CENTER, status));

    return status;
}
