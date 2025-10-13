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

#include "firmware_main_loop.h"

#include "communication_mode.h"
#include "da14531/da14531.h"
#include "da14531/da14531_handler.h"
#include "da14531/da14531_protocol.h"
#include "driver_init.h"
#include "hardfault.h"
#include "hid_hww.h"
#include "hww.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "touch/gestures.h"
#include "uart.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/class/hid/hww/hid_hww.h"
#include "usb/usb.h"
#include "usb/usb_frame.h"
#include "usb/usb_processing.h"
#include "workflow/orientation_screen.h"
#include <rust/rust.h>
#include <ui/fonts/monogram_5X9.h>
#include <ui/oled/oled.h>
#include <utils_ringbuffer.h>
#if APP_U2F == 1
#include "u2f.h"
#include "u2f/u2f_packet.h"
#include "usb/class/hid/u2f/hid_u2f.h"
#endif

// Must be power of 2
#define UART_OUT_BUF_LEN 2048

void firmware_main_loop(void)
{
    screen_process_init();
    // Set the size of uart_read_buf to the size of the ringbuffer in the UART driver so we can read
    // out all bytes
    uint8_t uart_read_buf[USART_0_BUFFER_SIZE] = {0};
    uint16_t uart_read_buf_len = 0;

    struct ringbuffer uart_write_queue;
    uint8_t uart_write_buf[UART_OUT_BUF_LEN];
    ringbuffer_init(&uart_write_queue, &uart_write_buf, UART_OUT_BUF_LEN);

    /// If the bootloader has booted the BLE chip, the BLE chip isn't aware of the name according to
    /// the fw. Send it over.
    char buf[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    memory_get_device_name(buf);
    da14531_set_name(buf, strlen(buf), &uart_write_queue);

    // This starts the async orientation screen workflow, which is processed by the loop below.
    orientation_screen(&uart_write_queue);

    const uint8_t* hww_data = NULL;
    uint8_t hww_frame[USB_REPORT_SIZE] = {0};

#if APP_U2F == 1
    u2f_packet_init();
    const uint8_t* u2f_data = NULL;
    uint8_t u2f_frame[USB_REPORT_SIZE] = {0};
#endif

    if (!memory_ble_enabled()) {
        communication_mode_ble_disable();
    }

    while (1) {
        // Do UART I/O
        if (communication_mode_ble_enabled()) {
            if (uart_read_buf_len < sizeof(uart_read_buf) ||
                ringbuffer_num(&uart_write_queue) > 0) {
                uart_poll(
                    &uart_read_buf[0],
                    sizeof(uart_read_buf),
                    &uart_read_buf_len,
                    &uart_write_queue);
            }
        }

        // Check if there is outgoing data
        if (!hww_data) {
            hww_data = queue_pull(queue_hww_queue());
        }
#if APP_U2F == 1
        // Generate timeout packets
        uint32_t timeout_cid;
        while (u2f_packet_timeout_get(&timeout_cid)) {
            u2f_packet_timeout(timeout_cid);
        }
        if (!u2f_data) {
            u2f_data = queue_pull(queue_u2f_queue());
            // If USB stack was locked and there is no more messages to send out, time to
            // unlock it.
            if (!u2f_data && usb_processing_locked(usb_processing_u2f())) {
                usb_processing_unlock();
            }
        }
#endif
        // Do USB Input
        if (!hww_data && hid_hww_read(&hww_frame[0])) {
            usb_packet_process((const USB_FRAME*)hww_frame);
            if (communication_mode_ble_enabled()) {
                // Enqueue a power down command to the da14531
                da14531_power_down(&uart_write_queue);
                // Flush out the power down command. This will be the last UART communication we do.
                while (ringbuffer_num(&uart_write_queue) > 0) {
                    uart_poll(NULL, 0, NULL, &uart_write_queue);
                }
                communication_mode_ble_disable();
            }
        }
#if APP_U2F == 1
        if (!u2f_data && hid_u2f_read(&u2f_frame[0])) {
            util_log("u2f data %s", util_dbg_hex((void*)u2f_frame, 16));
            u2f_packet_process((const USB_FRAME*)u2f_frame);
            if (communication_mode_ble_enabled()) {
                // Enqueue a power down command to the da14531
                da14531_power_down(&uart_write_queue);
                // Flush out the power down command. This will be the last UART communication we do.
                while (ringbuffer_num(&uart_write_queue) > 0) {
                    uart_poll(NULL, 0, NULL, &uart_write_queue);
                }
                communication_mode_ble_disable();
            }
        }
#endif

        // Do UART Output
        if (communication_mode_ble_enabled()) {
            struct da14531_protocol_frame* frame = da14531_protocol_poll(
                &uart_read_buf[0], &uart_read_buf_len, &hww_data, &uart_write_queue);

            if (frame) {
                da14531_handler(frame, &uart_write_queue);
            }
        }

        // Do USB Output
        if (!communication_mode_ble_enabled() && hww_data) {
            if (hid_hww_write_poll(hww_data)) {
                hww_data = NULL;
            }
        }
#if APP_U2F == 1
        if (!communication_mode_ble_enabled() && u2f_data) {
            if (hid_u2f_write_poll(u2f_data)) {
                util_log("u2f wrote %s", util_dbg_hex(u2f_data, 16));
                u2f_data = NULL;
            }
        }
#endif

        /* First, process all the incoming USB traffic. */
        usb_processing_process(usb_processing_hww());
#if APP_U2F == 1
        usb_processing_process(usb_processing_u2f());
#endif
        /*
         * If USB has generated events at the application level,
         * process them now.
         */
        hww_process();
#if APP_U2F == 1
        u2f_process();
#endif
        screen_process();
        oled_present(false);

        /* And finally, run the high-level event processing. */

        rust_workflow_spin();
        rust_async_usb_spin();
    }
}
