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

#include "confirm.h"
#include "../event.h"
#include "confirm_gesture.h"
#include "icon_button.h"
#include "label.h"

#include <hardfault.h>
#include <screen.h>

#include <string.h>

typedef struct {
    void (*callback)(bool, void* param);
    void* callback_param;
} data_t;

static void _dispatch_confirm(component_t* self)
{
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(true, data->callback_param);
        data->callback = NULL;
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    if (event->id == EVENT_CONFIRM) {
        _dispatch_confirm(component);
    }
}

static void _on_confirm(component_t* component)
{
    component_t* self = component->parent;
    _dispatch_confirm(self);
}

static void _on_cancel(component_t* component)
{
    component_t* self = component->parent;
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(false, data->callback_param);
        data->callback = NULL;
    }
}

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

component_t* confirm_create(
    const confirm_params_t* params,
    void (*callback)(bool, void* param),
    void* callback_param)
{
    if (!callback) {
        Abort("confirm_create callback missing");
    }
    component_t* confirm = malloc(sizeof(component_t));
    if (!confirm) {
        Abort("Error: malloc confirm");
    }
    memset(confirm, 0, sizeof(component_t));

    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc confirm data");
    }
    memset(data, 0, sizeof(data_t));
    data->callback = callback;
    data->callback_param = callback_param;

    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    const char* body = params->body;
    size_t body_len = strlen(params->body);
    char short_body[64 + 3 + 1] = {0};
    if (params->shorten_body && body_len > 64) {
        snprintf(
            short_body,
            sizeof(short_body),
            "%.32s...%.32s",
            params->body,
            &params->body[body_len - 32]);
        body = short_body;
    }

    if (params->display_size) {
        char size_label[20];
        snprintf(size_label, sizeof(size_label), "Size: %uB", params->display_size);
        ui_util_add_sub_component(
            confirm, label_create(size_label, params->font, LEFT_BOTTOM, confirm));
    }

    slider_location_t slider_position = top_slider;
    // Create labels
    if (params->scrollable) {
        ui_util_add_sub_component(
            confirm, label_create_scrollable(body, params->font, CENTER, confirm));
    } else {
        ui_util_add_sub_component(confirm, label_create(body, params->font, CENTER, confirm));
    }
    ui_util_add_sub_component(confirm, label_create(params->title, NULL, CENTER_TOP, confirm));
    // Create buttons
    if (!params->accept_only) {
        ui_util_add_sub_component(
            confirm, icon_button_create(slider_position, ICON_BUTTON_CROSS, _on_cancel));
    }
    if (params->longtouch) {
        ui_util_add_sub_component(confirm, confirm_gesture_create(confirm));
    } else {
        ui_util_add_sub_component(
            confirm,
            icon_button_create(
                slider_position,
                params->accept_is_nextarrow ? ICON_BUTTON_NEXT : ICON_BUTTON_CHECK,
                _on_confirm));
    }

    return confirm;
}
