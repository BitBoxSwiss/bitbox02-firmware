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

#ifndef __BITBOXBASE_LEDS_H
#define __BITBOXBASE_LEDS_H

#include <stdbool.h>

/** Color setting for an RGB LED. */
typedef enum {
    LED_COLOR_WHITE,
    LED_COLOR_RED,
    LED_COLOR_GREEN,
    LED_COLOR_BLUE,
    LED_COLOR_YELLOW,
    LED_COLOR_PURPLE,
    LED_COLOR_CYAN,
    LED_COLOR_NONE
} led_color_t;

/**
 * Initializes the LED array.
 * Turns every pin to output, turns off every LED.
 */
void leds_init(void);

/**
 * Turn on/off the given small LED on/off.
 *
 * @param[in] led Index of the LED to turn on/off (0 < led < 5).
 *                 0 is the leftmost LED, 4 is the rightmost one.
 * @param[in] enabled Whether to turn the LED on (true) or off (false).
 */
void leds_turn_small_led(int led, bool enabled);

/**
 * Turn on/off the given RGB LED.
 *
 * @param[in] led Index of the LED to turn on/off (0 < led < 2).
 *                 0 is the left LED, 1 is the right one.
 * @param[in] color Which color to turn the LED (White, red, green, blue, or off).
 */
void leds_turn_big_led(int led, led_color_t color);

#endif // __BITBOXBASE_LEDS_H
