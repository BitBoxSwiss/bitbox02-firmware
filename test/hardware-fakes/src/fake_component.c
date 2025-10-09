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

/**
 * Creates a label with the given font either upside down or normal.
 * @param[in] text The text of the label.
 * @param[in] upside_down Whether the text should be rotated 180 degree or not.
 * @param[in] font The font of the label.
 */
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
