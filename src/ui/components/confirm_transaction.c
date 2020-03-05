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
#include "confirm_button.h"
#include "icon_button.h"
#include "label.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <stdbool.h>
#include <string.h>
#include <util.h>

typedef struct {
    bool has_address;
    void (*cancel_callback)(void);
    void (*confirm_callback)(void);
} data_t;

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    ui_util_component_render_subcomponents(component);
    if (data->has_address) {
        image_arrow(SCREEN_WIDTH / 2 - IMAGE_DEFAULT_ARROW_HEIGHT, 34, IMAGE_DEFAULT_ARROW_HEIGHT, ARROW_DOWN);
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    if (event->id == EVENT_CONFIRM) {
        data_t* data = (data_t*)component->data;
        if (data->confirm_callback) {
            data->confirm_callback();
            data->confirm_callback = NULL;
        }
    }
}

static void _cancel(component_t* cancel_button)
{
    component_t* component = cancel_button->parent;
    data_t* data = (data_t*)component->data;
    if (data->cancel_callback != NULL) {
        data->cancel_callback();
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
    void (*confirm_callback)(void),
    void (*cancel_callback)(void))
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
    data->confirm_callback = confirm_callback;
    data->cancel_callback = cancel_callback;
    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(confirm, icon_button_create(top_slider, ICON_BUTTON_CROSS, _cancel));

    bool longtouch = verify_total;
    ui_util_add_sub_component(confirm, confirm_button_create(longtouch, ICON_BUTTON_NEXT));

    if (data->has_address) {
        ui_util_add_sub_component(confirm, label_create_scrollable_offset(address, NULL, CENTER, 0, 20, confirm));
    }
    if (strlens(fee)) {
        ui_util_add_sub_component(confirm, label_create_offset("Fee", NULL, CENTER_TOP, 0, 38, confirm));
        ui_util_add_sub_component(confirm, label_create_offset(fee, NULL, CENTER_TOP, 0, 50, confirm));
    }
    if (verify_total) {
        ui_util_add_sub_component(confirm, label_create_offset("Total", NULL, CENTER_TOP, 0, 8, confirm));
        ui_util_add_sub_component(confirm, label_create_offset(amount, NULL, CENTER_TOP, 0, 22, confirm));
    } else {
        ui_util_add_sub_component(confirm, label_create_offset(amount, NULL, CENTER_TOP, 0, 17, confirm));
    }

    return confirm;
}

component_t* confirm_transaction_address_create(
    const char* amount,
    const char* address,
    void (*confirm_callback)(void),
    void (*cancel_callback)(void))
{
    return _confirm_transaction_create(
        amount, address, NULL, false, confirm_callback, cancel_callback);
}

component_t* confirm_transaction_fee_create(
    const char* amount,
    const char* fee,
    void (*confirm_callback)(void),
    void (*cancel_callback)(void))
{
    return _confirm_transaction_create(amount, NULL, fee, true, confirm_callback, cancel_callback);
}
