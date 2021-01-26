#include "util.h"

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

static void test_util_format_datetime(void** state)
{
    char out[100];
    util_format_datetime(1601281809, 0, true, out, sizeof(out));
    assert_string_equal(out, "Mon 2020-09-28");

    util_format_datetime(1601281809, 0, false, out, sizeof(out));
    assert_string_equal(out, "Mon 2020-09-28\n08:30");

    util_format_datetime(1601281809, 18000, false, out, sizeof(out));
    assert_string_equal(out, "Mon 2020-09-28\n13:30");

    util_format_datetime(1601281809, -32400, false, out, sizeof(out));
    assert_string_equal(out, "Sun 2020-09-27\n23:30");
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_minmax),
        cmocka_unit_test(test_util_format_datetime),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
