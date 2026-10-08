// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "../../src/ui/components/lockscreen.c"

static UG_GUI gui;

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    (void)x;
    (void)y;
    (void)color;
}

static int _setup(void** state)
{
    (void)state;
    UG_Init(&gui, _set_pixel, &font_regular_9, SCREEN_WIDTH, SCREEN_HEIGHT);
    return 0;
}

static void test_truncate_to_fit_utf8(void** state)
{
    (void)state;
    const char* names[] = {
        "ThisIsANiceValidNameWhichIsMoreThan48ButLessThan63ü",
        "üüüüüüüüüüüüüüüüüüüüüüüüüüüüüüü",
    };
    for (size_t i = 0; i < sizeof(names) / sizeof(*names); i++) {
        char out[MEMORY_DEVICE_MAX_LEN_WITH_NULL + 3] = {0};
        assert_false(label_fits_width(names[i], &font_regular_9, SCREEN_WIDTH));
        _truncate_to_fit(names[i], out, sizeof(out), &font_regular_9, SCREEN_WIDTH);

        const size_t len = strlen(out);
        assert_true(len > 3);
        assert_string_equal(out + len - 3, "...");
        assert_memory_equal(out, names[i], len - 3);
        assert_int_equal(
            rust_util_utf8_truncate(rust_util_bytes((const uint8_t*)out, len), len), len);
        assert_true(label_fits_width(out, &font_regular_9, SCREEN_WIDTH));
    }
}

static void test_truncate_to_fit_capacity(void** state)
{
    (void)state;
    const char* name = "1234567üThisNameIsTooLongToFitOnTheScreen";
    char out[12] = {0};
    // Reserving the ellipsis leaves eight bytes, which would split the two-byte ü.
    _truncate_to_fit(name, out, sizeof(out), &font_regular_9, SCREEN_WIDTH);
    assert_string_equal(out, "1234567...");

    char ellipsis_only[4] = {0};
    _truncate_to_fit(name, ellipsis_only, sizeof(ellipsis_only), &font_regular_9, SCREEN_WIDTH);
    assert_string_equal(ellipsis_only, "...");
}

static void test_truncate_to_fit_short_name(void** state)
{
    (void)state;
    char out[MEMORY_DEVICE_MAX_LEN_WITH_NULL + 3] = {0};
    _truncate_to_fit("BïtBöx", out, sizeof(out), &font_regular_9, SCREEN_WIDTH);
    assert_string_equal(out, "BïtBöx");
    _truncate_to_fit("", out, sizeof(out), &font_regular_9, SCREEN_WIDTH);
    assert_string_equal(out, "");
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_truncate_to_fit_utf8),
        cmocka_unit_test(test_truncate_to_fit_capacity),
        cmocka_unit_test(test_truncate_to_fit_short_name),
    };
    return cmocka_run_group_tests(tests, _setup, NULL);
}
