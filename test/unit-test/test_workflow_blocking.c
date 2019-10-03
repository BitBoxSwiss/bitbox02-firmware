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

static bool _force_unblock = false;
static bool _unblock = false;

void __wrap_screen_process(void)
{
    if (_force_unblock) {
        workflow_blocking_unblock_force();
    } else if (_unblock) {
        workflow_blocking_unblock();
    }
}

static void _test_workflow_blocking(void** state)
{
    _force_unblock = false;
    _unblock = true;
    assert_true(workflow_blocking_block());

    _force_unblock = true;
    _unblock = false;
    assert_false(workflow_blocking_block());
}

static void _test_workflow_blocking_with_timeout(void** state)
{
    const uint32_t timeout = 123;

    /* Normal unblock. */
    _force_unblock = false;
    _unblock = true;
    assert_true(workflow_blocking_block_with_timeout(timeout));

    /* No timeout, forced unblock. */
    _force_unblock = true;
    _unblock = false;
    assert_false(workflow_blocking_block_with_timeout(timeout));

    /* Let the workflow timeout. */
    _force_unblock = false;
    _unblock = false;
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
