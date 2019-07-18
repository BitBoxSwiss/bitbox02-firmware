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
#include "usb_frame.h"
#include <hardfault.h>
#include <queue.h>
#include <stdlib.h>
#include <util.h>

/**
 * The commands that were registered by other modules (U2F/HWW) and that
 * will be executed when a specific command has been received.
 */
static CMD_Callback* _registered_cmds = NULL;

static int _num_registered_cmds = 0;
static Packet _in_packet = {0};
static bool _in_packet_queued = false;
static void (*_send)(void) = NULL;

/**
 * Responds with data of a certain length.
 * @param[in] packet The packet to be sent.
 */
static uint8_t _enqueue_frames(const Packet* out_packet)
{
    return usb_frame_reply(
        out_packet->cmd, out_packet->data_addr, out_packet->len, out_packet->cid, queue_push);
}

/**
 * Builds a packet from the passed state.
 * @param[in] in_state The packet is loaded from the state.
 */
static void _build_packet(const State* in_state)
{
    memcpy(_in_packet.data_addr, in_state->data, USB_DATA_MAX_LEN);
    _in_packet.len = in_state->len;
    _in_packet.cmd = in_state->cmd;
    _in_packet.cid = in_state->cid;
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
void usb_processing_register_cmds(const CMD_Callback* cmd_callbacks, int num_cmds)
{
    if (_registered_cmds == NULL) {
        _registered_cmds = malloc(num_cmds * sizeof(CMD_Callback));
        if (!_registered_cmds) {
            Abort("Error: malloc usb commands");
        }
        memcpy(_registered_cmds, cmd_callbacks, num_cmds * sizeof(CMD_Callback));
    } else {
        size_t old_size = _num_registered_cmds * sizeof(CMD_Callback);
        size_t added_size = num_cmds * sizeof(CMD_Callback);
        size_t new_size = old_size + added_size;
        CMD_Callback* new_registered_cmds = (CMD_Callback*)realloc(_registered_cmds, new_size);
        if (new_registered_cmds == NULL) {
            free(_registered_cmds);
            Abort("Error: realloc usb commands");
        }
        _registered_cmds = new_registered_cmds;
        memcpy(_registered_cmds + _num_registered_cmds, cmd_callbacks, added_size);
    }
    _num_registered_cmds += num_cmds;
}

bool usb_processing_enqueue(const State* in_state, void (*send)(void))
{
    if (_in_packet_queued) {
        return false;
    }
    _build_packet(in_state);
    _in_packet_queued = true;
    usb_processing_set_send(send);
    return true;
}

void usb_processing_set_send(void (*send)(void))
{
    _send = send;
}

#include "screen.h"

void usb_processing_process(void)
{
    uint32_t timeout_cid;
    // If there are any timeouts, send them first
    while (usb_packet_timeout_get(&timeout_cid)) {
        // screen_sprintf_debug(100, "%u timed out", timeout_cid);
        usb_packet_timeout(timeout_cid);
        _send();
    }
    if (!_in_packet_queued) {
        return;
    }
    // Received all data
    int cmd_valid = 0;
    for (int i = 0; i < _num_registered_cmds; i++) {
        if (_in_packet.cmd == _registered_cmds[i].cmd) {
            cmd_valid = 1;
            // process_cmd calls commander(...) or U2F functions.

            Packet out_packet;
            _prepare_out_packet(&_in_packet, &out_packet);
            _registered_cmds[i].process_cmd(&_in_packet, &out_packet, USB_DATA_MAX_LEN);
            _enqueue_frames((const Packet*)&out_packet);
            break;
        }
    }
    if (!cmd_valid) {
        // TODO: if U2F is disabled, we used to return a 'channel busy' command.
        // now we return an invalid cmd, because there is not going to be a matching
        // cmd in '_registered_cmds' if the U2F bit it not set (== U2F disabled).
        // TODO: figure out the consequences and either implement a solution or
        // inform U2F hijack vendors.
        usb_frame_prepare_err(FRAME_ERR_INVALID_CMD, _in_packet.cid, queue_push);
    }
    _send();
    _in_packet_queued = false;
    util_zero(&_in_packet, sizeof(_in_packet));
}
