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

#include "da14531/da14531_protocol.h"
#include "crc.h"
#include "da14531/da14531_binary.h"
#include "platform_config.h"
#include "uart.h"
#include "util.h"
#include <stdlib.h>
#include <utils_assert.h>
#ifndef TESTING
#include "dap.h"
#else
#include "mock_dap.h"
#endif

#include <hardfault.h>
#include <memory/memory_spi.h>

#include <stdbool.h>
#include <stdint.h>

enum firmware_loader_state {
    FIRMWARE_LOADER_STATE_IDLE,
    FIRMWARE_LOADER_STATE_SEEN_STX,
    FIRMWARE_LOADER_STATE_SENT_HEADER,
    FIRMWARE_LOADER_STATE_SENT_FIRMWARE,
    FIRMWARE_LOADER_STATE_DONE,
};

struct firmware_loader {
    enum firmware_loader_state state;
};

enum serial_link_in_state {
    // In the READING state we wait for a complete frame
    SERIAL_LINK_STATE_READING,
    // In the CHECK state we validate the frame and return it iff valid
    SERIAL_LINK_STATE_CHECK,
};

enum escape_state {
    // WAIT is the state we are in before we have seen a single SOH. When we detect a SOH we go to
    // ACCEPT state.
    ESCAPE_STATE_WAIT,
    // ACCEPT is the state we are in when we are accepting characters. As soon as we see the escape
    // character here we drop it and go to the ESCAPE state.
    // We allow a single SOH between frames, so during normal operations we never go back to WAIT
    // state. We only go back to WAIT state in case we need to load the firmware again.
    // When we see the SOH character we issue a frame and reset the frame buffer
    ESCAPE_STATE_ACCEPT,
    // In the ESCAPE state we un-escape a single character and then go back to ACCEPT state
    ESCAPE_STATE_ESCAPE,
};

struct serial_link_in {
    enum serial_link_in_state state;
    enum escape_state escape_state;
    uint8_t buf_in[128];
    size_t buf_in_len;
    uint8_t frame[2048];
    size_t frame_len;
};

struct da14531_protocol {
    struct serial_link_in serial_link;
    struct firmware_loader loader;
};

static struct da14531_protocol _protocol;

#if 0
static const char* _firmware_loader_state_str(enum firmware_loader_state state)
{
    switch (state) {
    case FIRMWARE_LOADER_STATE_IDLE:
        return "IDLE";
    case FIRMWARE_LOADER_STATE_SEEN_STX:
        return "SEEN_STX";
    case FIRMWARE_LOADER_STATE_SENT_HEADER:
        return "SENT_SOH";
    case FIRMWARE_LOADER_STATE_SENT_FIRMWARE:
        return "SENT_FIRMWARE";
    case FIRMWARE_LOADER_STATE_DONE:
        return "DONE";
    default:
        return "(invalid)";
    }
}
#endif

static void _firmware_loader_init(struct firmware_loader* self)
{
// If we are in bootloader or factory setup we expect to load the ble firmware, so we start in IDLE.
// In production firmware we expect the da14531 to already be booted.
#if defined(BOOTLOADER) || FACTORYSETUP == 1
    self->state = FIRMWARE_LOADER_STATE_IDLE;
#else
    self->state = FIRMWARE_LOADER_STATE_DONE;
#endif
}

#define SOH 0x01
#define STX 0x02
#define ACK 0x06
#define NAK 0x15

static void _firmware_loader_poll(
    struct firmware_loader* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len,
    struct ringbuffer* out_queue)
{
    static uint8_t* ble_fw = NULL;
    static size_t ble_fw_size = 0;
    static uint8_t ble_fw_checksum = 0;

    // if (*buf_in_len > 0) {
    //     util_log(
    //         "%s, got bytes %s",
    //         _firmware_loader_state_str(self->state),
    //         util_dbg_hex(buf_in, *buf_in_len));
    // }

    switch (self->state) {
    case FIRMWARE_LOADER_STATE_IDLE:
        if (ble_fw == NULL) {
            if (!memory_spi_get_active_ble_firmware(&ble_fw, &ble_fw_size, &ble_fw_checksum)) {
                Abort("TODO");
            }
            *buf_in_len = 0;
            break;
        }

        for (uint16_t i = 0; i < *buf_in_len; i++) {
            if (buf_in[i] == STX) {
                util_log("da14531: requested firmware");
                self->state = FIRMWARE_LOADER_STATE_SEEN_STX;
                break;
            }
        }
        *buf_in_len = 0;
        break;
    case FIRMWARE_LOADER_STATE_SEEN_STX: {
        ASSERT(ringbuffer_num(out_queue) + 3 <= out_queue->size);
        ringbuffer_put(out_queue, SOH);
        ringbuffer_put(out_queue, ble_fw_size & 0xff);
        ringbuffer_put(out_queue, (ble_fw_size >> 8) & 0xff);
        self->state = FIRMWARE_LOADER_STATE_SENT_HEADER;
    } break;

    case FIRMWARE_LOADER_STATE_SENT_HEADER:
        if (*buf_in_len == 1) {
            if (buf_in[0] == ACK) {
                util_log("da14513: sending firmware");
                // Wait until uart tx is ready, and issue a write for the firmware.
                // Don't use ringbuffer as the source is static
                if (ble_fw != NULL) {
                    // This should never happen
                    // TODO
                }

                while (!(uart_0_write(ble_fw, ble_fw_size)));
                self->state = FIRMWARE_LOADER_STATE_SENT_FIRMWARE;
            } else {
                self->state = FIRMWARE_LOADER_STATE_IDLE;
            }
            *buf_in_len = 0;
        }
        break;
    case FIRMWARE_LOADER_STATE_SENT_FIRMWARE:
        if (*buf_in_len == 1) {
            if (ble_fw == NULL || ble_fw_size == 0) {
                Abort("ble_fw is NULL");
            }
            if (buf_in[0] == ble_fw_checksum) {
                util_log("da14531: checksum success (%x)", buf_in[0]);
                ASSERT(ringbuffer_num(out_queue) + 1 <= out_queue->size);
                ringbuffer_put(out_queue, ACK);
                self->state = FIRMWARE_LOADER_STATE_DONE;
            } else {
                util_log(
                    "da14531: checksum failure, their:%02X, our:%02X", buf_in[0], ble_fw_checksum);
                self->state = FIRMWARE_LOADER_STATE_IDLE;
            }
            ASSERT(buf_in[0] == ble_fw_checksum);
            free(ble_fw);
            ble_fw = NULL;
            *buf_in_len = 0;
        }
        break;
    case FIRMWARE_LOADER_STATE_DONE:
    default:
        break;
    }
}

static void _serial_link_in_init(struct serial_link_in* self)
{
    self->state = SERIAL_LINK_STATE_READING;
    self->escape_state = ESCAPE_STATE_WAIT;
    memset(self->buf_in, 0x55, sizeof(self->buf_in));
    self->buf_in_len = 0;
    memset(self->frame, 0x55, sizeof(self->frame));
    self->frame_len = 0;
}

#define SL_SOF 0x7E
#define SL_ESCAPE 0x7D
#define SL_XOR 0x20

static struct da14531_protocol_frame* _serial_link_in_poll(
    struct serial_link_in* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len)
{
    // copy over new bytes
    for (uint16_t i = 0; i < *buf_in_len && self->buf_in_len < sizeof(self->buf_in); i++) {
        self->buf_in[self->buf_in_len++] = buf_in[i];
    }
    ASSERT(self->buf_in_len < sizeof(self->buf_in));
    *buf_in_len = 0;

    switch (self->state) {
    case SERIAL_LINK_STATE_READING: {
        int len = self->buf_in_len;
        for (int i = 0; i < len; i++) {
            // util_log("i:%d,b:%02x,l:%u", i, self->buf_in[i], self->buf_in_len);
            self->buf_in_len--;
            // Reset firmware loader on STX
            if (self->buf_in[i] == STX) {
                // Reset ourselves
                _serial_link_in_init(self);
                // It is very important that we go straight to SEEN_STX. In case the BLE chip hasn't
                // been configured it will only request the firmware once. We must not miss that
                // request.
                _protocol.loader.state = FIRMWARE_LOADER_STATE_SEEN_STX;
                break;
            }
            // Always reset on SOF
            if (self->buf_in[i] == SL_SOF) {
                // We allow a single SOF between frames, which is why we don't reset to "WAIT"
                self->escape_state = ESCAPE_STATE_ACCEPT;
                if (self->frame_len > 0) {
                    // save the bytes that wasn't consumed
                    memcpy(&self->buf_in[0], &self->buf_in[i + 1], self->buf_in_len);
                    self->state = SERIAL_LINK_STATE_CHECK;
                    break;
                }
                continue;
            }

            switch (self->escape_state) {
            case ESCAPE_STATE_WAIT:
                break;
            case ESCAPE_STATE_ACCEPT:
                if (self->buf_in[i] == SL_ESCAPE) {
                    self->escape_state = ESCAPE_STATE_ESCAPE;
                } else {
                    self->frame[self->frame_len++] = self->buf_in[i];
                }
                break;
            case ESCAPE_STATE_ESCAPE: {
                uint8_t c = self->buf_in[i] ^ SL_XOR;
                self->frame[self->frame_len++] = c;
                self->escape_state = ESCAPE_STATE_ACCEPT;
            } break;
            default:
                break;
            }
        }
        // util_log("frame len so far: %d", self->frame_len);
    } break;
    case SERIAL_LINK_STATE_CHECK: {
        // bytes with index 1-2 are the length
        uint16_t len = *((uint16_t*)&self->frame[1]);

        if (len > self->frame_len - 5) {
            util_log("da14531: ERROR, invalid len %d, dropped frame", len);
            util_log(
                "da14531: frame_len: %u, frame: %s",
                (unsigned)self->frame_len,
                util_dbg_hex(self->frame, (int)self->frame_len));
            self->state = SERIAL_LINK_STATE_READING;
            self->frame_len = 0;
            return NULL;
        }

        // CRC in frame
        // bytes with index n-2 and n-1 are the crc
        uint16_t crc_frame = *(uint16_t*)&self->frame[3 + len];

        // Recalculate CRC
        crc_t crc = crc_init();
        crc = crc_update(crc, &self->frame[0], 3 + len);
        crc = crc_finalize(crc);

        self->state = SERIAL_LINK_STATE_READING;
        self->frame_len = 0;

        ASSERT(crc == crc_frame);
        if (crc == crc_frame) {
            return (struct da14531_protocol_frame*)&self->frame[0];
        }
        util_log("da14531: ERROR, invalid crc, dropped frame");
    } break;
    default:
        break;
    }
    return NULL;
}

static void _serial_link_format_byte(uint8_t data, uint8_t* buf, uint16_t buf_len, uint16_t* idx)
{
    ASSERT(*idx + 2 < buf_len);
    (void)buf_len;
    switch (data) {
    case SL_SOF:
    case SL_ESCAPE:
        buf[(*idx)++] = SL_ESCAPE;
        buf[(*idx)++] = data ^ SL_XOR;
        break;
    default:
        buf[(*idx)++] = data;
        break;
    }
}

uint16_t da14531_protocol_format(
    uint8_t* buf,
    uint16_t buf_len,
    enum da14531_protocol_packet_type type,
    const uint8_t* payload,
    uint16_t payload_len)
{
    uint16_t idx = 0;
    crc_t crc = crc_init();

    ASSERT(idx + 1 < buf_len);
    buf[idx++] = SL_SOF;

    crc = crc_update(crc, &type, 1);
    _serial_link_format_byte(type, buf, buf_len, &idx);

    uint8_t len = payload_len & 0xff;
    crc = crc_update(crc, &len, 1);
    _serial_link_format_byte(len, buf, buf_len, &idx);

    len = (payload_len >> 8) & 0xff;
    crc = crc_update(crc, &len, 1);
    _serial_link_format_byte(len, buf, buf_len, &idx);

    for (int i = 0; i < payload_len; i++) {
        _serial_link_format_byte(payload[i], buf, buf_len, &idx);
    }

    crc = crc_update(crc, &payload[0], payload_len);
    crc = crc_finalize(crc);

    // crc_t is the "fastest" type that holds u16, so can be longer than 2 bytes
    for (unsigned int i = 0; i < sizeof(uint16_t); i++) {
        _serial_link_format_byte(crc & 0xff, buf, buf_len, &idx);
        crc >>= 8;
    }
    ASSERT(idx + 1 < buf_len);
    buf[idx++] = SL_SOF;
    return idx;
}

struct da14531_protocol_frame* da14531_protocol_poll(
    uint8_t* in_buf,
    uint16_t* in_buf_len,
    const uint8_t* hww_data,
    struct ringbuffer* out_queue)
{
    if (hww_data) {
        util_log("out: %s", util_dbg_hex(hww_data, 64));
        uint8_t tmp[128];
        int len = da14531_protocol_format(
            &tmp[0], sizeof(tmp), DA14531_PROTOCOL_PACKET_TYPE_BLE_DATA, hww_data, 64);
        ASSERT(len < (int)sizeof(tmp));
        ASSERT(ringbuffer_num(out_queue) + len <= out_queue->size);
        for (int i = 0; i < len; i++) {
            ringbuffer_put(out_queue, tmp[i]);
        }
    }
    struct da14531_protocol_frame* frame = NULL;
    if (_protocol.loader.state != FIRMWARE_LOADER_STATE_DONE) {
        _firmware_loader_poll(&_protocol.loader, in_buf, in_buf_len, out_queue);
    } else {
        frame = _serial_link_in_poll(&_protocol.serial_link, in_buf, in_buf_len);
    }
    return frame;
}

#if FACTORYSETUP == 1 || !defined(NDEBUG)
static bool _swd_reset_da14531(void)
{
    dap_init();
    dap_connect();
    dap_reset_link();
    if (!dap_target_prepare(1000)) {
        // SWD not available
        // If it has run the factory setup this is expected in bootloader mode
        // Otherwise device might be dead
        return false;
    }
    dap_target_select();
    uint32_t id = dap_read_idcode();
    if (id != 0xbc11477) {
        util_log("da14531: ERROR: Invalid idcode: %x", (unsigned int)id);
    } else {
        util_log("da14531: Connected to BT chip");
    }

    // remap address space to ROM
    uint16_t w = dap_read_hword(0x50000012);
    // We check if all bits except for the memory-mapping (3 smallest bits) are the same.
    // The memory mapping is 0 i the device is in bootloader and 2 if it is in firmware mode
    if ((w & ~0x3) != 0x01a0) {
        util_log("da14531: ERROR: SYS_CTRL_REG: %04x, expected 0x01a2/0x01a0", (unsigned int)w);
        return false;
    }
    w &= ~3;
    dap_write_hword(0x50000012, w);
    // Issue reset (deselect resets chip)
    dap_target_deselect();
    dap_disconnect();
    return true;
}
#endif

void da14531_protocol_init(void)
{
    _firmware_loader_init(&_protocol.loader);
    _serial_link_in_init(&_protocol.serial_link);

// Only attempt swd reset in factory setup or debug builds. In production swd is turned off and
// this is therefore useless.
#if FACTORYSETUP == 1 || !defined(NDEBUG)
    // Reset the device if possible, if we cannot reset it over SWD, it must already be running
    if (!_swd_reset_da14531()) {
        util_log("da14531: Failed to reset over SWD");
    }
#endif
}
