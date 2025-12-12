// SPDX-License-Identifier: Apache-2.0

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

void screen_print_debug(const char* message, int duration)
{
    (void)duration;
    printf("%s\n", message);
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

void screen_clear(void) {}
