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

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>

#include <screen.h>

UG_COLOR screen_front_color = C_WHITE;
UG_COLOR screen_back_color = C_BLACK;

slider_location_t top_slider = 1;
slider_location_t bottom_slider = 0;

void screen_init(void) {}

void screen_print_debug(const char* message, int duration)
{
    printf("%s\n", message);
}

void screen_sprintf_debug(int duration, const char* message, ...)
{
    va_list args;
    va_start(args, message);
    char print[100] = {0};
    vsnprintf(print, sizeof(print) - 1, message, args);
    va_end(args);
    printf("%s\n", print);
}

void screen_splash(void)
{
    puts("screen splash\n");
}

void screen_rotate(void) {}

bool screen_is_upside_down(void)
{
    return false;
}
