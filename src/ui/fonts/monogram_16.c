/*******************************************************************************
 * Size: 16 px
 * Bpp: 1
 * Opts: --bpp 1 --size 16 --font monogram.ttf --range 32-126 --format lvgl -o monogram_16.c
 ******************************************************************************/

#ifdef __has_include
    #if __has_include("lvgl.h")
        #ifndef LV_LVGL_H_INCLUDE_SIMPLE
            #define LV_LVGL_H_INCLUDE_SIMPLE
        #endif
    #endif
#endif

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
    #include "lvgl.h"
#else
    #include "lvgl/lvgl.h"
#endif

#include "monogram_16.h"

#ifndef MONOGRAM_16
#define MONOGRAM_16 1
#endif

#if MONOGRAM_16

static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
    0x00,

    /* U+0021 "!" */
    0xFA,

    /* U+0022 "\"" */
    0xB6, 0x80,

    /* U+0023 "#" */
    0x57, 0xD4, 0xAF, 0xA8,

    /* U+0024 "$" */
    0x23, 0xE8, 0xE2, 0xF8, 0x80,

    /* U+0025 "%" */
    0x8C, 0x44, 0x44, 0x46, 0x20,

    /* U+0026 "&" */
    0x64, 0xA4, 0xF9, 0x49, 0xA0,

    /* U+0027 "'" */
    0xE0,

    /* U+0028 "(" */
    0x6A, 0xA4,

    /* U+0029 ")" */
    0x95, 0x58,

    /* U+002A "*" */
    0x25, 0x5D, 0x52, 0x00,

    /* U+002B "+" */
    0x21, 0x3E, 0x42, 0x00,

    /* U+002C "," */
    0x58,

    /* U+002D "-" */
    0xF8,

    /* U+002E "." */
    0xC0,

    /* U+002F "/" */
    0x08, 0x44, 0x44, 0x42, 0x00,

    /* U+0030 "0" */
    0x74, 0x67, 0x5C, 0xC5, 0xC0,

    /* U+0031 "1" */
    0x23, 0x08, 0x42, 0x13, 0xE0,

    /* U+0032 "2" */
    0x74, 0x42, 0x22, 0x23, 0xE0,

    /* U+0033 "3" */
    0x74, 0x42, 0x60, 0xC5, 0xC0,

    /* U+0034 "4" */
    0x4A, 0x63, 0xF0, 0x84, 0x20,

    /* U+0035 "5" */
    0xFC, 0x3C, 0x10, 0xC5, 0xC0,

    /* U+0036 "6" */
    0x74, 0x21, 0xE8, 0xC5, 0xC0,

    /* U+0037 "7" */
    0xF8, 0x42, 0x22, 0x10, 0x80,

    /* U+0038 "8" */
    0x74, 0x62, 0xE8, 0xC5, 0xC0,

    /* U+0039 "9" */
    0x74, 0x62, 0xF0, 0xC5, 0xC0,

    /* U+003A ":" */
    0xCC,

    /* U+003B ";" */
    0x50, 0x58,

    /* U+003C "<" */
    0x1B, 0x20, 0xC1, 0x80,

    /* U+003D "=" */
    0xF8, 0x3E,

    /* U+003E ">" */
    0xC1, 0x82, 0x6C, 0x00,

    /* U+003F "?" */
    0x74, 0x42, 0x22, 0x00, 0x80,

    /* U+0040 "@" */
    0x74, 0xEB, 0x59, 0xC1, 0xC0,

    /* U+0041 "A" */
    0x74, 0x63, 0x1F, 0xC6, 0x20,

    /* U+0042 "B" */
    0xF4, 0x63, 0xE8, 0xC7, 0xC0,

    /* U+0043 "C" */
    0x74, 0x61, 0x08, 0x45, 0xC0,

    /* U+0044 "D" */
    0xF4, 0x63, 0x18, 0xC7, 0xC0,

    /* U+0045 "E" */
    0xFC, 0x21, 0xE8, 0x43, 0xE0,

    /* U+0046 "F" */
    0xFC, 0x21, 0xE8, 0x42, 0x00,

    /* U+0047 "G" */
    0x74, 0x61, 0x78, 0xC5, 0xC0,

    /* U+0048 "H" */
    0x8C, 0x63, 0xF8, 0xC6, 0x20,

    /* U+0049 "I" */
    0xF9, 0x08, 0x42, 0x13, 0xE0,

    /* U+004A "J" */
    0x08, 0x42, 0x18, 0xC5, 0xC0,

    /* U+004B "K" */
    0x8C, 0xA9, 0x8A, 0x4A, 0x20,

    /* U+004C "L" */
    0x84, 0x21, 0x08, 0x43, 0xE0,

    /* U+004D "M" */
    0x8E, 0xEB, 0x18, 0xC6, 0x20,

    /* U+004E "N" */
    0x8C, 0x73, 0x59, 0xC6, 0x20,

    /* U+004F "O" */
    0x74, 0x63, 0x18, 0xC5, 0xC0,

    /* U+0050 "P" */
    0xF4, 0x63, 0xE8, 0x42, 0x00,

    /* U+0051 "Q" */
    0x74, 0x63, 0x18, 0xC5, 0xC3,

    /* U+0052 "R" */
    0xF4, 0x63, 0xE8, 0xC6, 0x20,

    /* U+0053 "S" */
    0x74, 0x60, 0xE0, 0xC5, 0xC0,

    /* U+0054 "T" */
    0xF9, 0x08, 0x42, 0x10, 0x80,

    /* U+0055 "U" */
    0x8C, 0x63, 0x18, 0xC5, 0xC0,

    /* U+0056 "V" */
    0x8C, 0x63, 0x15, 0x28, 0x80,

    /* U+0057 "W" */
    0x8C, 0x63, 0x1A, 0xEE, 0x20,

    /* U+0058 "X" */
    0x8C, 0x54, 0x45, 0x46, 0x20,

    /* U+0059 "Y" */
    0x8C, 0x54, 0x42, 0x10, 0x80,

    /* U+005A "Z" */
    0xF8, 0x44, 0x44, 0x43, 0xE0,

    /* U+005B "[" */
    0xEA, 0xAC,

    /* U+005C "\\" */
    0x84, 0x10, 0x41, 0x04, 0x20,

    /* U+005D "]" */
    0xD5, 0x5C,

    /* U+005E "^" */
    0x22, 0xA2,

    /* U+005F "_" */
    0xF8,

    /* U+0060 "`" */
    0x90,

    /* U+0061 "a" */
    0x7C, 0x63, 0x17, 0x80,

    /* U+0062 "b" */
    0x84, 0x3D, 0x18, 0xC7, 0xC0,

    /* U+0063 "c" */
    0x74, 0x61, 0x17, 0x00,

    /* U+0064 "d" */
    0x08, 0x5F, 0x18, 0xC5, 0xE0,

    /* U+0065 "e" */
    0x74, 0x7F, 0x07, 0x00,

    /* U+0066 "f" */
    0x32, 0x51, 0xE4, 0x21, 0x00,

    /* U+0067 "g" */
    0x7C, 0x63, 0x17, 0x85, 0xC0,

    /* U+0068 "h" */
    0x84, 0x3D, 0x18, 0xC6, 0x20,

    /* U+0069 "i" */
    0x20, 0x18, 0x42, 0x13, 0xE0,

    /* U+006A "j" */
    0x08, 0x06, 0x10, 0x84, 0x31, 0x70,

    /* U+006B "k" */
    0x84, 0x23, 0x2E, 0x4A, 0x20,

    /* U+006C "l" */
    0xC2, 0x10, 0x84, 0x20, 0xE0,

    /* U+006D "m" */
    0xF5, 0x6B, 0x5A, 0x80,

    /* U+006E "n" */
    0xF4, 0x63, 0x18, 0x80,

    /* U+006F "o" */
    0x74, 0x63, 0x17, 0x00,

    /* U+0070 "p" */
    0xF4, 0x63, 0x1F, 0x42, 0x00,

    /* U+0071 "q" */
    0x7C, 0x63, 0x17, 0x84, 0x20,

    /* U+0072 "r" */
    0xB6, 0x61, 0x08, 0x00,

    /* U+0073 "s" */
    0x7C, 0x1C, 0x1F, 0x00,

    /* U+0074 "t" */
    0x42, 0x3C, 0x84, 0x20, 0xE0,

    /* U+0075 "u" */
    0x8C, 0x63, 0x17, 0x80,

    /* U+0076 "v" */
    0x8C, 0x62, 0xA2, 0x00,

    /* U+0077 "w" */
    0x8C, 0x6B, 0x55, 0x00,

    /* U+0078 "x" */
    0x8A, 0x88, 0xA8, 0x80,

    /* U+0079 "y" */
    0x8C, 0x63, 0x17, 0x85, 0xC0,

    /* U+007A "z" */
    0xF8, 0x88, 0x8F, 0x80,

    /* U+007B "{" */
    0x29, 0x44, 0x88,

    /* U+007C "|" */
    0xFE,

    /* U+007D "}" */
    0x89, 0x14, 0xA0,

    /* U+007E "~" */
    0x4D, 0x80,
};

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 96, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+0020 */,
    {.bitmap_index = 1, .adv_w = 96, .box_w = 1, .box_h = 7, .ofs_x = 2, .ofs_y = 0} /* U+0021 */,
    {.bitmap_index = 2, .adv_w = 96, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = 4} /* U+0022 */,
    {.bitmap_index = 4, .adv_w = 96, .box_w = 5, .box_h = 6, .ofs_x = 0, .ofs_y = 0} /* U+0023 */,
    {.bitmap_index = 8, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0024 */,
    {.bitmap_index = 13, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0025 */,
    {.bitmap_index = 18, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0026 */,
    {.bitmap_index = 23, .adv_w = 96, .box_w = 1, .box_h = 3, .ofs_x = 2, .ofs_y = 4} /* U+0027 */,
    {.bitmap_index = 24, .adv_w = 96, .box_w = 2, .box_h = 7, .ofs_x = 2, .ofs_y = 0} /* U+0028 */,
    {.bitmap_index = 26, .adv_w = 96, .box_w = 2, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0029 */,
    {.bitmap_index = 28, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 1} /* U+002A */,
    {.bitmap_index = 32, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 1} /* U+002B */,
    {.bitmap_index = 36, .adv_w = 96, .box_w = 2, .box_h = 3, .ofs_x = 1, .ofs_y = -1} /* U+002C */,
    {.bitmap_index = 37, .adv_w = 96, .box_w = 5, .box_h = 1, .ofs_x = 0, .ofs_y = 3} /* U+002D */,
    {.bitmap_index = 38, .adv_w = 96, .box_w = 1, .box_h = 2, .ofs_x = 2, .ofs_y = 0} /* U+002E */,
    {.bitmap_index = 39, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+002F */,
    {.bitmap_index = 44, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0030 */,
    {.bitmap_index = 49, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0031 */,
    {.bitmap_index = 54, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0032 */,
    {.bitmap_index = 59, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0033 */,
    {.bitmap_index = 64, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0034 */,
    {.bitmap_index = 69, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0035 */,
    {.bitmap_index = 74, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0036 */,
    {.bitmap_index = 79, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0037 */,
    {.bitmap_index = 84, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0038 */,
    {.bitmap_index = 89, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0039 */,
    {.bitmap_index = 94, .adv_w = 96, .box_w = 1, .box_h = 6, .ofs_x = 2, .ofs_y = 0} /* U+003A */,
    {.bitmap_index = 95, .adv_w = 96, .box_w = 2, .box_h = 7, .ofs_x = 1, .ofs_y = -1} /* U+003B */,
    {.bitmap_index = 97, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 1} /* U+003C */,
    {.bitmap_index = 101, .adv_w = 96, .box_w = 5, .box_h = 3, .ofs_x = 0, .ofs_y = 2} /* U+003D */,
    {.bitmap_index = 103, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 1} /* U+003E */,
    {.bitmap_index = 107, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+003F */,
    {.bitmap_index = 112, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0040 */,
    {.bitmap_index = 117, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0041 */,
    {.bitmap_index = 122, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0042 */,
    {.bitmap_index = 127, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0043 */,
    {.bitmap_index = 132, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0044 */,
    {.bitmap_index = 137, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0045 */,
    {.bitmap_index = 142, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0046 */,
    {.bitmap_index = 147, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0047 */,
    {.bitmap_index = 152, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0048 */,
    {.bitmap_index = 157, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0049 */,
    {.bitmap_index = 162, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004A */,
    {.bitmap_index = 167, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004B */,
    {.bitmap_index = 172, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004C */,
    {.bitmap_index = 177, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004D */,
    {.bitmap_index = 182, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004E */,
    {.bitmap_index = 187, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004F */,
    {.bitmap_index = 192, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0050 */,
    {.bitmap_index = 197, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = -1} /* U+0051 */,
    {.bitmap_index = 202, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0052 */,
    {.bitmap_index = 207, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0053 */,
    {.bitmap_index = 212, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0054 */,
    {.bitmap_index = 217, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0055 */,
    {.bitmap_index = 222, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0056 */,
    {.bitmap_index = 227, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0057 */,
    {.bitmap_index = 232, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0058 */,
    {.bitmap_index = 237, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0059 */,
    {.bitmap_index = 242, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+005A */,
    {.bitmap_index = 247, .adv_w = 96, .box_w = 2, .box_h = 7, .ofs_x = 2, .ofs_y = 0} /* U+005B */,
    {.bitmap_index = 249, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+005C */,
    {.bitmap_index = 254, .adv_w = 96, .box_w = 2, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+005D */,
    {.bitmap_index = 256, .adv_w = 96, .box_w = 5, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+005E */,
    {.bitmap_index = 258, .adv_w = 96, .box_w = 5, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+005F */,
    {.bitmap_index = 259, .adv_w = 96, .box_w = 2, .box_h = 2, .ofs_x = 1, .ofs_y = 5} /* U+0060 */,
    {.bitmap_index = 260, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0061 */,
    {.bitmap_index = 264, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0062 */,
    {.bitmap_index = 269, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0063 */,
    {.bitmap_index = 273, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0064 */,
    {.bitmap_index = 278, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0065 */,
    {.bitmap_index = 282, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0066 */,
    {.bitmap_index = 287, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0067 */,
    {.bitmap_index = 292, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0068 */,
    {.bitmap_index = 297, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0069 */,
    {.bitmap_index = 302, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+006A */,
    {.bitmap_index = 308, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+006B */,
    {.bitmap_index = 313, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+006C */,
    {.bitmap_index = 318, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006D */,
    {.bitmap_index = 322, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006E */,
    {.bitmap_index = 326, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006F */,
    {.bitmap_index = 330, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0070 */,
    {.bitmap_index = 335, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0071 */,
    {.bitmap_index = 340, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0072 */,
    {.bitmap_index = 344, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0073 */,
    {.bitmap_index = 348, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0074 */,
    {.bitmap_index = 353, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0075 */,
    {.bitmap_index = 357, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0076 */,
    {.bitmap_index = 361, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0077 */,
    {.bitmap_index = 365, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0078 */,
    {.bitmap_index = 369, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0079 */,
    {.bitmap_index = 374, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+007A */,
    {.bitmap_index = 378, .adv_w = 96, .box_w = 3, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+007B */,
    {.bitmap_index = 381, .adv_w = 96, .box_w = 1, .box_h = 7, .ofs_x = 2, .ofs_y = 0} /* U+007C */,
    {.bitmap_index = 382, .adv_w = 96, .box_w = 3, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+007D */,
    {.bitmap_index = 385, .adv_w = 96, .box_w = 5, .box_h = 2, .ofs_x = 0, .ofs_y = 5} /* U+007E */
};

static const lv_font_fmt_txt_cmap_t cmaps[] = {
    {.range_start = 32, .range_length = 95, .glyph_id_start = 1,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY}
};

static const lv_font_fmt_txt_dsc_t font_dsc = {
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 1,
    .bpp = 1,
    .kern_classes = 0,
    .bitmap_format = 0,
    .stride = 0,
};

const UG_FONT font_monogram_16 = {
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,
    .line_height = 9,
    .base_line = 2,
    .subpx = LV_FONT_SUBPX_NONE,
    .underline_position = -1,
    .underline_thickness = 0,
    .static_bitmap = 0,
    .dsc = &font_dsc,
    .fallback = NULL,
    .user_data = NULL,
};

#endif /* MONOGRAM_16 */
