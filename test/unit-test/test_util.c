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

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_minmax),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
