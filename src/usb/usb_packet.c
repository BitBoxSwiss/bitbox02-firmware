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

#include "usb_packet.h"
#include "queue.h"
#include "usb_processing.h"
#include <stdbool.h>
#include <stdlib.h>

/**
 * Keeps a state for the frame processing of incoming frames.
 */
static State _in_state;

/**
 * Resets the current state.
 */
static void _reset_state(void)
{
    queue_clear();
    memset(&_in_state, 0, sizeof(_in_state));
}

/**
 * Responds with an error.
 * @param[in] err The error.
 * @param[in] cid The channel identifier.
 * No return value needed as long as _reset_state clears the queue.
 */
static void _queue_err(const uint8_t err, uint32_t cid)
{
    _reset_state();
    usb_frame_prepare_err(err, cid, queue_push);
}

/**
 * Prepares an error response and possibly clears the USB queue
 * when an initialization frame is received but not expected.
 * No return value needed as long as _reset_state clears the queue.
 */
static void _handle_unexpected_frame(const USB_FRAME* frame)
{
    if (frame->cid == _in_state.cid) {
        _reset_state();
        usb_frame_prepare_err(FRAME_ERR_INVALID_SEQ, frame->cid, queue_push);
    } else {
        usb_frame_prepare_err(FRAME_ERR_CHANNEL_BUSY, frame->cid, queue_push);
    }
}

static bool _need_more_data(void)
{
    return (_in_state.buf_ptr - _in_state.data) < (signed)_in_state.len;
}

bool usb_packet_process(const USB_FRAME* frame, void (*send_packet)(void))
{
    switch (usb_frame_process(frame, &_in_state)) {
    case ERR_UNEXPECTED_CMD_INIT:
    case ERR_UNEXPECTED_CMD_CONT:
        _handle_unexpected_frame(frame);
        break;
    case ERR_INVALID_SEQ:
        _queue_err(FRAME_ERR_INVALID_SEQ, frame->cid);
        break;
    case ERR_CHANNEL_BUSY:
        _queue_err(FRAME_ERR_CHANNEL_BUSY, frame->cid);
        break;
    case ERR_INVALID_LENGTH:
        _queue_err(FRAME_ERR_INVALID_LEN, frame->cid);
        break;
    case ERR_NONE:
        if (_need_more_data()) {
            // Do not send a message yet
            return true;
        }
        if (usb_processing_enqueue(&_in_state, send_packet)) {
            // Queue filled and will be sent during usb processing
            _reset_state();
            return false;
        }
        // Else: Currently processing a message
        _queue_err(ERR_CHANNEL_BUSY, frame->cid);
        break;
    default:
        // other errors
        _queue_err(FRAME_ERR_OTHER, frame->cid);
        break;
    }
    send_packet();
    return false;
}
