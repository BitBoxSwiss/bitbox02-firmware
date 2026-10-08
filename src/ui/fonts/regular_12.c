/*******************************************************************************
 * Size: 12 px
 * Bpp: 1
 * Opts: --dump --bpp 1 --size 12 --dpi 72 --font /usr/share/fonts/truetype/msttcorefonts/Arial.ttf --range 32-126,161,167,170-171,176,186-187,191-194,196,198-207,209-212,214,217-220,223-226,228,230-239,241-244,246,249-252,255,338-339,376,8211-8212,8218,8222,8230,8249-8250,8364 --name regular_12 --output regular_12.c
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

#include "regular_12.h"

#ifndef REGULAR_12
#define REGULAR_12 1
#endif

#if REGULAR_12

static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 "\u0020" */
    0x00,

    /* U+0021 "!" */
    0xFE, 0x80,

    /* U+0022 "\"" */
    0xB6, 0x80,

    /* U+0023 "#" */
    0x14, 0x2B, 0xF9, 0x42, 0x9F, 0xCA, 0x28, 0x50,

    /* U+0024 "$" */
    0x75, 0x69, 0x47, 0x16, 0xB5, 0x71, 0x00,

    /* U+0025 "%" */
    0x62, 0x4A, 0x25, 0x13, 0x06, 0xB0, 0x64, 0x52, 0x29, 0x23, 0x00,

    /* U+0026 "&" */
    0x30, 0x91, 0x22, 0x86, 0x12, 0xA2, 0x46, 0x72,

    /* U+0027 "'" */
    0xE0,

    /* U+0028 "(" */
    0x29, 0x49, 0x24, 0x48, 0x80,

    /* U+0029 ")" */
    0x89, 0x12, 0x49, 0x4A, 0x00,

    /* U+002A "*" */
    0x27, 0xC8, 0xA0,

    /* U+002B "+" */
    0x21, 0x3E, 0x42, 0x00,

    /* U+002C "," */
    0xE0,

    /* U+002D "-" */
    0xE0,

    /* U+002E "." */
    0x80,

    /* U+002F "/" */
    0x25, 0x24, 0x94, 0x80,

    /* U+0030 "0" */
    0x74, 0x63, 0x18, 0xC6, 0x31, 0x70,

    /* U+0031 "1" */
    0x2E, 0x92, 0x49, 0x20,

    /* U+0032 "2" */
    0x74, 0x42, 0x11, 0x08, 0x88, 0xF8,

    /* U+0033 "3" */
    0x74, 0x42, 0x13, 0x04, 0x31, 0x70,

    /* U+0034 "4" */
    0x11, 0x8C, 0xA5, 0x4B, 0xE2, 0x10,

    /* U+0035 "5" */
    0x7A, 0x21, 0xE8, 0x84, 0x31, 0x70,

    /* U+0036 "6" */
    0x74, 0x61, 0x6C, 0xC6, 0x31, 0x70,

    /* U+0037 "7" */
    0xF8, 0x84, 0x42, 0x11, 0x08, 0x40,

    /* U+0038 "8" */
    0x74, 0x63, 0x17, 0x46, 0x31, 0x70,

    /* U+0039 "9" */
    0x74, 0x63, 0x19, 0xB4, 0x31, 0x70,

    /* U+003A ":" */
    0x82,

    /* U+003B ";" */
    0x83, 0x80,

    /* U+003C "<" */
    0x0B, 0xA0, 0xE0, 0x80,

    /* U+003D "=" */
    0xFC, 0x00, 0x3F,

    /* U+003E ">" */
    0x83, 0x82, 0xE8, 0x00,

    /* U+003F "?" */
    0x74, 0x62, 0x11, 0x10, 0x80, 0x20,

    /* U+0040 "@" */
    0x0F, 0x06, 0x19, 0x01, 0x26, 0x99, 0x33, 0x44, 0x68, 0x8D, 0x12, 0x9F,
    0x88, 0x04, 0x83, 0x0F, 0x80,

    /* U+0041 "A" */
    0x10, 0x50, 0xA1, 0x44, 0x4F, 0x91, 0x41, 0x82,

    /* U+0042 "B" */
    0xFA, 0x18, 0x61, 0xFE, 0x18, 0x61, 0xF8,

    /* U+0043 "C" */
    0x38, 0x8A, 0x0C, 0x08, 0x10, 0x20, 0xA2, 0x38,

    /* U+0044 "D" */
    0xF9, 0x0A, 0x0C, 0x18, 0x30, 0x60, 0xC2, 0xF8,

    /* U+0045 "E" */
    0xFE, 0x08, 0x20, 0xFE, 0x08, 0x20, 0xFC,

    /* U+0046 "F" */
    0xFC, 0x21, 0x0F, 0x42, 0x10, 0x80,

    /* U+0047 "G" */
    0x38, 0x8A, 0x0C, 0x08, 0xF0, 0x60, 0xA2, 0x38,

    /* U+0048 "H" */
    0x83, 0x06, 0x0C, 0x1F, 0xF0, 0x60, 0xC1, 0x82,

    /* U+0049 "I" */
    0xFF, 0x80,

    /* U+004A "J" */
    0x08, 0x42, 0x10, 0x86, 0x31, 0x70,

    /* U+004B "K" */
    0x83, 0x0A, 0x24, 0x8A, 0x1A, 0x22, 0x42, 0x82,

    /* U+004C "L" */
    0x82, 0x08, 0x20, 0x82, 0x08, 0x20, 0xFC,

    /* U+004D "M" */
    0x83, 0x8F, 0x1D, 0x5A, 0xB5, 0x6A, 0xC9, 0x92,

    /* U+004E "N" */
    0x83, 0x86, 0x8D, 0x19, 0x31, 0x62, 0xC3, 0x82,

    /* U+004F "O" */
    0x38, 0x8A, 0x0C, 0x18, 0x30, 0x60, 0xA2, 0x38,

    /* U+0050 "P" */
    0xFA, 0x18, 0x61, 0xFA, 0x08, 0x20, 0x80,

    /* U+0051 "Q" */
    0x38, 0x8A, 0x0C, 0x18, 0x30, 0x66, 0xA2, 0x3A, 0x00,

    /* U+0052 "R" */
    0xFD, 0x06, 0x0C, 0x1F, 0xD1, 0x21, 0x42, 0x82,

    /* U+0053 "S" */
    0x7A, 0x18, 0x60, 0x78, 0x18, 0x61, 0x78,

    /* U+0054 "T" */
    0xFE, 0x20, 0x40, 0x81, 0x02, 0x04, 0x08, 0x10,

    /* U+0055 "U" */
    0x83, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xA2, 0x38,

    /* U+0056 "V" */
    0x83, 0x05, 0x12, 0x24, 0x45, 0x0A, 0x08, 0x10,

    /* U+0057 "W" */
    0x84, 0x31, 0x46, 0x29, 0x25, 0x25, 0x14, 0xA2, 0x94, 0x51, 0x04, 0x20,
    0x80,

    /* U+0058 "X" */
    0x82, 0x89, 0x11, 0x41, 0x05, 0x11, 0x22, 0x82,

    /* U+0059 "Y" */
    0x82, 0x89, 0x11, 0x41, 0x02, 0x04, 0x08, 0x10,

    /* U+005A "Z" */
    0x7E, 0x08, 0x20, 0x41, 0x04, 0x08, 0x20, 0xFE,

    /* U+005B "[" */
    0xEA, 0xAA, 0xAC,

    /* U+005C "\\" */
    0x91, 0x24, 0x91, 0x20,

    /* U+005D "]" */
    0xD5, 0x55, 0x5C,

    /* U+005E "^" */
    0x22, 0x94, 0xA8, 0x80,

    /* U+005F "_" */
    0xFE,

    /* U+0060 "`" */
    0x90,

    /* U+0061 "a" */
    0x74, 0x42, 0xF8, 0xCD, 0xA0,

    /* U+0062 "b" */
    0x84, 0x2D, 0x98, 0xC6, 0x39, 0xB0,

    /* U+0063 "c" */
    0x69, 0x88, 0x89, 0x60,

    /* U+0064 "d" */
    0x08, 0x5B, 0x38, 0xC6, 0x33, 0x68,

    /* U+0065 "e" */
    0x74, 0x63, 0xF8, 0x45, 0xC0,

    /* U+0066 "f" */
    0x24, 0xE4, 0x44, 0x44, 0x40,

    /* U+0067 "g" */
    0x6C, 0xE3, 0x18, 0xCD, 0xA1, 0xF0,

    /* U+0068 "h" */
    0x84, 0x2D, 0x98, 0xC6, 0x31, 0x88,

    /* U+0069 "i" */
    0xBF, 0x80,

    /* U+006A "j" */
    0x20, 0x92, 0x49, 0x25, 0x00,

    /* U+006B "k" */
    0x84, 0x23, 0x2A, 0x72, 0x52, 0x88,

    /* U+006C "l" */
    0xFF, 0x80,

    /* U+006D "m" */
    0xB3, 0x66, 0x62, 0x31, 0x18, 0x8C, 0x46, 0x22,

    /* U+006E "n" */
    0xB6, 0x63, 0x18, 0xC6, 0x20,

    /* U+006F "o" */
    0x74, 0x63, 0x18, 0xC5, 0xC0,

    /* U+0070 "p" */
    0xB6, 0x63, 0x18, 0xE6, 0xD0, 0x80,

    /* U+0071 "q" */
    0x6C, 0xE3, 0x18, 0xCD, 0xA1, 0x08,

    /* U+0072 "r" */
    0xBA, 0x49, 0x20,

    /* U+0073 "s" */
    0x74, 0x60, 0xE0, 0xC5, 0xC0,

    /* U+0074 "t" */
    0x4B, 0xA4, 0x92, 0x60,

    /* U+0075 "u" */
    0x8C, 0x63, 0x18, 0xC5, 0xE0,

    /* U+0076 "v" */
    0x8C, 0x54, 0xA5, 0x10, 0x80,

    /* U+0077 "w" */
    0x88, 0xC4, 0x55, 0x4A, 0xA5, 0x51, 0x10, 0x88,

    /* U+0078 "x" */
    0x8A, 0x94, 0x45, 0x2A, 0x20,

    /* U+0079 "y" */
    0x8C, 0x54, 0xA5, 0x10, 0x84, 0x40,

    /* U+007A "z" */
    0xF8, 0x84, 0x44, 0x23, 0xE0,

    /* U+007B "{" */
    0x29, 0x25, 0x12, 0x48, 0x80,

    /* U+007C "|" */
    0xFF, 0xE0,

    /* U+007D "}" */
    0x89, 0x24, 0x52, 0x4A, 0x00,

    /* U+007E "~" */
    0x01, 0x99, 0x80,

    /* U+00A1 "¡" */
    0xBF, 0x80,

    /* U+00A7 "§" */
    0x31, 0x24, 0x1C, 0x9A, 0x16, 0x4E, 0x0A, 0x27, 0x00,

    /* U+00AA "ª" */
    0x71, 0xFF,

    /* U+00AB "«" */
    0x2A, 0xA8, 0xA2, 0x80,

    /* U+00B0 "°" */
    0xF7, 0x80,

    /* U+00BA "º" */
    0x69, 0x96,

    /* U+00BB "»" */
    0xA2, 0x8A, 0xAA, 0x00,

    /* U+00BF "¿" */
    0x20, 0x08, 0x44, 0x42, 0x31, 0x70,

    /* U+00C0 "À" */
    0x20, 0x20, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C1 "Á" */
    0x08, 0x20, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C2 "Â" */
    0x30, 0x50, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C4 "Ä" */
    0x28, 0x00, 0x41, 0x42, 0x85, 0x11, 0x3E, 0x45, 0x06, 0x08,

    /* U+00C6 "Æ" */
    0x07, 0xF0, 0x90, 0x09, 0x01, 0x10, 0x11, 0xF3, 0xF0, 0x21, 0x04, 0x10,
    0x41, 0xF0,

    /* U+00C7 "Ç" */
    0x38, 0x8A, 0x0C, 0x08, 0x10, 0x20, 0xA2, 0x38, 0x20, 0x21, 0xC0,

    /* U+00C8 "È" */
    0x20, 0x40, 0x3F, 0x82, 0x08, 0x3F, 0x82, 0x08, 0x3F,

    /* U+00C9 "É" */
    0x10, 0x80, 0x3F, 0x82, 0x08, 0x3F, 0x82, 0x08, 0x3F,

    /* U+00CA "Ê" */
    0x21, 0x40, 0x3F, 0x82, 0x08, 0x3F, 0x82, 0x08, 0x3F,

    /* U+00CB "Ë" */
    0x28, 0x0F, 0xE0, 0x82, 0x0F, 0xE0, 0x82, 0x0F, 0xC0,

    /* U+00CC "Ì" */
    0x91, 0x55, 0x55,

    /* U+00CD "Í" */
    0x62, 0xAA, 0xAA,

    /* U+00CE "Î" */
    0x22, 0x80, 0x42, 0x10, 0x84, 0x21, 0x08, 0x40,

    /* U+00CF "Ï" */
    0xA1, 0x24, 0x92, 0x49, 0x00,

    /* U+00D1 "Ñ" */
    0x14, 0x50, 0x04, 0x1C, 0x34, 0x68, 0xC9, 0x8B, 0x16, 0x1C, 0x10,

    /* U+00D2 "Ò" */
    0x20, 0x20, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D3 "Ó" */
    0x08, 0x20, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D4 "Ô" */
    0x18, 0x28, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D6 "Ö" */
    0x28, 0x00, 0xE2, 0x28, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+00D9 "Ù" */
    0x20, 0x20, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DA "Ú" */
    0x08, 0x20, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DB "Û" */
    0x10, 0x50, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DC "Ü" */
    0x28, 0x02, 0x0C, 0x18, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+00DF "ß" */
    0x72, 0x28, 0xA4, 0x92, 0x28, 0x69, 0x98,

    /* U+00E0 "à" */
    0x20, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E1 "á" */
    0x11, 0x00, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E2 "â" */
    0x33, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E4 "ä" */
    0x50, 0x1D, 0x10, 0xBE, 0x33, 0x68,

    /* U+00E6 "æ" */
    0x77, 0x44, 0x4E, 0x39, 0xF8, 0x84, 0x45, 0xDC,

    /* U+00E7 "ç" */
    0x69, 0x88, 0x89, 0x64, 0x2E,

    /* U+00E8 "è" */
    0x41, 0x00, 0xE8, 0xC7, 0xF0, 0x8B, 0x80,

    /* U+00E9 "é" */
    0x11, 0x00, 0xE8, 0xC7, 0xF0, 0x8B, 0x80,

    /* U+00EA "ê" */
    0x33, 0x80, 0xE8, 0xC7, 0xF0, 0x8B, 0x80,

    /* U+00EB "ë" */
    0x50, 0x1D, 0x18, 0xFE, 0x11, 0x70,

    /* U+00EC "ì" */
    0x91, 0x55, 0x50,

    /* U+00ED "í" */
    0x62, 0xAA, 0xA0,

    /* U+00EE "î" */
    0x65, 0x02, 0x22, 0x22, 0x22,

    /* U+00EF "ï" */
    0xA1, 0x24, 0x92, 0x40,

    /* U+00F1 "ñ" */
    0x2A, 0x81, 0x6C, 0xC6, 0x31, 0x8C, 0x40,

    /* U+00F2 "ò" */
    0x41, 0x00, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F3 "ó" */
    0x11, 0x00, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F4 "ô" */
    0x33, 0x80, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F6 "ö" */
    0x50, 0x1D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00F9 "ù" */
    0x41, 0x01, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FA "ú" */
    0x11, 0x01, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FB "û" */
    0x22, 0x81, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FC "ü" */
    0x50, 0x23, 0x18, 0xC6, 0x31, 0x78,

    /* U+00FF "ÿ" */
    0x50, 0x23, 0x15, 0x29, 0x44, 0x21, 0x10,

    /* U+0152 "Œ" */
    0x77, 0xE3, 0x08, 0x42, 0x10, 0x87, 0xE1, 0x08, 0x42, 0x30, 0x77, 0xC0,

    /* U+0153 "œ" */
    0x77, 0x44, 0x62, 0x31, 0xF8, 0x84, 0x45, 0xDC,

    /* U+0178 "Ÿ" */
    0x28, 0x02, 0x0A, 0x24, 0x45, 0x04, 0x08, 0x10, 0x20, 0x40,

    /* U+2013 "–" */
    0xFE,

    /* U+2014 "—" */
    0xFF, 0xF0,

    /* U+201A "‚" */
    0xE0,

    /* U+201E "„" */
    0xB6, 0x80,

    /* U+2026 "…" */
    0x88, 0x80,

    /* U+2039 "‹" */
    0x2A, 0x22,

    /* U+203A "›" */
    0x88, 0xA8,

    /* U+20AC "€" */
    0x1E, 0x45, 0x07, 0xF4, 0x1F, 0xD0, 0x10, 0x1E,
};

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+0020 */,
    {.bitmap_index = 1, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0021 */,
    {.bitmap_index = 3, .adv_w = 64, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 6} /* U+0022 */,
    {.bitmap_index = 5, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0023 */,
    {.bitmap_index = 13, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -1} /* U+0024 */,
    {.bitmap_index = 20, .adv_w = 176, .box_w = 9, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0025 */,
    {.bitmap_index = 31, .adv_w = 128, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0026 */,
    {.bitmap_index = 39, .adv_w = 32, .box_w = 1, .box_h = 3, .ofs_x = 1, .ofs_y = 6} /* U+0027 */,
    {.bitmap_index = 40, .adv_w = 64, .box_w = 3, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+0028 */,
    {.bitmap_index = 45, .adv_w = 64, .box_w = 3, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+0029 */,
    {.bitmap_index = 50, .adv_w = 80, .box_w = 5, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+002A */,
    {.bitmap_index = 53, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+002B */,
    {.bitmap_index = 57, .adv_w = 48, .box_w = 1, .box_h = 3, .ofs_x = 1, .ofs_y = -2} /* U+002C */,
    {.bitmap_index = 58, .adv_w = 64, .box_w = 3, .box_h = 1, .ofs_x = 0, .ofs_y = 3} /* U+002D */,
    {.bitmap_index = 59, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 1, .ofs_y = 0} /* U+002E */,
    {.bitmap_index = 60, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+002F */,
    {.bitmap_index = 64, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0030 */,
    {.bitmap_index = 70, .adv_w = 112, .box_w = 3, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0031 */,
    {.bitmap_index = 74, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0032 */,
    {.bitmap_index = 80, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0033 */,
    {.bitmap_index = 86, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0034 */,
    {.bitmap_index = 92, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0035 */,
    {.bitmap_index = 98, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0036 */,
    {.bitmap_index = 104, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0037 */,
    {.bitmap_index = 110, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0038 */,
    {.bitmap_index = 116, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0039 */,
    {.bitmap_index = 122, .adv_w = 48, .box_w = 1, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+003A */,
    {.bitmap_index = 123, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+003B */,
    {.bitmap_index = 125, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+003C */,
    {.bitmap_index = 129, .adv_w = 112, .box_w = 6, .box_h = 4, .ofs_x = 0, .ofs_y = 2} /* U+003D */,
    {.bitmap_index = 132, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+003E */,
    {.bitmap_index = 136, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+003F */,
    {.bitmap_index = 142, .adv_w = 192, .box_w = 11, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+0040 */,
    {.bitmap_index = 159, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0041 */,
    {.bitmap_index = 167, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0042 */,
    {.bitmap_index = 174, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0043 */,
    {.bitmap_index = 182, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0044 */,
    {.bitmap_index = 190, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0045 */,
    {.bitmap_index = 197, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0046 */,
    {.bitmap_index = 203, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0047 */,
    {.bitmap_index = 211, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0048 */,
    {.bitmap_index = 219, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0049 */,
    {.bitmap_index = 221, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+004A */,
    {.bitmap_index = 227, .adv_w = 128, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+004B */,
    {.bitmap_index = 235, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+004C */,
    {.bitmap_index = 242, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+004D */,
    {.bitmap_index = 250, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+004E */,
    {.bitmap_index = 258, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+004F */,
    {.bitmap_index = 266, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0050 */,
    {.bitmap_index = 273, .adv_w = 144, .box_w = 7, .box_h = 10, .ofs_x = 1, .ofs_y = -1} /* U+0051 */,
    {.bitmap_index = 282, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0052 */,
    {.bitmap_index = 290, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0053 */,
    {.bitmap_index = 297, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0054 */,
    {.bitmap_index = 305, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0055 */,
    {.bitmap_index = 313, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0056 */,
    {.bitmap_index = 321, .adv_w = 176, .box_w = 11, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0057 */,
    {.bitmap_index = 334, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0058 */,
    {.bitmap_index = 342, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0059 */,
    {.bitmap_index = 350, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+005A */,
    {.bitmap_index = 358, .adv_w = 48, .box_w = 2, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+005B */,
    {.bitmap_index = 361, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+005C */,
    {.bitmap_index = 365, .adv_w = 48, .box_w = 2, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+005D */,
    {.bitmap_index = 368, .adv_w = 80, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 4} /* U+005E */,
    {.bitmap_index = 372, .adv_w = 112, .box_w = 7, .box_h = 1, .ofs_x = 0, .ofs_y = -2} /* U+005F */,
    {.bitmap_index = 373, .adv_w = 64, .box_w = 2, .box_h = 2, .ofs_x = 1, .ofs_y = 7} /* U+0060 */,
    {.bitmap_index = 374, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0061 */,
    {.bitmap_index = 379, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0062 */,
    {.bitmap_index = 385, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0063 */,
    {.bitmap_index = 389, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0064 */,
    {.bitmap_index = 395, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0065 */,
    {.bitmap_index = 400, .adv_w = 48, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0066 */,
    {.bitmap_index = 405, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0067 */,
    {.bitmap_index = 411, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0068 */,
    {.bitmap_index = 417, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0069 */,
    {.bitmap_index = 419, .adv_w = 48, .box_w = 3, .box_h = 11, .ofs_x = -1, .ofs_y = -2} /* U+006A */,
    {.bitmap_index = 424, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+006B */,
    {.bitmap_index = 430, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+006C */,
    {.bitmap_index = 432, .adv_w = 176, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+006D */,
    {.bitmap_index = 440, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+006E */,
    {.bitmap_index = 445, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+006F */,
    {.bitmap_index = 450, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0070 */,
    {.bitmap_index = 456, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0071 */,
    {.bitmap_index = 462, .adv_w = 64, .box_w = 3, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0072 */,
    {.bitmap_index = 465, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0073 */,
    {.bitmap_index = 470, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0074 */,
    {.bitmap_index = 474, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0075 */,
    {.bitmap_index = 479, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0076 */,
    {.bitmap_index = 484, .adv_w = 144, .box_w = 9, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0077 */,
    {.bitmap_index = 492, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0078 */,
    {.bitmap_index = 497, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0079 */,
    {.bitmap_index = 503, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+007A */,
    {.bitmap_index = 508, .adv_w = 64, .box_w = 3, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+007B */,
    {.bitmap_index = 513, .adv_w = 48, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+007C */,
    {.bitmap_index = 515, .adv_w = 64, .box_w = 3, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+007D */,
    {.bitmap_index = 520, .adv_w = 112, .box_w = 6, .box_h = 4, .ofs_x = 1, .ofs_y = 2} /* U+007E */,
    {.bitmap_index = 523, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+00A1 */,
    {.bitmap_index = 525, .adv_w = 112, .box_w = 6, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+00A7 */,
    {.bitmap_index = 534, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00AA */,
    {.bitmap_index = 536, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+00AB */,
    {.bitmap_index = 540, .adv_w = 80, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = 6} /* U+00B0 */,
    {.bitmap_index = 542, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00BA */,
    {.bitmap_index = 544, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+00BB */,
    {.bitmap_index = 548, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+00BF */,
    {.bitmap_index = 554, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C0 */,
    {.bitmap_index = 565, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C1 */,
    {.bitmap_index = 576, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C2 */,
    {.bitmap_index = 587, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+00C4 */,
    {.bitmap_index = 597, .adv_w = 192, .box_w = 12, .box_h = 9, .ofs_x = -1, .ofs_y = 0} /* U+00C6 */,
    {.bitmap_index = 611, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+00C7 */,
    {.bitmap_index = 622, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00C8 */,
    {.bitmap_index = 631, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00C9 */,
    {.bitmap_index = 640, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00CA */,
    {.bitmap_index = 649, .adv_w = 128, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00CB */,
    {.bitmap_index = 658, .adv_w = 48, .box_w = 2, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00CC */,
    {.bitmap_index = 661, .adv_w = 48, .box_w = 2, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00CD */,
    {.bitmap_index = 664, .adv_w = 48, .box_w = 5, .box_h = 12, .ofs_x = -1, .ofs_y = 0} /* U+00CE */,
    {.bitmap_index = 672, .adv_w = 48, .box_w = 3, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+00CF */,
    {.bitmap_index = 677, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D1 */,
    {.bitmap_index = 688, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D2 */,
    {.bitmap_index = 699, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D3 */,
    {.bitmap_index = 710, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D4 */,
    {.bitmap_index = 721, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00D6 */,
    {.bitmap_index = 731, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D9 */,
    {.bitmap_index = 742, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00DA */,
    {.bitmap_index = 753, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00DB */,
    {.bitmap_index = 764, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00DC */,
    {.bitmap_index = 774, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DF */,
    {.bitmap_index = 781, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E0 */,
    {.bitmap_index = 788, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E1 */,
    {.bitmap_index = 795, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E2 */,
    {.bitmap_index = 802, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00E4 */,
    {.bitmap_index = 808, .adv_w = 176, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00E6 */,
    {.bitmap_index = 816, .adv_w = 96, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+00E7 */,
    {.bitmap_index = 821, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E8 */,
    {.bitmap_index = 828, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E9 */,
    {.bitmap_index = 835, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00EA */,
    {.bitmap_index = 842, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00EB */,
    {.bitmap_index = 848, .adv_w = 48, .box_w = 2, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+00EC */,
    {.bitmap_index = 851, .adv_w = 48, .box_w = 2, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00ED */,
    {.bitmap_index = 854, .adv_w = 48, .box_w = 4, .box_h = 10, .ofs_x = -1, .ofs_y = 0} /* U+00EE */,
    {.bitmap_index = 859, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00EF */,
    {.bitmap_index = 863, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F1 */,
    {.bitmap_index = 870, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F2 */,
    {.bitmap_index = 877, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F3 */,
    {.bitmap_index = 884, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F4 */,
    {.bitmap_index = 891, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00F6 */,
    {.bitmap_index = 897, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F9 */,
    {.bitmap_index = 904, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00FA */,
    {.bitmap_index = 911, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00FB */,
    {.bitmap_index = 918, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00FC */,
    {.bitmap_index = 924, .adv_w = 80, .box_w = 5, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+00FF */,
    {.bitmap_index = 931, .adv_w = 192, .box_w = 10, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0152 */,
    {.bitmap_index = 943, .adv_w = 176, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0153 */,
    {.bitmap_index = 951, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+0178 */,
    {.bitmap_index = 961, .adv_w = 112, .box_w = 7, .box_h = 1, .ofs_x = 0, .ofs_y = 3} /* U+2013 */,
    {.bitmap_index = 962, .adv_w = 192, .box_w = 12, .box_h = 1, .ofs_x = 0, .ofs_y = 3} /* U+2014 */,
    {.bitmap_index = 964, .adv_w = 48, .box_w = 1, .box_h = 3, .ofs_x = 1, .ofs_y = -2} /* U+201A */,
    {.bitmap_index = 965, .adv_w = 64, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = -2} /* U+201E */,
    {.bitmap_index = 967, .adv_w = 192, .box_w = 9, .box_h = 1, .ofs_x = 1, .ofs_y = 0} /* U+2026 */,
    {.bitmap_index = 969, .adv_w = 64, .box_w = 3, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+2039 */,
    {.bitmap_index = 971, .adv_w = 64, .box_w = 3, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+203A */,
    {.bitmap_index = 973, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+20AC */
};

static const lv_font_fmt_txt_cmap_t cmaps[] = {
    {.range_start = 32, .range_length = 95, .glyph_id_start = 1,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 161, .range_length = 1, .glyph_id_start = 96,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 167, .range_length = 1, .glyph_id_start = 97,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 170, .range_length = 2, .glyph_id_start = 98,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 176, .range_length = 1, .glyph_id_start = 100,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 186, .range_length = 2, .glyph_id_start = 101,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 191, .range_length = 4, .glyph_id_start = 103,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 196, .range_length = 1, .glyph_id_start = 107,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 198, .range_length = 10, .glyph_id_start = 108,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 209, .range_length = 4, .glyph_id_start = 118,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 214, .range_length = 1, .glyph_id_start = 122,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 217, .range_length = 4, .glyph_id_start = 123,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 223, .range_length = 4, .glyph_id_start = 127,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 228, .range_length = 1, .glyph_id_start = 131,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 230, .range_length = 10, .glyph_id_start = 132,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 241, .range_length = 4, .glyph_id_start = 142,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 246, .range_length = 1, .glyph_id_start = 146,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 249, .range_length = 4, .glyph_id_start = 147,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 255, .range_length = 1, .glyph_id_start = 151,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 338, .range_length = 2, .glyph_id_start = 152,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 376, .range_length = 1, .glyph_id_start = 154,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8211, .range_length = 2, .glyph_id_start = 155,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8218, .range_length = 1, .glyph_id_start = 157,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8222, .range_length = 1, .glyph_id_start = 158,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8230, .range_length = 1, .glyph_id_start = 159,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8249, .range_length = 2, .glyph_id_start = 160,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 8364, .range_length = 1, .glyph_id_start = 162,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY}
};

static const lv_font_fmt_txt_dsc_t font_dsc = {
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 27,
    .bpp = 1,
    .kern_classes = 0,
    .bitmap_format = 0,
    .stride = 0,
};

const UG_FONT font_regular_12 = {
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,
    .line_height = 17,
    .base_line = 4,
    .subpx = LV_FONT_SUBPX_NONE,
    .underline_position = -1,
    .underline_thickness = 0,
    .static_bitmap = 0,
    .dsc = &font_dsc,
    .fallback = NULL,
    .user_data = NULL,
};

#endif /* REGULAR_12 */
