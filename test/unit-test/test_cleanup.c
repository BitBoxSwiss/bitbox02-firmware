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

#include <stdint.h>
#include <string.h>

#include <util.h>

static uint8_t expected[] = {0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15,
                             16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31};
static int cleanup_done = 0;
void __wrap_util_cleanup_32(uint8_t** buf);
void __wrap_util_cleanup_32(uint8_t** buf)
{
    check_expected(*buf);
    cleanup_done = 1;
}

static void cleanvar(void)
{
    uint8_t buf[32];
    memcpy(buf, expected, 32);
    UTIL_CLEANUP_32(buf);
    // buf still not cleaned, that happens after the function exists and buf
    // goes out of scope.
    assert_true(!memcmp(buf, expected, 32));
}

static void test_cleanup(void** state)
{
    expect_memory(__wrap_util_cleanup_32, *buf, expected, 32);
    cleanvar();
    assert_true(cleanup_done);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_cleanup),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
