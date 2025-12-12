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

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    /* nop */
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

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_ugui_word_wrap),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
