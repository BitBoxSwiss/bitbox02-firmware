/**
 * \file
 *
 * \brief SSD1306 OLED display controller driver.
 *
 * Copyright (c) 2013-2015 Atmel Corporation. All rights reserved.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. The name of Atmel may not be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * 4. This software may only be redistributed and used in connection with an
 *    Atmel microcontroller product.
 *
 * THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * EXPRESSLY AND SPECIFICALLY DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 * ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * \asf_license_stop
 *
 */
/*
 * Support and FAQ: visit <a href="http://www.atmel.com/design-support/">Atmel Support</a>
 */
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

// We use two screens:
// 1. BitBox02 controller SH1107:
//    The actual size of the GDDR is something like 128x128. But our display only uses the middle 64
//    columns. The start column is 32 and end column is 95.
// 2. BitBox02 (second screen) controller SSD1312

#include "oled.h"

#include <driver_init.h>
#include <hardfault.h>
#include <memory/memory_shared.h>
#include <screen.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <ui/canvas.h>
#include <ui/oled/oled_writer.h>
#include <ui/oled/sh1107.h>
#include <ui/oled/ssd1312.h>
#include <ui/ugui/ugui.h>

static volatile bool _enabled = false;

struct bb02_display {
    void (*configure)(void);
    void (*set_pixel)(int16_t x, int16_t y, uint8_t c);
    void (*present)(void);
    void (*off)(void);
    void (*mirror)(bool);
};

static struct bb02_display bb02_display = {
    .configure = sh1107_configure,
    .set_pixel = sh1107_set_pixel,
    .present = sh1107_present,
    .off = sh1107_off,
    .mirror = sh1107_mirror,
};

void oled_init(void)
{
    canvas_init();

    if (memory_get_screen_type() == MEMORY_SCREEN_TYPE_SSD1312) {
        bb02_display.configure = ssd1312_configure;
        bb02_display.set_pixel = ssd1312_set_pixel;
        bb02_display.present = ssd1312_present;
        bb02_display.off = ssd1312_off;
        bb02_display.mirror = ssd1312_mirror;
    }
    if (_enabled) {
        return;
    }
    // DC-DC OFF
    gpio_set_pin_level(PIN_OLED_ON, 0);
    delay_us(5);

    // Hard reset OLED display controller
    gpio_set_pin_level(PIN_OLED_RES, 0);
    delay_us(5);
    gpio_set_pin_level(PIN_OLED_RES, 1);
    delay_us(5);

    oled_present();

    bb02_display.configure();

    delay_ms(100);

    // DC-DC ON
    gpio_set_pin_level(PIN_OLED_ON, 1);
    _enabled = true;
}

void oled_present(void)
{
    bb02_display.present();
}

void oled_mirror(bool mirror)
{
    bb02_display.mirror(mirror);
}

void oled_set_pixel(int16_t x, int16_t y, uint8_t c)
{
    bb02_display.set_pixel(x, y, c);
}

void oled_off(void)
{
    if (!_enabled) {
        return;
    }
    bb02_display.off();
    // OFF VCC
    gpio_set_pin_level(PIN_OLED_ON, 0);
    _enabled = false;
}

void oled_set_brightness(uint8_t value)
{
    // brightness uses the same command on all displays 0x81.
    oled_writer_write_cmd_with_param(0x81, value);
}
