// Copyright 2025 Shift Crypto AG
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

#include <delay.h>
#include <hal_timer.h>
#include <hardfault.h>
#include <platform/driver_init.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <util.h>
#include <utils_assert.h>

struct task {
    struct timer_task timer;
    volatile bool done;
};

static struct task _tasks[10] = {0};

static void _hal_timer_cb(const struct timer_task* const timer)
{
    for (size_t i = 0; i < COUNT_OF(_tasks); i++) {
        if (&_tasks[i].timer == timer) {
            _tasks[i].done = true;
        }
    }
}

void delay_init_ms(delay_t* self, uint32_t ms)
{
    // find an unused slot in tasks
    size_t i;
    bool full = false;
    CRITICAL_SECTION_ENTER()
    for (i = 0; i < COUNT_OF(_tasks); i++) {
        if (_tasks[i].timer.cb == NULL && _tasks[i].done == false) {
            break;
        }
    }
    if (i == COUNT_OF(_tasks)) {
        full = true;
    } else if (ms == 0) {
        _tasks[i].done = true;
    } else {
        _tasks[i].done = false;
        memset(&_tasks[i], 0, sizeof(struct task));
        _tasks[i].timer.interval = ms;
        _tasks[i].timer.cb = _hal_timer_cb;
        _tasks[i].timer.mode = TIMER_TASK_ONE_SHOT;
        timer_add_task(&TIMER_0, &_tasks[i].timer);
    }
    CRITICAL_SECTION_LEAVE()
    if (full) {
        Abort("Too many concurrent delays");
    }
    self->id = i;
}

bool delay_is_elapsed(const delay_t* self)
{
    ASSERT(self->id < COUNT_OF(_tasks));
    if (_tasks[self->id].done) {
        memset(&_tasks[self->id], 0, sizeof(struct task));
        return true;
    }
    return false;
}

void delay_cancel(const delay_t* self)
{
    ASSERT(self->id < COUNT_OF(_tasks));
    // Check and remove task with disabled interrupts. Otherwise the interrupt may occur
    // after checking the done flag and then task is removed twice (not allowed).
    CRITICAL_SECTION_ENTER();
    if (_tasks[self->id].timer.cb && !_tasks[self->id].done) {
        timer_remove_task(&TIMER_0, &_tasks[self->id].timer);
    }
    memset(&_tasks[self->id], 0, sizeof(struct task));
    CRITICAL_SECTION_LEAVE();
}
