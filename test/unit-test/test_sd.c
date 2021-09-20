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

#include <sd.h>

static void _test_sd_write_read(void** state)
{
    assert_true(sd_format());
    uint8_t data[4] = "data";

    assert_false(sd_write_bin("test.pdf", NULL, NULL, 0, false));
    assert_false(sd_write_bin("test.pdf", NULL, data, 0, false));
    assert_false(sd_write_bin("", NULL, data, sizeof(data), false));
    assert_false(sd_write_bin(NULL, NULL, data, sizeof(data), false));

    assert_true(sd_write_bin("test.pdf", NULL, data, sizeof(data), false));
    uint8_t read[100] = {0};
    size_t readlen;
    assert_true(sd_load_bin("test.pdf", NULL, read, &readlen));
    assert_int_equal(readlen, 4);
    assert_memory_equal(read, data, readlen);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_sd_write_read),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
