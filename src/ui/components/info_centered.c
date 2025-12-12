// SPDX-License-Identifier: Apache-2.0

#include "info_centered.h"
#include "../ui_util.h"
#include "button.h"
#include "label.h"

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
    .on_event = NULL,
};

/********************************** Create Instance **********************************/

/**
 * Creates a info screen that renders the given text in the center. If a skip callback
 * is defined, a skip button is rendered, which calls the callback if pushed.
 * @param[in] text The info text.
 * @param[in] skip_callback The optional skip callback.
 */
component_t* info_centered_create(const char* text, void (*skip_callback)(component_t*))
{
    component_t* info_centered = malloc(sizeof(component_t));
    if (!info_centered) {
        Abort("Error: malloc info_centered");
    }
    memset(info_centered, 0, sizeof(component_t));

    info_centered->f = &_component_functions;

    info_centered->dimension.width = SCREEN_WIDTH;
    info_centered->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(info_centered, label_create(text, NULL, CENTER, info_centered));
    if (skip_callback != NULL) {
        component_t* skip_button =
            button_create("Skip", bottom_slider, SCREEN_WIDTH / 2, skip_callback, info_centered);
        ui_util_add_sub_component(info_centered, skip_button);
    }

    return info_centered;
}
