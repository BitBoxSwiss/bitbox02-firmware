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

#include "queue.h"
#include <string.h>
#include <util.h>
// TODO: get rid of this dependency when USB_DATA_MAX_LEN/USB_REPORT_SIZE is
// removed.
#include "usb/usb_frame.h"

static uint32_t _index_start = 0;
static uint32_t _index_end = 0;
// We use the queue to send and receive usb packets. Allocate enough space in
// case all pulls come after all pushes.
#define QUEUE_NUM_PACKETS ((USB_DATA_MAX_LEN / USB_REPORT_SIZE) * 2)
// TODO: specify generic size
static uint8_t _packets[QUEUE_NUM_PACKETS][USB_REPORT_SIZE];

void queue_clear(void)
{
    util_zero(_packets, sizeof(_packets));
    _index_start = _index_end;
}

const uint8_t* queue_pull(void)
{
    uint32_t p = _index_start;
    if (p == _index_end) {
        // queue is empty
        return NULL;
    }
    _index_start = (p + 1) % QUEUE_NUM_PACKETS;
    return _packets[p];
}

uint8_t queue_push(const uint8_t* data)
{
    uint32_t next = (_index_end + 1) % QUEUE_NUM_PACKETS;
    if (_index_start == next) {
        return ERR_QUEUE_FULL; // Buffer full
    }
    memcpy(_packets[_index_end], data, USB_REPORT_SIZE);
    _index_end = next;
    return ERR_NONE;
}

const uint8_t* queue_peek(void)
{
    uint32_t p = _index_start;
    if (p == _index_end) {
        // queue is empty
        return NULL;
    }
    return _packets[p];
}
