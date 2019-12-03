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

#include "bitboxbase_screensaver.h"
#include <hal_timer.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <ui/oled/oled.h>

// The periodicity to increment the timeout counter
#define INTERVAL_MS 1000
// Number of periods we wait until we enable screensaver
#define TIMEOUT 120

extern struct timer_descriptor TIMER_0;

static volatile uint32_t timeout_counter;

static void _timer_task_cb(const struct timer_task* const timer_task)
{
    (void)timer_task;
    timeout_counter++;
}

static void _timer_config(void)
{
    static struct timer_task Timer_task;
    Timer_task.interval = INTERVAL_MS;
    Timer_task.cb = _timer_task_cb;
    Timer_task.mode = TIMER_TASK_REPEAT;

    timer_stop(&TIMER_0);
    timer_add_task(&TIMER_0, &Timer_task);
    timer_start(&TIMER_0);
}

void bitboxbase_screensaver_init(void)
{
    bitboxbase_screensaver_reset();
    _timer_config();
}

void bitboxbase_screensaver_process(void)
{
    if (timeout_counter > TIMEOUT) {
        oled_off();
    }
}

void bitboxbase_screensaver_reset(void)
{
    timeout_counter = 0;
    oled_init();
}
