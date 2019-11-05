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

static void _turn_small_led(int led, bool level)
{
    gpio_set_pin_level(led, level);
}

static void _turn_big_led(int led, bool level)
{
    gpio_set_pin_level(led, !level);
}

typedef enum { LED_WHITE, LED_RED, LED_GREEN, LED_BLUE } led_color_t;

static const uint8_t big_leds[] =
    {PIN_BLED0_R, PIN_BLED0_G, PIN_BLED0_B, PIN_BLED1_R, PIN_BLED1_G, PIN_BLED1_B};

static void _set_pin_directions(void)
{
    for (int i = 0; i < 5; ++i) {
        gpio_set_pin_direction(PIN_LED_SMALL(i), GPIO_DIRECTION_OUT);
        _turn_small_led(PIN_LED_SMALL(i), false);
    }
    for (size_t i = 0U; i < (sizeof(big_leds) / sizeof(*big_leds)); ++i) {
        gpio_set_pin_direction(big_leds[i], GPIO_DIRECTION_OUT);
        _turn_big_led(big_leds[i], false);
    }
    gpio_set_pin_direction(GPIO(GPIO_PORTB, 10), GPIO_DIRECTION_OUT);
    gpio_set_pin_level(GPIO(GPIO_PORTB, 10), true);
}

void leds_init(void)
{
    _set_pin_directions();
}
