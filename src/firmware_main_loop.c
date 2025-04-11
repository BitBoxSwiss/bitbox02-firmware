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

#include "da14531/da14531_handler.h"
#include "da14531/da14531_protocol.h"
#include "driver_init.h"
#include "hardfault.h"
#include "hid_hww.h"
#include "hww.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "touch/gestures.h"
#include "u2f.h"
#include "uart.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/usb.h"
#include "usb/usb_frame.h"
#include "usb/usb_processing.h"
#include "workflow/orientation_screen.h"
#include <rust/rust.h>
#include <ui/fonts/monogram_5X9.h>
#include <utils_ringbuffer.h>

// Must be power of 2
#define UART_OUT_BUF_LEN 2048

static bool _usb_request_seen = false;

void firmware_main_loop(void)
{
    // This starts the async orientation screen workflow, which is processed by the loop below.
    orientation_screen();
    // Set it to the size of the ringbuffer in the UART driver so we can read out all bytes
    uint8_t uart_read_buf[USART_0_BUFFER_SIZE] = {0};
    uint16_t uart_read_buf_len = 0;

    uint8_t usb_frame[64];

    struct ringbuffer uart_write_queue;
    uint8_t uart_write_buf[UART_OUT_BUF_LEN];
    ringbuffer_init(&uart_write_queue, &uart_write_buf, UART_OUT_BUF_LEN);

    // int counter = 0;
    const uint8_t* data = NULL;

    while (1) {
        // Do UART I/O
        if (!_usb_request_seen) {
            if (uart_read_buf_len < sizeof(uart_read_buf) ||
                ringbuffer_num(&uart_write_queue) > 0) {
                uart_poll(
                    &uart_read_buf[0],
                    sizeof(uart_read_buf),
                    &uart_read_buf_len,
                    &uart_write_queue);
            }
        }

        // Do USB I/O
        if (hid_hww_read(&usb_frame[0])) {
            util_log("got usb data");
            usb_packet_process((const USB_FRAME*)usb_frame);
            _usb_request_seen = true;
            // TODO, reset more things?
            // TODO, send command to turn off BLE
        }

        // HWW queue contains packets to send out from HWW
        if (!data) {
            data = queue_pull(queue_hww_queue());
        }

        // If we haven't gotten any request from the app over USB, keep trying BLE
        if (!_usb_request_seen) {
            struct da14531_protocol_frame* frame = da14531_protocol_poll(
                &uart_read_buf[0], &uart_read_buf_len, data, &uart_write_queue);
            // ble doesnt' need data anymore
            data = NULL;

            if (frame) {
                da14531_handler(frame, &uart_write_queue);
            }
        } else {
            // Try sending out data over USB
            if (data) {
                if (hid_hww_poll(data)) {
                    data = NULL;
                }
            }
        }

        screen_process();
        /* And finally, run the high-level event processing. */

        rust_workflow_spin();
        rust_async_usb_spin();

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
    }
}
