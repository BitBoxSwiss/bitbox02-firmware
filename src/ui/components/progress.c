// Copyright 2020 Shift Crypto AG
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

#include "progress.h"
#include "label.h"
#include <hardfault.h>
#include <screen.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#include <string.h>

typedef struct {
    float progress;
} data_t;

static void _render(component_t* component)
{
    const data_t* data = (const data_t*)component->data;
    const uint16_t bar_height = 5;
    UG_FillFrame(
        0, SCREEN_HEIGHT - bar_height, SCREEN_WIDTH * data->progress, SCREEN_HEIGHT, C_WHITE);

    ui_util_component_render_subcomponents(component);
}

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

component_t* progress_create(const char* title)
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc progress component");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc progress data");
    }
    memset(component, 0, sizeof(component_t));
    memset(data, 0, sizeof(data_t));
    component->data = data;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;
    component->f = &_component_functions;
    ui_util_add_sub_component(component, label_create(title, NULL, CENTER, component));
    return component;
}

void progress_set(component_t* component, float progress)
{
    data_t* data = (data_t*)component->data;
    data->progress = progress;
}
