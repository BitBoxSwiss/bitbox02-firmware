// SPDX-License-Identifier: Apache-2.0

#include "reset.h"

#include "da14531/da14531.h"
#include "hardfault.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "system.h"
#include "uart.h"
#include <rust/rust.h>
#include <screen.h>

#ifndef TESTING
    #include <driver_init.h>
#endif

void reset_ble(void)
{
#if !defined(TESTING)
    struct ringbuffer uart_queue;
    uint8_t uart_queue_buf[64];
    ringbuffer_init(&uart_queue, &uart_queue_buf[0], sizeof(uart_queue_buf));
    da14531_reset(&uart_queue);
    while (ringbuffer_num(&uart_queue)) {
        uart_poll(NULL, 0, NULL, &uart_queue);
    }
#endif
}
