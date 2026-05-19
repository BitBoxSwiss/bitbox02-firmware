// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include <screen.h>
#include <stdlib.h>
#include <ui/ui_util.h>

#include "fake_component.h"

/********************************** Label Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t FAKE_COMPONENT_FUNCTIONS = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = NULL};

/********************************** Create Instance **********************************/

component_t* fake_component_create(void)
{
    component_t* fake = malloc(sizeof(component_t));
    memset(fake, 0, sizeof(component_t));
    fake->f = &FAKE_COMPONENT_FUNCTIONS;

    fake->dimension.width = SCREEN_WIDTH;
    fake->dimension.height = SCREEN_HEIGHT;
    fake->position.left = 0;
    fake->position.top = 0;

    return fake;
}
