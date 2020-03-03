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
#include <stdio.h>
#include <cmocka.h>

#include <apps/btc/confirm_locktime_rbf.h>
#include <ui/components/confirm.h>
#include <ui/ugui/ugui.h>

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    assert_false(params->longtouch);
    assert_false(params->accept_only);
    check_expected(params->title);
    check_expected(params->body);
    return mock();
}

void __wrap_workflow_status_create(const char* msg, bool status_success)
{
    assert_false(status_success);
    check_expected(msg);
}

static void _test_reject_locktime(void** state)
{
    expect_string(__wrap_workflow_confirm_blocking, params->title, "");
    expect_string(__wrap_workflow_confirm_blocking, params->body, "Locktime on block:\n1\n");
    will_return(__wrap_workflow_confirm_blocking, false);

    expect_string(__wrap_workflow_status_create, msg, "Transaction\ncanceled");
    assert_false(apps_btc_confirm_locktime_rbf(1, CONFIRM_LOCKTIME_RBF_DISABLED));
}

static void _test_0_locktime_and_rbf(void** state)
{
    expect_string(__wrap_workflow_confirm_blocking, params->title, "");
    expect_string(
        __wrap_workflow_confirm_blocking,
        params->body,
        "Locktime on block:\n0\nTransaction is RBF");
    will_return(__wrap_workflow_confirm_blocking, true);

    assert_true(apps_btc_confirm_locktime_rbf(0, CONFIRM_LOCKTIME_RBF_ON));
}

static void _test_high_locktime_and_rbf(void** state)
{
    expect_string(__wrap_workflow_confirm_blocking, params->title, "");
    expect_string(
        __wrap_workflow_confirm_blocking,
        params->body,
        "Locktime on block:\n100000000\nTransaction is RBF");
    will_return(__wrap_workflow_confirm_blocking, true);

    assert_true(apps_btc_confirm_locktime_rbf(100000000, CONFIRM_LOCKTIME_RBF_ON));
}

static void _test_locktime_no_rbf(void** state)
{
    expect_string(__wrap_workflow_confirm_blocking, params->title, "");
    expect_string(
        __wrap_workflow_confirm_blocking,
        params->body,
        "Locktime on block:\n10\nTransaction is not RBF");
    will_return(__wrap_workflow_confirm_blocking, true);

    assert_true(apps_btc_confirm_locktime_rbf(10, CONFIRM_LOCKTIME_RBF_OFF));
}

static void _test_no_locktime_no_rbf(void** state)
{
    // it is the function caller's job to make sure there is something to verify
    // no values will just create an empty screen
    expect_string(__wrap_workflow_confirm_blocking, params->title, "");
    expect_string(__wrap_workflow_confirm_blocking, params->body, "Locktime on block:\n0\n");
    will_return(__wrap_workflow_confirm_blocking, true);
    assert_true(apps_btc_confirm_locktime_rbf(0, CONFIRM_LOCKTIME_RBF_DISABLED));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_reject_locktime),
        cmocka_unit_test(_test_no_locktime_no_rbf),
        cmocka_unit_test(_test_0_locktime_and_rbf),
        cmocka_unit_test(_test_high_locktime_and_rbf),
        cmocka_unit_test(_test_locktime_no_rbf),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
