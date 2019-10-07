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
        image_arrow(SCREEN_WIDTH / 2, 33, IMAGE_DEFAULT_ARROW_HEIGHT, ARROW_DOWN);
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
    ui_util_add_sub_component(confirm, confirm_button_create(longtouch));

    if (data->has_address) {
        char addr[128];
        snprintf(addr, sizeof(addr), " \n \n \n%s", address);
        ui_util_add_sub_component(confirm, label_create_scrollable(addr, NULL, CENTER, confirm));
    }
    if (strlens(fee)) {
        char formatted_fee[64];
        snprintf(formatted_fee, sizeof(formatted_fee), "\n\n\n\n\n%s", fee);
        ui_util_add_sub_component(confirm, label_create("\n\n\n\nFee:", NULL, CENTER_TOP, confirm));
        ui_util_add_sub_component(confirm, label_create(formatted_fee, NULL, CENTER_TOP, confirm));
    }
    if (verify_total) {
        char formatted_amt[64];
        snprintf(formatted_amt, sizeof(formatted_amt), "\n\n%s", amount);
        ui_util_add_sub_component(confirm, label_create("\nTotal", NULL, CENTER_TOP, confirm));
        ui_util_add_sub_component(confirm, label_create(formatted_amt, NULL, CENTER_TOP, confirm));
    } else {
        char formatted_amt[64];
        snprintf(formatted_amt, sizeof(formatted_amt), "\n\n%s", amount);
        ui_util_add_sub_component(confirm, label_create(formatted_amt, NULL, CENTER_TOP, confirm));
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
