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

#include "image.h"

#include <hardfault.h>
#include <screen.h>
#include <ui/component.h>

#include <stdint.h>
#include <string.h>

/**
 * The image data.
 */
typedef struct {
    const uint8_t* image_bytes;
    uint16_t num_bytes;
} image_data_t;

/**
 * Renders the image.
 * @param[in] component The image component.
 */
static void _render(component_t* component)
{
    image_data_t* data = (image_data_t*)component->data;
    int x = component->position.left;
    int y = component->position.top;
    for (uint16_t i = 0; i < data->num_bytes; i++) {
        uint8_t b = data->image_bytes[i];
        for (int j = 0; j < 8; j++) {
            if (b & 0x80) {
                UG_DrawPixel(x, y, screen_front_color);
            }
            b <<= 1;
            x++;
            if (((x - component->position.left) % component->dimension.width) == 0) {
                x = component->position.left;
                y++;
            }
        }
    }
}

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

/**
 * Creates an image from the given image bytes, width and height and
 * displays it in the given position.
 * @param[IN] image_bytes The image bytes.
 * @param[IN] width The width of the image.
 * @param[IN] height The height of the image.
 * @param[IN] position The position of the image.
 * @param[IN] parent The parent component.
 */
component_t* image_create(
    const uint8_t* image_bytes,
    const uint16_t num_bytes,
    const uint16_t width,
    const uint16_t height,
    const enum screen_position_t position,
    component_t* parent)
{
    image_data_t* data = malloc(sizeof(image_data_t));
    if (!data) {
        Abort("Error: malloc image data");
    }
    memset(data, 0, sizeof(image_data_t));

    data->image_bytes = image_bytes;
    data->num_bytes = num_bytes;

    component_t* image = malloc(sizeof(component_t));
    if (!image) {
        Abort("Error: malloc image");
    }
    memset(image, 0, sizeof(component_t));

    image->f = &_component_functions;
    image->data = data;
    image->parent = parent;
    image->dimension.width = width;
    image->dimension.height = height;

    switch (position) {
    case CENTER:
        ui_util_position_center(parent, image);
        break;
    case CENTER_TOP:
        ui_util_position_center_top(parent, image);
        break;
    case CENTER_BOTTOM:
        ui_util_position_center_bottom(parent, image);
        break;
    case LEFT_CENTER:
        ui_util_position_left_center(parent, image);
        break;
    case RIGHT_CENTER:
        ui_util_position_right_center(parent, image);
        break;
    default:
        Abort("image position not implemented");
    }
    return image;
}
