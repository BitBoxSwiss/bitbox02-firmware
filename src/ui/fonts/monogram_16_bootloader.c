/*******************************************************************************
 * Size: 16 px
 * Bpp: 1
 * Opts: --bpp 1 --size 16 --font src/ui/fonts/monogram.ttf --range 32,48-57,97-102 --format lvgl -o src/ui/fonts/monogram_16_bootloader.c
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

#include "monogram_16_bootloader.h"

#ifndef MONOGRAM_16
#define MONOGRAM_16 1
#endif

#if MONOGRAM_16

static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+0020 " " */
    0x00,

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
};

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 96, .box_w = 1, .box_h = 1, .ofs_x = 0, .ofs_y = 0} /* U+0020 */,
    {.bitmap_index = 1, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0030 */,
    {.bitmap_index = 6, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0031 */,
    {.bitmap_index = 11, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0032 */,
    {.bitmap_index = 16, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0033 */,
    {.bitmap_index = 21, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0034 */,
    {.bitmap_index = 26, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0035 */,
    {.bitmap_index = 31, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0036 */,
    {.bitmap_index = 36, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0037 */,
    {.bitmap_index = 41, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0038 */,
    {.bitmap_index = 46, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0039 */,
    {.bitmap_index = 51, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0061 */,
    {.bitmap_index = 55, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0062 */,
    {.bitmap_index = 60, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0063 */,
    {.bitmap_index = 64, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0064 */,
    {.bitmap_index = 69, .adv_w = 96, .box_w = 5, .box_h = 5, .ofs_x = 0, .ofs_y = 0} /* U+0065 */,
    {.bitmap_index = 73, .adv_w = 96, .box_w = 5, .box_h = 7, .ofs_x = 0, .ofs_y = 0} /* U+0066 */
};

static const lv_font_fmt_txt_cmap_t cmaps[] = {
    {.range_start = 32, .range_length = 1, .glyph_id_start = 1,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 48, .range_length = 10, .glyph_id_start = 2,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY},
    {.range_start = 97, .range_length = 6, .glyph_id_start = 12,
     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,
     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY}
};

static const lv_font_fmt_txt_dsc_t font_dsc = {
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 3,
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
