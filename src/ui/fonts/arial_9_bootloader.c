/*******************************************************************************
 * Size: 9 px
 * Bpp: 1
 * Opts: bootloader subset of arial_9.c, range 32-126
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

#include "arial_9_bootloader.h"

#ifndef ARIAL_9
#define ARIAL_9 1
#endif

#if ARIAL_9

static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
    0x00,

    /* U+0021 "!" */
    0xFA,

    /* U+0022 "\"" */
    0xB4,

    /* U+0023 "#" */
    0x55, 0xF5, 0xFA, 0xA0,

    /* U+0024 "$" */
    0x75, 0x68, 0xE2, 0xD5, 0xC4,

    /* U+0025 "%" */
    0x4A, 0xC5, 0x04, 0x28, 0xD2, 0x80,

    /* U+0026 "&" */
    0x31, 0x24, 0x9C, 0x96, 0x27, 0x40,

    /* U+0027 "'" */
    0xC0,

    /* U+0028 "(" */
    0x2A, 0x49, 0x22, 0x20,

    /* U+0029 ")" */
    0x88, 0x92, 0x4A, 0x80,

    /* U+002A "*" */
    0xEA, 0x80,

    /* U+002B "+" */
    0x21, 0x3E, 0x42, 0x00,

    /* U+002C "," */
    0xC0,

    /* U+002D "-" */
    0xC0,

    /* U+002E "." */
    0x80,

    /* U+002F "/" */
    0x25, 0x25, 0x20,

    /* U+0030 "0" */
    0x69, 0x99, 0x99, 0x60,

    /* U+0031 "1" */
    0x75, 0x54,

    /* U+0032 "2" */
    0x69, 0x12, 0x24, 0xF0,

    /* U+0033 "3" */
    0x69, 0x12, 0x19, 0x60,

    /* U+0034 "4" */
    0x11, 0x95, 0x2F, 0x88, 0x40,

    /* U+0035 "5" */
    0x74, 0xE9, 0x19, 0x60,

    /* U+0036 "6" */
    0x69, 0xE9, 0x99, 0x60,

    /* U+0037 "7" */
    0xF1, 0x22, 0x44, 0x40,

    /* U+0038 "8" */
    0x69, 0x96, 0x99, 0x60,

    /* U+0039 "9" */
    0x69, 0x99, 0x79, 0x60,

    /* U+003A ":" */
    0x88,

    /* U+003B ";" */
    0x8C,

    /* U+003C "<" */
    0x2A, 0x22,

    /* U+003D "=" */
    0xF0, 0xF0,

    /* U+003E ">" */
    0x88, 0xA8,

    /* U+003F "?" */
    0x74, 0x42, 0x62, 0x00, 0x80,

    /* U+0040 "@" */
    0x3C, 0x42, 0x95, 0xAD, 0xA9, 0xAA, 0xBC, 0x43, 0x3C,

    /* U+0041 "A" */
    0x10, 0x51, 0x12, 0x27, 0xC8, 0x91, 0x00,

    /* U+0042 "B" */
    0xE9, 0x9F, 0x99, 0xE0,

    /* U+0043 "C" */
    0x74, 0x61, 0x08, 0x45, 0xC0,

    /* U+0044 "D" */
    0xF4, 0x63, 0x18, 0xC7, 0xC0,

    /* U+0045 "E" */
    0xFC, 0x21, 0xF8, 0x43, 0xE0,

    /* U+0046 "F" */
    0xF8, 0x8E, 0x88, 0x80,

    /* U+0047 "G" */
    0x74, 0x61, 0x38, 0xC5, 0xC0,

    /* U+0048 "H" */
    0x8C, 0x63, 0xF8, 0xC6, 0x20,

    /* U+0049 "I" */
    0xFE,

    /* U+004A "J" */
    0x11, 0x11, 0x19, 0xE0,

    /* U+004B "K" */
    0x8C, 0xA9, 0x8A, 0x4A, 0x20,

    /* U+004C "L" */
    0x88, 0x88, 0x88, 0xF0,

    /* U+004D "M" */
    0x83, 0x8F, 0x1D, 0x5A, 0xB5, 0x64, 0x80,

    /* U+004E "N" */
    0x8E, 0x73, 0x59, 0xCE, 0x20,

    /* U+004F "O" */
    0x74, 0x63, 0x18, 0xC5, 0xC0,

    /* U+0050 "P" */
    0xF9, 0x9F, 0x88, 0x80,

    /* U+0051 "Q" */
    0x74, 0x63, 0x18, 0xC9, 0xE0,

    /* U+0052 "R" */
    0xF4, 0x63, 0xE9, 0x46, 0x20,

    /* U+0053 "S" */
    0x69, 0x86, 0x19, 0x60,

    /* U+0054 "T" */
    0xF9, 0x08, 0x42, 0x10, 0x80,

    /* U+0055 "U" */
    0x8C, 0x63, 0x18, 0xC5, 0xC0,

    /* U+0056 "V" */
    0x44, 0x89, 0x12, 0x22, 0x85, 0x04, 0x00,

    /* U+0057 "W" */
    0x88, 0xCA, 0x55, 0x4A, 0xA5, 0x52, 0xA8, 0x88,

    /* U+0058 "X" */
    0x8A, 0x94, 0x45, 0x2A, 0x20,

    /* U+0059 "Y" */
    0x8A, 0x94, 0x42, 0x10, 0x80,

    /* U+005A "Z" */
    0xFC, 0x21, 0x08, 0x42, 0x0F, 0xC0,

    /* U+005B "[" */
    0xEA, 0xAA, 0xC0,

    /* U+005C "\\" */
    0x91, 0x24, 0x48,

    /* U+005D "]" */
    0xD5, 0x55, 0xC0,

    /* U+005E "^" */
    0x56, 0x80,

    /* U+005F "_" */
    0xF8,

    /* U+0060 "`" */
    0x90,

    /* U+0061 "a" */
    0xF1, 0x79, 0xF0,

    /* U+0062 "b" */
    0x88, 0xE9, 0x99, 0xE0,

    /* U+0063 "c" */
    0x69, 0x89, 0x60,

    /* U+0064 "d" */
    0x11, 0x79, 0x99, 0x70,

    /* U+0065 "e" */
    0x69, 0xF8, 0x70,

    /* U+0066 "f" */
    0x2B, 0xA4, 0x90,

    /* U+0067 "g" */
    0x79, 0x99, 0x79, 0x60,

    /* U+0068 "h" */
    0x88, 0xE9, 0x99, 0x90,

    /* U+0069 "i" */
    0xBE,

    /* U+006A "j" */
    0x45, 0x55, 0x80,

    /* U+006B "k" */
    0x88, 0x9A, 0xEA, 0x90,

    /* U+006C "l" */
    0xFE,

    /* U+006D "m" */
    0xFD, 0x26, 0x4C, 0x99, 0x20,

    /* U+006E "n" */
    0xE9, 0x99, 0x90,

    /* U+006F "o" */
    0x69, 0x99, 0x60,

    /* U+0070 "p" */
    0xE9, 0x99, 0xE8, 0x80,

    /* U+0071 "q" */
    0x79, 0x99, 0x71, 0x10,

    /* U+0072 "r" */
    0xF2, 0x48,

    /* U+0073 "s" */
    0x78, 0x61, 0xE0,

    /* U+0074 "t" */
    0x09, 0xA4, 0x98,

    /* U+0075 "u" */
    0x99, 0x99, 0x70,

    /* U+0076 "v" */
    0x8A, 0x94, 0xA2, 0x00,

    /* U+0077 "w" */
    0xAD, 0x6B, 0x55, 0x00,

    /* U+0078 "x" */
    0x96, 0x46, 0x90,

    /* U+0079 "y" */
    0x8A, 0x94, 0xA2, 0x11, 0x00,

    /* U+007A "z" */
    0xE5, 0x4E,

    /* U+007B "{" */
    0x69, 0x28, 0x92, 0x60,

    /* U+007C "|" */
    0xFF,

    /* U+007D "}" */
    0xC9, 0x22, 0x92, 0xC0,

    /* U+007E "~" */
    0xDB,

};

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+0020 */,
    {.bitmap_index = 1, .adv_w = 48, .box_w = 1, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0021 */,
    {.bitmap_index = 2, .adv_w = 48, .box_w = 3, .box_h = 2, .ofs_x = 0, .ofs_y = 5} /* U+0022 */,
    {.bitmap_index = 3, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0023 */,
    {.bitmap_index = 7, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = -1} /* U+0024 */,
    {.bitmap_index = 12, .adv_w = 128, .box_w = 6, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0025 */,
    {.bitmap_index = 18, .adv_w = 96, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0026 */,
    {.bitmap_index = 24, .adv_w = 32, .box_w = 1, .box_h = 2, .ofs_x = 0, .ofs_y = 5} /* U+0027 */,
    {.bitmap_index = 25, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0028 */,
    {.bitmap_index = 29, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0029 */,
    {.bitmap_index = 33, .adv_w = 64, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+002A */,
    {.bitmap_index = 35, .adv_w = 80, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+002B */,
    {.bitmap_index = 39, .adv_w = 48, .box_w = 1, .box_h = 2, .ofs_x = 1, .ofs_y = -1} /* U+002C */,
    {.bitmap_index = 40, .adv_w = 48, .box_w = 2, .box_h = 1, .ofs_x = 0, .ofs_y = 2} /* U+002D */,
    {.bitmap_index = 41, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 1, .ofs_y = 0} /* U+002E */,
    {.bitmap_index = 42, .adv_w = 48, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+002F */,
    {.bitmap_index = 45, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0030 */,
    {.bitmap_index = 49, .adv_w = 80, .box_w = 2, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0031 */,
    {.bitmap_index = 51, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0032 */,
    {.bitmap_index = 55, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0033 */,
    {.bitmap_index = 59, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0034 */,
    {.bitmap_index = 64, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0035 */,
    {.bitmap_index = 68, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0036 */,
    {.bitmap_index = 72, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0037 */,
    {.bitmap_index = 76, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0038 */,
    {.bitmap_index = 80, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0039 */,
    {.bitmap_index = 84, .adv_w = 48, .box_w = 1, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+003A */,
    {.bitmap_index = 85, .adv_w = 48, .box_w = 1, .box_h = 6, .ofs_x = 1, .ofs_y = -1} /* U+003B */,
    {.bitmap_index = 86, .adv_w = 80, .box_w = 3, .box_h = 5, .ofs_x = 1, .ofs_y = 1} /* U+003C */,
    {.bitmap_index = 88, .adv_w = 80, .box_w = 4, .box_h = 3, .ofs_x = 0, .ofs_y = 2} /* U+003D */,
    {.bitmap_index = 90, .adv_w = 80, .box_w = 3, .box_h = 5, .ofs_x = 1, .ofs_y = 1} /* U+003E */,
    {.bitmap_index = 92, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+003F */,
    {.bitmap_index = 97, .adv_w = 144, .box_w = 8, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0040 */,
    {.bitmap_index = 106, .adv_w = 96, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0041 */,
    {.bitmap_index = 113, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0042 */,
    {.bitmap_index = 117, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0043 */,
    {.bitmap_index = 122, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0044 */,
    {.bitmap_index = 127, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0045 */,
    {.bitmap_index = 132, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0046 */,
    {.bitmap_index = 136, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0047 */,
    {.bitmap_index = 141, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0048 */,
    {.bitmap_index = 146, .adv_w = 48, .box_w = 1, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0049 */,
    {.bitmap_index = 147, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004A */,
    {.bitmap_index = 151, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+004B */,
    {.bitmap_index = 156, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+004C */,
    {.bitmap_index = 160, .adv_w = 112, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+004D */,
    {.bitmap_index = 167, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+004E */,
    {.bitmap_index = 172, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+004F */,
    {.bitmap_index = 177, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0050 */,
    {.bitmap_index = 181, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0051 */,
    {.bitmap_index = 186, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0052 */,
    {.bitmap_index = 191, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0053 */,
    {.bitmap_index = 195, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0054 */,
    {.bitmap_index = 200, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0055 */,
    {.bitmap_index = 205, .adv_w = 96, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0056 */,
    {.bitmap_index = 212, .adv_w = 144, .box_w = 9, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0057 */,
    {.bitmap_index = 220, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0058 */,
    {.bitmap_index = 225, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0059 */,
    {.bitmap_index = 230, .adv_w = 96, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+005A */,
    {.bitmap_index = 236, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+005B */,
    {.bitmap_index = 239, .adv_w = 48, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+005C */,
    {.bitmap_index = 242, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+005D */,
    {.bitmap_index = 245, .adv_w = 48, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+005E */,
    {.bitmap_index = 247, .adv_w = 80, .box_w = 5, .box_h = 1, .ofs_x = 0, .ofs_y = -2} /* U+005F */,
    {.bitmap_index = 248, .adv_w = 48, .box_w = 2, .box_h = 2, .ofs_x = 0, .ofs_y = 5} /* U+0060 */,
    {.bitmap_index = 249, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0061 */,
    {.bitmap_index = 252, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0062 */,
    {.bitmap_index = 256, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0063 */,
    {.bitmap_index = 259, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0064 */,
    {.bitmap_index = 263, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0065 */,
    {.bitmap_index = 266, .adv_w = 64, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0066 */,
    {.bitmap_index = 269, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0067 */,
    {.bitmap_index = 273, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0068 */,
    {.bitmap_index = 277, .adv_w = 32, .box_w = 1, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0069 */,
    {.bitmap_index = 278, .adv_w = 32, .box_w = 2, .box_h = 9, .ofs_x = -1, .ofs_y = -2} /* U+006A */,
    {.bitmap_index = 281, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+006B */,
    {.bitmap_index = 285, .adv_w = 32, .box_w = 1, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+006C */,
    {.bitmap_index = 286, .adv_w = 128, .box_w = 7, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006D */,
    {.bitmap_index = 291, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006E */,
    {.bitmap_index = 294, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+006F */,
    {.bitmap_index = 297, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0070 */,
    {.bitmap_index = 301, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0071 */,
    {.bitmap_index = 305, .adv_w = 48, .box_w = 3, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0072 */,
    {.bitmap_index = 307, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0073 */,
    {.bitmap_index = 310, .adv_w = 48, .box_w = 3, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+0074 */,
    {.bitmap_index = 313, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0075 */,
    {.bitmap_index = 316, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0076 */,
    {.bitmap_index = 320, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0077 */,
    {.bitmap_index = 324, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0078 */,
    {.bitmap_index = 327, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0079 */,
    {.bitmap_index = 332, .adv_w = 64, .box_w = 3, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+007A */,
    {.bitmap_index = 334, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+007B */,
    {.bitmap_index = 338, .adv_w = 48, .box_w = 1, .box_h = 8, .ofs_x = 1, .ofs_y = -1} /* U+007C */,
    {.bitmap_index = 339, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+007D */,
    {.bitmap_index = 343, .adv_w = 80, .box_w = 4, .box_h = 2, .ofs_x = 0, .ofs_y = 2} /* U+007E */,
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

const UG_FONT font_arial_9 = {
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

#endif /* ARIAL_9 */
