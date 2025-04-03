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

#include "da14531/da14531_serial_link.h"
#include "hardfault.h"
#include "hww.h"
#include "touch/gestures.h"
#include "u2f.h"
#include "uart.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/usb.h"
#include "usb/usb_processing.h"
#include <rust/rust.h>

static uint8_t uart_out_buf[100];

#define DN "My BitBox"

static void _ctrl_handler(
    struct serial_link_frame* frame,
    const uint8_t** buf_out,
    uint16_t* buf_out_len)
{
    switch (frame->payload[0]) {
    case 1: {
        util_log("da14531: get device name");
        // 1 byte cmd
        // rest device name

        uint8_t payload[1 + sizeof(DN) - 1];
        payload[0] = 1;
        memcpy(&payload[1], DN, sizeof(DN) - 1);
        uint16_t len = serial_link_out_format(
            &uart_out_buf[0],
            sizeof(uart_out_buf),
            SERIAL_LINK_TYPE_CTRL_DATA,
            &payload[0],
            sizeof(payload));
        *buf_out = &uart_out_buf[0];
        *buf_out_len = len;
    } break;
    case 2: {
        util_log("da14531: get bond db");
        uint8_t payload = 2;
        uint16_t len = serial_link_out_format(
            &uart_out_buf[0], sizeof(uart_out_buf), SERIAL_LINK_TYPE_CTRL_DATA, &payload, 1);
        *buf_out = &uart_out_buf[0];
        *buf_out_len = len;
    } break;
    case 3:
        util_log("da14531: set bond db");
        break;
    case 4: {
        util_log("da14531: show/confirm pairing code");
        uint8_t payload[18] = {0};
        payload[0] = 11; /* code for confirm pairind code message */
        memcpy(&payload[1], &frame->payload[1], frame->payload_length - 1);
        payload[17] = 1; /* 1 yes, 0 no */
        uint16_t len = serial_link_out_format(
            &uart_out_buf[0],
            sizeof(uart_out_buf),
            SERIAL_LINK_TYPE_CTRL_DATA,
            &payload[0],
            sizeof(payload));
        *buf_out = &uart_out_buf[0];
        *buf_out_len = len;
    } break;
    case 5:
        util_log("da14531: BLE status update");
        switch (frame->payload[1]) {
        case 0:
            util_log("da14531: adveritising");
            break;
        case 1:
            util_log("da14531: connected");
            break;
        case 2:
            util_log("da14531: connected secure");
            break;
        default:
            break;
        }
        break;
    case 6: {
        util_log("da14531: get irk");
        // 1 byte cmd
        // 16 bytes irk
        uint8_t payload[17] = {6, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
        uint16_t len = serial_link_out_format(
            &uart_out_buf[0],
            sizeof(uart_out_buf),
            SERIAL_LINK_TYPE_CTRL_DATA,
            &payload[0],
            sizeof(payload));
        *buf_out = &uart_out_buf[0];
        *buf_out_len = len;
    } break;
    case 7:
        // Not used
        break;
    case 8:
        // Not used
        break;
    case 9: {
        util_log("da14531: get addr");
        // 1 byte cmd
        // 6 bytes addr
        uint8_t payload[7] = {9, 0, 1, 2, 3, 4, 5};
        uint16_t len = serial_link_out_format(
            &uart_out_buf[0],
            sizeof(uart_out_buf),
            SERIAL_LINK_TYPE_CTRL_DATA,
            &payload[0],
            sizeof(payload));
        *buf_out = &uart_out_buf[0];
        *buf_out_len = len;
    } break;
    case 10:
        util_log("da14531: pairing successful");
        break;
    case SL_CTRL_CMD_DEBUG:
        util_log("da14531-debug: %.*s", frame->payload_length - 1, &frame->payload[1]);
        break;
    default:
        break;
    }
}

static void _in_handler(
    struct serial_link_frame* frame,
    const uint8_t** buf_out,
    uint16_t* buf_out_len)
{
    switch (frame->type) {
    case SERIAL_LINK_TYPE_CTRL_DATA:
        _ctrl_handler(frame, buf_out, buf_out_len);
        break;
    default:
        break;
    }
}

void firmware_main_loop(void)
{
    uint8_t uart_read_buf[64] = {0};
    uint16_t uart_read_buf_len = 0;

    const uint8_t* uart_write_buf = NULL;
    uint16_t uart_write_buf_len = 0;

    struct SerialLinkIn serial_link;
    serial_link_in_init(&serial_link);

    struct serial_link_frame* frame = NULL;
    while (1) {
        // UART IO
        if (uart_read_buf_len == 0) {
            uart_read_buf_len = uart_0_read(uart_read_buf, sizeof(uart_read_buf));
        }

        if (uart_write_buf_len > 0) {
            // util_log("debug: %s", util_dbg_hex(uart_write_buf, uart_write_buf_len));
            if (uart_0_write(uart_write_buf, uart_write_buf_len)) {
                uart_write_buf_len = 0;
            }
        }

        // Process any IO from UART
        // Only poll serial_link if there is no pending frame, nor pending uart output
        if (!frame && !uart_write_buf_len) {
            frame = serial_link_in_poll(&serial_link, uart_read_buf, &uart_read_buf_len);

            if (frame) {
                _in_handler(frame, &uart_write_buf, &uart_write_buf_len);
                free(frame);
                frame = NULL;
            }
        }

        screen_process();
        /* And finally, run the high-level event processing. */

        rust_workflow_spin();

        if (usb_is_enabled()) {
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
}
