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

#include "system.h"
#include <memory/memory.h>
#include <screen.h>
#ifndef TESTING
#include <driver_init.h>
#endif

#include "da14531/da14531.h"
#include "uart.h"
#include "utils_ringbuffer.h"

void reboot(void)
{
    struct ringbuffer uart_queue;
    uint8_t uart_queue_buf[64];
    ringbuffer_init(&uart_queue, &uart_queue_buf[0], sizeof(uart_queue_buf));
    da14531_set_product("", 0, &uart_queue);
    while (ringbuffer_num(&uart_queue)) {
        uart_poll(NULL, 0, NULL, &uart_queue);
    }
    auto_enter_t auto_enter = {
        .value = sectrue_u8,
    };
    upside_down_t upside_down = {
        .value = screen_is_upside_down(),
    };
    if (!memory_bootloader_set_flags(auto_enter, upside_down)) {
        // If this failed, we might not be able to reboot into the bootloader.
        // We will try anyway, no point in aborting here.
    }
#ifndef TESTING
    _reset_mcu();
#endif
}
