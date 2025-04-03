#include "da14531_serial_link.h"
#include "crc.h"
#include "util.h"
#include <stdlib.h>
#include <utils_assert.h>

static struct serial_link_frame* _serial_link_frame_alloc(uint16_t length)
{
    return (struct serial_link_frame*)malloc(3 + length);
}

void serial_link_in_init(struct SerialLinkIn* self)
{
    self->state = SERIAL_LINK_STATE_READING;
    self->escape_state = ESCAPE_STATE_WAIT;
    memset(self->buf_in, 0x55, sizeof(self->buf_in));
    self->buf_in_len = 0;
    memset(self->frame, 0x55, sizeof(self->frame));
    self->frame_len = 0;
    self->buf_out_len = 0;
    // self->counter = 0;
}

#define SL_SOF 0x7E
#define SL_EOF 0x7E
#define SL_ESCAPE 0x7D
#define SL_XOR 0x20

struct serial_link_frame* serial_link_in_poll(
    struct SerialLinkIn* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len)
{
    // copy over new bytes
    for (uint16_t i = 0; i < *buf_in_len && self->buf_in_len < sizeof(self->buf_in); i++) {
        self->buf_in[self->buf_in_len++] = buf_in[i];
    }
    if (*buf_in_len > 0) {
        // util_log("%d, got bytes %s", *buf_in_len, util_dbg_hex(self->buf_in, self->buf_in_len));
    }
    if (self->buf_in_len == sizeof(self->buf_in)) {
        // util_log("buf in maxed out");
    }
    *buf_in_len = 0;

    switch (self->state) {
    case SERIAL_LINK_STATE_READING: {
        int len = self->buf_in_len;
        for (int i = 0; i < len; i++) {
            // util_log("i:%d,b:%02x,l:%u", i, self->buf_in[i], self->buf_in_len);
            self->buf_in_len--;
            // Always reset on SOF
            if (self->buf_in[i] == SL_SOF) {
                if (self->frame_len >= 1) {
                    // save the bytes that wasn't consumed
                    memcpy(&self->buf_in[0], &self->buf_in[i + 1], self->buf_in_len);
                    self->state = SERIAL_LINK_STATE_CHECK;
                    break;
                }
                self->escape_state = ESCAPE_STATE_ACCEPT;
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
        util_log("frame len %u", self->frame_len);
        uint8_t type = self->frame[0];
        uint16_t len = self->frame[1] | (self->frame[2] << 8);
        util_log("da14531: type: %x payload_len %d %c", type, len, self->frame[3]);

        if (len != self->frame_len - 5) {
            util_log("da14531: ERROR, invalid len %d, dropped frame", len);
            util_log(
                "da14531: frame_len: %u, frame: %s",
                self->frame_len,
                util_dbg_hex(self->frame, self->frame_len));
            self->state = SERIAL_LINK_STATE_READING;
            self->frame_len = 0;
            return NULL;
        }

        // CRC in frame
        uint16_t crc_frame = self->frame[3 + len] | self->frame[3 + len + 1] << 8;
        // Recalculate CRC
        crc_t crc = crc_init();
        crc = crc_update(crc, &self->frame[0], 3 + len);
        crc = crc_finalize(crc);

        // reset frame_len
        self->state = SERIAL_LINK_STATE_READING;
        self->frame_len = 0;

        if (crc == crc_frame) {
            struct serial_link_frame* frame = _serial_link_frame_alloc(len);
            frame->type = type;
            frame->payload_length = len;
            memcpy(&frame->payload[0], &self->frame[3], len);
            util_log("da14531: payload: %s", util_dbg_hex(frame->payload, frame->payload_length));
            return frame;
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

/// Formats a packet into buf for sending over serial
/// Returns number of bytes formatted
uint16_t serial_link_out_format(
    uint8_t* buf,
    uint16_t buf_len,
    uint8_t type,
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

    // crc_t is the "fastest" type that holds u16, so can be longer than 2
    // bytes
    for (unsigned int i = 0; i < sizeof(uint16_t); i++) {
        _serial_link_format_byte(crc & 0xff, buf, buf_len, &idx);
        crc >>= 8;
    }
    ASSERT(idx + 1 < buf_len);
    buf[idx++] = SL_SOF;
    return idx;
}
