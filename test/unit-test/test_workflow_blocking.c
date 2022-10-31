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

void __wrap_screen_process(void)
{
    workflow_blocking_unblock();
}

static void _test_workflow_blocking(void** state)
{
    workflow_blocking_block();
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_workflow_blocking),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
