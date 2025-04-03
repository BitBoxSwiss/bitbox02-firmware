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
#include "usb/usb_frame.h"
#include "usb/usb_processing.h"
#include <rust/rust.h>
#include <ui/components/confirm.h>
#include <ui/fonts/monogram_5X9.h>
#include <utils_ringbuffer.h>

static component_t* _ble_pairing_component = NULL;

struct pairing_callback {
    uint8_t key[4];
    struct ringbuffer* queue;
};

static struct pairing_callback _ble_pairing_callback_data;

static void _ble_pairing_callback(bool ok, void* param)
{
    struct pairing_callback* data = (struct pairing_callback*)param;

    uint8_t payload[18] = {0};
    payload[0] = 11; /* code for confirm pairind code message */
    memcpy(&payload[1], &data->key[0], sizeof(data->key));
    payload[17] = ok ? 1 : 0; /* 1 yes, 0 no */

    uint8_t tmp[100];
    uint16_t len = serial_link_out_format(
        &tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_CTRL_DATA, payload, sizeof(payload));
    ASSERT(len <= sizeof(tmp));
    for (int i = 0; i < len; i++) {
        ringbuffer_put(data->queue, tmp[i]);
    }

    ui_screen_stack_pop();
    _ble_pairing_component = NULL;
}

#define DN "My BitBox"

static void _ctrl_handler(struct serial_link_frame* frame, struct ringbuffer* queue)
{
    uint8_t tmp[128];
    switch (frame->payload[0]) {
    case 1: {
        util_log("da14531: get device name");
        // 1 byte cmd
        // rest device name

        uint8_t payload[1 + sizeof(DN) - 1];
        payload[0] = 1;
        memcpy(&payload[1], DN, sizeof(DN) - 1);
        uint16_t len = serial_link_out_format(
            &tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_CTRL_DATA, &payload[0], sizeof(payload));
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case 2: {
        util_log("da14531: get bond db");
        uint8_t payload = 2;
        uint16_t len =
            serial_link_out_format(&tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_CTRL_DATA, &payload, 1);
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case 3:
        util_log("da14531: set bond db");
        break;
    case 4: {
        if (frame->payload_length < 5) {
            // TODO handle error.
            Abort("Invalid payload length for BLE pairing code");
        }
        memcpy(
            &(_ble_pairing_callback_data.key)[0],
            &frame->payload[1],
            sizeof(_ble_pairing_callback_data.key));
        _ble_pairing_callback_data.queue = queue;
        uint32_t pairing_code_int = (*(uint32_t*)&frame->payload[1]) % 1000000;
        char pairing_code[7] = {0};
        snprintf(pairing_code, sizeof(pairing_code), "%06lu", (long unsigned int)pairing_code_int);
        util_log("da14531: show/confirm pairing code: %s", pairing_code);
        const confirm_params_t confirm_params = {
            .title = "Pairing code",
            .body = pairing_code,
            .font = &font_monogram_5X9,
        };
        _ble_pairing_component = confirm_create(
            &confirm_params, _ble_pairing_callback, (void*)&_ble_pairing_callback_data);
        ui_screen_stack_push(_ble_pairing_component);
    } break;
    case 5:
        util_log("da14531: BLE status update");
        switch (frame->payload[1]) {
        case 0:
            util_log("da14531: adveritising");
            if (_ble_pairing_component != NULL && ui_screen_stack_top() == _ble_pairing_component) {
                ui_screen_stack_pop();
                _ble_pairing_component = NULL;
            }
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
            &tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_CTRL_DATA, &payload[0], sizeof(payload));
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case 7: {
        util_log("da14531: get device mode");
#define DEVICE_MODE "{\"p\":\"bb02p-multi\",\"v\":\"9.22.0\"}"
        uint8_t payload[64] = {0};
        payload[0] = 7;
        memcpy(&payload[1], DEVICE_MODE, sizeof(DEVICE_MODE) - 1);
        uint16_t len = serial_link_out_format(
            &tmp[0],
            sizeof(tmp),
            SERIAL_LINK_TYPE_CTRL_DATA,
            &payload[0],
            1 + sizeof(DEVICE_MODE) - 1);
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case 8:
        // Not used
        break;
    case 9: {
        util_log("da14531: get addr");
        // 1 byte cmd
        // 6 bytes addr
        uint8_t payload[7] = {9, 0, 1, 2, 3, 4, 5};
        uint16_t len = serial_link_out_format(
            &tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_CTRL_DATA, &payload[0], sizeof(payload));
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
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

static bool _usb_packet_packet_complete(State* state)
{
    return (state->buf_ptr - state->data) == (signed)state->len;
}

static void _hww_handler(struct serial_link_frame* frame, struct ringbuffer* queue)
{
    util_log(" in: %s...", util_dbg_hex(frame->payload, 32));
    (void)queue;
    static State state = {0};
    ASSERT(frame->payload_length == 64);
    usb_frame_process((USB_FRAME*)&frame->payload[0], &state);
    if (_usb_packet_packet_complete(&state)) {
        usb_processing_set_send(usb_processing_hww(), NULL);
        usb_processing_enqueue(usb_processing_hww(), state.data, state.len, state.cmd, state.cid);
        util_log("u2fhid packet len %d", (int)state.len);
        memset(&state, 0, sizeof(state));
    }
}

static void _in_handler(struct serial_link_frame* frame, struct ringbuffer* queue)
{
    switch (frame->type) {
    case SERIAL_LINK_TYPE_CTRL_DATA:
        _ctrl_handler(frame, queue);
        break;
    case SERIAL_LINK_TYPE_BLE_DATA:
        _hww_handler(frame, queue);
        break;
    default:
        break;
    }
}

void firmware_main_loop(void)
{
    uint8_t uart_read_buf[64] = {0};
    uint16_t uart_read_buf_len = 0;

// Might need to increase to fit bonddb later.
// Must be power of 2
#define UART_OUT_BUF_LEN 512

    struct ringbuffer uart_out_queue;
    uint8_t uart_out_buf[UART_OUT_BUF_LEN];
    ringbuffer_init(&uart_out_queue, &uart_out_buf, UART_OUT_BUF_LEN);

    struct SerialLinkIn serial_link;
    serial_link_in_init(&serial_link);

    struct serial_link_frame* frame = NULL;
    const uint8_t* data = NULL;

    while (1) {
        // UART IO
        if (uart_read_buf_len == 0) {
            uart_read_buf_len = uart_0_read(uart_read_buf, sizeof(uart_read_buf));
        }

        if (ringbuffer_num(&uart_out_queue) > 0) {
            // util_log("debug: %s", util_dbg_hex(uart_write_buf, uart_write_buf_len));
            uart_0_write_from_queue(&uart_out_queue);
        }

        // Process any IO from UART
        // Only poll serial_link if there is no pending frame
        if (!frame) {
            frame = serial_link_in_poll(&serial_link, uart_read_buf, &uart_read_buf_len);

            if (frame) {
                _in_handler(frame, &uart_out_queue);
                free(frame);
                frame = NULL;
            }
        }

        if (!data) data = queue_pull(queue_hww_queue());
        if (data) {
            util_log("out: %s...", util_dbg_hex(data, 32));
            uint8_t tmp[128];
            int len =
                serial_link_out_format(&tmp[0], sizeof(tmp), SERIAL_LINK_TYPE_BLE_DATA, data, 64);
            ASSERT(len < (int)sizeof(tmp));
            int32_t places;
            CRITICAL_SECTION_ENTER()
            places = uart_out_queue.size - ringbuffer_num(&uart_out_queue);
            CRITICAL_SECTION_LEAVE()
            if (places >= len) {
                for (int i = 0; i < len; i++) {
                    ringbuffer_put(&uart_out_queue, tmp[i]);
                }
                data = NULL;
            }
        }

        screen_process();
        /* And finally, run the high-level event processing. */

        rust_workflow_spin();
        rust_async_usb_spin();

        if (usb_is_enabled()) {
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
