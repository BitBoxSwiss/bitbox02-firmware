// SPDX-License-Identifier: Apache-2.0

#ifndef _IMAGE_H_
#define _IMAGE_H_

#include <ui/component.h>
#include <ui/ui_util.h>

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
    uint16_t num_bytes,
    uint16_t width,
    uint16_t height,
    enum screen_position_t position,
    component_t* parent);

#endif
