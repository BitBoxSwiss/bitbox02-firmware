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

#include "confirm_transaction.h"
#include "confirm_gesture.h"
#include "icon_button.h"
#include "label.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <stdbool.h>
#include <string.h>
#include <ui/fonts/arial_fonts.h>
#include <util.h>

// Empirically measured when the amount goes out of screen with the 11x10 font and we should switch
// to the smaller 9x9 font.
#define BIG_FONT_MAX_CHARS 19

typedef struct {
    bool has_address;
    // accepted: true means the user accepted the info shown, false means the user rejected the
    // info.
    void (*callback)(bool accepted, void* user_data);
    void* user_data;
} data_t;

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    ui_util_component_render_subcomponents(component);
    if (data->has_address) {
        image_arrow(
            SCREEN_WIDTH / 2 - IMAGE_DEFAULT_ARROW_HEIGHT,
            34,
            IMAGE_DEFAULT_ARROW_HEIGHT,
            ARROW_DOWN);
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    if (event->id == EVENT_CONFIRM) {
        data_t* data = (data_t*)component->data;
        if (data->callback) {
            data->callback(true, data->user_data);
            data->callback = NULL;
        }
    }
}

static void _cancel(component_t* cancel_button)
{
    component_t* component = cancel_button->parent;
    data_t* data = (data_t*)component->data;
    if (data->callback != NULL) {
        data->callback(false, data->user_data);
        data->callback = NULL;
    }
}

static void _confirm_button_cb(component_t* confirm_button)
{
    component_t* component = confirm_button->parent;
    data_t* data = (data_t*)component->data;
    if (data->callback) {
        data->callback(true, data->user_data);
        data->callback = NULL;
    }
}

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

static component_t* _confirm_transaction_create(
    const char* amount,
    const char* address,
    const char* fee,
    bool verify_total, /* if true, verify total and fee, otherwise verify amount and address */
    bool longtouch,
    void (*callback)(bool accepted, void* user_data),
    void* user_data)
{
    if (address && fee) {
        Abort("Error: confirm btc does not support displaying both address and fee");
    }
    if (!amount) {
        Abort("Error: confirm btc amount not present");
    }
    component_t* confirm = malloc(sizeof(component_t));
    if (!confirm) {
        Abort("Error: malloc confirm btc");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc confirm btc data");
    }
    memset(data, 0, sizeof(data_t));
    memset(confirm, 0, sizeof(component_t));

    data->has_address = strlens(address);
    data->callback = callback;
    data->user_data = user_data;
    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(confirm, icon_button_create(top_slider, ICON_BUTTON_CROSS, _cancel));

    if (longtouch) {
        ui_util_add_sub_component(confirm, confirm_gesture_create());
    } else {
        ui_util_add_sub_component(
            confirm, icon_button_create(top_slider, ICON_BUTTON_NEXT, _confirm_button_cb));
    }

    if (data->has_address) {
        ui_util_add_sub_component(
            confirm, label_create_scrollable_offset(address, NULL, CENTER, 0, 20, confirm));
    }
    if (strlens(fee)) {
        ui_util_add_sub_component(
            confirm, label_create_offset("Fee", &font_font_a_9X9, CENTER_TOP, 0, 38, confirm));

        ui_util_add_sub_component(
            confirm, label_create_offset(fee, &font_font_a_9X9, CENTER_TOP, 0, 50, confirm));
    }
    const UG_FONT* amount_font = NULL;
    if (strlen(amount) > BIG_FONT_MAX_CHARS) {
        amount_font = &font_font_a_9X9;
    }
    if (verify_total) {
        ui_util_add_sub_component(
            confirm, label_create_offset("Total", NULL, CENTER_TOP, 0, 8, confirm));
        ui_util_add_sub_component(
            confirm, label_create_offset(amount, amount_font, CENTER_TOP, 0, 22, confirm));
    } else {
        ui_util_add_sub_component(
            confirm, label_create_offset(amount, amount_font, CENTER_TOP, 0, 17, confirm));
    }

    return confirm;
}

component_t* confirm_transaction_address_create(
    const char* amount,
    const char* address,
    void (*callback)(bool accepted, void* user_data),
    void* user_data)
{
    return _confirm_transaction_create(amount, address, NULL, false, false, callback, user_data);
}

component_t* confirm_transaction_fee_create(
    const char* amount,
    const char* fee,
    bool longtouch,
    void (*callback)(bool accepted, void* user_data),
    void* user_data)
{
    return _confirm_transaction_create(amount, NULL, fee, true, longtouch, callback, user_data);
}
