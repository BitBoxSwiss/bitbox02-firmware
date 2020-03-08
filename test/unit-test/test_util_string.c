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

static const char* _all_ascii =
    "! \"#$%&\'()*+,-./"
    "0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

static void _test_util_string_validate_name(void** state)
{
    // Max len.
    assert_true(util_string_validate_name("foo", 5));
    assert_true(util_string_validate_name("foo", 4));
    assert_true(util_string_validate_name("foo", 3));
    assert_false(util_string_validate_name("foo", 2));

    // Ascii.
    assert_true(util_string_validate_name(_all_ascii, 100));
    assert_false(util_string_validate_name("\n", 100));
    assert_false(util_string_validate_name("\t", 100));

    // Starts / ends with space.
    assert_false(util_string_validate_name(" foo", 100));
    assert_false(util_string_validate_name("foo ", 100));

    // Can contain space otherwise.
    assert_true(util_string_validate_name("hello world", 100));

    // Empty string.
    assert_false(util_string_validate_name("", 100));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_util_string_validate_name),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
