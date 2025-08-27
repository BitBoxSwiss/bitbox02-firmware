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

#include "mock_component.h"

/********************************** Label Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t MOCK_COMPONENT_FUNCTIONS = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop};

/********************************** Create Instance **********************************/

/**
 * Creates a label with the given font either upside down or normal.
 * @param[in] text The text of the label.
 * @param[in] upside_down Whether the text should be rotated 180 degree or not.
 * @param[in] font The font of the label.
 */
component_t* mock_component_create(void)
{
    component_t* mock = malloc(sizeof(component_t));
    memset(mock, 0, sizeof(component_t));
    mock->f = &MOCK_COMPONENT_FUNCTIONS;

    mock->dimension.width = SCREEN_WIDTH;
    mock->dimension.height = SCREEN_HEIGHT;
    mock->position.left = 0;
    mock->position.top = 0;

    return mock;
}
