// SPDX-License-Identifier: Apache-2.0

#include "confirm_swap.h"
#include "icon_button.h"
#include "label.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <string.h>
#include <ui/fonts/arial_fonts.h>
#include <util.h>

// Empirically measured when the amount goes out of screen with the 11x10 font and we should switch
// to the smaller 9x9 font.
#define BIG_FONT_MAX_CHARS 19

typedef struct {
    void (*callback)(bool accepted, void* user_data);
    void* user_data;
} data_t;

static void _render(component_t* component)
{
    ui_util_component_render_subcomponents(component);
    image_arrow(
        SCREEN_WIDTH / 2 - IMAGE_DEFAULT_ARROW_HEIGHT, 34, IMAGE_DEFAULT_ARROW_HEIGHT, ARROW_DOWN);
}

static void _cancel_cb(void* user_data)
{
    component_t* self = (component_t*)user_data;
    data_t* data = (data_t*)self->data;
    if (data->callback != NULL) {
        data->callback(false, data->user_data);
        data->callback = NULL;
    }
}

static void _confirm_cb(void* user_data)
{
    component_t* self = (component_t*)user_data;
    data_t* data = (data_t*)self->data;
    if (data->callback != NULL) {
        data->callback(true, data->user_data);
        data->callback = NULL;
    }
}

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = NULL,
};

component_t* confirm_swap_create(
    const char* title,
    const char* from,
    const char* to,
    void (*callback)(bool accepted, void* user_data),
    void* user_data)
{
    if (!callback) {
        Abort("confirm_swap_create callback missing");
    }
    if (!strlens(title)) {
        Abort("confirm_swap_create title missing");
    }
    if (!strlens(from)) {
        Abort("confirm_swap_create from missing");
    }
    if (!strlens(to)) {
        Abort("confirm_swap_create to missing");
    }

    component_t* confirm = malloc(sizeof(component_t));
    if (!confirm) {
        Abort("Error: malloc confirm swap");
    }
    memset(confirm, 0, sizeof(component_t));

    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc confirm swap data");
    }
    memset(data, 0, sizeof(data_t));
    data->callback = callback;
    data->user_data = user_data;

    confirm->data = data;
    confirm->f = &_component_functions;
    confirm->dimension.width = SCREEN_WIDTH;
    confirm->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(
        confirm, icon_button_create(top_slider, ICON_BUTTON_CROSS, _cancel_cb, confirm));
    ui_util_add_sub_component(
        confirm, icon_button_create(top_slider, ICON_BUTTON_NEXT, _confirm_cb, confirm));

    component_t* title_component = label_create(title, &font_font_a_11X10, CENTER_TOP, confirm);
    ui_util_add_sub_component(confirm, title_component);

    const UG_FONT* from_font = NULL;
    if (strlen(from) > BIG_FONT_MAX_CHARS) {
        from_font = &font_font_a_9X9;
    }
    const UG_FONT* to_font = NULL;
    if (strlen(to) > BIG_FONT_MAX_CHARS) {
        to_font = &font_font_a_9X9;
    }

    ui_util_add_sub_component(
        confirm, label_create_offset(from, from_font, CENTER_TOP, 0, 17, confirm));
    ui_util_add_sub_component(confirm, label_create_offset(to, to_font, CENTER, 0, 20, confirm));

    return confirm;
}
