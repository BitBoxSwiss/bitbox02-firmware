// Copyright 2020 Shift Cryptosecurity AG
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

#include "empty.h"
#include <hardfault.h>
#include <ui/ui_util.h>

#include <string.h>

static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop,
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
