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

#include <alloca.h>
#include <string.h>

#include <assert_ff.h>
#include <ff.h>
#include <sd.h>
#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

/* tests */

static void _test_sd_write_bin(void** state)
{
    uint8_t data[4] = "data";
    assert_false(sd_write_bin("test.pdf", NULL, NULL, 0, false));
    assert_false(sd_write_bin("test.pdf", NULL, data, 0, false));
    assert_false(sd_write_bin("", NULL, data, sizeof(data), false));
    assert_false(sd_write_bin(NULL, NULL, data, sizeof(data), false));

    assert_will_mount_unmount();
    expect_string(f_open, path, "0:/bitbox02/test.pdf");
    expect_memory(f_write, buff, data, sizeof(data));
    expect_value(f_write, btw, sizeof(data));
    assert_true(sd_write_bin("test.pdf", NULL, data, sizeof(data), false));

    { // write max length
        uint8_t maxdata[SD_MAX_FILE_SIZE] = {0};
        memset(maxdata, 'x', sizeof(maxdata));
        assert_will_mount_unmount();
        expect_string(f_open, path, "0:/bitbox02/test.pdf");
        expect_memory(f_write, buff, maxdata, sizeof(maxdata));
        expect_value(f_write, btw, sizeof(maxdata));
        assert_true(sd_write_bin("test.pdf", NULL, maxdata, sizeof(maxdata), false));
    }

    { // write more than max length
        uint8_t maxdata[SD_MAX_FILE_SIZE + 1] = {0};
        assert_false(sd_write_bin("test.pdf", NULL, maxdata, sizeof(maxdata), false));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_sd_write_bin),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
