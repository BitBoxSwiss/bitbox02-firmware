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
    Packet in_packet;
    struct queue* (*out_queue)(void);
    void (*send)(void);
};

enum packet_queued {
    NO_PACKET,
    HWW_PACKET,
    U2F_PACKET,
};
static volatile enum packet_queued _in_packet_queued;

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
static uint8_t _enqueue_frames(struct usb_processing* ctx, const Packet* out_packet)
{
    _global_queue = ctx->out_queue();
    return usb_frame_reply(
        out_packet->cmd, out_packet->data_addr, out_packet->len, out_packet->cid, _queue_push);
}

/**
 * Builds a packet from the passed state.
 * @param[in] in_state The packet is loaded from the state.
 */
static void _build_packet(struct usb_processing* ctx, const State* in_state)
{
    memcpy(ctx->in_packet.data_addr, in_state->data, USB_DATA_MAX_LEN);
    ctx->in_packet.len = in_state->len;
    ctx->in_packet.cmd = in_state->cmd;
    ctx->in_packet.cid = in_state->cid;
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

bool usb_processing_enqueue(struct usb_processing* ctx, const State* in_state)
{
    if (_in_packet_queued != NO_PACKET) {
        return false;
    }
    _build_packet(ctx, in_state);
    _in_packet_queued = ctx == usb_processing_hww() ? HWW_PACKET : U2F_PACKET;
    return true;
}

void usb_processing_set_send(struct usb_processing* ctx, void (*send)(void))
{
    ctx->send = send;
}

void usb_processing_process(struct usb_processing* ctx)
{
#if !defined(BOOTLOADER)
    uint32_t timeout_cid;
    // If there are any timeouts, send them first
    while (u2f_packet_timeout_get(&timeout_cid)) {
        // screen_sprintf_debug(250, "u2f %u timed out", timeout_cid);
        u2f_packet_timeout(timeout_cid);
        usb_processing_u2f()->send();
    }

#endif
    if (ctx == usb_processing_hww() && _in_packet_queued != HWW_PACKET) {
        return;
    }
#if !defined(BOOTLOADER)

    if (ctx == usb_processing_u2f() && _in_packet_queued != U2F_PACKET) {
        return;
    }
#endif
    // Received all data
    int cmd_valid = 0;
    for (uint32_t i = 0; i < ctx->registered_cmds_len; i++) {
        if (ctx->in_packet.cmd == ctx->registered_cmds[i].cmd) {
            cmd_valid = 1;
            // process_cmd calls commander(...) or U2F functions.

            Packet out_packet;
            _prepare_out_packet(&ctx->in_packet, &out_packet);
            ctx->registered_cmds[i].process_cmd(&ctx->in_packet, &out_packet, USB_DATA_MAX_LEN);
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
        usb_frame_prepare_err(FRAME_ERR_INVALID_CMD, ctx->in_packet.cid, _queue_push);
    }
    if (ctx->send == NULL) {
        Abort("send is null");
    }
    ctx->send();
    _in_packet_queued = NO_PACKET;
    util_zero(&ctx->in_packet, sizeof(ctx->in_packet));
}

struct usb_processing* usb_processing_u2f(void)
{
    static struct usb_processing usb_processing;
    return &usb_processing;
}

struct usb_processing* usb_processing_hww(void)
{
    static struct usb_processing usb_processing;
    return &usb_processing;
}

void usb_processing_init(void)
{
    usb_processing_u2f()->out_queue = queue_u2f_queue;
    usb_processing_hww()->out_queue = queue_hww_queue;
}
