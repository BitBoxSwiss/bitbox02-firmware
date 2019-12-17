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

#include "hardfault.h"
// TODO: get rid of this dependency when USB_DATA_MAX_LEN/USB_REPORT_SIZE is
// removed.
#include "usb/usb_frame.h"

// TODO: specify generic size
// The queue has enough room for a single maximum size packet
#define QUEUE_NUM_REPORTS (USB_DATA_MAX_LEN / USB_REPORT_SIZE)
#define QUEUE_SIZE (QUEUE_NUM_REPORTS * USB_REPORT_SIZE)

// `start` and `end` are indices into `items`
struct queue {
    uint32_t volatile start;
    uint32_t volatile end;
    size_t item_size;
    uint8_t items[QUEUE_SIZE];
};

void queue_clear(struct queue* ctx)
{
    util_zero(ctx->items, sizeof(ctx->items));
    ctx->start = ctx->end = 0;
}

void queue_init(struct queue* ctx, size_t item_size)
{
    ctx->item_size = item_size;
    /*
     * The queue only works if the size of each item is a submultiple of
     * QUEUE_SIZE.
     */
    if (QUEUE_SIZE % item_size != 0) {
        Abort("Queue initialized with wrong item size.");
    }
    queue_clear(ctx);
}

const uint8_t* queue_pull(struct queue* ctx)
{
    uint32_t p = ctx->start;
    if (p == ctx->end) {
        // queue is empty
        return NULL;
    }
    ctx->start = (p + ctx->item_size) % QUEUE_SIZE;
    return ctx->items + p;
}

queue_error_t queue_push(struct queue* ctx, const uint8_t* data)
{
    uint32_t next = (ctx->end + ctx->item_size) % QUEUE_SIZE;
    if (ctx->start == next) {
        return QUEUE_ERR_FULL; // Buffer full
    }
    memcpy(ctx->items + ctx->end, data, ctx->item_size);
    ctx->end = next;
    return QUEUE_ERR_NONE;
}

const uint8_t* queue_peek(struct queue* ctx)
{
    uint32_t p = ctx->start;
    if (p == ctx->end) {
        // queue is empty
        return NULL;
    }
    return ctx->items + p;
}

struct queue* queue_hww_queue(void)
{
    static struct queue queue;
    return &queue;
}

#if APP_U2F == 1
struct queue* queue_u2f_queue(void)
{
    static struct queue queue;
    return &queue;
}
#endif
