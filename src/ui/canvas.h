// Copyright 2025 Shift Crypto AG
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
#ifndef CANVAS_H
#define CANVAS_H

#include <screen.h>

// 8 pixels per byte in the canvas.
#define CANVAS_SIZE ((SCREEN_WIDTH * SCREEN_HEIGHT) / 8)

#include <stdint.h>

/*
 * Initialize canvas
 */

void canvas_init(void);

/*
 * Fill the whole working canvas with one color
 */
void canvas_fill(uint8_t color);

/*
 * Clear working canvas (fill with 0)
 */
void canvas_clear(void);

/*
 * Commit the current "working" buffer to become "active" and clear the working buffer.
 *
 * Invalidates pointer returned from `canvas_working()`. `canvas_working()` must be called again to
 * get the current working frame buffer.
 */
void canvas_commit(void);

/*
 * Get a pointer to current working canvas. This is the canvas that can be updated and isn't
 * currently being displayed.
 */
uint8_t* canvas_working(void);

/*
 * Get a pointer ot the current active canvas, being sent to the display. (Should only be used by
 * the screen driver.)
 */
uint8_t* canvas_active(void);
#endif
