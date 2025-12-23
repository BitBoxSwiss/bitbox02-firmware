// SPDX-License-Identifier: Apache-2.0

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
