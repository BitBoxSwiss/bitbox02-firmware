// SPDX-License-Identifier: Apache-2.0

#include "knight_rider.h"

#include <hardfault.h>
#include <ui/ui_util.h>

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
    .on_event = NULL,
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
