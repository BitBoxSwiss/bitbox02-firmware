// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_GRAPHICS_H
#define _UI_GRAPHICS_H

#include <stdint.h>

#include <util.h>

typedef struct {
    // max width is SCREEEN_WIDTH (128)
    int16_t width;
    // max height is SCREEEN_HEIGHT (64)
    int16_t height;
} dimension_t;

typedef struct {
    int16_t left;
    int16_t top;
} position_t;

void graphics_draw_image(
    const position_t* position,
    const dimension_t* dimension,
    const in_buffer_t* image);

#endif // _UI_GRAPHICS_H
