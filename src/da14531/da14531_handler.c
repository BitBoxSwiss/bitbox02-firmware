// Copyright 2025 Shift Crypto AG
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

#include "da14531/da14531_handler.h"
#include "hardfault.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "ui/screen_stack.h"
#include "usb/usb_packet.h"
#include "utils_ringbuffer.h"
#include <ui/components/confirm.h>
#include <ui/fonts/monogram_5X9.h>

// Control commands
#define CTRL_CMD_DEVICE_NAME 1
#define CTRL_CMD_BOND_DB_GET 2
#define CTRL_CMD_BOND_DB_SET 3
#define CTRL_CMD_PAIRING_CODE 4
#define CTRL_CMD_BLE_STATUS 5
#define CTRL_CMD_IRK 6
#define CTRL_CMD_PRODUCT_STRING 7
#define CTRL_CMD_BLE_CHIP_RESET 8
#define CTRL_CMD_IDENTITY_ADDRESS 9
#define CTRL_CMD_PAIRING_SUCCESSFUL 10
#define CTRL_CMD_TK_CONFIRM 11
#define CTRL_CMD_DEBUG 254

static component_t* _ble_pairing_component = NULL;

struct pairing_callback {
    uint8_t key[4];
    struct ringbuffer* queue;
};

struct da14531_ctrl_frame {
    enum da14531_protocol_packet_type type;
    uint16_t payload_length; // includes length of cmd
    uint8_t cmd;
    uint8_t cmd_data[];
} __attribute((packed));

static struct pairing_callback _ble_pairing_callback_data;

static void _ble_pairing_callback(bool ok, void* param)
{
    struct pairing_callback* data = (struct pairing_callback*)param;

    uint8_t payload[18] = {0};
    payload[0] = 11; /* code for confirm pairind code message */
    memcpy(&payload[1], &data->key[0], sizeof(data->key));
    payload[17] = ok ? 1 : 0; /* 1 yes, 0 no */

    uint8_t tmp[32];
    uint16_t len = da14531_protocol_format(
        &tmp[0], sizeof(tmp), DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA, payload, sizeof(payload));
    ASSERT(len <= sizeof(tmp));
    ASSERT(ringbuffer_num(data->queue) + len <= data->queue->size);
    for (int i = 0; i < len; i++) {
        ringbuffer_put(data->queue, tmp[i]);
    }

    ui_screen_stack_pop();
    _ble_pairing_component = NULL;
}

static void _ctrl_handler(struct da14531_ctrl_frame* frame, struct ringbuffer* queue)
{
    uint8_t tmp[1152]; // 1024 + some margin (128)
    switch (frame->cmd) {
    case CTRL_CMD_DEVICE_NAME: {
        // util_log("da14531: get device name");
        //  1 byte cmd
        //  rest device name
        uint8_t response[1 + MEMORY_DEVICE_NAME_MAX_LEN];
        memset(&response[0], 0, sizeof(response));
        response[0] = 1;
        memory_get_device_name((char*)&response[1]);
        uint16_t len = da14531_protocol_format(
            &tmp[0],
            sizeof(tmp),
            DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
            &response[0],
            1 + strlen((char*)&response[1]));
        ASSERT(len <= sizeof(tmp));
        ASSERT(ringbuffer_num(queue) + len <= queue->size);
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case CTRL_CMD_BOND_DB_GET: {
        // util_log("da14531: get bond db");
        uint8_t response[1023]; // MAX BOND DB + 1 for cmd
        response[0] = 2;
        int16_t len = memory_get_ble_bond_db(&response[1]);
        // util_log("da14531: bond db len %d", len);
        uint16_t tmp_len;
        if (len != -1) {
            tmp_len = da14531_protocol_format(
                &tmp[0],
                sizeof(tmp),
                DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
                &response[0],
                1 + len);
        } else {
            tmp_len = da14531_protocol_format(
                &tmp[0], sizeof(tmp), DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA, &response[0], 1);
        }
        ASSERT(tmp_len <= sizeof(tmp));
        ASSERT(ringbuffer_num(queue) + tmp_len <= queue->size);
        for (int i = 0; i < tmp_len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case CTRL_CMD_BOND_DB_SET:
        // util_log("da14531: set bond db");
        // util_log("da14531: bond db len %d", frame->payload_length - 1);
        memory_set_ble_bond_db(&frame->cmd_data[0], frame->payload_length - 1);
        break;
    case CTRL_CMD_PAIRING_CODE: {
        if (frame->payload_length < 5) {
            // TODO handle error.
            Abort("Invalid payload length for BLE pairing code");
        }
        memcpy(
            &(_ble_pairing_callback_data.key)[0],
            &frame->cmd_data[0],
            sizeof(_ble_pairing_callback_data.key));
        _ble_pairing_callback_data.queue = queue;
        uint32_t pairing_code_int = (*(uint32_t*)&frame->cmd_data[0]) % 1000000;
        char pairing_code[7] = {0};
        snprintf(pairing_code, sizeof(pairing_code), "%06lu", (long unsigned int)pairing_code_int);
        // util_log("da14531: show/confirm pairing code: %s", pairing_code);
        const confirm_params_t confirm_params = {
            .title = "Pairing code",
            .body = pairing_code,
            .font = &font_monogram_5X9,
        };
        _ble_pairing_component = confirm_create(
            &confirm_params, _ble_pairing_callback, (void*)&_ble_pairing_callback_data);
        ui_screen_stack_push(_ble_pairing_component);
    } break;
    case CTRL_CMD_BLE_STATUS:
        // util_log("da14531: BLE status update");
        switch (frame->cmd_data[0]) {
        case 0:
            // util_log("da14531: adveritising");
            if (_ble_pairing_component != NULL && ui_screen_stack_top() == _ble_pairing_component) {
                ui_screen_stack_pop();
                _ble_pairing_component = NULL;
            }
            break;
        case 1:
            // util_log("da14531: connected");
            break;
        case 2:
            // util_log("da14531: connected secure");
            break;
        default:
            break;
        }
        break;
    case CTRL_CMD_IRK: {
        // util_log("da14531: get irk");
        //  1 byte cmd
        //  16 bytes irk
        uint8_t response[17] = {0};
        response[0] = 6;
        memory_get_ble_irk(&response[1]);
        uint16_t len = da14531_protocol_format(
            &tmp[0],
            sizeof(tmp),
            DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
            &response[0],
            sizeof(response));
        ASSERT(len <= sizeof(tmp));
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case CTRL_CMD_PRODUCT_STRING: {
        // util_log("da14531: get device mode");
#if defined(BOOTLOADER)
#define DEVICE_MODE "{\"p\":\"bb02p-bl-multi\",\"v\":\"1.1.0\"}"
#else
#define DEVICE_MODE "{\"p\":\"bb02p-multi\",\"v\":\"9.22.0\"}"
#endif
        uint8_t response[64] = {0};
        response[0] = 7;
        memcpy(&response[1], DEVICE_MODE, sizeof(DEVICE_MODE) - 1);
        uint16_t len = da14531_protocol_format(
            &tmp[0],
            sizeof(tmp),
            DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
            &response[0],
            1 + sizeof(DEVICE_MODE) - 1);
        ASSERT(len <= sizeof(tmp));
        ASSERT(ringbuffer_num(queue) + len <= queue->size);
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case CTRL_CMD_IDENTITY_ADDRESS: {
        // util_log("da14531: get addr");
        //  1 byte cmd
        //  6 bytes addr
        uint8_t response[7] = {0};
        response[0] = 9;
        memory_get_ble_identity_address(&response[1]);
        uint16_t len = da14531_protocol_format(
            &tmp[0],
            sizeof(tmp),
            DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
            &response[0],
            sizeof(response));
        ASSERT(len <= sizeof(tmp));
        ASSERT(ringbuffer_num(queue) + len <= queue->size);
        for (int i = 0; i < len; i++) {
            ringbuffer_put(queue, tmp[i]);
        }
    } break;
    case CTRL_CMD_PAIRING_SUCCESSFUL:
        // util_log("da14531: pairing successful");
        break;
    case CTRL_CMD_DEBUG:
        util_log(
            "da14531-debug: %.*s (%d bytes)",
            frame->payload_length - 1,
            &frame->cmd_data[0],
            frame->payload_length - 1);
        break;
    default:
        break;
    }
}

static void _hww_handler(struct da14531_protocol_frame* frame, struct ringbuffer* queue)
{
    // util_log(" in: %s", util_dbg_hex(frame->payload, 64));
    (void)queue;
    ASSERT(frame->payload_length == 64);
    usb_packet_process((USB_FRAME*)&frame->payload[0]);
}

// Handler must not use the frame pointer after it has returned
void da14531_handler(struct da14531_protocol_frame* frame, struct ringbuffer* queue)
{
    // util_log("handler called");
    switch (frame->type) {
    case DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA:
        _ctrl_handler((struct da14531_ctrl_frame*)frame, queue);
        break;
    case DA14531_PROTOCOL_PACKET_TYPE_BLE_DATA:
        _hww_handler(frame, queue);
        break;
    default:
        break;
    }
}
