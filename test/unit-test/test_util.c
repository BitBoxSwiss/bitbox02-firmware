// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include "util.h"

static void test_minmax(void** state)
{
    assert_true(1);

    int res = MIN(5, 10);
    assert_int_equal(res, 5);

    res = MIN(10, 5);
    assert_int_equal(res, 5);

    res = MAX(5, 10);
    assert_int_equal(res, 10);

    res = MAX(10, 5);
    assert_int_equal(res, 10);

    res = MIN(5, 10);
    assert_int_not_equal(res, 10);

    res = MIN(10, 5);
    assert_int_not_equal(res, 10);

    res = MAX(5, 10);
    assert_int_not_equal(res, 5);

    res = MAX(10, 5);
    assert_int_not_equal(res, 5);
}

static void test_util_strlcpy(void** state)
{
    (void)state;

    char out[] = "xxxx";
    util_strlcpy(out, "abc", sizeof(out));
    assert_string_equal(out, "abc");

    char truncated[5];
    util_strlcpy(truncated, "truncated", sizeof(truncated));
    assert_string_equal(truncated, "trun");

    char zero_len = 'x';
    util_strlcpy(&zero_len, "abc", 0);
    assert_int_equal(zero_len, 'x');
}

static void test_util_utf8_copy(void** state)
{
    (void)state;

    const char utf8[] = {'t', (char)0xC3, (char)0xA4, 's', 't', '\0'};
    char out[6] = {0};
    assert_int_equal(util_utf8_strlcpy(out, utf8, sizeof(out)), 5);
    assert_memory_equal(out, utf8, sizeof(out));

    char truncated[3] = {0};
    assert_int_equal(util_utf8_strlcpy(truncated, utf8, sizeof(truncated)), 1);
    assert_string_equal(truncated, "t");

    char empty[1] = {'x'};
    assert_int_equal(util_utf8_strlcpy(empty, "", sizeof(empty)), 0);
    assert_string_equal(empty, "");

    const char invalid[] = {'t', (char)0xFF, '\0'};
    assert_int_equal(util_utf8_strlcpy(out, invalid, sizeof(out)), -1);
    assert_string_equal(out, "");
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_minmax),
        cmocka_unit_test(test_util_strlcpy),
        cmocka_unit_test(test_util_utf8_copy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
