// SPDX-License-Identifier: Apache-2.0

#include "system.h"
#include "da14531/da14531.h"
#include <bootloader/boot_args.h>
#include <memory/memory_shared.h>
#include <rust/rust.h>
#ifndef TESTING
    #include "uart.h"
    #include <driver_init.h>
#endif

static void _ble_clear_product(void)
{
    struct RustByteQueue* uart_queue = rust_bytequeue_init(64);
    if (uart_queue == NULL) {
        return;
    }
    rust_da14531_set_product(rust_util_bytes(NULL, 0), uart_queue);
    while (rust_bytequeue_num(uart_queue)) {
#ifndef TESTING
        uart_poll(NULL, 0, NULL, uart_queue);
#else
        rust_bytequeue_flush(uart_queue);
#endif
    }
    rust_bytequeue_free(uart_queue);
}

void boot_bootloader_wait(bool upside_down)
{
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        _ble_clear_product();
    }
#ifndef TESTING
    boot_args_write_bootloader_wait(upside_down);
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
