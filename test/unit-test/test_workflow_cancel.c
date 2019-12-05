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

#include <workflow/cancel.h>

#include <mock_blocking.h>
#include <mock_screen_stack.h>
#include <ui/component.h>
#include <ui/components/confirm.h>
#include <workflow/blocking.h>

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    assert_false(params->longtouch);
    assert_false(params->accept_only);
    return mock();
}

void __wrap_workflow_status_create(const char* msg, bool status_success)
{
    assert_false(status_success);
}

static void (*_unblock_func_first)(void) = NULL;
static void (*_unblock_func_second)(void) = NULL;
static void _unblock_func(void)
{
    if (_unblock_func_first != NULL) {
        _unblock_func_first();
        _unblock_func_first = _unblock_func_second;
        _unblock_func_second = NULL;
    }
}

static void _test_workflow_cancel(void** state)
{
    component_t component = {0};
    mock_blocking_set_unblock_func(_unblock_func);
    expect_value_count(__wrap_ui_screen_stack_push, component, &component, -1);
    { // go through without cancel with normal unblocking
        _unblock_func_first = workflow_blocking_unblock;
        will_return(__wrap_workflow_blocking_block, true);
        // will_return(__wrap_workflow_confirm_blocking, true);
        assert_true(workflow_cancel_run("My Operation", &component));
        mock_screen_stack_assert_clean();
    }
    { // pressing cancel, but declining the prompt to cancel
        _unblock_func_first = workflow_cancel;
        _unblock_func_second = workflow_blocking_unblock;
        will_return(__wrap_workflow_blocking_block, true);
        will_return(__wrap_workflow_confirm_blocking, false);
        will_return(__wrap_workflow_blocking_block, true);

        assert_true(workflow_cancel_run("My Operation", &component));
        mock_screen_stack_assert_clean();
    }
    { // pressing cancel, accepting cancel
        _unblock_func_first = workflow_cancel;
        will_return(__wrap_workflow_blocking_block, true);
        will_return(__wrap_workflow_confirm_blocking, true);
        assert_false(workflow_cancel_run("My Operation", &component));
        mock_screen_stack_assert_clean();
    }
    { // block fails
        will_return(__wrap_workflow_blocking_block, false);
        assert_false(workflow_cancel_run("My Operation", &component));
        mock_screen_stack_assert_clean();
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_workflow_cancel),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
