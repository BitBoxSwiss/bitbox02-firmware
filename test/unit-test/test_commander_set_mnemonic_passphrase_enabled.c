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
#include <ui/ugui/ugui.h>

bool __wrap_workflow_confirm(
    const char* title,
    const char* body,
    UG_FONT* font,
    bool longtouch,
    bool accept_only)
{
    check_expected(title);
    check_expected(body);
    check_expected(font);
    check_expected(longtouch);
    check_expected(accept_only);
    return mock();
}

static void _test_api_set_mnemonic_passphrase_enabled(void** state)
{
    expect_string_count(__wrap_workflow_confirm, body, "Optional\npassphrase", -1);
    expect_value_count(__wrap_workflow_confirm, font, NULL, -1);
    expect_value_count(__wrap_workflow_confirm, longtouch, true, -1);
    expect_value_count(__wrap_workflow_confirm, accept_only, false, -1);

    const bool bools[2] = {false, true};
    for (int i = 0; i < 2; i++) {
        const SetMnemonicPassphraseEnabledRequest request = {
            .enabled = bools[i],
        };

        // All A-Okay.
        expect_string_count(
            __wrap_workflow_confirm, title, request.enabled ? "Enable" : "Disable", 3);

        will_return(__wrap_workflow_confirm, true);
        expect_value(__wrap_memory_set_mnemonic_passphrase_enabled, enabled, request.enabled);
        will_return(__wrap_memory_set_mnemonic_passphrase_enabled, true);
        assert_int_equal(COMMANDER_OK, commander_api_set_mnemonic_passphrase_enabled(&request));

        // User rejects.
        will_return(__wrap_workflow_confirm, false);
        assert_int_equal(
            COMMANDER_ERR_USER_ABORT, commander_api_set_mnemonic_passphrase_enabled(&request));

        will_return(__wrap_workflow_confirm, true);
        expect_value(__wrap_memory_set_mnemonic_passphrase_enabled, enabled, request.enabled);
        will_return(__wrap_memory_set_mnemonic_passphrase_enabled, false);
        assert_int_equal(
            COMMANDER_ERR_MEMORY, commander_api_set_mnemonic_passphrase_enabled(&request));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_api_set_mnemonic_passphrase_enabled),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
