// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "ui/fonts/arial_fonts.h"
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

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    last_x = x;
    last_y = y;
    last_color = color;
    pixels_set++;
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
    UG_Init(&gui, _set_pixel, &font_font_a_11X10, 128, 64);
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
    UG_Init(&gui, _set_pixel, &font_font_a_11X10, 128, 64);

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

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_ugui_word_wrap),
        cmocka_unit_test(_test_ugui_render_rotated_180),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
