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

#include "leds.h"

#include <stdint.h>
#include <stdio.h>

/** List of small LEDs (leftmost to rightmost) */
static uint8_t small_leds[] = {0, 1, 2, 3, 4};

void leds_turn_small_led(int led, bool enabled)
{
    printf("turn small led %d", small_leds[led]);
}

/**
 * Turn on/off the given RBG LED pin. Note that these pins
 * are active low!
 */
static void _leds_turn_big_led_component(uint8_t pin, bool level)
{
    printf("turn big led %d", pin);
}

/** Position of each component in each RGB LED. */
static const uint8_t big_leds[2][3] = {{5, 6, 7}, {8, 9, 10}};

void leds_turn_big_led(int led, led_color_t color)
{
    switch (color) {
    case LED_COLOR_WHITE:
        _leds_turn_big_led_component(big_leds[led][0], true);
        _leds_turn_big_led_component(big_leds[led][1], true);
        _leds_turn_big_led_component(big_leds[led][2], true);
        break;
    case LED_COLOR_RED:
        _leds_turn_big_led_component(big_leds[led][0], true);
        _leds_turn_big_led_component(big_leds[led][1], false);
        _leds_turn_big_led_component(big_leds[led][2], false);
        break;
    case LED_COLOR_GREEN:
        _leds_turn_big_led_component(big_leds[led][0], false);
        _leds_turn_big_led_component(big_leds[led][1], true);
        _leds_turn_big_led_component(big_leds[led][2], false);
        break;
    case LED_COLOR_BLUE:
        _leds_turn_big_led_component(big_leds[led][0], false);
        _leds_turn_big_led_component(big_leds[led][1], false);
        _leds_turn_big_led_component(big_leds[led][2], true);
        break;
    case LED_COLOR_YELLOW:
        _leds_turn_big_led_component(big_leds[led][0], true);
        _leds_turn_big_led_component(big_leds[led][1], true);
        _leds_turn_big_led_component(big_leds[led][2], false);
        break;
    case LED_COLOR_PURPLE:
        _leds_turn_big_led_component(big_leds[led][0], true);
        _leds_turn_big_led_component(big_leds[led][1], false);
        _leds_turn_big_led_component(big_leds[led][2], true);
        break;
    case LED_COLOR_CYAN:
        _leds_turn_big_led_component(big_leds[led][0], false);
        _leds_turn_big_led_component(big_leds[led][1], true);
        _leds_turn_big_led_component(big_leds[led][2], true);
        break;
    case LED_COLOR_NONE:
    default:
        _leds_turn_big_led_component(big_leds[led][0], false);
        _leds_turn_big_led_component(big_leds[led][1], false);
        _leds_turn_big_led_component(big_leds[led][2], false);
    }
}

void leds_init(void)
{
    printf("leds init");
}
