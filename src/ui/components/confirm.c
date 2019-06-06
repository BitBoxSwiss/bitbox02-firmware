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
#include "ui_components.h"
#include <hardfault.h>
#include <screen.h>
#include <string.h>

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

static component_t* _confirm_create(
    const char* title,
    const char* body,
    bool scrollable,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
    component_t* confirm = malloc(sizeof(component_t));
    if (!confirm) {
        Abort("Error: malloc confirm");
    }
    memset(confirm, 0, sizeof(component_t));

    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;
    uint8_t slider_position = scrollable ? top_slider : bottom_slider;
    // Create labels
    if (scrollable) {
        ui_util_add_sub_component(confirm, label_create_scrollable(body, NULL, CENTER, confirm));
    } else {
        ui_util_add_sub_component(confirm, label_create(body, NULL, CENTER, confirm));
    }
    ui_util_add_sub_component(confirm, label_create(title, NULL, CENTER_TOP, confirm));
    // Create buttons
    if (cancel_callback != NULL) {
        ui_util_add_sub_component(
            confirm, icon_button_create(slider_position, ICON_BUTTON_CROSS, cancel_callback));
    }
    if (confirm_callback != NULL) {
        ui_util_add_sub_component(
            confirm, icon_button_create(slider_position, ICON_BUTTON_CHECK, confirm_callback));
    }

    return confirm;
}

component_t* confirm_create(
    const char* title,
    const char* body,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
    return _confirm_create(title, body, false, confirm_callback, cancel_callback);
}

component_t* confirm_create_scrollable(
    const char* title,
    const char* body,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*))
{
    return _confirm_create(title, body, true, confirm_callback, cancel_callback);
}
