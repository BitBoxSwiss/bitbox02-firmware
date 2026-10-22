// SPDX-License-Identifier: Apache-2.0

#include "status.h"
#include "label.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <stdbool.h>
#include <string.h>
#include <ui/fonts/regular_fonts.h>

typedef struct {
    bool status;
    bool icon_below;
} status_data_t;

static void _render(component_t* component)
{
    status_data_t* data = (status_data_t*)component->data;
    uint8_t height = 10;
    int16_t width = data->status ? height + height / 2 - 1 : height + 1;
    int16_t x = data->icon_below ? (SCREEN_WIDTH - width) / 2 : SCREEN_WIDTH / 6 * 5;
    int16_t y = data->icon_below ? SCREEN_HEIGHT - height - 2 : SCREEN_HEIGHT / 2 - height / 2;
    if (data->status) {
        image_checkmark(x, y, height);
    } else {
        image_cross(x, y, height);
    }
    ui_util_component_render_subcomponents(component);
}

/********************************** Component Functions **********************************/

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = NULL,
};

/********************************** Create Instance **********************************/

component_t* status_create(const char* text, bool status_success)
{
    component_t* status = malloc(sizeof(component_t));
    if (!status) {
        Abort("Error: malloc status");
    }
    status_data_t* data = malloc(sizeof(status_data_t));
    if (!data) {
        Abort("Error: malloc status data");
    }
    memset(data, 0, sizeof(status_data_t));
    memset(status, 0, sizeof(component_t));

    data->status = status_success;
    status->data = data;
    status->f = &_component_functions;
    status->dimension.width = SCREEN_WIDTH;
    status->dimension.height = SCREEN_HEIGHT;

    data->icon_below = !label_fits_width(text, NULL, SCREEN_WIDTH / 6 * 5 - 5);
    component_t* label;
    if (data->icon_below) {
        const UG_FONT* font =
            label_fits_width(text, NULL, SCREEN_WIDTH) ? &font_regular_11 : &font_regular_9;
        label = label_create(text, font, CENTER, status);
        label->position.top = (SCREEN_HEIGHT - 12 - label->dimension.height) / 2;
    } else {
        label = label_create(text, NULL, LEFT_CENTER, status);
    }
    ui_util_add_sub_component(status, label);

    return status;
}
