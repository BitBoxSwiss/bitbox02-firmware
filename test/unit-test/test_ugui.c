// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "ui/fonts/arial_fonts.h"
#include "ui/fonts/password_12.h"
#include "ui/fonts/password_9.h"
#include "ui/ugui/ugui.h"
#include <string.h>

const char* data[][2] = {
    {"Bitcoin ", "Bitcoin "},
    {"Bitcoin", "Bitcoin"},
    {" Bitcoin", " Bitcoin"},
    {" BitcoinBitcoinBitcoin", "\nBitcoinBitcoinBitcoin"},
    {"Bitcoin Legacy", "Bitcoin\nLegacy"},
    {"Ethereum", "Ethereum"},
    {"Basic Attention Token", "Basic\nAttention Token"},
    {"BasicAttentionToken", "\nBasicAttentionToken"},
};

static UG_GUI gui;
static UG_S16 last_x;
static UG_S16 last_y;
static UG_COLOR last_color;
static uint8_t pixels_set;
static size_t set_pixel_count = 0;
static size_t black_pixel_count = 0;

static void _assert_supported_font_subset(const UG_FONT* font)
{
    assert_non_null(font);
    assert_ptr_equal(font->get_glyph_dsc, lv_font_get_glyph_dsc_fmt_txt);
    assert_ptr_equal(font->get_glyph_bitmap, lv_font_get_bitmap_fmt_txt);
    assert_non_null(font->dsc);

    const lv_font_fmt_txt_dsc_t* font_dsc = (const lv_font_fmt_txt_dsc_t*)font->dsc;
    assert_int_equal(font_dsc->bpp, 1);
    assert_int_equal(font_dsc->stride, 0);
    assert_int_equal(font_dsc->bitmap_format, LV_FONT_FMT_TXT_PLAIN);
    assert_null(font_dsc->kern_dsc);
    assert_int_equal(font_dsc->kern_scale, 0);
    assert_int_equal(font_dsc->kern_classes, 0);
    for (uint16_t i = 0; i < font_dsc->cmap_num; i++) {
        const lv_font_fmt_txt_cmap_t* cmap = &font_dsc->cmaps[i];
        assert_int_equal(cmap->type, LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY);
        assert_null(cmap->unicode_list);
        assert_null(cmap->glyph_id_ofs_list);
        assert_int_equal(cmap->list_length, 0);
    }

    if (font->fallback != NULL) {
        _assert_supported_font_subset(font->fallback);
    }
}

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    last_x = x;
    last_y = y;
    last_color = color;
    pixels_set++;
    set_pixel_count++;
    if (color == C_BLACK) {
        black_pixel_count++;
    }
}

static void _reset_pixel_capture(void)
{
    last_x = 0;
    last_y = 0;
    last_color = 0;
    pixels_set = 0;
}

static void _test_ugui_word_wrap(void** state)
{
    (void)state; /* unused */
    UG_Init(&gui, _set_pixel, &font_arial_9, 128, 64);
    for (size_t i = 0; i < sizeof(data) / sizeof(*data); ++i) {
        char buf[1024] = {0};
        printf("test:\n%s\n", data[i][0]);
        UG_WrapTitleString(data[i][0], buf, 55);
        printf("result:\n%s\n", buf);
        assert_true(memcmp(buf, data[i][1], strlen(data[i][1])) == 0);
    }
}

static void _draw_pixel(void* ctx)
{
    (void)ctx;
    UG_DrawPixel(12, 24, C_WHITE);
}

static void _test_ugui_render_rotated_180(void** state)
{
    (void)state; /* unused */
    UG_Init(&gui, _set_pixel, &font_arial_11, 128, 64);

    _reset_pixel_capture();
    UG_RenderRotated180(10, 20, 30, 10, _draw_pixel, NULL);
    assert_int_equal(pixels_set, 1);
    assert_int_equal(last_x, 37);
    assert_int_equal(last_y, 25);
    assert_int_equal(last_color, C_WHITE);

    _reset_pixel_capture();
    UG_DrawPixel(12, 24, C_WHITE);
    assert_int_equal(pixels_set, 1);
    assert_int_equal(last_x, 12);
    assert_int_equal(last_y, 24);
}

static void _test_ugui_measure_string_centered(void** state)
{
    (void)state;
    UG_Init(&gui, _set_pixel, &font_arial_11, 128, 64);

    UG_S16 centered_width = 0;
    UG_S16 centered_height = 0;
    UG_S16 single_line_width = 0;
    UG_S16 single_line_height = 0;
    UG_MeasureStringCentered(&centered_width, &centered_height, "A\nAA");
    UG_MeasureStringNoBreak(&single_line_width, &single_line_height, "AA");

    assert_int_equal(centered_width, single_line_width);
    assert_int_equal(centered_height, 2 * single_line_height + 2 * gui.char_v_space);
}

static void _test_ugui_lvgl_font(void** state)
{
    (void)state; /* unused */
    UG_Init(&gui, _set_pixel, &font_arial_9, 128, 64);

    UG_S16 width = 0;
    UG_S16 height = 0;
    UG_MeasureStringNoBreak(
        &width,
        &height,
        "A"
        "\xc3"
        "\xa4");
    assert_true(width > 0);
    assert_int_equal(height, font_arial_9.line_height);

    set_pixel_count = 0;
    black_pixel_count = 0;
    UG_PutString(0, 0, " ");
    assert_true(set_pixel_count >= (size_t)font_arial_9.line_height);
    assert_int_equal(set_pixel_count, black_pixel_count);
}

static void _test_ugui_lvgl_font_fallback(void** state)
{
    (void)state; /* unused */
    UG_Init(&gui, _set_pixel, &font_password_12, 128, 64);

    UG_U16 width = 0;
    assert_true(UG_GetCharWidth(&font_password_12, ' ', &width));
    assert_true(width > 0);
    assert_true(UG_GetCharWidth(&font_password_12, 'A', &width));
    assert_true(width > 0);

    set_pixel_count = 0;
    black_pixel_count = 0;
    UG_PutString(0, 0, " ");
    assert_true(set_pixel_count > black_pixel_count);

    set_pixel_count = 0;
    black_pixel_count = 0;
    UG_PutString(0, 0, "A");
    assert_true(set_pixel_count > black_pixel_count);
}

static void _test_ugui_fonts_use_supported_subset(void** state)
{
    (void)state; /* unused */
    _assert_supported_font_subset(&font_arial_9);
    _assert_supported_font_subset(&font_arial_11);
    _assert_supported_font_subset(&font_arial_12);
    _assert_supported_font_subset(&font_monogram_16);
    _assert_supported_font_subset(&font_password_9);
    _assert_supported_font_subset(&font_password_12);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_ugui_word_wrap),
        cmocka_unit_test(_test_ugui_render_rotated_180),
        cmocka_unit_test(_test_ugui_measure_string_centered),
        cmocka_unit_test(_test_ugui_lvgl_font),
        cmocka_unit_test(_test_ugui_lvgl_font_fallback),
        cmocka_unit_test(_test_ugui_fonts_use_supported_subset),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
