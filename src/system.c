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
#include "da14531/da14531.h"
#include "utils_ringbuffer.h"
#include <memory/memory.h>
#include <memory/memory_shared.h>
#include <screen.h>
#include <stdint.h>
#ifndef TESTING
#include "uart.h"
#include <driver_init.h>
#endif

// Section is fixed in ram, so can be used to communicate between fw/bl
// Must stay synchronized with bootloader.c, bootloader.ld, firmware.ld
volatile secbool_u32 auto_enter __attribute__((section(".auto_enter")));

static void _ble_clear_product(void)
{
    struct ringbuffer uart_queue;
    uint8_t uart_queue_buf[64];
    ringbuffer_init(&uart_queue, &uart_queue_buf[0], sizeof(uart_queue_buf));
    da14531_set_product(NULL, 0, &uart_queue);
    while (ringbuffer_num(&uart_queue)) {
#ifndef TESTING
        uart_poll(NULL, 0, NULL, &uart_queue);
#else
        ringbuffer_flush(&uart_queue);
#endif
    }
}

void reboot_to_bootloader(void)
{
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        _ble_clear_product();
    }
    auto_enter = sectrue_u32;
#ifndef TESTING
    _reset_mcu();
#endif
}

void reboot(void)
{
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        _ble_clear_product();
    }
#ifndef TESTING
    _reset_mcu();
#endif
}
