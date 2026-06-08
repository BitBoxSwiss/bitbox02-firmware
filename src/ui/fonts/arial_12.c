/*******************************************************************************
 * Size: 12 px
 * Bpp: 1
 * Opts: --bpp 1 --size 12 --dpi 72 --font /usr/share/fonts/truetype/msttcorefonts/Arial.ttf --range 32-126,160-383 --format lvgl -o arial_12.c
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

#include "arial_12.h"

#ifndef ARIAL_12
#define ARIAL_12 1
#endif

#if ARIAL_12

static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
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
    0x34, 0xE4, 0x44, 0x44, 0x40,

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

    /* U+00A0 " " */
    0x00,

    /* U+00A1 "¡" */
    0xBF, 0x80,

    /* U+00A2 "¢" */
    0x10, 0x9D, 0x3A, 0x52, 0xB9, 0x72, 0x10, 0x00,

    /* U+00A3 "£" */
    0x32, 0x50, 0x8F, 0x90, 0x8E, 0x98,

    /* U+00A4 "¤" */
    0xAA, 0xA2, 0xAA, 0x80,

    /* U+00A5 "¥" */
    0x82, 0x89, 0xA1, 0x4F, 0xE2, 0x3F, 0x88, 0x10,

    /* U+00A6 "¦" */
    0xF3, 0xE0,

    /* U+00A7 "§" */
    0x31, 0x24, 0x1C, 0x9A, 0x16, 0x4E, 0x0A, 0x27, 0x00,

    /* U+00A8 "¨" */
    0xA0,

    /* U+00A9 "©" */
    0x3E, 0x20, 0xA7, 0x34, 0x5A, 0x0D, 0x16, 0x72, 0x82, 0x3E, 0x00,

    /* U+00AA "ª" */
    0x71, 0xFF,

    /* U+00AB "«" */
    0x2A, 0xA8, 0xA2, 0x80,

    /* U+00AC "¬" */
    0xFC, 0x10, 0x41,

    /* U+00AD "­" */
    0xE0,

    /* U+00AE "®" */
    0x3E, 0x20, 0xAF, 0x34, 0x5B, 0xCD, 0x26, 0x8A, 0x82, 0x3E, 0x00,

    /* U+00AF "¯" */
    0xFE,

    /* U+00B0 "°" */
    0xF7, 0x80,

    /* U+00B1 "±" */
    0x21, 0x3E, 0x42, 0x03, 0xE0,

    /* U+00B2 "²" */
    0xF1, 0x6F,

    /* U+00B3 "³" */
    0xF2, 0x1F,

    /* U+00B4 "´" */
    0x60,

    /* U+00B5 "µ" */
    0x8C, 0x63, 0x18, 0xC7, 0xF0, 0x80,

    /* U+00B6 "¶" */
    0x7F, 0xAE, 0xBA, 0x68, 0xA2, 0x8A, 0x28, 0xA2, 0x80,

    /* U+00B7 "·" */
    0x80,

    /* U+00B8 "¸" */
    0x47, 0x80,

    /* U+00B9 "¹" */
    0x75,

    /* U+00BA "º" */
    0x69, 0x96,

    /* U+00BB "»" */
    0xA2, 0x8A, 0xAA, 0x00,

    /* U+00BC "¼" */
    0x42, 0x61, 0x11, 0x09, 0x00, 0x80, 0x88, 0x8C, 0x8F, 0x41, 0x00,

    /* U+00BD "½" */
    0x42, 0x61, 0x11, 0x09, 0x00, 0x80, 0x98, 0x92, 0x82, 0x47, 0x80,

    /* U+00BE "¾" */
    0xE0, 0x90, 0x42, 0x23, 0x90, 0x04, 0x02, 0x21, 0x18, 0x4F, 0x20, 0x80,

    /* U+00BF "¿" */
    0x20, 0x08, 0x44, 0x42, 0x31, 0x70,

    /* U+00C0 "À" */
    0x20, 0x20, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C1 "Á" */
    0x08, 0x20, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C2 "Â" */
    0x30, 0x50, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C3 "Ã" */
    0x14, 0x50, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+00C4 "Ä" */
    0x28, 0x00, 0x41, 0x42, 0x85, 0x11, 0x3E, 0x45, 0x06, 0x08,

    /* U+00C5 "Å" */
    0x38, 0x50, 0xE1, 0x42, 0x85, 0x11, 0x3E, 0x45, 0x06, 0x08,

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

    /* U+00D0 "Ð" */
    0x7C, 0x42, 0x41, 0x41, 0xF9, 0x41, 0x41, 0x42, 0x7C,

    /* U+00D1 "Ñ" */
    0x14, 0x50, 0x04, 0x1C, 0x34, 0x68, 0xC9, 0x8B, 0x16, 0x1C, 0x10,

    /* U+00D2 "Ò" */
    0x20, 0x20, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D3 "Ó" */
    0x08, 0x20, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D4 "Ô" */
    0x18, 0x28, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D5 "Õ" */
    0x14, 0x50, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00D6 "Ö" */
    0x28, 0x00, 0xE2, 0x28, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+00D7 "×" */
    0x8B, 0x88, 0xE8, 0x80,

    /* U+00D8 "Ø" */
    0x3A, 0x8A, 0x2C, 0x59, 0x34, 0x68, 0xA2, 0xB8,

    /* U+00D9 "Ù" */
    0x20, 0x20, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DA "Ú" */
    0x08, 0x20, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DB "Û" */
    0x10, 0x50, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+00DC "Ü" */
    0x28, 0x02, 0x0C, 0x18, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+00DD "Ý" */
    0x08, 0x20, 0x04, 0x14, 0x48, 0x8A, 0x08, 0x10, 0x20, 0x40, 0x80,

    /* U+00DE "Þ" */
    0x82, 0x0F, 0xA1, 0x86, 0x1F, 0xA0, 0x80,

    /* U+00DF "ß" */
    0x72, 0x28, 0xA4, 0x92, 0x28, 0x69, 0x98,

    /* U+00E0 "à" */
    0x20, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E1 "á" */
    0x11, 0x00, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E2 "â" */
    0x33, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E3 "ã" */
    0x2A, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+00E4 "ä" */
    0x50, 0x1D, 0x10, 0xBE, 0x33, 0x68,

    /* U+00E5 "å" */
    0x72, 0x9C, 0x07, 0x44, 0x2F, 0x8C, 0xDA,

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

    /* U+00F0 "ð" */
    0x29, 0xE0, 0x9F, 0x86, 0x18, 0x61, 0x78,

    /* U+00F1 "ñ" */
    0x2A, 0x81, 0x6C, 0xC6, 0x31, 0x8C, 0x40,

    /* U+00F2 "ò" */
    0x41, 0x00, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F3 "ó" */
    0x11, 0x00, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F4 "ô" */
    0x33, 0x80, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F5 "õ" */
    0x2A, 0x80, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+00F6 "ö" */
    0x50, 0x1D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00F7 "÷" */
    0x20, 0x3E, 0x02, 0x00,

    /* U+00F8 "ø" */
    0x7C, 0xA7, 0x5C, 0xA7, 0xC0,

    /* U+00F9 "ù" */
    0x41, 0x01, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FA "ú" */
    0x11, 0x01, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FB "û" */
    0x22, 0x81, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+00FC "ü" */
    0x50, 0x23, 0x18, 0xC6, 0x31, 0x78,

    /* U+00FD "ý" */
    0x11, 0x01, 0x18, 0xA9, 0x4A, 0x21, 0x08, 0x80,

    /* U+00FE "þ" */
    0x84, 0x3D, 0x18, 0xC6, 0x31, 0xF4, 0x20,

    /* U+00FF "ÿ" */
    0x50, 0x23, 0x15, 0x29, 0x44, 0x21, 0x10,

    /* U+0100 "Ā" */
    0x78, 0x00, 0x41, 0x42, 0x85, 0x11, 0x3E, 0x45, 0x06, 0x08,

    /* U+0101 "ā" */
    0xF0, 0x1D, 0x10, 0xBE, 0x33, 0x68,

    /* U+0102 "Ă" */
    0x28, 0x70, 0x00, 0x82, 0x85, 0x0A, 0x22, 0x7C, 0x8A, 0x0C, 0x10,

    /* U+0103 "ă" */
    0x53, 0x80, 0xE8, 0x85, 0xF1, 0x9B, 0x40,

    /* U+0104 "Ą" */
    0x10, 0x28, 0x28, 0x28, 0x44, 0x7C, 0x44, 0x82, 0x82, 0x02, 0x03,

    /* U+0105 "ą" */
    0x72, 0x20, 0x9E, 0x8A, 0x66, 0x82, 0x0C,

    /* U+0106 "Ć" */
    0x08, 0x20, 0x01, 0xC4, 0x50, 0x60, 0x40, 0x81, 0x05, 0x11, 0xC0,

    /* U+0107 "ć" */
    0x24, 0x06, 0x98, 0x88, 0x96,

    /* U+0108 "Ĉ" */
    0x18, 0x68, 0x01, 0xC4, 0x50, 0x60, 0x40, 0x81, 0x05, 0x11, 0xC0,

    /* U+0109 "ĉ" */
    0x6D, 0x06, 0x98, 0x88, 0x96,

    /* U+010A "Ċ" */
    0x10, 0x00, 0xE2, 0x28, 0x30, 0x20, 0x40, 0x82, 0x88, 0xE0,

    /* U+010B "ċ" */
    0x20, 0x69, 0x88, 0x89, 0x60,

    /* U+010C "Č" */
    0x34, 0x30, 0x01, 0xC4, 0x50, 0x60, 0x40, 0x81, 0x05, 0x11, 0xC0,

    /* U+010D "č" */
    0x51, 0x80, 0xC9, 0x42, 0x10, 0x93, 0x00,

    /* U+010E "Ď" */
    0x50, 0x40, 0x07, 0xC8, 0x50, 0x60, 0xC1, 0x83, 0x06, 0x17, 0xC0,

    /* U+010F "ď" */
    0x0A, 0x15, 0xAC, 0xC8, 0x91, 0x22, 0x4C, 0x68,

    /* U+0110 "Đ" */
    0x7C, 0x42, 0x41, 0x41, 0xF9, 0x41, 0x41, 0x42, 0x7C,

    /* U+0111 "đ" */
    0x3C, 0x26, 0xA6, 0x8A, 0x28, 0xA6, 0x68,

    /* U+0112 "Ē" */
    0x78, 0x0F, 0xE0, 0x82, 0x0F, 0xE0, 0x82, 0x0F, 0xC0,

    /* U+0113 "ē" */
    0xF0, 0x1D, 0x18, 0xFE, 0x11, 0x70,

    /* U+0114 "Ĕ" */
    0x51, 0xC0, 0x3F, 0x82, 0x08, 0x3F, 0x82, 0x08, 0x3F,

    /* U+0115 "ĕ" */
    0x53, 0x80, 0xE8, 0xC7, 0xF0, 0x8B, 0x80,

    /* U+0116 "Ė" */
    0x10, 0x0F, 0xE0, 0x82, 0x0F, 0xE0, 0x82, 0x0F, 0xC0,

    /* U+0117 "ė" */
    0x20, 0x1D, 0x18, 0xFE, 0x11, 0x70,

    /* U+0118 "Ę" */
    0xFE, 0x08, 0x20, 0xFE, 0x08, 0x20, 0xFC, 0x20, 0xC0,

    /* U+0119 "ę" */
    0x74, 0x63, 0xF8, 0x45, 0xC4, 0x30,

    /* U+011A "Ě" */
    0x50, 0x80, 0x3F, 0x82, 0x08, 0x3F, 0x82, 0x08, 0x3F,

    /* U+011B "ě" */
    0x51, 0x00, 0xE8, 0xC7, 0xF0, 0x8B, 0x80,

    /* U+011C "Ĝ" */
    0x10, 0x50, 0x01, 0xC4, 0x50, 0x60, 0x47, 0x83, 0x05, 0x11, 0xC0,

    /* U+011D "ĝ" */
    0x32, 0x80, 0xD9, 0xC6, 0x31, 0x9B, 0x43, 0xE0,

    /* U+011E "Ğ" */
    0x28, 0x70, 0x01, 0xC4, 0x50, 0x60, 0x47, 0x83, 0x05, 0x11, 0xC0,

    /* U+011F "ğ" */
    0x53, 0x80, 0xD9, 0xC6, 0x31, 0x9B, 0x43, 0xE0,

    /* U+0120 "Ġ" */
    0x10, 0x00, 0xE2, 0x28, 0x30, 0x23, 0xC1, 0x82, 0x88, 0xE0,

    /* U+0121 "ġ" */
    0x20, 0x1B, 0x38, 0xC6, 0x33, 0x68, 0x7C,

    /* U+0122 "Ģ" */
    0x38, 0x8A, 0x0C, 0x08, 0xF0, 0x60, 0xA2, 0x38, 0x20, 0x21, 0xC0,

    /* U+0123 "ģ" */
    0x11, 0x08, 0x06, 0xCE, 0x31, 0x8C, 0xDA, 0x1F, 0x00,

    /* U+0124 "Ĥ" */
    0x18, 0x68, 0x04, 0x18, 0x30, 0x60, 0xFF, 0x83, 0x06, 0x0C, 0x10,

    /* U+0125 "ĥ" */
    0x22, 0x81, 0x08, 0x5B, 0x31, 0x8C, 0x63, 0x10,

    /* U+0126 "Ħ" */
    0x41, 0x7F, 0xD0, 0x48, 0x27, 0xF2, 0x09, 0x04, 0x82, 0x41, 0x00,

    /* U+0127 "ħ" */
    0xF1, 0x05, 0x99, 0x45, 0x14, 0x51, 0x44,

    /* U+0128 "Ĩ" */
    0xFA, 0x04, 0x44, 0x44, 0x44, 0x44,

    /* U+0129 "ĩ" */
    0xFA, 0x04, 0x44, 0x44, 0x44,

    /* U+012A "Ī" */
    0xF0, 0x44, 0x44, 0x44, 0x44, 0x40,

    /* U+012B "ī" */
    0xF0, 0x44, 0x44, 0x44, 0x40,

    /* U+012C "Ĭ" */
    0xB8, 0x49, 0x24, 0x92, 0x40,

    /* U+012D "ĭ" */
    0xB8, 0x49, 0x24, 0x90,

    /* U+012E "Į" */
    0x92, 0x49, 0x24, 0x93, 0x80,

    /* U+012F "į" */
    0x8A, 0xAA, 0xAC,

    /* U+0130 "İ" */
    0xBF, 0xE0,

    /* U+0131 "ı" */
    0xFE,

    /* U+0132 "Ĳ" */
    0x83, 0x06, 0x0C, 0x18, 0x30, 0x68, 0xD1, 0x9C,

    /* U+0133 "ĳ" */
    0xA2, 0xDB, 0x6D, 0xA5, 0x00,

    /* U+0134 "Ĵ" */
    0x0C, 0x14, 0x00, 0x40, 0x81, 0x02, 0x04, 0x09, 0x12, 0x23, 0x80,

    /* U+0135 "ĵ" */
    0x22, 0x80, 0x42, 0x10, 0x84, 0x21, 0x08, 0x80,

    /* U+0136 "Ķ" */
    0x83, 0x0A, 0x24, 0x8A, 0x1A, 0x22, 0x42, 0x82, 0x00, 0x20, 0x21, 0x80,

    /* U+0137 "ķ" */
    0x84, 0x23, 0x2A, 0x72, 0x52, 0x88, 0x04, 0x13, 0x00,

    /* U+0138 "ĸ" */
    0x95, 0x31, 0x4A, 0x4A, 0x40,

    /* U+0139 "Ĺ" */
    0x21, 0x00, 0x20, 0x82, 0x08, 0x20, 0x82, 0x08, 0x3F,

    /* U+013A "ĺ" */
    0x62, 0xAA, 0xAA,

    /* U+013B "Ļ" */
    0x82, 0x08, 0x20, 0x82, 0x08, 0x20, 0xFC, 0x01, 0x82, 0x38,

    /* U+013C "ļ" */
    0x49, 0x24, 0x92, 0x41, 0x9E,

    /* U+013D "Ľ" */
    0x94, 0xA5, 0x08, 0x42, 0x10, 0xF8,

    /* U+013E "ľ" */
    0xB6, 0xC9, 0x24, 0x80,

    /* U+013F "Ŀ" */
    0x82, 0x08, 0x20, 0x92, 0x08, 0x20, 0xFC,

    /* U+0140 "ŀ" */
    0x92, 0x4B, 0x24, 0x80,

    /* U+0141 "Ł" */
    0x40, 0x81, 0x43, 0x04, 0x18, 0x10, 0x20, 0x7E,

    /* U+0142 "ł" */
    0x49, 0x35, 0x92, 0x40,

    /* U+0143 "Ń" */
    0x08, 0x20, 0x04, 0x1C, 0x34, 0x68, 0xC9, 0x8B, 0x16, 0x1C, 0x10,

    /* U+0144 "ń" */
    0x11, 0x01, 0x6C, 0xC6, 0x31, 0x8C, 0x40,

    /* U+0145 "Ņ" */
    0x83, 0x86, 0x8D, 0x19, 0x31, 0x62, 0xC3, 0x82, 0x00, 0x60, 0x43, 0x00,

    /* U+0146 "ņ" */
    0xB6, 0x63, 0x18, 0xC6, 0x20, 0x30, 0x9C,

    /* U+0147 "Ň" */
    0x28, 0x30, 0x04, 0x1C, 0x34, 0x68, 0xC9, 0x8B, 0x16, 0x1C, 0x10,

    /* U+0148 "ň" */
    0x71, 0x81, 0x6C, 0xC6, 0x31, 0x8C, 0x40,

    /* U+0149 "ŉ" */
    0x82, 0x0D, 0x99, 0x45, 0x14, 0x51, 0x44,

    /* U+014A "Ŋ" */
    0xBD, 0x86, 0x0C, 0x18, 0x30, 0x60, 0xC2, 0x98,

    /* U+014B "ŋ" */
    0xB6, 0x63, 0x18, 0xC6, 0x21, 0x18,

    /* U+014C "Ō" */
    0x3C, 0x00, 0xE2, 0x28, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+014D "ō" */
    0x78, 0x1D, 0x18, 0xC6, 0x31, 0x70,

    /* U+014E "Ŏ" */
    0x28, 0x70, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+014F "ŏ" */
    0x51, 0x80, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+0150 "Ő" */
    0x14, 0x50, 0x01, 0xC4, 0x50, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+0151 "ő" */
    0x2A, 0x80, 0xE8, 0xC6, 0x31, 0x8B, 0x80,

    /* U+0152 "Œ" */
    0x77, 0xE3, 0x08, 0x42, 0x10, 0x87, 0xE1, 0x08, 0x42, 0x30, 0x77, 0xC0,

    /* U+0153 "œ" */
    0x77, 0x44, 0x62, 0x31, 0xF8, 0x84, 0x45, 0xDC,

    /* U+0154 "Ŕ" */
    0x08, 0x20, 0x07, 0xE8, 0x30, 0x60, 0xFE, 0x89, 0x0A, 0x14, 0x10,

    /* U+0155 "ŕ" */
    0x28, 0x5D, 0x24, 0x90,

    /* U+0156 "Ŗ" */
    0xFD, 0x06, 0x0C, 0x1F, 0xD1, 0x21, 0x42, 0x82, 0x00, 0x60, 0x43, 0x00,

    /* U+0157 "ŗ" */
    0x56, 0x44, 0x44, 0x40, 0x62, 0xE0,

    /* U+0158 "Ř" */
    0x50, 0x40, 0x07, 0xE8, 0x30, 0x60, 0xFE, 0x89, 0x0A, 0x14, 0x10,

    /* U+0159 "ř" */
    0x51, 0x00, 0xA6, 0x21, 0x08, 0x42, 0x00,

    /* U+015A "Ś" */
    0x10, 0x80, 0x1E, 0x86, 0x18, 0x1E, 0x06, 0x18, 0x5E,

    /* U+015B "ś" */
    0x22, 0x00, 0xE8, 0xC1, 0xC1, 0x8B, 0x80,

    /* U+015C "Ŝ" */
    0x10, 0xA0, 0x1E, 0x86, 0x18, 0x1E, 0x06, 0x18, 0x5E,

    /* U+015D "ŝ" */
    0x22, 0x80, 0xE8, 0xC1, 0xC1, 0x8B, 0x80,

    /* U+015E "Ş" */
    0x7A, 0x18, 0x60, 0x78, 0x18, 0x61, 0x78, 0x81, 0x1C,

    /* U+015F "ş" */
    0x74, 0x60, 0xE0, 0xC5, 0xC4, 0x13, 0x80,

    /* U+0160 "Š" */
    0x50, 0x80, 0x1E, 0x86, 0x18, 0x1E, 0x06, 0x18, 0x5E,

    /* U+0161 "š" */
    0x51, 0x00, 0xE8, 0xC1, 0xC1, 0x8B, 0x80,

    /* U+0162 "Ţ" */
    0xFE, 0x20, 0x40, 0x81, 0x02, 0x04, 0x08, 0x10, 0x00, 0x40, 0x80,

    /* U+0163 "ţ" */
    0x4B, 0xA4, 0x92, 0x60, 0x80,

    /* U+0164 "Ť" */
    0x28, 0x20, 0x07, 0xF1, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,

    /* U+0165 "ť" */
    0x4A, 0x38, 0x84, 0x21, 0x08, 0x60,

    /* U+0166 "Ŧ" */
    0xFE, 0x20, 0x40, 0x87, 0xC2, 0x04, 0x08, 0x10,

    /* U+0167 "ŧ" */
    0x5D, 0x2E, 0x93,

    /* U+0168 "Ũ" */
    0x38, 0xD0, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+0169 "ũ" */
    0x57, 0x81, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+016A "Ū" */
    0x78, 0x02, 0x0C, 0x18, 0x30, 0x60, 0xC1, 0x82, 0x88, 0xE0,

    /* U+016B "ū" */
    0xF0, 0x23, 0x18, 0xC6, 0x31, 0x78,

    /* U+016C "Ŭ" */
    0x28, 0x30, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+016D "ŭ" */
    0x53, 0x81, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+016E "Ů" */
    0x38, 0x50, 0xE0, 0x08, 0x30, 0x60, 0xC1, 0x83, 0x06, 0x0A, 0x23, 0x80,

    /* U+016F "ů" */
    0x32, 0x8C, 0x08, 0xC6, 0x31, 0x8C, 0x5E,

    /* U+0170 "Ű" */
    0x14, 0x50, 0x04, 0x18, 0x30, 0x60, 0xC1, 0x83, 0x05, 0x11, 0xC0,

    /* U+0171 "ű" */
    0x2A, 0x81, 0x18, 0xC6, 0x31, 0x8B, 0xC0,

    /* U+0172 "Ų" */
    0x83, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xA2, 0x38, 0x20, 0x60,

    /* U+0173 "ų" */
    0x8A, 0x28, 0xA2, 0x8A, 0x27, 0x82, 0x0C,

    /* U+0174 "Ŵ" */
    0x04, 0x01, 0x40, 0x00, 0x42, 0x18, 0xA3, 0x14, 0x92, 0x92, 0x8A, 0x51, 
    0x4A, 0x28, 0x82, 0x10, 0x40,

    /* U+0175 "ŵ" */
    0x08, 0x0A, 0x00, 0x11, 0x18, 0x8A, 0xA9, 0x54, 0xAA, 0x22, 0x11, 0x00,

    /* U+0176 "Ŷ" */
    0x20, 0xA0, 0x04, 0x14, 0x48, 0x8A, 0x08, 0x10, 0x20, 0x40, 0x80,

    /* U+0177 "ŷ" */
    0x22, 0x81, 0x18, 0xA9, 0x4A, 0x21, 0x08, 0x80,

    /* U+0178 "Ÿ" */
    0x28, 0x02, 0x0A, 0x24, 0x45, 0x04, 0x08, 0x10, 0x20, 0x40,

    /* U+0179 "Ź" */
    0x08, 0x20, 0x03, 0xF0, 0x41, 0x02, 0x08, 0x20, 0x41, 0x07, 0xF0,

    /* U+017A "ź" */
    0x11, 0x01, 0xF1, 0x08, 0x88, 0x47, 0xC0,

    /* U+017B "Ż" */
    0x10, 0x01, 0xF8, 0x20, 0x81, 0x04, 0x10, 0x20, 0x83, 0xF8,

    /* U+017C "ż" */
    0x20, 0x3E, 0x21, 0x11, 0x08, 0xF8,

    /* U+017D "Ž" */
    0x34, 0x30, 0x03, 0xF0, 0x41, 0x02, 0x08, 0x20, 0x41, 0x07, 0xF0,

    /* U+017E "ž" */
    0x51, 0x01, 0xF1, 0x08, 0x88, 0x47, 0xC0,

    /* U+017F "ſ" */
    0xEA, 0xAA, 0x80,
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
    {.bitmap_index = 523, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+00A0 */,
    {.bitmap_index = 524, .adv_w = 48, .box_w = 1, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+00A1 */,
    {.bitmap_index = 526, .adv_w = 112, .box_w = 5, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+00A2 */,
    {.bitmap_index = 534, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00A3 */,
    {.bitmap_index = 540, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+00A4 */,
    {.bitmap_index = 544, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00A5 */,
    {.bitmap_index = 552, .adv_w = 48, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+00A6 */,
    {.bitmap_index = 554, .adv_w = 112, .box_w = 6, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+00A7 */,
    {.bitmap_index = 563, .adv_w = 64, .box_w = 3, .box_h = 1, .ofs_x = 0, .ofs_y = 8} /* U+00A8 */,
    {.bitmap_index = 564, .adv_w = 144, .box_w = 9, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00A9 */,
    {.bitmap_index = 575, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00AA */,
    {.bitmap_index = 577, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+00AB */,
    {.bitmap_index = 581, .adv_w = 112, .box_w = 6, .box_h = 4, .ofs_x = 0, .ofs_y = 2} /* U+00AC */,
    {.bitmap_index = 584, .adv_w = 64, .box_w = 3, .box_h = 1, .ofs_x = 0, .ofs_y = 3} /* U+00AD */,
    {.bitmap_index = 585, .adv_w = 144, .box_w = 9, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00AE */,
    {.bitmap_index = 596, .adv_w = 112, .box_w = 7, .box_h = 1, .ofs_x = 0, .ofs_y = 9} /* U+00AF */,
    {.bitmap_index = 597, .adv_w = 80, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = 6} /* U+00B0 */,
    {.bitmap_index = 599, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00B1 */,
    {.bitmap_index = 604, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00B2 */,
    {.bitmap_index = 606, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00B3 */,
    {.bitmap_index = 608, .adv_w = 64, .box_w = 2, .box_h = 2, .ofs_x = 1, .ofs_y = 7} /* U+00B4 */,
    {.bitmap_index = 609, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+00B5 */,
    {.bitmap_index = 615, .adv_w = 96, .box_w = 6, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+00B6 */,
    {.bitmap_index = 624, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 1, .ofs_y = 4} /* U+00B7 */,
    {.bitmap_index = 625, .adv_w = 64, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = -3} /* U+00B8 */,
    {.bitmap_index = 627, .adv_w = 64, .box_w = 2, .box_h = 4, .ofs_x = 1, .ofs_y = 5} /* U+00B9 */,
    {.bitmap_index = 628, .adv_w = 64, .box_w = 4, .box_h = 4, .ofs_x = 0, .ofs_y = 5} /* U+00BA */,
    {.bitmap_index = 630, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 0} /* U+00BB */,
    {.bitmap_index = 634, .adv_w = 160, .box_w = 9, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00BC */,
    {.bitmap_index = 645, .adv_w = 160, .box_w = 9, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00BD */,
    {.bitmap_index = 656, .adv_w = 160, .box_w = 10, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00BE */,
    {.bitmap_index = 668, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+00BF */,
    {.bitmap_index = 674, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C0 */,
    {.bitmap_index = 685, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C1 */,
    {.bitmap_index = 696, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C2 */,
    {.bitmap_index = 707, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00C3 */,
    {.bitmap_index = 718, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+00C4 */,
    {.bitmap_index = 728, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+00C5 */,
    {.bitmap_index = 738, .adv_w = 192, .box_w = 12, .box_h = 9, .ofs_x = -1, .ofs_y = 0} /* U+00C6 */,
    {.bitmap_index = 752, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+00C7 */,
    {.bitmap_index = 763, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00C8 */,
    {.bitmap_index = 772, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00C9 */,
    {.bitmap_index = 781, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00CA */,
    {.bitmap_index = 790, .adv_w = 128, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00CB */,
    {.bitmap_index = 799, .adv_w = 48, .box_w = 2, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00CC */,
    {.bitmap_index = 802, .adv_w = 48, .box_w = 2, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00CD */,
    {.bitmap_index = 805, .adv_w = 48, .box_w = 5, .box_h = 12, .ofs_x = -1, .ofs_y = 0} /* U+00CE */,
    {.bitmap_index = 813, .adv_w = 48, .box_w = 3, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+00CF */,
    {.bitmap_index = 818, .adv_w = 144, .box_w = 8, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00D0 */,
    {.bitmap_index = 827, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D1 */,
    {.bitmap_index = 838, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D2 */,
    {.bitmap_index = 849, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D3 */,
    {.bitmap_index = 860, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D4 */,
    {.bitmap_index = 871, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D5 */,
    {.bitmap_index = 882, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00D6 */,
    {.bitmap_index = 892, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+00D7 */,
    {.bitmap_index = 896, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D8 */,
    {.bitmap_index = 904, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00D9 */,
    {.bitmap_index = 915, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00DA */,
    {.bitmap_index = 926, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+00DB */,
    {.bitmap_index = 937, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00DC */,
    {.bitmap_index = 947, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+00DD */,
    {.bitmap_index = 958, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DE */,
    {.bitmap_index = 965, .adv_w = 128, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DF */,
    {.bitmap_index = 972, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E0 */,
    {.bitmap_index = 979, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E1 */,
    {.bitmap_index = 986, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E2 */,
    {.bitmap_index = 993, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E3 */,
    {.bitmap_index = 1000, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00E4 */,
    {.bitmap_index = 1006, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+00E5 */,
    {.bitmap_index = 1013, .adv_w = 176, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00E6 */,
    {.bitmap_index = 1021, .adv_w = 96, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+00E7 */,
    {.bitmap_index = 1026, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E8 */,
    {.bitmap_index = 1033, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00E9 */,
    {.bitmap_index = 1040, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00EA */,
    {.bitmap_index = 1047, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00EB */,
    {.bitmap_index = 1053, .adv_w = 48, .box_w = 2, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+00EC */,
    {.bitmap_index = 1056, .adv_w = 48, .box_w = 2, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00ED */,
    {.bitmap_index = 1059, .adv_w = 48, .box_w = 4, .box_h = 10, .ofs_x = -1, .ofs_y = 0} /* U+00EE */,
    {.bitmap_index = 1064, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00EF */,
    {.bitmap_index = 1068, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00F0 */,
    {.bitmap_index = 1075, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F1 */,
    {.bitmap_index = 1082, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F2 */,
    {.bitmap_index = 1089, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F3 */,
    {.bitmap_index = 1096, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F4 */,
    {.bitmap_index = 1103, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F5 */,
    {.bitmap_index = 1110, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00F6 */,
    {.bitmap_index = 1116, .adv_w = 112, .box_w = 5, .box_h = 5, .ofs_x = 1, .ofs_y = 2} /* U+00F7 */,
    {.bitmap_index = 1120, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00F8 */,
    {.bitmap_index = 1125, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00F9 */,
    {.bitmap_index = 1132, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00FA */,
    {.bitmap_index = 1139, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+00FB */,
    {.bitmap_index = 1146, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00FC */,
    {.bitmap_index = 1152, .adv_w = 80, .box_w = 5, .box_h = 12, .ofs_x = 0, .ofs_y = -2} /* U+00FD */,
    {.bitmap_index = 1160, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+00FE */,
    {.bitmap_index = 1167, .adv_w = 80, .box_w = 5, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+00FF */,
    {.bitmap_index = 1174, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+0100 */,
    {.bitmap_index = 1184, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0101 */,
    {.bitmap_index = 1190, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0102 */,
    {.bitmap_index = 1201, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0103 */,
    {.bitmap_index = 1208, .adv_w = 112, .box_w = 8, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+0104 */,
    {.bitmap_index = 1219, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0105 */,
    {.bitmap_index = 1226, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0106 */,
    {.bitmap_index = 1237, .adv_w = 96, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0107 */,
    {.bitmap_index = 1242, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0108 */,
    {.bitmap_index = 1253, .adv_w = 96, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0109 */,
    {.bitmap_index = 1258, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+010A */,
    {.bitmap_index = 1268, .adv_w = 96, .box_w = 4, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+010B */,
    {.bitmap_index = 1273, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+010C */,
    {.bitmap_index = 1284, .adv_w = 96, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+010D */,
    {.bitmap_index = 1291, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+010E */,
    {.bitmap_index = 1302, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+010F */,
    {.bitmap_index = 1310, .adv_w = 144, .box_w = 8, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0110 */,
    {.bitmap_index = 1319, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0111 */,
    {.bitmap_index = 1326, .adv_w = 128, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+0112 */,
    {.bitmap_index = 1335, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0113 */,
    {.bitmap_index = 1341, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0114 */,
    {.bitmap_index = 1350, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0115 */,
    {.bitmap_index = 1357, .adv_w = 128, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+0116 */,
    {.bitmap_index = 1366, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0117 */,
    {.bitmap_index = 1372, .adv_w = 128, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+0118 */,
    {.bitmap_index = 1381, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0119 */,
    {.bitmap_index = 1387, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+011A */,
    {.bitmap_index = 1396, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+011B */,
    {.bitmap_index = 1403, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+011C */,
    {.bitmap_index = 1414, .adv_w = 112, .box_w = 5, .box_h = 12, .ofs_x = 1, .ofs_y = -2} /* U+011D */,
    {.bitmap_index = 1422, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+011E */,
    {.bitmap_index = 1433, .adv_w = 112, .box_w = 5, .box_h = 12, .ofs_x = 1, .ofs_y = -2} /* U+011F */,
    {.bitmap_index = 1441, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+0120 */,
    {.bitmap_index = 1451, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+0121 */,
    {.bitmap_index = 1458, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+0122 */,
    {.bitmap_index = 1469, .adv_w = 112, .box_w = 5, .box_h = 13, .ofs_x = 1, .ofs_y = -2} /* U+0123 */,
    {.bitmap_index = 1478, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0124 */,
    {.bitmap_index = 1489, .adv_w = 112, .box_w = 5, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0125 */,
    {.bitmap_index = 1497, .adv_w = 144, .box_w = 9, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0126 */,
    {.bitmap_index = 1508, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0127 */,
    {.bitmap_index = 1515, .adv_w = 48, .box_w = 4, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0128 */,
    {.bitmap_index = 1521, .adv_w = 48, .box_w = 4, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+0129 */,
    {.bitmap_index = 1526, .adv_w = 48, .box_w = 4, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+012A */,
    {.bitmap_index = 1532, .adv_w = 48, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+012B */,
    {.bitmap_index = 1537, .adv_w = 48, .box_w = 3, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+012C */,
    {.bitmap_index = 1542, .adv_w = 48, .box_w = 3, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+012D */,
    {.bitmap_index = 1546, .adv_w = 48, .box_w = 3, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+012E */,
    {.bitmap_index = 1551, .adv_w = 48, .box_w = 2, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+012F */,
    {.bitmap_index = 1554, .adv_w = 48, .box_w = 1, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+0130 */,
    {.bitmap_index = 1556, .adv_w = 48, .box_w = 1, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0131 */,
    {.bitmap_index = 1557, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0132 */,
    {.bitmap_index = 1565, .adv_w = 80, .box_w = 3, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+0133 */,
    {.bitmap_index = 1570, .adv_w = 96, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0134 */,
    {.bitmap_index = 1581, .adv_w = 48, .box_w = 5, .box_h = 12, .ofs_x = -1, .ofs_y = -2} /* U+0135 */,
    {.bitmap_index = 1589, .adv_w = 128, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = -4} /* U+0136 */,
    {.bitmap_index = 1601, .adv_w = 96, .box_w = 5, .box_h = 13, .ofs_x = 1, .ofs_y = -4} /* U+0137 */,
    {.bitmap_index = 1610, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0138 */,
    {.bitmap_index = 1615, .adv_w = 112, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0139 */,
    {.bitmap_index = 1624, .adv_w = 48, .box_w = 2, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+013A */,
    {.bitmap_index = 1627, .adv_w = 112, .box_w = 6, .box_h = 13, .ofs_x = 1, .ofs_y = -4} /* U+013B */,
    {.bitmap_index = 1637, .adv_w = 48, .box_w = 3, .box_h = 13, .ofs_x = 0, .ofs_y = -4} /* U+013C */,
    {.bitmap_index = 1642, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+013D */,
    {.bitmap_index = 1648, .adv_w = 64, .box_w = 3, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+013E */,
    {.bitmap_index = 1652, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+013F */,
    {.bitmap_index = 1659, .adv_w = 64, .box_w = 3, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0140 */,
    {.bitmap_index = 1663, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0141 */,
    {.bitmap_index = 1671, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0142 */,
    {.bitmap_index = 1675, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0143 */,
    {.bitmap_index = 1686, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0144 */,
    {.bitmap_index = 1693, .adv_w = 144, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = -4} /* U+0145 */,
    {.bitmap_index = 1705, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = -4} /* U+0146 */,
    {.bitmap_index = 1712, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0147 */,
    {.bitmap_index = 1723, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0148 */,
    {.bitmap_index = 1730, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0149 */,
    {.bitmap_index = 1737, .adv_w = 144, .box_w = 7, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+014A */,
    {.bitmap_index = 1745, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+014B */,
    {.bitmap_index = 1751, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+014C */,
    {.bitmap_index = 1761, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+014D */,
    {.bitmap_index = 1767, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+014E */,
    {.bitmap_index = 1778, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+014F */,
    {.bitmap_index = 1785, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0150 */,
    {.bitmap_index = 1796, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0151 */,
    {.bitmap_index = 1803, .adv_w = 192, .box_w = 10, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0152 */,
    {.bitmap_index = 1815, .adv_w = 176, .box_w = 9, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0153 */,
    {.bitmap_index = 1823, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0154 */,
    {.bitmap_index = 1834, .adv_w = 64, .box_w = 3, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0155 */,
    {.bitmap_index = 1838, .adv_w = 144, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = -4} /* U+0156 */,
    {.bitmap_index = 1850, .adv_w = 64, .box_w = 4, .box_h = 11, .ofs_x = 0, .ofs_y = -4} /* U+0157 */,
    {.bitmap_index = 1856, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0158 */,
    {.bitmap_index = 1867, .adv_w = 64, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+0159 */,
    {.bitmap_index = 1874, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+015A */,
    {.bitmap_index = 1883, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+015B */,
    {.bitmap_index = 1890, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+015C */,
    {.bitmap_index = 1899, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+015D */,
    {.bitmap_index = 1906, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = -3} /* U+015E */,
    {.bitmap_index = 1915, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+015F */,
    {.bitmap_index = 1922, .adv_w = 128, .box_w = 6, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0160 */,
    {.bitmap_index = 1931, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0161 */,
    {.bitmap_index = 1938, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = -3} /* U+0162 */,
    {.bitmap_index = 1949, .adv_w = 48, .box_w = 3, .box_h = 12, .ofs_x = 0, .ofs_y = -3} /* U+0163 */,
    {.bitmap_index = 1954, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0164 */,
    {.bitmap_index = 1965, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0165 */,
    {.bitmap_index = 1971, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0166 */,
    {.bitmap_index = 1979, .adv_w = 48, .box_w = 3, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0167 */,
    {.bitmap_index = 1982, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0168 */,
    {.bitmap_index = 1993, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0169 */,
    {.bitmap_index = 2000, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+016A */,
    {.bitmap_index = 2010, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+016B */,
    {.bitmap_index = 2016, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+016C */,
    {.bitmap_index = 2027, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+016D */,
    {.bitmap_index = 2034, .adv_w = 144, .box_w = 7, .box_h = 13, .ofs_x = 1, .ofs_y = 0} /* U+016E */,
    {.bitmap_index = 2046, .adv_w = 112, .box_w = 5, .box_h = 11, .ofs_x = 1, .ofs_y = 0} /* U+016F */,
    {.bitmap_index = 2053, .adv_w = 144, .box_w = 7, .box_h = 12, .ofs_x = 1, .ofs_y = 0} /* U+0170 */,
    {.bitmap_index = 2064, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+0171 */,
    {.bitmap_index = 2071, .adv_w = 144, .box_w = 7, .box_h = 11, .ofs_x = 1, .ofs_y = -2} /* U+0172 */,
    {.bitmap_index = 2081, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0173 */,
    {.bitmap_index = 2088, .adv_w = 176, .box_w = 11, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0174 */,
    {.bitmap_index = 2105, .adv_w = 144, .box_w = 9, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+0175 */,
    {.bitmap_index = 2117, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0176 */,
    {.bitmap_index = 2128, .adv_w = 80, .box_w = 5, .box_h = 12, .ofs_x = 0, .ofs_y = -2} /* U+0177 */,
    {.bitmap_index = 2136, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+0178 */,
    {.bitmap_index = 2146, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+0179 */,
    {.bitmap_index = 2157, .adv_w = 80, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+017A */,
    {.bitmap_index = 2164, .adv_w = 112, .box_w = 7, .box_h = 11, .ofs_x = 0, .ofs_y = 0} /* U+017B */,
    {.bitmap_index = 2174, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+017C */,
    {.bitmap_index = 2180, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0} /* U+017D */,
    {.bitmap_index = 2191, .adv_w = 80, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = 0} /* U+017E */,
    {.bitmap_index = 2198, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+017F */
};

static const lv_font_fmt_txt_cmap_t cmaps[] = {
    {.range_start = 32, .range_length = 95, .glyph_id_start = 1,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 160, .range_length = 224, .glyph_id_start = 96,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY}
};

static const lv_font_fmt_txt_dsc_t font_dsc = {
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 2,
    .bpp = 1,
    .kern_classes = 0,
    .bitmap_format = 0,
    .stride = 0,
};

const UG_FONT font_arial_12 = {
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,
    .line_height = 12,
    .base_line = 2,
    .subpx = LV_FONT_SUBPX_NONE,
    .underline_position = -1,
    .underline_thickness = 0,
    .static_bitmap = 0,
    .dsc = &font_dsc,
    .fallback = NULL,
    .user_data = NULL,
};

#endif /* ARIAL_12 */
