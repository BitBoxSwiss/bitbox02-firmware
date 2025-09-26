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

// `canvas_commit` is called from an interrupt in `lock_animation.c` and therefore this must be
// volatile. This is probably not enough to make it completely UB free, but it is something
// ¯\_(ツ)_/¯. Eventually `lock_animation.c` will be refactored so that it doesn't call this
// function.
// TODO: When volatile is removed here, also remove the ignored diagnostics later in this file.
static uint8_t* volatile _canvas_active = NULL;
static uint8_t* volatile _canvas_working = NULL;

// One working buffer and one active buffer. The buffer must be 4 byte aligned for DMA transfers.
static uint8_t _canvas_0[CANVAS_SIZE] __attribute__((aligned(4))) = {0};
static uint8_t _canvas_1[CANVAS_SIZE] __attribute__((aligned(4))) = {0};

void canvas_init(void)
{
    _canvas_working = _canvas_0;
    _canvas_active = _canvas_1;
}

void canvas_fill(uint8_t color)
{
    memset(canvas_working(), color, CANVAS_SIZE);
}

void canvas_clear(void)
{
    canvas_fill(0);
}

void canvas_commit(void)
{
#ifndef TESTING
    CRITICAL_SECTION_ENTER()
#endif
    uint8_t* volatile _canvas_tmp = _canvas_working;
    _canvas_working = _canvas_active;
    _canvas_active = _canvas_tmp;
#ifndef TESTING
    CRITICAL_SECTION_LEAVE()
#endif
    canvas_clear();
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdiscarded-qualifiers"

uint8_t* canvas_working(void)
{
    ASSERT(_canvas_working);
    return _canvas_working;
}

uint8_t* canvas_active(void)
{
    ASSERT(_canvas_active);
    return _canvas_active;
}
#pragma GCC diagnostic pop
