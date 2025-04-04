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

#if !defined(NDEBUG) && 0
    uint8_t small_buf[20];
#endif

    usart_async_enable(&USART_0);

    while (1) {
        if (uart_read_buf_len == 0) {
            uart_read_buf_len = uart_0_read(uart_read_buf, sizeof(uart_read_buf));
        }

        if (uart_write_buf_len > 0) {
            if (uart_0_write(uart_write_buf, uart_write_buf_len)) {
                uart_write_buf_len = 0;
            } else {
                util_log("da14531L ERROR!! failed to write");
            }
        }

        // The flasher times out in case it never sees a request of the firmware from the da14531
        if (flasher_timed_out(&flasher)) {
            // Reset timeout
            flasher_reset(&flasher);

            util_log("da14531: attempting reset over SWD");
            // Assume the da14531 has not been configured.
            if (!da14531_swd_reset()) {
                util_log("da14531: reset over SWD failed");
                // The da14531 failed to reset over SWD and it never requested a firmware, it is
                // probably already booted and running. Set flasher to done. Can only happen if the
                // MCU was reset and board was not power cycled.
#if !defined(NDEBUG) && 0
                // For debug builds try to reset anyway to emulate a normal startup seuqence.
                util_log("da14531: attempting reset over uart");
                uint8_t payload = 8;
                uart_write_buf_len = serial_link_out_format(
                    &small_buf[0], sizeof(small_buf), SERIAL_LINK_TYPE_CTRL_DATA, &payload, 1);
                uart_write_buf = &small_buf[0];
#else
                flasher_set_done(&flasher);
#endif
            }
        }

        if (!uart_write_buf_len) {
            if (!flasher_done(&flasher)) {
                flasher_poll(
                    &flasher,
                    uart_read_buf,
                    &uart_read_buf_len,
                    &uart_write_buf,
                    &uart_write_buf_len);
            } else {
                return;
            }
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
        util_log("BLE chip should be running");
        screen_print_debug("BLE DONE", 0);
    }

    firmware_main_loop();
    return 0;
}
