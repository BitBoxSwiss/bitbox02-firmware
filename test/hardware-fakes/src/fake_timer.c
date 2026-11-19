// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <fake_timer.h>

struct timer_descriptor TIMER_0;

int32_t timer_add_task(struct timer_descriptor* const descr, struct timer_task* const task)
{
    assert_false(is_list_element(&descr->tasks, task));
    list_insert_as_head(&descr->tasks, task);
    return 0;
}

int32_t timer_remove_task(struct timer_descriptor* const descr, const struct timer_task* const task)
{
    assert_true(is_list_element(&descr->tasks, task));
    list_delete_element(&descr->tasks, task);
    return 0;
}

void fake_timer_fire(void)
{
    struct timer_task* task = list_remove_head(&TIMER_0.tasks);
    assert_non_null(task);
    assert_int_equal(task->mode, TIMER_TASK_ONE_SHOT);
    task->cb(task);
}
