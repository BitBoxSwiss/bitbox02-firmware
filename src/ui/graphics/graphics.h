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
