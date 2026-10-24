// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <fake_timer.h>
#include <pal_os_event.h>

static void _increment(void* context)
{
    (*(int*)context)++;
}

static void test_pal_os_event_replace(void** state)
{
    (void)state;
    int original = 0;
    int replacement = 0;
    pal_os_event_t* event = pal_os_event_create(_increment, &original);
    assert_non_null(list_get_head(&TIMER_0.tasks));
    pal_os_event_register_callback_oneshot(event, _increment, &replacement, 1500);
    const struct timer_task* task = list_get_head(&TIMER_0.tasks);
    assert_int_equal(task->interval, 15);
    assert_null(list_get_next_element(task));
    fake_timer_fire();
    assert_int_equal(original, 0);
    assert_int_equal(replacement, 1);
    assert_null(list_get_head(&TIMER_0.tasks));
    pal_os_event_destroy(event);
}

static void _rearm(void* context)
{
    pal_os_event_t* event = context;
    pal_os_event_register_callback_oneshot(event, _rearm, event, 1000);
}

static void test_pal_os_event_rearm_and_destroy(void** state)
{
    (void)state;
    pal_os_event_t* event = pal_os_event_create(NULL, NULL);
    pal_os_event_register_callback_oneshot(event, _rearm, event, 1000);
    fake_timer_fire();
    fake_timer_fire();
    assert_non_null(list_get_head(&TIMER_0.tasks));
    pal_os_event_destroy(event);
    assert_null(list_get_head(&TIMER_0.tasks));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_pal_os_event_replace),
        cmocka_unit_test(test_pal_os_event_rearm_and_destroy),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
