// Copyright 2019 Shift Cryptosecurity AG
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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "ui/fonts/arial_fonts.h"
#include "ui/ugui/ugui.h"
#include <string.h>

#include "../../src/apps/eth/eth_params.c"

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

static void _test_ugui_word_wrap_all(void** state)
{
    (void)state; /* unused */
    UG_Init(&gui, _set_pixel, &font_font_a_11X10, 128, 64);
    for (size_t i = 0; i < sizeof(_ethereum_erc20_params) / sizeof(*_ethereum_erc20_params); ++i) {
        char title[1024] = {0};
        snprintf(title, sizeof(title), "Ethereum\n%s", _ethereum_erc20_params[i].unit);
        char buf[1024] = {0};
        printf("test:\n%s\n", title);
        UG_WrapTitleString(title, buf, 55);
        printf("result:\n%s\n", buf);
        // This run will just check with the santizers.
        assert_true(true);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_ugui_word_wrap),
        cmocka_unit_test(_test_ugui_word_wrap_all),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
