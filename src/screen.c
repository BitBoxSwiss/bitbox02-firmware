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

#include "screen.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <ui/components/ui_images.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/oled/oled.h>
#include <ui/ugui/ugui.h>
#include <util.h>
#include <utils_assert.h>

#ifndef TESTING
    #include <hal_delay.h>
#endif

static UG_GUI guioled; // Global GUI structure for OLED screen
static bool screen_upside_down = false;
static void (*_mirror_fn)(bool);
static void (*_clear_fn)(void);

UG_COLOR screen_front_color = C_WHITE;
UG_COLOR screen_back_color = C_BLACK;

slider_location_t top_slider = 1;
slider_location_t bottom_slider = 0;

// message truncated to 99 chars. somewhere between 99 and 120 we start to get hardfaults...
void screen_print_debug(const char* message, int duration)
{
    char print[100];
    snprintf(print, sizeof(print), "%s", message);
    screen_clear();
    UG_FontSelect(&font_font_a_9X9);
    UG_PutString(0, 0, print, false);
    UG_SendBuffer();
#ifndef TESTING
    if (duration > 0) delay_ms(duration);
#endif
}

void screen_sprintf_debug(int duration, const char* fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    char print[100];
    // There is a bug in clang-tidy
    // See https://bugs.llvm.org/show_bug.cgi?id=41311
    vsnprintf(print, sizeof(print), fmt, args); // NOLINT
    va_end(args);
    screen_print_debug(print, duration);
}

void screen_print_debug_hex(const uint8_t* bytes, size_t len, int duration)
{
    if (len > 50) {
        len = 50;
    }
    char hex[2 * 50 + 1] = {0};
    util_uint8_to_hex(bytes, len, hex);
    screen_print_debug(hex, duration);
}

// Careful, this function is used in both the bootloader and the firmware.
void screen_splash(void)
{
    screen_clear();

    int height = IMAGE_DEFAULT_ARROW_HEIGHT;
    int x = 0;
    int y = SCREEN_HEIGHT / 2 - height;
    image_arrow(x - height + 2, y, height, ARROW_RIGHT);
    image_arrow(SCREEN_WIDTH - x - 2, y, height, ARROW_LEFT);

    UG_SendBuffer();
    screen_clear();
}

void screen_rotate(void)
{
    screen_upside_down = !screen_upside_down;
    top_slider = 1 - top_slider;
    bottom_slider = 1 - bottom_slider;
    ASSERT(_mirror_fn);
    _mirror_fn(screen_upside_down);
}

bool screen_is_upside_down(void)
{
    return screen_upside_down;
}

void screen_init(
    void (*pixel_fn)(UG_S16, UG_S16, UG_COLOR),
    void (*mirror_fn)(bool),
    void (*clear_fn)(void))
{
    _mirror_fn = mirror_fn;
    _clear_fn = clear_fn;
    UG_Init(&guioled, pixel_fn, &font_font_a_11X10, SCREEN_WIDTH, SCREEN_HEIGHT);
}

void screen_clear(void)
{
    ASSERT(_clear_fn);
    _clear_fn();
}
