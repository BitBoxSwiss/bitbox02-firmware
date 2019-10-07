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

#include "remove_sd_card.h"
#include "icon_button.h"
#include "label.h"
#include "ui_images.h"
#include <hardfault.h>
#include <screen.h>
#include <string.h>
#include <touch/gestures.h>
#include <ui/screen_stack.h>

typedef struct {
    void (*continue_callback)(void);
} data_t;

static void _dismiss(component_t* component)
{
    data_t* data = (data_t*)component->parent->data;
    if (data->continue_callback) {
        data->continue_callback();
        data->continue_callback = NULL;
    }
}

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

/**
 * Creates a remove SD card screen.
 */
component_t* remove_sd_card_create(void (*continue_callback)(void))
{
    component_t* remove_sd_card = malloc(sizeof(component_t));
    if (!remove_sd_card) {
        Abort("Error: malloc remove_sd_card");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc remove_sd_card data");
    }
    memset(data, 0, sizeof(data_t));
    memset(remove_sd_card, 0, sizeof(component_t));

    data->continue_callback = continue_callback;
    remove_sd_card->data = data;
    remove_sd_card->f = &_component_functions;
    remove_sd_card->dimension.width = SCREEN_WIDTH;
    remove_sd_card->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(
        remove_sd_card,
        label_create(
            "Remove SD card\nto continue",
            NULL,
            screen_is_upside_down() ? RIGHT_CENTER : LEFT_CENTER,
            remove_sd_card));
    ui_util_add_sub_component(
        remove_sd_card, icon_button_create(bottom_slider, ICON_BUTTON_CHECK, _dismiss));

    return remove_sd_card;
}
