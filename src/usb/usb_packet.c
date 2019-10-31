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
#include "screen.h"
#include "usb_processing.h"
#include <stdbool.h>
#include <stdlib.h>

#define ERR_NONE 0

/**
 * Keeps a state for the frame processing of incoming frames.
 */
static State _in_state;

/**
 * Resets the current state.
 */
static void _reset_state(void)
{
    queue_clear(queue_hww_queue());
    memset(&_in_state, 0, sizeof(_in_state));
}

static queue_error_t _queue_push(const uint8_t* data)
{
    return queue_push(queue_hww_queue(), data);
}

/**
 * Responds with an error.
 * @param[in] err The error.
 * @param[in] cid The channel identifier.
 * No return value needed as long as _reset_state clears the queue.
 */
static void _queue_err(const uint8_t err, uint32_t cid)
{
    usb_frame_prepare_err(err, cid, _queue_push);
}

static bool _need_more_data(void)
{
    return (_in_state.buf_ptr - _in_state.data) < (signed)_in_state.len;
}

bool usb_packet_process(const USB_FRAME* frame)
{
    struct usb_processing* ctx = usb_processing_hww();
    switch (usb_frame_process(frame, &_in_state)) {
    case FRAME_ERR_IGNORE:
        // Ignore this frame, i.e. no response.
        break;
    case FRAME_ERR_INVALID_SEQ:
        // Reset the state becuase this error indicates that there is a host application bug
        _reset_state();
        _queue_err(FRAME_ERR_INVALID_SEQ, frame->cid);
        break;
    case FRAME_ERR_CHANNEL_BUSY:
        // We don't reset the state because this error doesn't indicate something wrong with the
        // "current" connection.
        _queue_err(FRAME_ERR_CHANNEL_BUSY, frame->cid);
        break;
    case FRAME_ERR_INVALID_LEN:
        // Reset the state becuase this error indicates that there is a host application bug
        _reset_state();
        _queue_err(FRAME_ERR_INVALID_LEN, frame->cid);
        break;
    case ERR_NONE:
        if (_need_more_data()) {
            // Do not send a message yet
            return true;
        }
        if (usb_processing_enqueue(
                ctx, _in_state.data, _in_state.len, _in_state.cmd, _in_state.cid)) {
            // Queue filled and will be sent during usb processing
            _reset_state();
            return false;
        }
        // Else: Currently processing a message, reset the state and forget about this packet
        _reset_state();
        _queue_err(FRAME_ERR_CHANNEL_BUSY, frame->cid);
        break;
    default:
        // other errors
        _reset_state();
        _queue_err(FRAME_ERR_OTHER, frame->cid);
        break;
    }
    return false;
}
