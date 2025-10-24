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
#include <platform/driver_init.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <utils_assert.h>

struct task {
    struct timer_task timer;
    volatile bool done;
};

static struct task _tasks[10] = {0};
static struct task _empty = {0};

static void _hal_timer_cb(const struct timer_task* const timer)
{
    for (int i = 0; i < (int)(sizeof(_tasks) / sizeof(struct task)); i++) {
        if (&_tasks[i].timer == timer) {
            _tasks[i].done = true;
        }
    }
}

bool delay_init_ms(delay_t* self, uint32_t ms)
{
    // find an unused slot in tasks
    int i;
    CRITICAL_SECTION_ENTER();
    for (i = 0; i < (int)(sizeof(_tasks) / sizeof(struct task)); i++) {
        if (memcmp(&_tasks[i], &_empty, sizeof(struct task)) == 0) {
            break;
        }
    }
    CRITICAL_SECTION_LEAVE();
    if (i == sizeof(_tasks)) {
        return false;
    }
    _tasks[i].done = false;
    memset(&_tasks[i], 0, sizeof(struct task));
    _tasks[i].timer.interval = ms;
    _tasks[i].timer.cb = _hal_timer_cb;
    _tasks[i].timer.mode = TIMER_TASK_ONE_SHOT;
    self->id = i;
    return true;
}

void delay_start(const delay_t* self)
{
    ASSERT(self->id < (sizeof(_tasks) / sizeof(struct task)));
    ASSERT(!_tasks[self->id].done);
    timer_add_task(&TIMER_0, &_tasks[self->id].timer);
}

bool delay_poll(const delay_t* self)
{
    ASSERT(self->id < (sizeof(_tasks) / sizeof(struct task)));
    if (_tasks[self->id].done) {
        memset(&_tasks[self->id], 0, sizeof(struct task));
        return true;
    }
    return false;
}
