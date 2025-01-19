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

#include "common_main.h"
#include "da14531/da14531_binary.h"
#include "da14531/da14531_flasher.h"
#include "driver_init.h"
#include "firmware_main_loop.h"
#include "hardfault.h"
#include "memory/bitbox02_smarteeprom.h"
#include "memory/memory_shared.h"
#include "platform/platform_config.h"
#include "platform_init.h"
#include "qtouch.h"
#include "screen.h"
#include "uart.h"
#include "ui/screen_stack.h"
#include "workflow/idle_workflow.h"
#include "workflow/orientation_screen.h"

#if !defined(NDEBUG)
#include "da14531/da14531_serial_link.h"
#endif

uint32_t __stack_chk_guard = 0;

static void _load_da14531_firmware(void)
{
    uint8_t uart_read_buf[16] = {0};
    uint16_t uart_read_buf_len = 0;

    const uint8_t* uart_write_buf = NULL;
    uint16_t uart_write_buf_len = 0;

    struct Flasher flasher;
    flasher_init(&flasher, da14531_firmware_start, da14531_firmware_size);

    while (1) {
        if (uart_read_buf_len == 0) {
            uart_read_buf_len = uart_0_read(uart_read_buf, sizeof(uart_read_buf));
        }

        if (uart_write_buf_len > 0) {
            if (uart_0_write(uart_write_buf, uart_write_buf_len)) {
                uart_write_buf_len = 0;
            }
        }

        if (flasher_timed_out(&flasher)) {
            // Assume the da14531 has not been configured. Attempt reset
            util_log("da14531: attempting reset");
            if (!da14531_swd_reset()) {
                util_log("da14531: reset failed");
                // If it failed to reset and it never requested a firmware, it is probably already
                // booted and running. Reset flasher (so it doesn't look like timeout, and set it to
                // done)
                flasher_reset(&flasher);
                flasher_set_done(&flasher);
                // Try to reset a running BLE chip using the reset uart command
#if !defined(NDEBUG)
                util_log("da14531: attempting reset over uart");
                uint8_t buf[15];
                uint8_t payload = 8;
                uint16_t len = serial_link_out_format(
                    &buf[0], sizeof(buf), SERIAL_LINK_TYPE_CTRL_DATA, &payload, 1);
                uart_0_write(buf, len);
                // Set flasher to initial state again to be ready to flash
                flasher_reset(&flasher);
#endif
            }
        }

        if (!flasher_done(&flasher)) {
            flasher_poll(
                &flasher, uart_read_buf, &uart_read_buf_len, &uart_write_buf, &uart_write_buf_len);
        } else if (flasher_timed_out(&flasher)) {
            // If the flashing is done BUT the flasher first had to reset the da14531. The da14531
            // will turn off the debug interface, reset and ask for the firmware again.
            flasher_reset(&flasher);
        } else {
            return;
        }
    }
}

int main(void)
{
    init_mcu();
    system_init();
    platform_init();
    __stack_chk_guard = common_stack_chk_guard();
    screen_init();

    screen_splash();
    qtouch_init();
    common_main();
    bitbox02_smarteeprom_init();
    orientation_screen_blocking();
    idle_workflow_blocking();

    // The MCU needs to respond when the da14531 starts up
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        _load_da14531_firmware();
        util_log("BLE chip is running");
        screen_print_debug("BLE DONE", 0);
    }

    firmware_main_loop();
    return 0;
}
