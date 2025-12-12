// SPDX-License-Identifier: Apache-2.0

#include "screensaver.h"

#include "image.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <ui/ui_util.h>

#include <stdint.h>
#include <string.h>

static void _render(component_t* component)
{
    component_t* image = component->sub_components.sub_components[0];

    // The counter is used to slow down the animation, see the slowdown vars below.
    static uint16_t counter = 0;
    counter++;

    // these flip between positive and negative when boncing, can also be used to move multiple
    // pixels per frame
    static int8_t x_direction = 1;
    static int8_t y_direction = 1;
    // setting relative speed for both axes
    const int8_t x_slowdown = 6;
    const int8_t y_slowdown = 6;

    if (counter % x_slowdown == 0) {
        image->position.left += x_direction;
        // if the screensaver is at the edge (or outside e.g. due to screensaver_reset), and moving
        // away from the screen, flip the direction so it will always be moving inside or towards
        // the screen
        if ((x_direction > 0 &&
             (image->position.left + image->dimension.width) >= component->dimension.width) ||
            (x_direction < 0 && image->position.left < 0)) {
            x_direction *= -1;
        }
    }
    if (counter % y_slowdown == 0) {
        image->position.top += y_direction;
        // if the screensaver is at the edge (or outside e.g. due to screensaver_reset), and moving
        // away from the screen, flip the direction so it will always be moving inside or towards
        // the screen
        if ((y_direction > 0 &&
             (image->position.top + image->dimension.height) >= component->dimension.height) ||
            (y_direction < 0 && image->position.top < 0)) {
            y_direction *= -1;
        }
    }
    ui_util_component_render_subcomponents(component);
}

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = NULL,
};

component_t* screensaver_create(void)
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc screensaver");
    }
    memset(component, 0, sizeof(component_t));
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;
    component_t* screensaver_image = image_create(
        IMAGE_SCREENSAVER,
        sizeof(IMAGE_SCREENSAVER),
        IMAGE_SCREENSAVER_W,
        IMAGE_SCREENSAVER_H,
        CENTER,
        component);
    ui_util_add_sub_component(component, screensaver_image);
    screensaver_reset(component);
    return component;
}

void screensaver_reset(component_t* component)
{
    component_t* image = component->sub_components.sub_components[0];
    image->position.left = -image->dimension.width;
    // We move the image even more out of screen, to have a moment where the screen is just black
    // before the screensaver becomes visible.
    image->position.left -= 30;
}
