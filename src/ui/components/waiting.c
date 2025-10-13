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

#include "waiting.h"
#include "lockscreen.h"

#include "image.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <ui/ui_util.h>

#include <stdbool.h>
#include <string.h>

typedef struct {
    bool show_logo;
} data_t;

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = NULL,
};

/********************************** Create Instance **********************************/

/**
 * Creates a waiting screen.
 */
component_t* waiting_create(void)
{
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc waiting data");
    }
    memset(data, 0, sizeof(data_t));

    component_t* waiting = malloc(sizeof(component_t));
    if (!waiting) {
        Abort("Error: malloc waiting");
    }
    memset(waiting, 0, sizeof(component_t));
    waiting->f = &_component_functions;
    waiting->dimension.width = SCREEN_WIDTH;
    waiting->dimension.height = SCREEN_HEIGHT;
    waiting->position.top = 0;
    waiting->position.left = 0;
    waiting->data = data;

    ui_util_add_sub_component(waiting, lockscreen_create());

    return waiting;
}

void waiting_switch_to_logo(component_t* component)
{
    data_t* data = (data_t*)component->data;
    if (data->show_logo) {
        return;
    }
    data->show_logo = true;

    if (component->sub_components.amount != 1) {
        // Sanity check to avoid memory bugs, should never happen.
        Abort("waiting_switch_to_logo");
        return;
    }

    ui_util_component_cleanup(component->sub_components.sub_components[0]);

    image_logo_data_t logo = image_logo_data();
    component_t* bb2_logo = image_create(
        logo.buffer.data,
        logo.buffer.len,
        logo.dimensions.width,
        logo.dimensions.height,
        CENTER,
        component);

    component->sub_components.sub_components[0] = bb2_logo;
}
