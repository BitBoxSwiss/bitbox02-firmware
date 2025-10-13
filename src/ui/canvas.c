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

#include <driver_init.h>
#include <string.h>
#include <ui/canvas.h>

#define NUM_CANVASES 3

// The buffers must be 4 byte aligned for DMA transfers.
static uint8_t _canvas_0[CANVAS_SIZE] __attribute__((aligned(4))) = {0};
static uint8_t _canvas_1[CANVAS_SIZE] __attribute__((aligned(4))) = {0};
static uint8_t _canvas_2[CANVAS_SIZE] __attribute__((aligned(4))) = {0};

static uint8_t* _canvases[NUM_CANVASES] = {0};
static uint8_t _canvas_active = 0;

void canvas_init(void)
{
    _canvases[0] = _canvas_0;
    _canvases[1] = _canvas_1;
    _canvases[2] = _canvas_2;
}

void canvas_fill(uint8_t color)
{
    uint8_t pixels = color ? 0xff : 0;
    memset(canvas_working(), pixels, CANVAS_SIZE);
}

void canvas_clear(void)
{
    canvas_fill(0);
}

void canvas_commit(void)
{
    _canvas_active = (_canvas_active + 1) % NUM_CANVASES;
    canvas_clear();
}

uint8_t* canvas_working(void)
{
    uint8_t canvas_working = (_canvas_active + 1) % NUM_CANVASES;
    ASSERT(_canvases[canvas_working]);
    return _canvases[canvas_working];
}

uint8_t* canvas_active(void)
{
    ASSERT(_canvases[_canvas_active]);
    return _canvases[_canvas_active];
}
