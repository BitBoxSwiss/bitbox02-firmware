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

#include "reset.h"

#include "da14531/da14531.h"
#include "hardfault.h"
#include "keystore.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "memory/smarteeprom.h"
#include "system.h"
#include "uart.h"
#include <rust/rust.h>
#include <screen.h>
#include <ui/canvas.h>

#ifndef TESTING
    #include "securechip/securechip.h"
    #include <driver_init.h>
    #include <hal_delay.h>
    #include <ui/components/status.h>
    #include <ui/oled/oled.h>
    #include <ui/ugui/ugui.h>
#endif

#if !defined(TESTING)
/*
 * Shows a centered "Device reset" label.
 * Waits for 3000ms, then exit.
 */
static void _show_reset_label(bool status)
{
    const char* msg = "Device reset";
    component_t* comp = status_create(msg, status, NULL, NULL);
    canvas_clear();
    comp->f->render(comp);
    canvas_commit();
    oled_present();
    comp->f->cleanup(comp);
    delay_ms(3000);
}
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

void reset_reset(bool status)
{
    rust_keystore_lock();
#if !defined(TESTING)
    bool sc_result_reset_keys = false;
    for (int retries = 0; retries < 5; retries++) {
        sc_result_reset_keys = securechip_reset_keys();
        if (sc_result_reset_keys) {
            break;
        }
    }
    if (!sc_result_reset_keys) {
        Abort("Could not reset secure chip.");
    }
    #if APP_U2F == 1
    bool sc_result_u2f_counter_set = false;
    for (int retries = 0; retries < 5; retries++) {
        sc_result_u2f_counter_set = securechip_u2f_counter_set(0);
        if (sc_result_u2f_counter_set) {
            break;
        }
    }
    if (!sc_result_u2f_counter_set) {
        Abort("Could not initialize U2F counter.");
    }
    #endif
#endif
    if (!memory_reset_hww()) {
        Abort("Could not reset memory.");
    }
#if !defined(TESTING)
    /* Disable SmartEEPROM, so it will be erased on next reboot. */
    smarteeprom_disable();
    _show_reset_label(status);

    // The ble chip needs to be restarted to load the new secrets.
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        reset_ble();
    }

    reboot();
#else
    (void)status;
#endif
}
