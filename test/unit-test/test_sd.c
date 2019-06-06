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

static void _test_sd_write(void** state)
{
    (void)state;
    assert_false(sd_write("test.pdf", NULL, NULL, false));
    assert_false(sd_write("test.pdf", NULL, "", false));
    assert_false(sd_write("", NULL, "text", false));
    assert_false(sd_write(NULL, NULL, "text", false));

    assert_will_mount_unmount();
    expect_string(f_open, path, "0:/bitbox02/test.pdf");
    expect_string(f_puts, str, "text");
    assert_true(sd_write("test.pdf", NULL, "text", false));

    { // write max length
        char* maxtext = alloca(SD_MAX_FILE_SIZE + 1);
        memset(maxtext, 'x', SD_MAX_FILE_SIZE);
        maxtext[SD_MAX_FILE_SIZE] = 0;
        assert_will_mount_unmount();
        expect_string(f_open, path, "0:/bitbox02/test.pdf");
        expect_string(f_puts, str, maxtext);
        assert_true(sd_write("test.pdf", NULL, maxtext, false));
    }

    { // write more than max length
        char* maxtext = alloca(SD_MAX_FILE_SIZE + 2);
        memset(maxtext, 'x', SD_MAX_FILE_SIZE + 1);
        maxtext[SD_MAX_FILE_SIZE + 1] = 0;
        assert_false(sd_write("test.pdf", NULL, maxtext, false));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_sd_write),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
