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

#include <mock_blocking.h>
#include <mock_screen_stack.h>
#include <stdbool.h>

#include <ui/component.h>
#include <workflow/status.h>

const char* _msg = "message foo";

component_t* __real_status_create(
    const char* text,
    bool status_success,
    int delay,
    void (*callback)(void*),
    void* callback_param);
component_t* __wrap_status_create(
    const char* text,
    bool status_success,
    int delay,
    void (*callback)(void*),
    void* callback_param)
{
    assert_string_equal(text, _msg);
    check_expected(status_success);
    return __real_status_create(text, status_success, delay, callback, callback_param);
}

static void _test_workflow_status(void** state)
{
    for (int flags = 0; flags < 4; flags++) {
        const bool status = flags & 1;

        expect_value(__wrap_status_create, status_success, status);
        workflow_status_blocking(_msg, status);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_workflow_status),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
