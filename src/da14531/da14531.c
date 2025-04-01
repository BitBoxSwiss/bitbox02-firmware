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

#include "da14531.h"
#include "da14531_protocol.h"
#include "util.h"
#include "utils_ringbuffer.h"

void da14531_power_down(struct ringbuffer* uart_out)
{
    util_log("da14531_power_down");
    // TODO: 12 is defined in da14531_handler.c
    uint8_t payload[2] = {12, 0};
    uint8_t buf[10] = {0};
    int len = da14531_protocol_format(
        &buf[0], sizeof(buf), DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA, &payload[0], sizeof(payload));
    for (int i = 0; i < len; i++) {
        ringbuffer_put(uart_out, buf[i]);
    }
}
