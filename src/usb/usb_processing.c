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

#include "usb_processing.h"
#include "u2f/u2f_packet.h"
#include "usb_frame.h"
#include "usb_packet.h"

#include <hardfault.h>

#include <queue.h>
#include <stdlib.h>
#include <util.h>

struct usb_processing {
    CMD_Callback* registered_cmds;
    uint32_t registered_cmds_len;
    /* Whether the content of in_packet is a new, complete incoming packet. */
    bool has_packet;
    struct queue* (*out_queue)(void);
    void (*send)(void);
    usb_frame_formatter_t format_frame;
};

/*
 * FUTURE: this can be removed and packets can be queued when
 * hww workflows are independent from the USB processing layer.
 *
 * At that point, we can just use the usb_processing.has_packet flags to make
 * sure that we don't drop packets, send FRAME_ERR_CHANNEL_BUSY when layer-1
 * is busy (i.e. we are in the process of buffering a frame) and continuously accept
 * frames from both stacks (so we don't send FRAME_ERR_CHANNEL_BUSY improperly when
 * it's the user interface that is busy, and not the USB port).
 *
 * For now this is impossible as the UI being busy keeps the USB port busy as well...
 */
static volatile bool _in_packet_queued;

/*
 * Contains the USB packet that is currently being processed.
 * This is only valid if _in_packet_queued is true.
 * It is shared between all stacks (as we only process one packet at the time,
 * and send out FRAME_ERR_CHANNEL_BUSY if we are still processing the buffered one
 * and a new one arrives).
 */
static Packet _in_packet;

// TODO: remove this global in future refactoring
static struct queue* _global_queue;

static queue_error_t _queue_push(const uint8_t* data)
{
    if (_global_queue == NULL) {
        Abort("usb_processing: Internal error");
    }
    return queue_push(_global_queue, data);
}

/**
 * Responds with data of a certain length.
 * @param[in] packet The packet to be sent.
 */
static queue_error_t _enqueue_frames(struct usb_processing* ctx, const Packet* out_packet)
{
    _global_queue = ctx->out_queue();
    return ctx->format_frame(
        out_packet->cmd, out_packet->data_addr, out_packet->len, out_packet->cid, _queue_push);
}

/**
 * Builds a packet from the passed state.
 * @param[in] in_state The packet is loaded from the state.
 */
static void _build_packet(const uint8_t* buf, size_t length, uint8_t cmd, uint32_t cid)
{
    memcpy(_in_packet.data_addr, buf, MIN(USB_DATA_MAX_LEN, length));
    _in_packet.len = length;
    _in_packet.cmd = cmd;
    _in_packet.cid = cid;
}

/**
 * Prepares an outgoing packet.
 */
static void _prepare_out_packet(const Packet* in_packet, Packet* out_packet)
{
    memset(out_packet->data_addr, 0, sizeof(out_packet->data_addr));
    out_packet->len = 0;
    out_packet->cmd = in_packet->cmd;
    out_packet->cid = in_packet->cid;
}

/**
 * Register a command callback that is executed when a USB frame with
 * a specific cmd id is received.
 */
void usb_processing_register_cmds(
    struct usb_processing* ctx,
    const CMD_Callback* cmd_callbacks,
    int num_cmds)
{
    if (ctx->registered_cmds == NULL) {
        ctx->registered_cmds = malloc(num_cmds * sizeof(CMD_Callback));
        if (!ctx->registered_cmds) {
            Abort("Error: malloc usb commands");
        }
        memcpy(ctx->registered_cmds, cmd_callbacks, num_cmds * sizeof(CMD_Callback));
    } else {
        size_t old_size = ctx->registered_cmds_len * sizeof(CMD_Callback);
        size_t added_size = num_cmds * sizeof(CMD_Callback);
        size_t new_size = old_size + added_size;
        CMD_Callback* new_registered_cmds = (CMD_Callback*)realloc(ctx->registered_cmds, new_size);
        if (new_registered_cmds == NULL) {
            free(ctx->registered_cmds);
            Abort("Error: realloc usb commands");
        }
        ctx->registered_cmds = new_registered_cmds;
        memcpy(ctx->registered_cmds + ctx->registered_cmds_len, cmd_callbacks, added_size);
    }
    ctx->registered_cmds_len += num_cmds;
}

/**
 * Request to process a complete incoming USB packet.
 */
bool usb_processing_enqueue(
    struct usb_processing* ctx,
    const uint8_t* buf,
    size_t length,
    uint8_t cmd,
    uint32_t cid)
{
    if (_in_packet_queued) {
        /* We already have a buffered packet. */
        return false;
    }
    _build_packet(buf, length, cmd, cid);
    _in_packet_queued = true;
    ctx->has_packet = true;
    return true;
}

void usb_processing_set_send(struct usb_processing* ctx, void (*send)(void))
{
    ctx->send = send;
}

/**
 * Marks any buffered RX packet as fully processed.
 * This frees the RX buffer so that it's possible to
 * receive further packets.
 */
static void _usb_processing_drop_received(struct usb_processing* ctx)
{
    // Mark the packet as processed.
    if (ctx->has_packet) {
        ctx->has_packet = false;
        util_zero(&_in_packet, sizeof(_in_packet));
    }
    _in_packet_queued = false;
}

static void _usb_process_incoming_packet(struct usb_processing* ctx)
{
    if (!ctx->has_packet) {
        return;
    }
    // Received all data
    int cmd_valid = 0;
    for (uint32_t i = 0; i < ctx->registered_cmds_len; i++) {
        if (_in_packet.cmd == ctx->registered_cmds[i].cmd) {
            cmd_valid = 1;
            // process_cmd calls commander(...) or U2F functions.

            Packet out_packet;
            _prepare_out_packet(&_in_packet, &out_packet);
            ctx->registered_cmds[i].process_cmd(&_in_packet, &out_packet, USB_DATA_MAX_LEN);
            _enqueue_frames(ctx, (const Packet*)&out_packet);
            break;
        }
    }

    if (!cmd_valid) {
        // TODO: if U2F is disabled, we used to return a 'channel busy' command.
        // now we return an invalid cmd, because there is not going to be a matching
        // cmd in '_registered_cmds' if the U2F bit it not set (== U2F disabled).
        // TODO: figure out the consequences and either implement a solution or
        // inform U2F hijack vendors.
        _global_queue = ctx->out_queue();
        usb_frame_prepare_err(FRAME_ERR_INVALID_CMD, _in_packet.cid, _queue_push);
    }
    _usb_processing_drop_received(ctx);
}

void usb_processing_process(struct usb_processing* ctx)
{
#if defined(APP_U2F)
    uint32_t timeout_cid;
    // If there are any timeouts, send them first
    while (u2f_packet_timeout_get(&timeout_cid)) {
        u2f_packet_timeout(timeout_cid);
        usb_processing_u2f()->send();
    }

#endif
    _usb_process_incoming_packet(ctx);
    /*
     * If USB sends are not enabled (yet), send will be NULL.
     * Otherwise, we can call it now to flush outstanding writes.
     */
    if (ctx->send != NULL) {
        ctx->send();
    }
}

#if defined(APP_U2F)
struct usb_processing* usb_processing_u2f(void)
{
    static struct usb_processing usb_processing;
    return &usb_processing;
}
#endif

struct usb_processing* usb_processing_hww(void)
{
    static struct usb_processing usb_processing;
    return &usb_processing;
}

void usb_processing_init(void)
{
#if defined(APP_U2F)
    usb_processing_u2f()->out_queue = queue_u2f_queue;
    queue_init(queue_u2f_queue(), USB_REPORT_SIZE);
    usb_processing_u2f()->format_frame = usb_frame_reply;
    usb_processing_u2f()->has_packet = false;
#endif
    usb_processing_hww()->out_queue = queue_hww_queue;
#if defined(BBBASE_HMS_BOARD)
    queue_init(queue_hww_queue(), 1);
    usb_processing_hww()->format_frame = usart_format_frame;
#else
    queue_init(queue_hww_queue(), USB_REPORT_SIZE);
    usb_processing_hww()->format_frame = usb_frame_reply;
#endif
    usb_processing_hww()->has_packet = false;
}
