// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <fake_timer.h>
#include <hal_atomic.h>

struct timer_descriptor TIMER_0;
static hal_atomic_t _critical_depth;

void atomic_enter_critical(volatile hal_atomic_t* atomic)
{
    *atomic = _critical_depth++;
}

void atomic_leave_critical(volatile hal_atomic_t* atomic)
{
    assert_int_equal(_critical_depth, *atomic + 1);
    _critical_depth = *atomic;
}

int32_t timer_add_task(struct timer_descriptor* const descr, struct timer_task* const task)
{
    assert_true(_critical_depth > 0);
    assert_false(is_list_element(&descr->tasks, task));
    list_insert_as_head(&descr->tasks, task);
    return 0;
}

int32_t timer_remove_task(struct timer_descriptor* const descr, const struct timer_task* const task)
{
    assert_true(_critical_depth > 0);
    assert_true(is_list_element(&descr->tasks, task));
    list_delete_element(&descr->tasks, task);
    return 0;
}

void fake_timer_fire(void)
{
    assert_int_equal(_critical_depth, 0);
    struct timer_task* task = list_remove_head(&TIMER_0.tasks);
    assert_non_null(task);
    assert_int_equal(task->mode, TIMER_TASK_ONE_SHOT);
    task->cb(task);
}
