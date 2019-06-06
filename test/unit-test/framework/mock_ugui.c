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

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

#include <ui/ugui/ugui.h>
#include <util.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"

void UG_PutString(UG_S16 x, UG_S16 y, const char* str, bool inverted) {}

void UG_PutStringNoBreak(UG_S16 x, UG_S16 y, const char* str, bool inverted) {}

void UG_PutStringCentered(
    UG_S16 x,
    UG_S16 y,
    UG_S16 width,
    UG_S16 height,
    const char* str,
    bool inverted)
{
}

void UG_FontSelect(const UG_FONT* font) {}

void UG_FontSetHSpace(const UG_U16 s) {}

void UG_FontSetVSpace(const UG_U16 s) {}

void UG_DrawPixel(UG_S16 x0, UG_S16 y0, UG_COLOR c) {}

void UG_DrawLine(UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c) {}

void UG_DrawFrame(UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c) {}

void UG_FillFrame(UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c) {}

void UG_MeasureString(UG_S16* xout, UG_S16* yout, const char* str) {}

void UG_MeasureStringNoBreak(UG_S16* xout, UG_S16* yout, const char* str) {}

void UG_MeasureStringCentered(UG_S16* xout, UG_S16* yout, const char* str)
{
    *xout = 0;
    *yout = 0;
    UG_S16 calc_width_line = 0;
    UG_S16 calc_height_line = 0;
    const char* c;
    const uint8_t max_line = 128;
    char line[max_line];
    const char* start = str;
    for (c = str; *c != '\0'; c++) {
        memset(line, 0, max_line);
        if (*c == '\n') {
            memcpy(line, start, (c - start) + 1);
            // mock:
            calc_height_line = 1;
            calc_width_line = (c - start) + 1;
            // end mock
            *yout += calc_height_line;
            *xout = MAX(*xout, calc_width_line);
            start = c + 1;
        }
    }
}

void UG_DrawArc(UG_S16 x0, UG_S16 y0, UG_S16 r, UG_U8 s, UG_COLOR c) {}

void UG_DrawCircle(UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c) {}

void UG_SendBuffer(void) {}

void UG_ClearBuffer(void) {}

void UG_PutChar(char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc, bool inverted) {}

void UG_FillCircle(UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c) {}

#pragma GCC diagnostic pop
