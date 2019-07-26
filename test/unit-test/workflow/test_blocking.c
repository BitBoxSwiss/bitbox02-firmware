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

#include <workflow/blocking.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wimplicit-function-declaration"

static bool _force_unblock = false;
void __wrap_ui_screen_process(bool (*is_done)(void))
{
    assert_false(is_done());
    assert_false(is_done());
    if (_force_unblock) {
        workflow_blocking_unblock_force();
    } else {
        workflow_blocking_unblock();
    }
    assert_true(is_done());
    assert_true(is_done());
}

static bool _timeout = false;
void __wrap_ui_screen_process_with_timeout(
    bool (*is_done)(void),
    void (*on_timeout)(void),
    uint32_t timeout)
{
    check_expected(timeout);

    assert_false(is_done());
    assert_false(is_done());
    if (_timeout) {
        on_timeout();
    } else if (_force_unblock) {
        workflow_blocking_unblock_force();
    } else {
        workflow_blocking_unblock();
    }
    assert_true(is_done());
    assert_true(is_done());
}

static void _test_workflow_blocking(void** state)
{
    _force_unblock = false;
    assert_true(workflow_blocking_block());

    _force_unblock = true;
    assert_false(workflow_blocking_block());
}

static void _test_workflow_blocking_with_timeout(void** state)
{
    const uint32_t timeout = 123;
    _timeout = false;
    _force_unblock = false;
    expect_value(__wrap_ui_screen_process_with_timeout, timeout, timeout);
    assert_true(workflow_blocking_block_with_timeout(timeout));

    _timeout = false;
    _force_unblock = true;
    expect_value(__wrap_ui_screen_process_with_timeout, timeout, timeout);
    assert_false(workflow_blocking_block_with_timeout(timeout));

    _timeout = true;
    expect_value(__wrap_ui_screen_process_with_timeout, timeout, timeout);
    assert_false(workflow_blocking_block_with_timeout(timeout));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_workflow_blocking),
        cmocka_unit_test(_test_workflow_blocking_with_timeout),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
