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

#ifndef DA14531_PROTOCOL_H
#define DA14531_PROTOCOL_H
#include "utils_ringbuffer.h"
#include <stdint.h>

enum da14531_protocol_packet_type {
    DA14531_PROTOCOL_PACKET_TYPE_ACK = 0x2d, /* 0b00101101*/
    DA14531_PROTOCOL_PACKET_TYPE_NAK = 0x5a, /*0b01011010*/
    DA14531_PROTOCOL_PACKET_TYPE_BLE_DATA = 0x3C, /*0b00111100*/
    DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA = 0xb4, /*0b10110100*/
    DA14531_PROTOCOL_PACKET_TYPE_PING = 0x4b, /*0b01001011*/
};

struct da14531_protocol_frame {
    enum da14531_protocol_packet_type type;
    uint16_t payload_length;
    uint8_t payload[];
} __attribute((packed));

void da14531_protocol_init(void);

// Will return true once the firmware is loaded to the BLE chip
void da14531_protocol_firmware_loaded_done(void);

// Poll will try to build a complete frame from data in the in_buf and return that as a frame. If
// hww_data != NULL it will also format and queue that for transmission.
// hww_data is a 64 byte u2fhid packet
// returns a pointer to static memory if a valid frame has been parsed out otherwise NULL
// The data the frame pointer points to is invalidating the next time this function is called.
struct da14531_protocol_frame* da14531_protocol_poll(
    uint8_t* in_buf,
    uint16_t* in_buf_len,
    const uint8_t* hww_data,
    struct ringbuffer* out_queue);

/// Formats a packet into buf for sending over serial
/// Returns number of bytes formatted
uint16_t da14531_protocol_format(
    uint8_t* buf,
    uint16_t buf_len,
    enum da14531_protocol_packet_type type,
    const uint8_t* payload,
    uint16_t payload_len);
#endif
