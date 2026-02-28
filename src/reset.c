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
    struct RustByteQueue* uart_queue = rust_bytequeue_init(64);
    if (uart_queue == NULL) {
        return;
    }
    rust_da14531_reset(uart_queue);
    while (rust_bytequeue_num(uart_queue)) {
        uart_poll(NULL, 0, NULL, uart_queue);
    }
    rust_bytequeue_free(uart_queue);
#endif
}
