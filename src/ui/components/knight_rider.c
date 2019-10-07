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

#include "knight_rider.h"

#include <hardfault.h>
#include <ui/ui_util.h>
#include <util.h>

#include <stdbool.h>
#include <string.h>

typedef struct {
    uint8_t height;
    uint16_t screen_count;
} knight_rider_data_t;

/**
 * Cosine wave motion
 */
static void _render(component_t* component)
{
    knight_rider_data_t* data = (knight_rider_data_t*)component->data;
    int x, y;
    float cos = (float)((512 / 4 + data->screen_count++) % (512 + 1));
    cos = (float)(cos / 512 * 3.14 * 2 - 3.14);
    if (cos > 0) {
        cos = 1.27323954F * cos - 0.405284735F * cos * cos;
    } else {
        cos = 1.27323954F * cos + 0.405284735F * cos * cos;
    }
    x = (int)(cos * 32) + SCREEN_WIDTH / 2 - 2;
    y = data->height;
    UG_DrawLine(x - 4, y, x + 4, y, screen_front_color);
}

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

/**
 * Creates a Knight-Rider style moving bar component located at height vertical position.
 */
component_t* knight_rider_create(component_t* parent, uint8_t height)
{
    component_t* knight_rider = malloc(sizeof(component_t));
    if (!knight_rider) {
        Abort("Error: malloc knight_rider");
    }
    knight_rider_data_t* data = malloc(sizeof(knight_rider_data_t));
    if (!data) {
        Abort("Error: malloc knight_rider data");
    }
    memset(knight_rider, 0, sizeof(component_t));
    memset(data, 0, sizeof(knight_rider_data_t));
    data->screen_count = 0;
    data->height = height;

    knight_rider->data = data;
    knight_rider->f = &_component_functions;
    knight_rider->parent = parent;

    return knight_rider;
}
