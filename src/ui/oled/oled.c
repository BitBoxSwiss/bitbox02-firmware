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

#include "oled.h"

#include <driver_init.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <ui/ugui/ugui.h>

#define OLED_CMD_SET_LOW_COL(column) (0x00 | (column))
#define OLED_CMD_SET_HIGH_COL(column) (0x10 | (column))
#define OLED_CMD_SET_MEMORY_ADDRESSING_MODE 0x20
#define OLED_CMD_SET_COLUMN_ADDRESS 0x21
#define OLED_CMD_SET_PAGE_ADDRESS 0x22
#define OLED_CMD_SET_START_LINE(line) (0x40 | (line))
#define OLED_CMD_SET_CONTRAST_CONTROL_FOR_BANK0 0x81
#define OLED_CMD_SET_CHARGE_PUMP_SETTING 0x8D
#define OLED_CMD_SET_SEGMENT_RE_MAP_COL0_SEG0 0xA0
#define OLED_CMD_SET_SEGMENT_RE_MAP_COL127_SEG0 0xA1
#define OLED_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON 0xA4
#define OLED_CMD_ENTIRE_DISPLAY_ON 0xA5
#define OLED_CMD_SET_NORMAL_DISPLAY 0xA6
#define OLED_CMD_SET_INVERSE_DISPLAY 0xA7
#define OLED_CMD_SET_MULTIPLEX_RATIO 0xA8
#define OLED_CMD_SET_DISPLAY_ON 0xAF
#define OLED_CMD_SET_DISPLAY_OFF 0xAE
#define OLED_CMD_SET_PAGE_START_ADDRESS(page) \
    (0xB0 | ((page)&0x0f)) // changed to 0x0f for SH1107 (128x64) OLED
#define OLED_CMD_SET_COM_OUTPUT_SCAN_UP 0xC0
#define OLED_CMD_SET_COM_OUTPUT_SCAN_DOWN 0xC8
#define OLED_CMD_SET_DISPLAY_OFFSET 0xD3
#define OLED_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO 0xD5
#define OLED_CMD_SET_PRE_CHARGE_PERIOD 0xD9
#define OLED_CMD_SET_COM_PINS 0xDA
#define OLED_CMD_SET_VCOMH_DESELECT_LEVEL 0xDB
#define OLED_CMD_NOP 0xE3

#define OLED_CMD_SCROLL_H_RIGHT 0x26
#define OLED_CMD_SCROLL_H_LEFT 0x27
#define OLED_CMD_CONTINUOUS_SCROLL_V_AND_H_RIGHT 0x29
#define OLED_CMD_CONTINUOUS_SCROLL_V_AND_H_LEFT 0x2A
#define OLED_CMD_DEACTIVATE_SCROLL 0x2E
#define OLED_CMD_ACTIVATE_SCROLL 0x2F
#define OLED_CMD_SET_VERTICAL_SCROLL_AREA 0xA3

static bool _frame_buffer_updated = false;
static uint8_t _frame_buffer[128 * 8];

/**
 * Pulls the pin D/C# low before writing to the controller.
 * [in] command to write
 */
static inline void _write_command(uint8_t command)
{
    uint8_t spi_output[32];
    spi_output[0] = command;
    gpio_set_pin_level(PIN_OLED_CMD, 0);
    gpio_set_pin_level(PIN_OLED_CS, 0);
    SPI_0_write_block((void*)spi_output, 1);
    gpio_set_pin_level(PIN_OLED_CS, 1);
}

// The actual size of the GDDR is something like 128*128. But our display only uses the middle 64
// columns. The start column is 32 and end column is 95.

void oled_init(void)
{
    // DC-DC OFF
    gpio_set_pin_level(PIN_OLED_ON, 0);
    delay_us(5);

    // Hard reset OLED display controller
    gpio_set_pin_level(PIN_OLED_RES, 0);
    delay_us(5);
    gpio_set_pin_level(PIN_OLED_RES, 1);
    delay_us(5);

    // Initialize
    _write_command(OLED_CMD_SET_DISPLAY_OFF);
    // Set brightness
    _write_command(OLED_CMD_SET_CONTRAST_CONTROL_FOR_BANK0);
    _write_command(0x80); /* 0x00..0xff */
    _write_command(0x21); // Set vertical addressing mode
    // Set scan directions for our non-mirrored orientation
    _write_command(OLED_CMD_SET_SEGMENT_RE_MAP_COL127_SEG0);
    _write_command(OLED_CMD_SET_COM_OUTPUT_SCAN_UP);
    // Set normal display (not inverted)
    _write_command(OLED_CMD_SET_NORMAL_DISPLAY);
    // We only activate the 64 lines we use (0x3f == 64 Multiplex ratio)
    _write_command(OLED_CMD_SET_MULTIPLEX_RATIO);
    _write_command(0x3f); /* duty = 1/64; 0x00..0x7f */
    // Shift the columns by 96 when display is in non-mirrored orientation
    _write_command(OLED_CMD_SET_DISPLAY_OFFSET);
    _write_command(0x60);
    // Set clock frequency and divisor
    // Upper 4 bits are freqency, lower 4 bits are divisor
    _write_command(OLED_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO);
    _write_command(0xf0);
    // Set precharge and discharge
    // Upper 4 bits are dis-charge, lower 4 bits are pre-charge
    _write_command(OLED_CMD_SET_PRE_CHARGE_PERIOD);
    _write_command(0x22); /* 0x00..0xff */
    _write_command(OLED_CMD_SET_VCOMH_DESELECT_LEVEL);
    _write_command(0x35); /* 0x00..0xff */
    _write_command(0xad); /* DC-DC control mode set*/
    _write_command(0x8a); /* built-in DC-DC enable (8a:disable; 8b:enable) */
    _write_command(OLED_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON);
    oled_clear_buffer();
    oled_send_buffer();
    _write_command(OLED_CMD_SET_DISPLAY_ON);
    delay_ms(100);

    // DC-DC ON
    gpio_set_pin_level(PIN_OLED_ON, 1);
}

/*
 * The SH1107 Segment/Common driver specifies that there are 16 pages per column
 * In total we should be writing 64*128 pixels. 8 bits per page, 16 pages per column and 64 columns
 */

void oled_send_buffer(void)
{
    // 3.5msec
    for (size_t i = 0; i < 64; i++) {
        _write_command(0x00 + (i & 0xf)); /*set lower column address*/
        _write_command(0x10 + ((i >> 4) & 0x7)); /*set higher column address*/
        gpio_set_pin_level(PIN_OLED_CMD, 1);
        gpio_set_pin_level(PIN_OLED_CS, 0);
        SPI_0_write_block((unsigned char*)&_frame_buffer[i * 16], 16);
        gpio_set_pin_level(PIN_OLED_CS, 1);
    }
}

void oled_clear_buffer(void)
{
    memset(_frame_buffer, 0, sizeof(_frame_buffer));
}

void oled_mirror(bool mirror)
{
    if (mirror) {
        _write_command(OLED_CMD_SET_SEGMENT_RE_MAP_COL0_SEG0);
        _write_command(OLED_CMD_SET_COM_OUTPUT_SCAN_DOWN);
        // Shift the columns by 32 when display is in mirrored orientation
        _write_command(OLED_CMD_SET_DISPLAY_OFFSET);
        _write_command(0x20);
    } else {
        _write_command(OLED_CMD_SET_SEGMENT_RE_MAP_COL127_SEG0);
        _write_command(OLED_CMD_SET_COM_OUTPUT_SCAN_UP);
    }
}

/*
 * pixels can be accessed via buf[y*16+x/8] >> x%8
 */
void oled_set_pixel(uint16_t x, uint16_t y, uint8_t c)
{
    uint32_t p;
    if (x > 127) return;
    if (y > 63) return;
    p = y * 16;
    p += x / 8;
    if (c) {
        _frame_buffer[p] |= 1 << (x % 8);
    } else {
        _frame_buffer[p] &= ~(1 << (x % 8));
    }
    _frame_buffer_updated = true;
}
