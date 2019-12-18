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
    void (*confirm_callback)(component_t*);
} data_t;

static void _on_event(const event_t* event, component_t* component)
{
    (void)component;
    if (event->id == EVENT_CONFIRM) {
        data_t* data = (data_t*)component->data;
        if (data->confirm_callback) {
            data->confirm_callback(NULL);
            data->confirm_callback = NULL;
        }
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

static component_t* _confirm_create(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool scrollable,
    bool longtouch,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
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
    data->confirm_callback = confirm_callback;

    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    uint8_t slider_position = scrollable || longtouch ? top_slider : bottom_slider;
    // Create labels
    if (scrollable) {
        ui_util_add_sub_component(confirm, label_create_scrollable(body, font, CENTER, confirm));
    } else {
        ui_util_add_sub_component(confirm, label_create(body, font, CENTER, confirm));
    }
    ui_util_add_sub_component(confirm, label_create(title, NULL, CENTER_TOP, confirm));
    // Create buttons
    if (cancel_callback != NULL) {
        ui_util_add_sub_component(
            confirm, icon_button_create(slider_position, ICON_BUTTON_CROSS, cancel_callback));
    }
    if (confirm_callback != NULL) {
        if (longtouch) {
            ui_util_add_sub_component(confirm, confirm_gesture_create(confirm));
        } else {
            ui_util_add_sub_component(
                confirm, icon_button_create(slider_position, ICON_BUTTON_CHECK, confirm_callback));
        }
    }

    return confirm;
}

component_t* confirm_create(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool longtouch,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
    return _confirm_create(title, body, font, false, longtouch, confirm_callback, cancel_callback);
}

component_t* confirm_create_scrollable(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool longtouch,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
    return _confirm_create(title, body, font, true, longtouch, confirm_callback, cancel_callback);
}
