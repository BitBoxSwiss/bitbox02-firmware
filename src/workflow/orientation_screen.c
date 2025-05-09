// Copyright 2019 Shift Cryptosecurity AG
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

#include "orientation_screen.h"

#ifndef TESTING
#include <hal_timer.h>
#include <platform/driver_init.h>
#endif
#include <da14531/da14531.h>
#include <da14531/da14531_handler.h>
#include <hww.h>
#include <memory/memory_shared.h>
#include <screen.h>
#include <ui/components/lockscreen.h>
#include <ui/components/orientation_arrows.h>
#include <ui/screen_stack.h>
#include <usb/usb.h>
#include <utils_ringbuffer.h>

#if APP_U2F
#include <u2f.h>
#endif

#ifndef TESTING
#define IDLE_PERIOD_MS 1300

#define DEVICE_MODE "{\"p\":\"bb02p-multi\",\"v\":\"9.22.0\"}"

static struct timer_task _idle_timer_task = {0};

struct select_orientation_data {
    struct ringbuffer* uart_out_queue;
};

static struct select_orientation_data _data = {0};

static void _idle_timer_cb(const struct timer_task* const timer_task)
{
    (void)timer_task;

    // Setup usb_processing handlers
    hww_setup();
#if APP_U2F
    u2f_device_setup();
#endif

    // hww handler in usb_process must be setup before we can allow ble connections
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        da14531_handler_current_product = (const uint8_t*)DEVICE_MODE;
        da14531_handler_current_product_len = sizeof(DEVICE_MODE) - 1;
        da14531_set_product(
            da14531_handler_current_product,
            da14531_handler_current_product_len,
            _data.uart_out_queue);
    }

    usb_start();
    ui_screen_stack_push(lockscreen_create());
}
#endif

static void _select_orientation_done(bool upside_down, void* cb_param)
{
    (void)cb_param;
    if (upside_down) {
        screen_rotate();
    }
    ui_screen_stack_pop();

#ifndef TESTING
    // Added deliberately as a UX/visual improvement, to show the BB02 logo first before moving onto
    // the lock screen and unlocking USB.
    _idle_timer_task.interval = IDLE_PERIOD_MS;
    _idle_timer_task.cb = _idle_timer_cb;
    _idle_timer_task.mode = TIMER_TASK_ONE_SHOT;
    timer_add_task(&TIMER_0, &_idle_timer_task);
#endif
}

void orientation_screen(struct ringbuffer* uart_out_queue)
{
#ifndef TESTING
    _data.uart_out_queue = uart_out_queue;
#endif
    ui_screen_stack_push(orientation_arrows_create(_select_orientation_done, NULL));
}
