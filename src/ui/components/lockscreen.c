// SPDX-License-Identifier: Apache-2.0

#include "lockscreen.h"
#include "../ui_util.h"
#include "label.h"

#include <hardfault.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <screen.h>
#include <string.h>
#include <touch/gestures.h>
#include <ui/fonts/arial_fonts.h>
#include <util.h>

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

// Outputs `in` as is if it can be rendered to fit in `max_width`.
// If it can't, it is truncated (with appended "...") to a size where it fits.
static void _truncate_to_fit(
    const char* in,
    char* out,
    size_t out_len,
    const UG_FONT* font,
    UG_S16 max_width)
{
    if (out == NULL || out_len == 0) {
        return;
    }
    if (out_len < 4) {
        out[0] = 0;
        return;
    }
    if (in[0] == 0) {
        out[0] = 0;
        return;
    }
    UG_S16 width = 0;
    UG_S16 height = 0;
    UG_FontSelect(font);
    UG_MeasureStringCentered(&width, &height, in);

    // Name fits without truncation.
    if (width <= max_width) {
        if (util_utf8_strlcpy(out, in, out_len) < 0) {
            out[0] = 0;
        }
        return;
    }

    // Truncate if too long to a size where "<name>..." fits.
    const size_t text_capacity = out_len - 4;
    size_t truncate_len = MIN(strlen(in), text_capacity);
    do {
        const intptr_t result = util_utf8_copy(out, out_len, in, truncate_len);
        if (result < 0) {
            out[0] = 0;
            return;
        }
        const size_t copied_len = (size_t)result;
        memcpy(&out[copied_len], "...", 4);
        UG_MeasureStringCentered(&width, &height, out);
        if (truncate_len == 0) {
            break;
        }
        truncate_len--;
    } while (width >= max_width);
}

component_t* lockscreen_create(void)
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc lockscreen component");
    }
    memset(component, 0, sizeof(component_t));
    component->f = &_component_functions;

    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    const UG_FONT* device_name_font = &font_arial_9;

    char device_name[MEMORY_DEVICE_MAX_LEN_WITH_NULL] = {0};
    memory_get_device_name(device_name);
    // Show nothing if the name is the default name.
    if (STREQ(device_name, MEMORY_DEFAULT_DEVICE_NAME)) {
        device_name[0] = 0;
    }

    char display_name[MEMORY_DEVICE_MAX_LEN_WITH_NULL + 3] = {0};
    _truncate_to_fit(
        device_name,
        display_name,
        sizeof(display_name),
        device_name_font,
        component->dimension.width);
    ui_util_add_sub_component(
        component, label_create("See the BitBoxApp", NULL, CENTER, component));
    ui_util_add_sub_component(
        component, label_create(display_name, device_name_font, CENTER_BOTTOM, component));

    return component;
}
