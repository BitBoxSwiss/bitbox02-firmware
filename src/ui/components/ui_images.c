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

#include "ui_images.h"

#include <platform/platform_config.h>
#include <ui/ugui/ugui.h>

void image_arrow(int x, int y, int height, arrow_orientation_t orientation)
{
    int width = height * 2 - 1;
    switch(orientation) {
    case ARROW_RIGHT:
        for (int h = 0; h<height; ++h) {
            UG_DrawLine(x+h, y+h, x+h, y+width-1-h, C_WHITE);
        }
        break;
    case ARROW_LEFT:
        for (int h = 0; h<height; ++h) {
            UG_DrawLine(x+h, y+height-1-h, x+h, y+height-1+h, C_WHITE);
        }
        break;
    case ARROW_DOWN:
        for (int h = 0; h<height; ++h) {
            UG_DrawLine(x+h, y+h, x+width-1-h, y+h, C_WHITE);
        }
        break;
    case ARROW_UP:
        for (int h = 0; h<height; ++h) {
            UG_DrawLine(x+height-1-h, y+h, x+height-1+h, y+h, C_WHITE);
        }
        break;
    default:
        break;
    }
}

void image_arrow_hollow(int x, int y, int height, arrow_orientation_t orientation)
{
    int width = height * 2 - 1;
    switch (orientation) {
    case ARROW_RIGHT:
        UG_DrawLine(x, y, x+height-1, y+height-1, C_WHITE);
        UG_DrawLine(x+height-1, y+height-1, x, y+width-1, C_WHITE);
        break;
    case ARROW_LEFT:
        UG_DrawLine(x+height-1, y, x, y+height-1, C_WHITE);
        UG_DrawLine(x, y+height-1, x+height-1, y+width-1, C_WHITE);
        break;
    case ARROW_DOWN:
        UG_DrawLine(x, y, x+height-1, y+height-1, C_WHITE);
        UG_DrawLine(x+height-1, y+height-1, x+width-1, y, C_WHITE);
        break;
    case ARROW_UP:
        UG_DrawLine(x, y+height-1, x+height-1, y, C_WHITE);
        UG_DrawLine(x+height-1, y, x+width-1, y+height-1, C_WHITE);
        break;
    default:
        break;
    }
}

void image_checkmark(int x, int y, int h)
{
    UG_DrawLine(x, y + h - (h / 2 - 1) - 1, x + h / 2 - 1, y + h - 1, C_WHITE);
    UG_DrawLine(x + h / 2 - 1, y + h - 1, x + h - 1 + h / 2 - 1, y, C_WHITE);
}

void image_cross(int x, int y, int h)
{
    UG_DrawLine(x, y, x + h, y + h, C_WHITE);
    UG_DrawLine(x + h, y, x, y + h, C_WHITE);
}

void image_lock(int x, int y, int r)
{
    UG_DrawArc(x, y - 3, r, 0x0F, C_WHITE);
    UG_DrawArc(x, y - 3, r - 1, 0x0F, C_WHITE);
    UG_FillFrame(x - r, y, x - r + 1, y - 3, C_WHITE);
    UG_FillFrame(x + r, y, x + r - 1, y - 3, C_WHITE);
    UG_FillFrame(x - r, y, x + r, y + r + r / 2, C_WHITE);
}

void image_unlock(int x, int y, int r)
{
    UG_DrawArc(x - r * 2 + 1, y - 3, r, 0x0F, C_WHITE);
    UG_DrawArc(x - r * 2 + 1, y - 3, r - 1, 0x0F, C_WHITE);
    UG_FillFrame(x - r, y, x - r + 1, y - 3, C_WHITE);
    UG_FillFrame(x - r * 3 + 1, y, x - r * 3 + 2, y - 3, C_WHITE);
    UG_FillFrame(x - r, y, x + r, y + r + r / 2, C_WHITE);
}

void image_sdcard(bool mirror)
{
    uint8_t c = 6; // Corner
    uint8_t h = 20; // Height
    uint8_t w = h * 3 / 2; // Width
    int16_t y = 32; // Verticle center
    int16_t x = mirror ? 0 : 127; // Position of base edge
    int16_t m = mirror ? -1 : 1;
    // Base edge
    UG_FillFrame(x - m * (4), y - h / 2, x - m * (0), y + h / 2, C_WHITE);
    // Front edge
    UG_FillFrame(x - m * (w), y - h / 2 + c, x - m * (w - 1), y + h / 2, C_WHITE);
    // Top edge
    UG_FillFrame(x - m * (0), y - h / 2, x - m * (w - c - 1), y - h / 2 - 1, C_WHITE);
    // Bottom edge
    UG_FillFrame(x - m * (0), y + h / 2, x - m * (w), y + h / 2 - 1, C_WHITE);
    // Corner
    UG_DrawLine(x - m * (w - c), y - h / 2, x - m * (w), y - h / 2 + c, C_WHITE);
    UG_DrawLine(x - m * (w - c - 1), y - h / 2, x - m * (w - 1), y - h / 2 + c, C_WHITE);
}
