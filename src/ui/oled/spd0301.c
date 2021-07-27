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

#include "spd0301.h"
#include "oled_writer.h"

#define SPD0301_CMD_SET_LOW_COL(column) (0x00 | ((column)&0x0F))
#define SPD0301_CMD_SET_HIGH_COL(column) (0x10 | (((column) >> 4) & 0x0f))

// Double byte ecommand
#define SPD0301_CMD_SET_MEMORY_ADDRESSING_MODE 0x20
#define SPD0301_CMD_SET_HORIZONTAL_ADDRESSING_MODE 0x00
#define SPD0301_CMD_SET_VERTICAL_ADDRESSING_MODE 0x01
#define SPD0301_CMD_SET_PAGE_ADDRESSING_MODE 0x02

// Triple byte command (second byte start address, third byte end address)
#define SPD0301_CMD_SET_LINE_ADDRESS 0x21

// Triple byte command (second byte start address, third byte end address)
#define SPD0301_CMD_SET_PAGE_ADDRESS 0x22

// Specify column address to determine the initial line (0-63)
#define SPD0301_CMD_SET_DISPLAY_START_LINE(reg) (0x40 | ((reg) & 0x3f))

// Double byte command (0x01-0xff)
#define SPD0301_CMD_SET_CONTRAST_CONTROL 0x81

#define SPD0301_CMD_SET_SEGMENT_RE_MAP_SEG0_0 0xA0
#define SPD0301_CMD_SET_SEGMENT_RE_MAP_SEG0_127 0xA1

#define SPD0301_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON 0xA4
#define SPD0301_CMD_ENTIRE_DISPLAY_ON 0xA5

#define SPD0301_CMD_SET_NORMAL_DISPLAY 0xA6
#define SPD0301_CMD_SET_INVERSE_DISPLAY 0xA7

// Double byte command (16 (0x0f) to 64 (0x3f))
#define SPD0301_CMD_SET_MULTIPLEX_RATIO 0xA8

#define SPD0301_CMD_SET_DISPLAY_ON 0xAF
#define SPD0301_CMD_SET_DISPLAY_OFF 0xAE

#define SPD0301_CMD_SET_PAGE_START_ADDRESS(page) (0xB0 | ((page)&0x07))

#define SPD0301_CMD_SET_COM_OUTPUT_SCAN_UP 0xC0
#define SPD0301_CMD_SET_COM_OUTPUT_SCAN_DOWN 0xC8

// Double byte command (0x00 to 0x3f)
#define SPD0301_CMD_SET_DISPLAY_OFFSET 0xD3

// Double byte command
#define SPD0301_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO 0xD5

// Double byte command
#define SPD0301_CMD_SET_PRE_CHARGE_PERIOD 0xD9

// Double byte command
#define SPD0301_CMD_SET_COM_PINS 0xDA

// Double byte command
#define SPD0301_CMD_SET_VCOMH_DESELECT_LEVEL 0xDB

// Double byte command
#define SPD0301_CMD_SET_GPIO 0xDC

// Double byte command
#define SPD0301_CMD_SET_CHARGE_PUMP_SETTING 0x8D

#define PRE_CHARGE_PERIOD 0x88

static uint8_t* _frame_buffer;

void spd0301_configure(uint8_t* buf)
{
    _frame_buffer = buf;
    oled_writer_write_cmd(SPD0301_CMD_SET_DISPLAY_OFF);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_CONTRAST_CONTROL, 0xff);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_MEMORY_ADDRESSING_MODE, SPD0301_CMD_SET_PAGE_ADDRESSING_MODE);
    oled_writer_write_cmd(SPD0301_CMD_SET_NORMAL_DISPLAY);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_MULTIPLEX_RATIO, 0x3f);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO, 0xf0);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_PRE_CHARGE_PERIOD, 0x88);
    oled_writer_write_cmd_with_param(SPD0301_CMD_SET_VCOMH_DESELECT_LEVEL, 0x35);
    oled_writer_write_cmd(SPD0301_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON);
    spd0301_update();
    oled_writer_write_cmd(SPD0301_CMD_SET_DISPLAY_ON);
}

void spd0301_set_pixel(uint16_t x, uint16_t y, uint8_t c){
    /* pixels can be accessed via buf[y/8*128+x] >> y%8 */
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
void spd0301_update(void){
    /* The SPD0301 has one page per 8 rows. One page is 128 bytes. Every byte is 8 rows */
    for (size_t i = 0; i < 64 / 8; i++) {
        oled_writer_write_cmd(SPD0301_CMD_SET_PAGE_START_ADDRESS(i));
        oled_writer_write_data(&_frame_buffer[i * 128], 128);
    }
}
void spd0301_off(void) {
    oled_writer_write_cmd(SPD0301_CMD_SET_DISPLAY_OFF);
}
