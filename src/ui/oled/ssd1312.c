// Copyright 2021 Shift Crypto AG
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

#include "ssd1312.h"
#include "oled_writer.h"
#include <stdbool.h>

#define SSD1312_CMD_SET_LOW_COL(column) (0x00 | ((column) & 0x0F))
#define SSD1312_CMD_SET_HIGH_COL(column) (0x10 | (((column) >> 4) & 0x07))

// Double byte ecommand
#define SSD1312_CMD_SET_MEMORY_ADDRESSING_MODE 0x20
#define SSD1312_CMD_SET_COM_PAGE_H_MODE 0x01
#define SSD1312_CMD_SET_PAGE_ADDRESSING_MODE 0x02
#define SSD1312_CMD_SET_SEG_PAGE_H_MODE 0x09

// Triple byte command (second byte start address, third byte end address)
#define SSD1312_CMD_SET_LINE_ADDRESS 0x21

// Triple byte command (second byte start address, third byte end address)
#define SSD1312_CMD_SET_PAGE_ADDRESS 0x22

// Specify column address to determine the initial line (0-63)
#define SSD1312_CMD_SET_DISPLAY_START_LINE(reg) (0x40 | ((reg) & 0x3f))

// Double byte command (0x01-0xff)
#define SSD1312_CMD_SET_CONTRAST_CONTROL 0x81

#define SSD1312_CMD_SET_SEGMENT_RE_MAP_SEG0_0 0xA0
#define SSD1312_CMD_SET_SEGMENT_RE_MAP_SEG0_128 0xA1

#define SSD1312_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON 0xA4
#define SSD1312_CMD_ENTIRE_DISPLAY_ON 0xA5

#define SSD1312_CMD_SET_NORMAL_DISPLAY 0xA6
#define SSD1312_CMD_SET_INVERSE_DISPLAY 0xA7

// Double byte command (16 (0x0f) to 64 (0x3f))
#define SSD1312_CMD_SET_MULTIPLEX_RATIO 0xA8

// Double byte command (0x40 external, 0x50 internal)
#define SSD1312_CMD_SET_IREF 0xAD

#define SSD1312_CMD_SET_DISPLAY_ON 0xAF
#define SSD1312_CMD_SET_DISPLAY_OFF 0xAE

#define SSD1312_CMD_SET_PAGE_START_ADDRESS(page) (0xB0 | ((page) & 0x07))

#define SSD1312_CMD_SET_COM_OUTPUT_SCAN_UP 0xC0
#define SSD1312_CMD_SET_COM_OUTPUT_SCAN_DOWN 0xC8

// Double byte command (0x00 to 0x3f)
#define SSD1312_CMD_SET_DISPLAY_OFFSET 0xD3

// Double byte command
#define SSD1312_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO 0xD5

// Double byte command
#define SSD1312_CMD_SET_PRE_CHARGE_PERIOD 0xD9

// Double byte command
#define SSD1312_CMD_SET_SEG_PINS 0xDA

// Double byte command
#define SSD1312_CMD_SET_VCOMH_SELECT_LEVEL 0xDB

// Double byte command
#define SSD1312_CMD_SET_CHARGE_PUMP_SETTING 0x8D

static uint8_t* _frame_buffer;

void ssd1312_configure(uint8_t* buf)
{
    _frame_buffer = buf;
    oled_writer_write_cmd(SSD1312_CMD_SET_DISPLAY_OFF);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_CONTRAST_CONTROL, 0xff);
    oled_writer_write_cmd_with_param(
        SSD1312_CMD_SET_MEMORY_ADDRESSING_MODE, SSD1312_CMD_SET_PAGE_ADDRESSING_MODE);
    oled_writer_write_cmd(SSD1312_CMD_SET_SEGMENT_RE_MAP_SEG0_0);
    oled_writer_write_cmd(SSD1312_CMD_SET_COM_OUTPUT_SCAN_DOWN);
    oled_writer_write_cmd(SSD1312_CMD_SET_NORMAL_DISPLAY);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_MULTIPLEX_RATIO, 0x3f);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO, 0xf0);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_PRE_CHARGE_PERIOD, 0x22);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_VCOMH_SELECT_LEVEL, 0x35);
    oled_writer_write_cmd_with_param(SSD1312_CMD_SET_IREF, 0x40);
    oled_writer_write_cmd(SSD1312_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON);
    ssd1312_update();
    oled_writer_write_cmd(SSD1312_CMD_SET_DISPLAY_ON);
}

void ssd1312_set_pixel(uint16_t x, uint16_t y, uint8_t c)
{
    uint32_t p;
    if (x > 127) return;
    if (y > 63) return;
    p = (y / 8) * 128;
    p += x;
    if (c) {
        _frame_buffer[p] |= 1 << (y % 8);
    } else {
        _frame_buffer[p] &= ~(1 << (y % 8));
    }
}
void ssd1312_update(void)
{
    /* The SSD1312 has one page per 8 rows. One page is 128 bytes. Every byte is 8 rows */
    for (size_t i = 0; i < 64 / 8; i++) {
        oled_writer_write_cmd(SSD1312_CMD_SET_PAGE_START_ADDRESS(i));
        oled_writer_write_data(&_frame_buffer[i * 128], 128);
    }
}

void ssd1312_mirror(bool mirror)
{
    if (mirror) {
        oled_writer_write_cmd(SSD1312_CMD_SET_SEGMENT_RE_MAP_SEG0_128);
        oled_writer_write_cmd(SSD1312_CMD_SET_COM_OUTPUT_SCAN_UP);
    } else {
        oled_writer_write_cmd(SSD1312_CMD_SET_SEGMENT_RE_MAP_SEG0_0);
        oled_writer_write_cmd(SSD1312_CMD_SET_COM_OUTPUT_SCAN_DOWN);
    }
}

void ssd1312_off(void)
{
    oled_writer_write_cmd(SSD1312_CMD_SET_DISPLAY_OFF);
}
