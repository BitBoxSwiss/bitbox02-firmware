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

#include <ui/screen_stack.h>

static int _stack_counter = 0;
void __wrap_ui_screen_stack_push(component_t* component)
{
    _stack_counter++;
    check_expected(component);
}

void __wrap_ui_screen_stack_pop(void)
{
    _stack_counter--;
    assert_true(_stack_counter >= 0);
}

void mock_screen_stack_assert_clean(void)
{
    assert_int_equal(_stack_counter, 0);
}
