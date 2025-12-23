// SPDX-License-Identifier: Apache-2.0

#include "confirm.h"
#include "../event.h"
#include "confirm_gesture.h"
#include "empty.h"
#include "icon_button.h"
#include "label.h"

#include <hardfault.h>
#include <screen.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ugui/ugui.h>

#include <string.h>

typedef struct {
    void (*callback)(bool, void* param);
    void* user_data;
} data_t;

static void _on_confirm(void* user_data)
{
    component_t* self = (component_t*)user_data;
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(true, data->user_data);
        data->callback = NULL;
    }
}

static void _on_cancel(void* user_data)
{
    component_t* self = (component_t*)user_data;
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(false, data->user_data);
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
    .on_event = NULL,
};

/********************************** Create Instance **********************************/

component_t* confirm_create(
    const confirm_params_t* params,
    void (*callback)(bool result, void* user_data),
    void* user_data)
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
    data->user_data = user_data;

    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    if (params->display_size) {
        char size_label[20];
        // Ignore warning, %u works for 32bit but not 64 bit (unit tests). %zu does not work with
        // our compiler.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wformat"
        snprintf(size_label, sizeof(size_label), "Size: %uB", params->display_size);
#pragma GCC diagnostic pop
        ui_util_add_sub_component(
            confirm, label_create(size_label, params->font, LEFT_BOTTOM, confirm));
    }

    slider_location_t slider_position = top_slider;

    // Create labels. We nest them in a body component that covers the screen minus the title bar,
    // so that the CENTER positioning starts below the title bar.

    const UG_FONT* font = &font_font_a_11X10;
    const char* title = params->title;
    // Arbitrary size big enough to fit all wrapped titles. Increase if needed.
    char wrapped_title[128] = {0};
    if (params->title_autowrap) {
        UG_FontSelect(font);
        if (strlen(title) + 1 < sizeof(wrapped_title)) {
            UG_WrapTitleString(title, wrapped_title, 55);
            title = wrapped_title;
        }
    }
    component_t* title_component = label_create(title, font, CENTER_TOP, confirm);
    ui_util_add_sub_component(confirm, title_component);

    component_t* body_container = empty_create();
    body_container->position.left = 0;
    // title bar height plus small padding
    body_container->position.top = title_component->dimension.height + 1;
    body_container->dimension.width = SCREEN_WIDTH;
    body_container->dimension.height = SCREEN_HEIGHT - body_container->position.top;
    ui_util_add_sub_component(confirm, body_container);

    if (params->scrollable) {
        ui_util_add_sub_component(
            body_container,
            label_create_scrollable(params->body, params->font, CENTER, body_container));
    } else {
        ui_util_add_sub_component(
            body_container, label_create(params->body, params->font, CENTER, body_container));
    }

    // Create buttons
    if (!params->accept_only) {
        ui_util_add_sub_component(
            confirm, icon_button_create(slider_position, ICON_BUTTON_CROSS, _on_cancel, confirm));
    }
    if (params->longtouch) {
        ui_util_add_sub_component(confirm, confirm_gesture_create(_on_confirm, confirm));
    } else {
        ui_util_add_sub_component(
            confirm,
            icon_button_create(
                slider_position,
                params->accept_is_nextarrow ? ICON_BUTTON_NEXT : ICON_BUTTON_CHECK,
                _on_confirm,
                confirm));
    }

    return confirm;
}
