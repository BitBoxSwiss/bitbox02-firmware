// SPDX-License-Identifier: Apache-2.0

#include "empty.h"
#include <hardfault.h>
#include <ui/ui_util.h>

#include <string.h>

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = NULL,
};

/********************************** Create Instance **********************************/
component_t* empty_create(void)
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc empty component");
    }
    memset(component, 0, sizeof(component_t));
    component->f = &_component_functions;
    return component;
}
