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
#include "bitboxbase_pins.h"

#include <stdint.h>

/** List of small LEDs (leftmost to rightmost) */
static uint8_t small_leds[] = {PIN_LED_SMALL(0),
                               PIN_LED_SMALL(1),
                               PIN_LED_SMALL(2),
                               PIN_LED_SMALL(3),
                               PIN_LED_SMALL(4)};

void leds_turn_small_led(int led, bool enabled)
{
    gpio_set_pin_level(small_leds[led], enabled);
}

/**
 * Turn on/off the given RBG LED pin. Note that these pins
 * are active low!
 */
static void _leds_turn_big_led_component(uint8_t pin, bool level)
{
    gpio_set_pin_level(pin, !level);
}

/** Position of each component in each RGB LED. */
static const uint8_t big_leds[2][3] = {{PIN_BLED0_R, PIN_BLED0_G, PIN_BLED0_B},
                                       {PIN_BLED1_R, PIN_BLED1_G, PIN_BLED1_B}};

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
    for (size_t i = 0; i < (sizeof(small_leds) / sizeof(*small_leds)); ++i) {
        gpio_set_pin_direction(small_leds[i], GPIO_DIRECTION_OUT);
        leds_turn_small_led(i, false);
    }
    for (size_t i = 0; i < (sizeof(big_leds) / sizeof(*big_leds)); ++i) {
        for (size_t j = 0; j < 3; ++j) {
            gpio_set_pin_direction(big_leds[i][j], GPIO_DIRECTION_OUT);
            gpio_set_pin_level(big_leds[i][j], false);
        }
    }
}
