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

#include <ui/workflow_stack.h>
#include <workflow/blocking.h>
#include <workflow/workflow.h>

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

/**
 * NOP callback for the workflows functions.
 */
static void _do_nothing(workflow_t* self)
{
    check_expected(self);
}

/**
 * NOP callback for the workflows functions.
 */
static void _do_nothing_spin(workflow_t* self)
{
    (void)self;
}

static void _test_workflow_blocking(void** state)
{
    _force_unblock = false;
    _unblock = true;

    workflow_t* dummy_workflow = workflow_allocate(_do_nothing, NULL, _do_nothing_spin, 0);
    expect_value(_do_nothing, self, (uintptr_t)dummy_workflow);
    workflow_stack_start_workflow(dummy_workflow);
    assert_true(workflow_blocking_block());
    workflow_stack_stop_workflow();

    _force_unblock = true;
    _unblock = false;
    dummy_workflow = workflow_allocate(_do_nothing, NULL, _do_nothing_spin, 0);
    expect_value(_do_nothing, self, (uintptr_t)dummy_workflow);
    workflow_stack_start_workflow(dummy_workflow);
    assert_false(workflow_blocking_block());
    workflow_stack_stop_workflow();
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_workflow_blocking),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
