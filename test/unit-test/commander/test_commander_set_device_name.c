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

#include <test_commander.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wimplicit-function-declaration"

bool __wrap_workflow_confirm_scrollable(const char* title, const char* body, bool accept_only)
{
    check_expected(title);
    check_expected(body);
    check_expected(accept_only);
    return mock();
}

static void _test_api_set_device_name(void** state)
{
    (void)state;
    static SetDeviceNameRequest request = {
        .name = "Mia",
    };

    expect_string_count(__wrap_workflow_confirm_scrollable, title, "Name", -1);
    expect_string_count(__wrap_workflow_confirm_scrollable, body, request.name, -1);
    expect_value_count(__wrap_workflow_confirm_scrollable, accept_only, false, -1);

    // All A-Okay.
    will_return(__wrap_workflow_confirm_scrollable, true);
    will_return(__wrap_memory_set_device_name, true);
    assert_int_equal(COMMANDER_OK, commander_api_set_device_name(&request));

    // User rejects.
    will_return(__wrap_workflow_confirm_scrollable, false);
    assert_int_equal(COMMANDER_ERR_USER_ABORT, commander_api_set_device_name(&request));

    // Setting name fails.
    will_return(__wrap_workflow_confirm_scrollable, true);
    will_return(__wrap_memory_set_device_name, false);
    assert_int_equal(COMMANDER_ERR_MEMORY, commander_api_set_device_name(&request));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_api_set_device_name),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
