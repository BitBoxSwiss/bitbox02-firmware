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
