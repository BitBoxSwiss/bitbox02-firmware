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

#include "da14531_flasher.h"
#include "dap.h"
#include "util.h"

#define TIMEOUT 1000000

const char* flashing_state_str(enum flashing_state state)
{
    switch (state) {
    case FLASHING_STATE_IDLE:
        return "IDLE";
    case FLASHING_STATE_SEEN_STX:
        return "SEEN_STX";
    case FLASHING_STATE_SENT_SOH:
        return "SENT_SOH";
    case FLASHING_STATE_SEEN_SOH_ACK:
        return "SEEN_SOH_ACK";
    case FLASHING_STATE_SENT_FIRMWARE:
        return "SENT_FIRMWARE";
    case FLASHING_STATE_SEND_CHKSUM_ACK:
        return "SEND_CHKSUM_ACK";
    case FLASHING_STATE_DONE:
        return "DONE";
    default:
        return "(invalid)";
    }
}
static uint8_t checksum(const uint8_t* buf, uint16_t buf_len)
{
    uint8_t res = 0;
    for (uint16_t i = 0; i < buf_len; ++i) {
        res ^= buf[i];
    }
    return res;
}

void flasher_init(struct Flasher* self, const uint8_t* firmware_start, uint16_t firmware_size)
{
    self->state = FLASHING_STATE_IDLE;
    self->buf_in_len = 0;
    self->firmware_start = firmware_start;
    self->firmware_size = firmware_size;
    self->firmware_chksum = checksum(firmware_start, firmware_size);
    self->bytes_sent = 0;
    self->timeout = TIMEOUT;
}

#define SOH 0x01
#define STX 0x02
#define ACK 0x06
#define NAK 0x15

void flasher_poll(
    struct Flasher* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len,
    const uint8_t** buf_out,
    uint16_t* buf_out_len)
{
    // copy over new bytes
    for (uint16_t i = 0; i < *buf_in_len; i++) {
        self->buf_in[i] = buf_in[i];
    }
    if (*buf_in_len > 0) {
        util_log(
            "%s, got bytes %s",
            flashing_state_str(self->state),
            util_dbg_hex(self->buf_in, *buf_in_len));
    }
    self->buf_in_len = *buf_in_len;
    *buf_in_len = 0;

    switch (self->state) {
    case FLASHING_STATE_IDLE:
        for (uint16_t i = 0; i < self->buf_in_len; i++) {
            if (self->buf_in[i] == STX) {
                util_log("da14531: requested firmware");
                // delay_ms(500); // 20ms is OK, 30 is NOT
                self->state = FLASHING_STATE_SEEN_STX;
                break;
            }
        }
        self->buf_in_len = 0;
        self->timeout -= 1;
        break;
    case FLASHING_STATE_SEEN_STX:
        util_log("da14531: sending start of header");
        self->buf_out_small[0] = SOH;
        self->buf_out_small[1] = self->firmware_size & 0xff;
        self->buf_out_small[2] = (self->firmware_size >> 8) & 0xff;
        *buf_out = &self->buf_out_small[0];
        *buf_out_len = 3;
        self->state = FLASHING_STATE_SENT_SOH;
        break;
    case FLASHING_STATE_SENT_SOH:
        if (self->buf_in_len == 1) {
            if (self->buf_in[0] == ACK) {
                util_log("da14531: length acked");
                self->state = FLASHING_STATE_SEEN_SOH_ACK;
            } else {
                self->state = FLASHING_STATE_IDLE;
            }
            self->buf_in_len = 0;
        }
        break;
    case FLASHING_STATE_SEEN_SOH_ACK:
        if (self->bytes_sent == 0) {
            util_log("da14513: sending firmware");
        }
        *buf_out_len = self->firmware_size;
        *buf_out = self->firmware_start;
        self->state = FLASHING_STATE_SENT_FIRMWARE;
        break;
    case FLASHING_STATE_SENT_FIRMWARE:
        if (self->buf_in_len == 1) {
            if (self->buf_in[0] == self->firmware_chksum) {
                util_log("da14531: success");
                self->state = FLASHING_STATE_SEND_CHKSUM_ACK;
            } else {
                util_log("da14531: failure %02X %02X", self->buf_in[0], self->firmware_chksum);
                self->state = FLASHING_STATE_IDLE;
            }
        }
        break;
    case FLASHING_STATE_SEND_CHKSUM_ACK:
        util_log("da14531: ACK checksum");
        self->buf_out_small[0] = ACK;
        *buf_out = &self->buf_out_small[0];
        *buf_out_len = 1;
        self->state = FLASHING_STATE_DONE;
        break;
    case FLASHING_STATE_DONE:
    default:
        break;
    }
}

void flasher_reset(struct Flasher* self)
{
    self->state = FLASHING_STATE_IDLE;
    self->timeout = TIMEOUT;
}

void flasher_set_done(struct Flasher* self)
{
    self->state = FLASHING_STATE_DONE;
}

bool flasher_timed_out(struct Flasher* self)
{
    return self->state == FLASHING_STATE_IDLE && self->timeout < 0;
}
bool flasher_done(struct Flasher* self)
{
    return self->state == FLASHING_STATE_DONE;
}

bool da14531_swd_reset(void)
{
    dap_init();
    dap_connect();
    dap_reset_link();
    if (!dap_target_prepare(1000)) {
        // SWD not available
        // This likely means that the chip will try to boot repeatedly via UART
        // and a reset is unnecessary
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
    if (w != 0x01a2) {
        util_log("da14531: ERROR: SYS_CTRL_REG: %04x, expected 0x01a2", (unsigned int)w);
        return false;
    }
    w &= ~3;
    dap_write_hword(0x50000012, w);
    // Issue reset (deselect resets chip)
    dap_target_deselect();
    dap_disconnect();
    return true;
}
