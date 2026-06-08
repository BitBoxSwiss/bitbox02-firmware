/*******************************************************************************
 * Size: 9 px
 * Bpp: 1
 * Opts: --bpp 1 --size 9 --dpi 72 --font /usr/share/fonts/truetype/msttcorefonts/Arial.ttf --range 32-126,160-383 --format lvgl -o arial_9.c
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

#include "arial_9.h"

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

    /* U+00A0 " " */
    0x00,

    /* U+00A1 "¡" */
    0xBE,

    /* U+00A2 "¢" */
    0x22, 0x6B, 0xAD, 0x64, 0x40,

    /* U+00A3 "£" */
    0x32, 0x51, 0xC4, 0x23, 0xE0,

    /* U+00A4 "¤" */
    0xF9, 0x9F,

    /* U+00A5 "¥" */
    0x8A, 0x95, 0xF2, 0x7C, 0x80,

    /* U+00A6 "¦" */
    0xE7,

    /* U+00A7 "§" */
    0xE9, 0x4A, 0x95, 0x29, 0x70,

    /* U+00A8 "¨" */
    0xA0,

    /* U+00A9 "©" */
    0x7D, 0x06, 0x6D, 0x19, 0xB0, 0x5F, 0x00,

    /* U+00AA "ª" */
    0xEF, 0x80,

    /* U+00AB "«" */
    0x2A, 0x94, 0x50,

    /* U+00AC "¬" */
    0xF1, 0x10,

    /* U+00AD "­" */
    0xC0,

    /* U+00AE "®" */
    0x7D, 0x06, 0xCD, 0xDA, 0xB0, 0x5F, 0x00,

    /* U+00AF "¯" */
    0xF8,

    /* U+00B0 "°" */
    0xF7, 0x80,

    /* U+00B1 "±" */
    0x21, 0x3E, 0x42, 0x7C,

    /* U+00B2 "²" */
    0xEB, 0x80,

    /* U+00B3 "³" */
    0xEB, 0x80,

    /* U+00B4 "´" */
    0x60,

    /* U+00B5 "µ" */
    0x99, 0x99, 0xF8, 0x80,

    /* U+00B6 "¶" */
    0x7E, 0xB5, 0xA5, 0x29, 0x4A,

    /* U+00B7 "·" */
    0x80,

    /* U+00B8 "¸" */
    0x5C,

    /* U+00B9 "¹" */
    0x74,

    /* U+00BA "º" */
    0xF7, 0x80,

    /* U+00BB "»" */
    0xA2, 0x95, 0x40,

    /* U+00BC "¼" */
    0x47, 0x25, 0x04, 0x25, 0x38, 0x40,

    /* U+00BD "½" */
    0x45, 0x91, 0x40, 0x82, 0xE8, 0xA3, 0x80,

    /* U+00BE "¾" */
    0xE2, 0x8B, 0xA0, 0x41, 0x24, 0xC8, 0x80,

    /* U+00BF "¿" */
    0x20, 0x26, 0x89, 0x60,

    /* U+00C0 "À" */
    0x10, 0x10, 0x41, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+00C1 "Á" */
    0x08, 0x20, 0x41, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+00C2 "Â" */
    0x18, 0x68, 0x41, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+00C3 "Ã" */
    0x28, 0xA0, 0x41, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+00C4 "Ä" */
    0x28, 0x20, 0xA1, 0x44, 0x4F, 0x91, 0x41,

    /* U+00C5 "Å" */
    0x38, 0x50, 0xE1, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+00C6 "Æ" */
    0x0F, 0xC5, 0x01, 0x40, 0x9F, 0x3C, 0x11, 0x04, 0x7C,

    /* U+00C7 "Ç" */
    0x74, 0x61, 0x08, 0x45, 0xC2, 0x11, 0x80,

    /* U+00C8 "È" */
    0x20, 0xBF, 0x08, 0x7E, 0x10, 0xF8,

    /* U+00C9 "É" */
    0x11, 0x3F, 0x08, 0x7E, 0x10, 0xF8,

    /* U+00CA "Ê" */
    0x22, 0xBF, 0x08, 0x7E, 0x10, 0xF8,

    /* U+00CB "Ë" */
    0x57, 0xE1, 0x0F, 0xC2, 0x1F,

    /* U+00CC "Ì" */
    0x9A, 0xAA, 0x80,

    /* U+00CD "Í" */
    0x65, 0x55, 0x40,

    /* U+00CE "Î" */
    0x22, 0x84, 0x21, 0x08, 0x42, 0x10,

    /* U+00CF "Ï" */
    0xA9, 0x24, 0x92,

    /* U+00D0 "Ð" */
    0x79, 0x14, 0x7D, 0x45, 0x17, 0x80,

    /* U+00D1 "Ñ" */
    0x55, 0x23, 0x9C, 0xD6, 0x73, 0x88,

    /* U+00D2 "Ò" */
    0x20, 0x9D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00D3 "Ó" */
    0x11, 0x1D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00D4 "Ô" */
    0x22, 0x9D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00D5 "Õ" */
    0x55, 0x1D, 0x18, 0xC6, 0x31, 0x70,

    /* U+00D6 "Ö" */
    0x53, 0xA3, 0x18, 0xC6, 0x2E,

    /* U+00D7 "×" */
    0x8B, 0x88, 0xE8, 0x80,

    /* U+00D8 "Ø" */
    0x7C, 0xA7, 0x5C, 0xA7, 0xC0,

    /* U+00D9 "Ù" */
    0x41, 0x23, 0x18, 0xC6, 0x31, 0x70,

    /* U+00DA "Ú" */
    0x22, 0x23, 0x18, 0xC6, 0x31, 0x70,

    /* U+00DB "Û" */
    0x22, 0xA3, 0x18, 0xC6, 0x31, 0x70,

    /* U+00DC "Ü" */
    0x54, 0x63, 0x18, 0xC6, 0x2E,

    /* U+00DD "Ý" */
    0x11, 0x22, 0xA5, 0x10, 0x84, 0x20,

    /* U+00DE "Þ" */
    0x87, 0xA3, 0x18, 0xFA, 0x00,

    /* U+00DF "ß" */
    0x4A, 0xAC, 0xBD, 0xA0,

    /* U+00E0 "à" */
    0x42, 0x0F, 0x17, 0x9F,

    /* U+00E1 "á" */
    0x24, 0x0F, 0x17, 0x9F,

    /* U+00E2 "â" */
    0x25, 0x0F, 0x17, 0x9F,

    /* U+00E3 "ã" */
    0x5A, 0x0F, 0x17, 0x9F,

    /* U+00E4 "ä" */
    0x50, 0xF1, 0x79, 0xF0,

    /* U+00E5 "å" */
    0x75, 0x70, 0xF1, 0x79, 0xF0,

    /* U+00E6 "æ" */
    0xEC, 0x25, 0xFC, 0x8E, 0xE0,

    /* U+00E7 "ç" */
    0x69, 0x89, 0x62, 0x26,

    /* U+00E8 "è" */
    0x42, 0x06, 0x9F, 0x87,

    /* U+00E9 "é" */
    0x24, 0x06, 0x9F, 0x87,

    /* U+00EA "ê" */
    0x25, 0x06, 0x9F, 0x87,

    /* U+00EB "ë" */
    0xA0, 0x69, 0xF8, 0x70,

    /* U+00EC "ì" */
    0x92, 0xAA,

    /* U+00ED "í" */
    0x61, 0x55,

    /* U+00EE "î" */
    0x65, 0x04, 0x44, 0x44,

    /* U+00EF "ï" */
    0xA1, 0x24, 0x90,

    /* U+00F0 "ð" */
    0x62, 0x79, 0x99, 0x60,

    /* U+00F1 "ñ" */
    0x5A, 0x0E, 0x99, 0x99,

    /* U+00F2 "ò" */
    0x42, 0x06, 0x99, 0x96,

    /* U+00F3 "ó" */
    0x24, 0x06, 0x99, 0x96,

    /* U+00F4 "ô" */
    0x25, 0x06, 0x99, 0x96,

    /* U+00F5 "õ" */
    0x5A, 0x06, 0x99, 0x96,

    /* U+00F6 "ö" */
    0xA0, 0x69, 0x99, 0x60,

    /* U+00F7 "÷" */
    0x20, 0x3E, 0x02, 0x00,

    /* U+00F8 "ø" */
    0x7B, 0xDD, 0xE0,

    /* U+00F9 "ù" */
    0x42, 0x09, 0x99, 0x97,

    /* U+00FA "ú" */
    0x24, 0x09, 0x99, 0x97,

    /* U+00FB "û" */
    0x22, 0x80, 0x94, 0xA5, 0x27,

    /* U+00FC "ü" */
    0x50, 0x99, 0x99, 0x70,

    /* U+00FD "ý" */
    0x11, 0x01, 0x15, 0x29, 0x44, 0x22, 0x00,

    /* U+00FE "þ" */
    0x88, 0xE9, 0x99, 0xE8, 0x80,

    /* U+00FF "ÿ" */
    0x50, 0x22, 0xA5, 0x28, 0x84, 0x40,

    /* U+0100 "Ā" */
    0x38, 0x20, 0xA1, 0x44, 0x4F, 0x91, 0x41,

    /* U+0101 "ā" */
    0x70, 0xF1, 0x79, 0xF0,

    /* U+0102 "Ă" */
    0x28, 0x70, 0x41, 0x42, 0x88, 0x9F, 0x22, 0x82,

    /* U+0103 "ă" */
    0x57, 0x0F, 0x17, 0x9F,

    /* U+0104 "Ą" */
    0x10, 0x28, 0x28, 0x44, 0x7C, 0x44, 0x82, 0x02, 0x03,

    /* U+0105 "ą" */
    0xF0, 0x9D, 0x2F, 0x08, 0x60,

    /* U+0106 "Ć" */
    0x11, 0x1D, 0x18, 0x42, 0x11, 0x70,

    /* U+0107 "ć" */
    0x24, 0x06, 0x98, 0x96,

    /* U+0108 "Ĉ" */
    0x22, 0x9D, 0x18, 0x42, 0x11, 0x70,

    /* U+0109 "ĉ" */
    0x22, 0x80, 0xC9, 0x42, 0x4C,

    /* U+010A "Ċ" */
    0x23, 0xA3, 0x08, 0x42, 0x2E,

    /* U+010B "ċ" */
    0x20, 0x69, 0x89, 0x60,

    /* U+010C "Č" */
    0x51, 0x1D, 0x18, 0x42, 0x11, 0x70,

    /* U+010D "č" */
    0x52, 0x06, 0x98, 0x96,

    /* U+010E "Ď" */
    0x50, 0x87, 0x91, 0x45, 0x14, 0x51, 0x78,

    /* U+010F "ď" */
    0x14, 0x57, 0x24, 0x92, 0x47, 0x00,

    /* U+0110 "Đ" */
    0x79, 0x14, 0x7D, 0x45, 0x17, 0x80,

    /* U+0111 "đ" */
    0x38, 0x9D, 0x29, 0x49, 0xC0,

    /* U+0112 "Ē" */
    0x3F, 0xE1, 0x0F, 0xC2, 0x1F,

    /* U+0113 "ē" */
    0x70, 0x69, 0xF8, 0x70,

    /* U+0114 "Ĕ" */
    0x53, 0xBF, 0x08, 0x7E, 0x10, 0xF8,

    /* U+0115 "ĕ" */
    0x57, 0x06, 0x9F, 0x87,

    /* U+0116 "Ė" */
    0x17, 0xE1, 0x0F, 0xC2, 0x1F,

    /* U+0117 "ė" */
    0x20, 0x69, 0xF8, 0x70,

    /* U+0118 "Ę" */
    0xFC, 0x21, 0xF8, 0x43, 0xE2, 0x18,

    /* U+0119 "ę" */
    0x69, 0xF8, 0x72, 0x30,

    /* U+011A "Ě" */
    0x51, 0x3F, 0x08, 0x7E, 0x10, 0xF8,

    /* U+011B "ě" */
    0x51, 0x00, 0x64, 0xBD, 0x07,

    /* U+011C "Ĝ" */
    0x22, 0x9D, 0x18, 0x4E, 0x31, 0x70,

    /* U+011D "ĝ" */
    0x65, 0x07, 0x99, 0x97, 0x96,

    /* U+011E "Ğ" */
    0x53, 0x9D, 0x18, 0x4E, 0x31, 0x70,

    /* U+011F "ğ" */
    0xAE, 0x07, 0x99, 0x97, 0x96,

    /* U+0120 "Ġ" */
    0x23, 0xA3, 0x09, 0xC6, 0x2E,

    /* U+0121 "ġ" */
    0x20, 0x79, 0x99, 0x79, 0x60,

    /* U+0122 "Ģ" */
    0x74, 0x61, 0x38, 0xC5, 0xC4, 0x23, 0x00,

    /* U+0123 "ģ" */
    0x22, 0x20, 0x79, 0x99, 0x79, 0x60,

    /* U+0124 "Ĥ" */
    0x22, 0xA3, 0x18, 0xFE, 0x31, 0x88,

    /* U+0125 "ĥ" */
    0x22, 0x90, 0x87, 0x25, 0x29, 0x48,

    /* U+0126 "Ħ" */
    0x45, 0xFD, 0x13, 0xE4, 0x48, 0x91, 0x00,

    /* U+0127 "ħ" */
    0xE8, 0xE9, 0x99, 0x90,

    /* U+0128 "Ĩ" */
    0xFA, 0x44, 0x44, 0x44, 0x40,

    /* U+0129 "ĩ" */
    0xFA, 0x04, 0x44, 0x44,

    /* U+012A "Ī" */
    0xF2, 0x49, 0x24,

    /* U+012B "ī" */
    0xE1, 0x24, 0x90,

    /* U+012C "Ĭ" */
    0xB9, 0x24, 0x92, 0x40,

    /* U+012D "ĭ" */
    0xB8, 0x24, 0x92,

    /* U+012E "Į" */
    0xAA, 0xAA, 0xC0,

    /* U+012F "į" */
    0x8A, 0xAA, 0xC0,

    /* U+0130 "İ" */
    0xFF,

    /* U+0131 "ı" */
    0xF8,

    /* U+0132 "Ĳ" */
    0x86, 0x18, 0x61, 0x86, 0x9B, 0x80,

    /* U+0133 "ĳ" */
    0xA2, 0xDB, 0x69, 0x40,

    /* U+0134 "Ĵ" */
    0x11, 0x44, 0x21, 0x08, 0x52, 0xE0,

    /* U+0135 "ĵ" */
    0x22, 0x80, 0x42, 0x10, 0x84, 0x22, 0x00,

    /* U+0136 "Ķ" */
    0x8C, 0xA9, 0x8A, 0x4A, 0x26, 0x11, 0x80,

    /* U+0137 "ķ" */
    0x88, 0x9A, 0xEA, 0x96, 0x26,

    /* U+0138 "ĸ" */
    0x95, 0x31, 0x49, 0x00,

    /* U+0139 "Ĺ" */
    0x24, 0x88, 0x88, 0x88, 0xF0,

    /* U+013A "ĺ" */
    0x65, 0x55, 0x40,

    /* U+013B "Ļ" */
    0x88, 0x88, 0x88, 0xF6, 0x26,

    /* U+013C "ļ" */
    0x92, 0x49, 0x22, 0x28,

    /* U+013D "Ľ" */
    0xB6, 0x49, 0x38,

    /* U+013E "ľ" */
    0xB6, 0x49, 0x20,

    /* U+013F "Ŀ" */
    0x88, 0x8A, 0x88, 0xF0,

    /* U+0140 "ŀ" */
    0x92, 0x59, 0x20,

    /* U+0141 "Ł" */
    0x42, 0x1D, 0x84, 0x21, 0xE0,

    /* U+0142 "ł" */
    0x49, 0xE4, 0x90,

    /* U+0143 "Ń" */
    0x22, 0x23, 0x9C, 0xD6, 0x73, 0x88,

    /* U+0144 "ń" */
    0x24, 0x0E, 0x99, 0x99,

    /* U+0145 "Ņ" */
    0x8E, 0x73, 0x59, 0xCE, 0x26, 0x11, 0x80,

    /* U+0146 "ņ" */
    0xE9, 0x99, 0x96, 0x26,

    /* U+0147 "Ň" */
    0x29, 0xA3, 0x9C, 0xD6, 0x73, 0x88,

    /* U+0148 "ň" */
    0x51, 0x01, 0xC9, 0x4A, 0x52,

    /* U+0149 "ŉ" */
    0x84, 0x1C, 0x94, 0xA5, 0x20,

    /* U+014A "Ŋ" */
    0xB6, 0x63, 0x18, 0xC6, 0xC0,

    /* U+014B "ŋ" */
    0xE9, 0x99, 0x91, 0x30,

    /* U+014C "Ō" */
    0x73, 0xA3, 0x18, 0xC6, 0x2E,

    /* U+014D "ō" */
    0x70, 0x69, 0x99, 0x60,

    /* U+014E "Ŏ" */
    0x53, 0x9D, 0x18, 0xC6, 0x31, 0x70,

    /* U+014F "ŏ" */
    0x53, 0x06, 0x99, 0x96,

    /* U+0150 "Ő" */
    0x2A, 0x9D, 0x18, 0xC6, 0x31, 0x70,

    /* U+0151 "ő" */
    0x5A, 0x06, 0x99, 0x96,

    /* U+0152 "Œ" */
    0x7F, 0x88, 0x88, 0x8F, 0x88, 0x88, 0x7F,

    /* U+0153 "œ" */
    0x76, 0x89, 0x8F, 0x88, 0x77,

    /* U+0154 "Ŕ" */
    0x22, 0x3D, 0x18, 0xFA, 0x51, 0x88,

    /* U+0155 "ŕ" */
    0x28, 0x79, 0x24,

    /* U+0156 "Ŗ" */
    0xF4, 0x63, 0xE9, 0x46, 0x26, 0x11, 0x80,

    /* U+0157 "ŗ" */
    0xF2, 0x49, 0x96,

    /* U+0158 "Ř" */
    0x50, 0x87, 0x91, 0x45, 0xE4, 0x91, 0x44,

    /* U+0159 "ř" */
    0x51, 0x00, 0xE4, 0x21, 0x08,

    /* U+015A "Ś" */
    0x24, 0x69, 0x86, 0x19, 0x60,

    /* U+015B "ś" */
    0x24, 0x07, 0x86, 0x1E,

    /* U+015C "Ŝ" */
    0x6D, 0x69, 0x86, 0x19, 0x60,

    /* U+015D "ŝ" */
    0x65, 0x07, 0x86, 0x1E,

    /* U+015E "Ş" */
    0x69, 0x86, 0x19, 0x64, 0x4C,

    /* U+015F "ş" */
    0x78, 0x61, 0xE4, 0x4C,

    /* U+0160 "Š" */
    0x51, 0x0C, 0x94, 0x18, 0x29, 0x30,

    /* U+0161 "š" */
    0x51, 0x00, 0x74, 0x18, 0x2E,

    /* U+0162 "Ţ" */
    0xF9, 0x08, 0x42, 0x10, 0x80, 0x21, 0x00,

    /* U+0163 "ţ" */
    0x0B, 0xA4, 0x98, 0x24,

    /* U+0164 "Ť" */
    0x51, 0x3E, 0x42, 0x10, 0x84, 0x20,

    /* U+0165 "ť" */
    0x15, 0xE4, 0x44, 0x60,

    /* U+0166 "Ŧ" */
    0xF9, 0x08, 0xE2, 0x10, 0x80,

    /* U+0167 "ŧ" */
    0x5D, 0x74, 0xC0,

    /* U+0168 "Ũ" */
    0x2A, 0xA3, 0x18, 0xC6, 0x31, 0x70,

    /* U+0169 "ũ" */
    0x5A, 0x09, 0x99, 0x97,

    /* U+016A "Ū" */
    0x74, 0x63, 0x18, 0xC6, 0x2E,

    /* U+016B "ū" */
    0x70, 0x99, 0x99, 0x70,

    /* U+016C "Ŭ" */
    0x53, 0x23, 0x18, 0xC6, 0x31, 0x70,

    /* U+016D "ŭ" */
    0x57, 0x09, 0x99, 0x97,

    /* U+016E "Ů" */
    0x72, 0x9D, 0x18, 0xC6, 0x31, 0x8B, 0x80,

    /* U+016F "ů" */
    0x75, 0x70, 0x99, 0x99, 0x70,

    /* U+0170 "Ű" */
    0x2A, 0xA3, 0x18, 0xC6, 0x31, 0x70,

    /* U+0171 "ű" */
    0x5A, 0x09, 0x99, 0x97,

    /* U+0172 "Ų" */
    0x8C, 0x63, 0x18, 0xC5, 0xC4, 0x30,

    /* U+0173 "ų" */
    0x94, 0xA5, 0x27, 0x08, 0x60,

    /* U+0174 "Ŵ" */
    0x08, 0x0A, 0x22, 0x32, 0x95, 0x52, 0xA9, 0x54, 0xAA, 0x22, 0x00,

    /* U+0175 "ŵ" */
    0x22, 0x81, 0x5A, 0xD6, 0xAA,

    /* U+0176 "Ŷ" */
    0x21, 0x44, 0x4A, 0x28, 0x41, 0x04, 0x10,

    /* U+0177 "ŷ" */
    0x22, 0x81, 0x15, 0x29, 0x44, 0x22, 0x00,

    /* U+0178 "Ÿ" */
    0x54, 0x54, 0xA2, 0x10, 0x84,

    /* U+0179 "Ź" */
    0x10, 0x8F, 0xC2, 0x10, 0x84, 0x20, 0xFC,

    /* U+017A "ź" */
    0x50, 0x72, 0xA7,

    /* U+017B "Ż" */
    0x13, 0xF0, 0x84, 0x21, 0x08, 0x3F,

    /* U+017C "ż" */
    0x43, 0x95, 0x38,

    /* U+017D "Ž" */
    0x28, 0x6F, 0xC2, 0x10, 0x84, 0x20, 0xFC,

    /* U+017E "ž" */
    0x56, 0x0E, 0x24, 0x8E,

    /* U+017F "ſ" */
    0xEA, 0xA8,
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
    {.bitmap_index = 344, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+00A0 */,
    {.bitmap_index = 345, .adv_w = 48, .box_w = 1, .box_h = 7, .ofs_x = 1, .ofs_y = -2} /* U+00A1 */,
    {.bitmap_index = 346, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+00A2 */,
    {.bitmap_index = 351, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00A3 */,
    {.bitmap_index = 356, .adv_w = 80, .box_w = 4, .box_h = 4, .ofs_x = 1, .ofs_y = 1} /* U+00A4 */,
    {.bitmap_index = 358, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00A5 */,
    {.bitmap_index = 363, .adv_w = 48, .box_w = 1, .box_h = 8, .ofs_x = 1, .ofs_y = -1} /* U+00A6 */,
    {.bitmap_index = 364, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+00A7 */,
    {.bitmap_index = 369, .adv_w = 48, .box_w = 3, .box_h = 1, .ofs_x = 0, .ofs_y = 6} /* U+00A8 */,
    {.bitmap_index = 370, .adv_w = 112, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00A9 */,
    {.bitmap_index = 377, .adv_w = 48, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+00AA */,
    {.bitmap_index = 379, .adv_w = 80, .box_w = 5, .box_h = 4, .ofs_x = 0, .ofs_y = 0} /* U+00AB */,
    {.bitmap_index = 382, .adv_w = 80, .box_w = 4, .box_h = 3, .ofs_x = 0, .ofs_y = 2} /* U+00AC */,
    {.bitmap_index = 384, .adv_w = 48, .box_w = 2, .box_h = 1, .ofs_x = 0, .ofs_y = 2} /* U+00AD */,
    {.bitmap_index = 385, .adv_w = 112, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00AE */,
    {.bitmap_index = 392, .adv_w = 80, .box_w = 5, .box_h = 1, .ofs_x = 0, .ofs_y = 7} /* U+00AF */,
    {.bitmap_index = 393, .adv_w = 64, .box_w = 3, .box_h = 3, .ofs_x = 1, .ofs_y = 4} /* U+00B0 */,
    {.bitmap_index = 395, .adv_w = 80, .box_w = 5, .box_h = 6, .ofs_x = 0, .ofs_y = 0} /* U+00B1 */,
    {.bitmap_index = 399, .adv_w = 48, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+00B2 */,
    {.bitmap_index = 401, .adv_w = 48, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+00B3 */,
    {.bitmap_index = 403, .adv_w = 48, .box_w = 2, .box_h = 2, .ofs_x = 1, .ofs_y = 5} /* U+00B4 */,
    {.bitmap_index = 404, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+00B5 */,
    {.bitmap_index = 408, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = -1} /* U+00B6 */,
    {.bitmap_index = 413, .adv_w = 48, .box_w = 1, .box_h = 1, .ofs_x = 1, .ofs_y = 3} /* U+00B7 */,
    {.bitmap_index = 414, .adv_w = 48, .box_w = 2, .box_h = 3, .ofs_x = 0, .ofs_y = -3} /* U+00B8 */,
    {.bitmap_index = 415, .adv_w = 48, .box_w = 2, .box_h = 3, .ofs_x = 1, .ofs_y = 4} /* U+00B9 */,
    {.bitmap_index = 416, .adv_w = 48, .box_w = 3, .box_h = 3, .ofs_x = 0, .ofs_y = 4} /* U+00BA */,
    {.bitmap_index = 418, .adv_w = 80, .box_w = 5, .box_h = 4, .ofs_x = 0, .ofs_y = 0} /* U+00BB */,
    {.bitmap_index = 421, .adv_w = 128, .box_w = 6, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00BC */,
    {.bitmap_index = 427, .adv_w = 128, .box_w = 7, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00BD */,
    {.bitmap_index = 434, .adv_w = 128, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00BE */,
    {.bitmap_index = 441, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = -2} /* U+00BF */,
    {.bitmap_index = 445, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00C0 */,
    {.bitmap_index = 453, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00C1 */,
    {.bitmap_index = 461, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00C2 */,
    {.bitmap_index = 469, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00C3 */,
    {.bitmap_index = 477, .adv_w = 96, .box_w = 7, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00C4 */,
    {.bitmap_index = 484, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00C5 */,
    {.bitmap_index = 492, .adv_w = 144, .box_w = 10, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+00C6 */,
    {.bitmap_index = 501, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+00C7 */,
    {.bitmap_index = 508, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00C8 */,
    {.bitmap_index = 514, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00C9 */,
    {.bitmap_index = 520, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00CA */,
    {.bitmap_index = 526, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+00CB */,
    {.bitmap_index = 531, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00CC */,
    {.bitmap_index = 534, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00CD */,
    {.bitmap_index = 537, .adv_w = 48, .box_w = 5, .box_h = 9, .ofs_x = -2, .ofs_y = 0} /* U+00CE */,
    {.bitmap_index = 543, .adv_w = 48, .box_w = 3, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00CF */,
    {.bitmap_index = 546, .adv_w = 112, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00D0 */,
    {.bitmap_index = 552, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D1 */,
    {.bitmap_index = 558, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D2 */,
    {.bitmap_index = 564, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D3 */,
    {.bitmap_index = 570, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D4 */,
    {.bitmap_index = 576, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D5 */,
    {.bitmap_index = 582, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+00D6 */,
    {.bitmap_index = 587, .adv_w = 80, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 1} /* U+00D7 */,
    {.bitmap_index = 591, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00D8 */,
    {.bitmap_index = 596, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00D9 */,
    {.bitmap_index = 602, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DA */,
    {.bitmap_index = 608, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DB */,
    {.bitmap_index = 614, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+00DC */,
    {.bitmap_index = 619, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+00DD */,
    {.bitmap_index = 625, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00DE */,
    {.bitmap_index = 630, .adv_w = 96, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+00DF */,
    {.bitmap_index = 634, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E0 */,
    {.bitmap_index = 638, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E1 */,
    {.bitmap_index = 642, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E2 */,
    {.bitmap_index = 646, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E3 */,
    {.bitmap_index = 650, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00E4 */,
    {.bitmap_index = 654, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+00E5 */,
    {.bitmap_index = 659, .adv_w = 128, .box_w = 7, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+00E6 */,
    {.bitmap_index = 664, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = -3} /* U+00E7 */,
    {.bitmap_index = 668, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E8 */,
    {.bitmap_index = 672, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00E9 */,
    {.bitmap_index = 676, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00EA */,
    {.bitmap_index = 680, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00EB */,
    {.bitmap_index = 684, .adv_w = 32, .box_w = 2, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00EC */,
    {.bitmap_index = 686, .adv_w = 32, .box_w = 2, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+00ED */,
    {.bitmap_index = 688, .adv_w = 32, .box_w = 4, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+00EE */,
    {.bitmap_index = 692, .adv_w = 32, .box_w = 3, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+00EF */,
    {.bitmap_index = 695, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00F0 */,
    {.bitmap_index = 699, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F1 */,
    {.bitmap_index = 703, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F2 */,
    {.bitmap_index = 707, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F3 */,
    {.bitmap_index = 711, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F4 */,
    {.bitmap_index = 715, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F5 */,
    {.bitmap_index = 719, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00F6 */,
    {.bitmap_index = 723, .adv_w = 80, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+00F7 */,
    {.bitmap_index = 727, .adv_w = 80, .box_w = 4, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+00F8 */,
    {.bitmap_index = 730, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00F9 */,
    {.bitmap_index = 734, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+00FA */,
    {.bitmap_index = 738, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+00FB */,
    {.bitmap_index = 743, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+00FC */,
    {.bitmap_index = 747, .adv_w = 96, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = -2} /* U+00FD */,
    {.bitmap_index = 754, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+00FE */,
    {.bitmap_index = 759, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+00FF */,
    {.bitmap_index = 765, .adv_w = 96, .box_w = 7, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0100 */,
    {.bitmap_index = 772, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0101 */,
    {.bitmap_index = 776, .adv_w = 96, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0102 */,
    {.bitmap_index = 784, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0103 */,
    {.bitmap_index = 788, .adv_w = 96, .box_w = 8, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0104 */,
    {.bitmap_index = 797, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0105 */,
    {.bitmap_index = 802, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0106 */,
    {.bitmap_index = 808, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0107 */,
    {.bitmap_index = 812, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0108 */,
    {.bitmap_index = 818, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0109 */,
    {.bitmap_index = 823, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+010A */,
    {.bitmap_index = 828, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+010B */,
    {.bitmap_index = 832, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+010C */,
    {.bitmap_index = 838, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+010D */,
    {.bitmap_index = 842, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+010E */,
    {.bitmap_index = 849, .adv_w = 96, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+010F */,
    {.bitmap_index = 855, .adv_w = 112, .box_w = 6, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0110 */,
    {.bitmap_index = 861, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0111 */,
    {.bitmap_index = 866, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+0112 */,
    {.bitmap_index = 871, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0113 */,
    {.bitmap_index = 875, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0114 */,
    {.bitmap_index = 881, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0115 */,
    {.bitmap_index = 885, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+0116 */,
    {.bitmap_index = 890, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0117 */,
    {.bitmap_index = 894, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0118 */,
    {.bitmap_index = 900, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0119 */,
    {.bitmap_index = 904, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+011A */,
    {.bitmap_index = 910, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+011B */,
    {.bitmap_index = 915, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+011C */,
    {.bitmap_index = 921, .adv_w = 80, .box_w = 4, .box_h = 10, .ofs_x = 0, .ofs_y = -2} /* U+011D */,
    {.bitmap_index = 926, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+011E */,
    {.bitmap_index = 932, .adv_w = 80, .box_w = 4, .box_h = 10, .ofs_x = 0, .ofs_y = -2} /* U+011F */,
    {.bitmap_index = 937, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+0120 */,
    {.bitmap_index = 942, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0121 */,
    {.bitmap_index = 947, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+0122 */,
    {.bitmap_index = 954, .adv_w = 80, .box_w = 4, .box_h = 11, .ofs_x = 0, .ofs_y = -2} /* U+0123 */,
    {.bitmap_index = 960, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0124 */,
    {.bitmap_index = 966, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = -1, .ofs_y = 0} /* U+0125 */,
    {.bitmap_index = 972, .adv_w = 112, .box_w = 7, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0126 */,
    {.bitmap_index = 979, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0127 */,
    {.bitmap_index = 983, .adv_w = 48, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0128 */,
    {.bitmap_index = 988, .adv_w = 32, .box_w = 4, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+0129 */,
    {.bitmap_index = 992, .adv_w = 48, .box_w = 3, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+012A */,
    {.bitmap_index = 995, .adv_w = 32, .box_w = 3, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+012B */,
    {.bitmap_index = 998, .adv_w = 48, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+012C */,
    {.bitmap_index = 1002, .adv_w = 32, .box_w = 3, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+012D */,
    {.bitmap_index = 1005, .adv_w = 48, .box_w = 2, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+012E */,
    {.bitmap_index = 1008, .adv_w = 32, .box_w = 2, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+012F */,
    {.bitmap_index = 1011, .adv_w = 48, .box_w = 1, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+0130 */,
    {.bitmap_index = 1012, .adv_w = 32, .box_w = 1, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0131 */,
    {.bitmap_index = 1013, .adv_w = 112, .box_w = 6, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0132 */,
    {.bitmap_index = 1019, .adv_w = 64, .box_w = 3, .box_h = 9, .ofs_x = 0, .ofs_y = -2} /* U+0133 */,
    {.bitmap_index = 1023, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0134 */,
    {.bitmap_index = 1029, .adv_w = 32, .box_w = 5, .box_h = 10, .ofs_x = -2, .ofs_y = -2} /* U+0135 */,
    {.bitmap_index = 1036, .adv_w = 96, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+0136 */,
    {.bitmap_index = 1043, .adv_w = 80, .box_w = 4, .box_h = 10, .ofs_x = 0, .ofs_y = -3} /* U+0137 */,
    {.bitmap_index = 1048, .adv_w = 80, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0138 */,
    {.bitmap_index = 1052, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0139 */,
    {.bitmap_index = 1057, .adv_w = 32, .box_w = 2, .box_h = 9, .ofs_x = -1, .ofs_y = 0} /* U+013A */,
    {.bitmap_index = 1060, .adv_w = 80, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+013B */,
    {.bitmap_index = 1065, .adv_w = 32, .box_w = 3, .box_h = 10, .ofs_x = 0, .ofs_y = -3} /* U+013C */,
    {.bitmap_index = 1069, .adv_w = 80, .box_w = 3, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+013D */,
    {.bitmap_index = 1072, .adv_w = 48, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+013E */,
    {.bitmap_index = 1075, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+013F */,
    {.bitmap_index = 1079, .adv_w = 48, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0140 */,
    {.bitmap_index = 1082, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0141 */,
    {.bitmap_index = 1087, .adv_w = 32, .box_w = 3, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+0142 */,
    {.bitmap_index = 1090, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0143 */,
    {.bitmap_index = 1096, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0144 */,
    {.bitmap_index = 1100, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+0145 */,
    {.bitmap_index = 1107, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = -3} /* U+0146 */,
    {.bitmap_index = 1111, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0147 */,
    {.bitmap_index = 1117, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0148 */,
    {.bitmap_index = 1122, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0149 */,
    {.bitmap_index = 1127, .adv_w = 112, .box_w = 5, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+014A */,
    {.bitmap_index = 1132, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+014B */,
    {.bitmap_index = 1136, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+014C */,
    {.bitmap_index = 1141, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+014D */,
    {.bitmap_index = 1145, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+014E */,
    {.bitmap_index = 1151, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+014F */,
    {.bitmap_index = 1155, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0150 */,
    {.bitmap_index = 1161, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0151 */,
    {.bitmap_index = 1165, .adv_w = 144, .box_w = 8, .box_h = 7, .ofs_x = 1, .ofs_y = 0} /* U+0152 */,
    {.bitmap_index = 1172, .adv_w = 144, .box_w = 8, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0153 */,
    {.bitmap_index = 1177, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0154 */,
    {.bitmap_index = 1183, .adv_w = 48, .box_w = 3, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0155 */,
    {.bitmap_index = 1186, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+0156 */,
    {.bitmap_index = 1193, .adv_w = 48, .box_w = 3, .box_h = 8, .ofs_x = 0, .ofs_y = -3} /* U+0157 */,
    {.bitmap_index = 1196, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0158 */,
    {.bitmap_index = 1203, .adv_w = 48, .box_w = 5, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+0159 */,
    {.bitmap_index = 1208, .adv_w = 96, .box_w = 4, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+015A */,
    {.bitmap_index = 1213, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+015B */,
    {.bitmap_index = 1217, .adv_w = 96, .box_w = 4, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+015C */,
    {.bitmap_index = 1222, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+015D */,
    {.bitmap_index = 1226, .adv_w = 96, .box_w = 4, .box_h = 10, .ofs_x = 1, .ofs_y = -3} /* U+015E */,
    {.bitmap_index = 1231, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = -3} /* U+015F */,
    {.bitmap_index = 1235, .adv_w = 96, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0160 */,
    {.bitmap_index = 1241, .adv_w = 80, .box_w = 5, .box_h = 8, .ofs_x = -1, .ofs_y = 0} /* U+0161 */,
    {.bitmap_index = 1246, .adv_w = 80, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = -3} /* U+0162 */,
    {.bitmap_index = 1253, .adv_w = 32, .box_w = 3, .box_h = 10, .ofs_x = -1, .ofs_y = -3} /* U+0163 */,
    {.bitmap_index = 1257, .adv_w = 80, .box_w = 5, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0164 */,
    {.bitmap_index = 1263, .adv_w = 48, .box_w = 4, .box_h = 7, .ofs_x = -1, .ofs_y = 0} /* U+0165 */,
    {.bitmap_index = 1267, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0166 */,
    {.bitmap_index = 1272, .adv_w = 48, .box_w = 3, .box_h = 6, .ofs_x = -1, .ofs_y = 0} /* U+0167 */,
    {.bitmap_index = 1275, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0168 */,
    {.bitmap_index = 1281, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0169 */,
    {.bitmap_index = 1285, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+016A */,
    {.bitmap_index = 1290, .adv_w = 80, .box_w = 4, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+016B */,
    {.bitmap_index = 1294, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+016C */,
    {.bitmap_index = 1300, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+016D */,
    {.bitmap_index = 1304, .adv_w = 112, .box_w = 5, .box_h = 10, .ofs_x = 1, .ofs_y = 0} /* U+016E */,
    {.bitmap_index = 1311, .adv_w = 80, .box_w = 4, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+016F */,
    {.bitmap_index = 1316, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0} /* U+0170 */,
    {.bitmap_index = 1322, .adv_w = 80, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0171 */,
    {.bitmap_index = 1326, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = -2} /* U+0172 */,
    {.bitmap_index = 1332, .adv_w = 80, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = -2} /* U+0173 */,
    {.bitmap_index = 1337, .adv_w = 144, .box_w = 9, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0174 */,
    {.bitmap_index = 1348, .adv_w = 96, .box_w = 5, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+0175 */,
    {.bitmap_index = 1353, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0176 */,
    {.bitmap_index = 1360, .adv_w = 96, .box_w = 5, .box_h = 10, .ofs_x = 0, .ofs_y = -2} /* U+0177 */,
    {.bitmap_index = 1367, .adv_w = 112, .box_w = 5, .box_h = 8, .ofs_x = 1, .ofs_y = 0} /* U+0178 */,
    {.bitmap_index = 1372, .adv_w = 96, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+0179 */,
    {.bitmap_index = 1379, .adv_w = 64, .box_w = 3, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+017A */,
    {.bitmap_index = 1382, .adv_w = 96, .box_w = 6, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+017B */,
    {.bitmap_index = 1388, .adv_w = 64, .box_w = 3, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+017C */,
    {.bitmap_index = 1391, .adv_w = 96, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0} /* U+017D */,
    {.bitmap_index = 1398, .adv_w = 64, .box_w = 4, .box_h = 8, .ofs_x = 0, .ofs_y = 0} /* U+017E */,
    {.bitmap_index = 1402, .adv_w = 32, .box_w = 2, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+017F */
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
