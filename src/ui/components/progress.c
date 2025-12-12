// SPDX-License-Identifier: Apache-2.0

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
    .on_event = NULL,
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
