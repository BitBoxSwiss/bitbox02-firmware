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

#include "sh1107.h"
#include "oled_writer.h"

// Specify the column address of display RAM 0-127
#define SH1107_CMD_SET_LOW_COL(column) (0x00 | ((column) & 0x0F))
#define SH1107_CMD_SET_HIGH_COL(column) (0x10 | (((column) >> 4) & 0x07))

// In page adressing mode, after the display RAM is written to, the column address is
// increased automatically by 1. When it reaches the last colument it is reset without incrementing
// the page address.
#define SH1107_CMD_SET_PAGE_ADDRESSING_MODE 0x20

// In vertical addressing mode, after the display RAM is written to, the page address is increased
// automatically by 1. If the page address reaches the last value it is reset and the column
// address is not incremented.
#define SH1107_CMD_SET_VERTICAL_ADDRESSING_MODE 0x21

// Double byte command steps from 0 to 255.
#define SH1107_CMD_SET_CONTRAST_CONTROL 0x81

#define SH1107_CMD_SET_SEGMENT_RE_MAP_NORMAL 0xA0
#define SH1107_CMD_SET_SEGMENT_RE_MAP_REVERSE 0xA1

// Double byte command steps 1 (0x00) to 128 (0x7F)
#define SH1107_CMD_SET_MULTIPLEX_RATIO 0xA8

#define SH1107_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON 0xA4
#define SH1107_CMD_ENTIRE_DISPLAY_ON 0xA5

#define SH1107_CMD_SET_NORMAL_DISPLAY 0xA6
#define SH1107_CMD_SET_INVERSE_DISPLAY 0xA7

// Double byte command steps 0x00 to 0x7F
#define SH1107_CMD_SET_DISPLAY_OFFSET 0xD3

// Double byte command (0x80 to 0x8F)
// Bit 0: 0 disable, 1 enable
// Bit 1-3: 0-7 switch frequency
#define SH1107_CMD_SET_DC_DC_MODE_ON 0xAD

#define SH1107_CMD_SET_DISPLAY_ON 0xAF
#define SH1107_CMD_SET_DISPLAY_OFF 0xAE

#define SH1107_CMD_SET_PAGE_START_ADDRESS(page) (0xB0 | ((page) & 0x0f))

#define SH1107_CMD_SET_COM_OUTPUT_SCAN_UP 0xC0
#define SH1107_CMD_SET_COM_OUTPUT_SCAN_DOWN 0xC8

// Double byte command
#define SH1107_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO 0xD5

// Double byte command
#define SH1107_CMD_SET_PRE_CHARGE_PERIOD 0xD9

// Double byte command
#define SH1107_CMD_SET_VCOMH_DESELECT_LEVEL 0xDB

// Specify column address to determine the initial display line or COM0
// Double byte command (0x00 to 0x7F)
#define SH1107_CMD_SET_DISPLAY_START_LINE 0xDC

static uint8_t* _frame_buffer;

void sh1107_configure(uint8_t* buf)
{
    _frame_buffer = buf;
    oled_writer_write_cmd(SH1107_CMD_SET_DISPLAY_OFF);
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_CONTRAST_CONTROL, 0xff);
    oled_writer_write_cmd(SH1107_CMD_SET_VERTICAL_ADDRESSING_MODE);
    oled_writer_write_cmd(SH1107_CMD_SET_SEGMENT_RE_MAP_REVERSE);
    oled_writer_write_cmd(SH1107_CMD_SET_COM_OUTPUT_SCAN_UP);
    oled_writer_write_cmd(SH1107_CMD_SET_NORMAL_DISPLAY);
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_MULTIPLEX_RATIO, 0x3f);
    // Shift the columns by 96 when display is in non-mirrored orientation
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_DISPLAY_OFFSET, 0x60);
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_DISPLAY_CLOCK_DIVIDE_RATIO, 0xf0);
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_PRE_CHARGE_PERIOD, 0x22);
    oled_writer_write_cmd_with_param(SH1107_CMD_SET_VCOMH_DESELECT_LEVEL, 0x35);
    oled_writer_write_cmd_with_param(0xad, 0x8a);
    oled_writer_write_cmd(SH1107_CMD_ENTIRE_DISPLAY_AND_GDDRAM_ON);
    sh1107_update();
    oled_writer_write_cmd(SH1107_CMD_SET_DISPLAY_ON);
}

/* pixels can be accessed via buf[y*16+x/8] >> x%8 */
void sh1107_set_pixel(uint16_t x, uint16_t y, uint8_t c)
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
}

/* The SH1107 Segment/Common driver specifies that there are 16 pages per column
 * In total we should be writing 64*128 pixels. 8 bits per page, 16 pages per column and 64
 * columns */
void sh1107_update(void)
{
    for (size_t i = 0; i < 64; i++) {
        oled_writer_write_cmd(SH1107_CMD_SET_LOW_COL(i));
        oled_writer_write_cmd(SH1107_CMD_SET_HIGH_COL(i));
        oled_writer_write_data(&_frame_buffer[i * 16], 16);
    }
}

void sh1107_mirror(bool mirror)
{
    if (mirror) {
        oled_writer_write_cmd(SH1107_CMD_SET_SEGMENT_RE_MAP_NORMAL);
        oled_writer_write_cmd(SH1107_CMD_SET_COM_OUTPUT_SCAN_DOWN);
        // Shift the columns by 32 when display is in mirrored orientation
        oled_writer_write_cmd_with_param(SH1107_CMD_SET_DISPLAY_OFFSET, 0x20);
    } else {
        oled_writer_write_cmd(SH1107_CMD_SET_SEGMENT_RE_MAP_REVERSE);
        oled_writer_write_cmd(SH1107_CMD_SET_COM_OUTPUT_SCAN_UP);
        oled_writer_write_cmd_with_param(SH1107_CMD_SET_DISPLAY_OFFSET, 0x60);
    }
}

void sh1107_off(void)
{
    oled_writer_write_cmd(SH1107_CMD_SET_DISPLAY_OFF);
}
