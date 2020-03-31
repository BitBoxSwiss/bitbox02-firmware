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

#include "hww.h"

#include <attestation.h>
#include <commander/commander.h>
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <platform_config.h>
#include <ui/workflow_stack.h>
#include <usb/noise.h>
#include <usb/usb_packet.h>
#include <usb/usb_processing.h>
#include <workflow/status.h>
#include <workflow/unlock.h>

#include <string.h>

#define OP_ATTESTATION ((uint8_t)'a')
#define OP_UNLOCK ((uint8_t)'u')

#define OP_STATUS_SUCCESS ((uint8_t)0)
#define OP_STATUS_FAILURE ((uint8_t)1)
#define OP_STATUS_FAILURE_UNINITIALIZED ((uint8_t)2)

/** Request command for HWW packets. */
typedef enum {
    HWW_REQ_NEW = 0,
    HWW_REQ_RETRY = 1,
    HWW_REQ_CANCEL = 2,
    HWW_REQ_INFO = ((uint8_t)'i'),
} hww_req_t;

/** Response status code for HWW packets. */
typedef enum {
    HWW_RSP_ACK = 0,
    HWW_RSP_NOT_READY = 1,
    HWW_RSP_BUSY = 2,
    HWW_RSP_NACK = 3,
} hww_rsp_t;

/** A HWW response is composed of a status code and a payload. */
typedef struct {
    /** Status byte. */
    hww_rsp_t status;
    /** Payload of the message. */
    buffer_t buffer;
} hww_packet_rsp_t;

/**
 * Type of operation that
 * the HWW stack is blocked in (i.e. what we should retry()
 * when asked).
 */
typedef enum {
    /** Not currently blocked. */
    HWW_BLOCKING_OP_NONE = 0,
    /** Blocked on a OP_UNLOCK */
    HWW_BLOCKING_OP_UNLOCK,
} hww_blocking_op_t;

static struct {
    hww_blocking_op_t blocking_op;
    workflow_t* unlock_wf;
    bool unlock_result;
} _state = {0};

// in: 'a' + 32 bytes host challenge
// out: bootloader_hash 32 | device_pubkey 64 | certificate 64 | root_pubkey_identifier 32 |
// challenge_signature 64
static void _api_attestation(const in_buffer_t* in_packet, buffer_t* out_packet)
{
    if (in_packet->len != 33) {
        out_packet->len = 1;
        out_packet->data[0] = OP_STATUS_FAILURE;
        return;
    }
    PerformAttestationResponse result;
    if (!attestation_perform(in_packet->data + 1, &result)) {
        out_packet->len = 1;
        out_packet->data[0] = OP_STATUS_FAILURE;
        return;
    }
    out_packet->len = 1 + sizeof(result.bootloader_hash) + sizeof(result.device_pubkey) +
                      sizeof(result.certificate) + sizeof(result.root_pubkey_identifier) +
                      sizeof(result.challenge_signature);

    uint8_t* data = out_packet->data;

    data[0] = OP_STATUS_SUCCESS;
    data += 1;

    memcpy(data, result.bootloader_hash, sizeof(result.bootloader_hash));
    data += sizeof(result.bootloader_hash);
    memcpy(data, result.device_pubkey, sizeof(result.device_pubkey));
    data += sizeof(result.device_pubkey);
    memcpy(data, result.certificate, sizeof(result.certificate));
    data += sizeof(result.certificate);
    memcpy(data, result.root_pubkey_identifier, sizeof(result.root_pubkey_identifier));
    data += sizeof(result.root_pubkey_identifier);
    memcpy(data, result.challenge_signature, sizeof(result.challenge_signature));
}

/**
 * Serializes sytem information to the buffer.
 * The following bytes are written:
 * 1 byte: length of the version string that follows.
 * N bytes: short firmware version string, ascii encoded. E.g. "v4.12.2". Not null terminated.
 * 1 byte: platform code:
 * - 0x00 - BitBox02
 * - 0x01 - BitBoxBase
 * 1 byte: edition code:
 * - For the BitBox02 edition:
 * - - 0x00 - Multi
 * - - 0x01 - Bitcoin-only
 * - For the BitBoxBase platform:
 " - 0x00 - Standard
 * 1 byte: 0x00 if the device is locked, 0x01 if the device is unlocked.
 * @param[out] buf serialize info to this buffer.
 * @return number of bytes written
 */
static size_t _api_info(uint8_t* buf)
{
    uint8_t* current = buf;
    // version string, 1 byte len prefix
    size_t version_string_len = sizeof(DIGITAL_BITBOX_VERSION_SHORT) - 1;
    if (version_string_len > 255) {
        Abort("OP_INFO: version string too long");
    }
    *current = (uint8_t)version_string_len;
    current++;
    memcpy((char*)current, DIGITAL_BITBOX_VERSION_SHORT, version_string_len);
    current += version_string_len;

    // 1 byte platform code and 1 byte edition code
#if PRODUCT_BITBOX_MULTI == 1 || PRODUCT_BITBOX02_FACTORYSETUP == 1
    *current = 0x00;
    current++;
    *current = 0x00;
#elif PRODUCT_BITBOX_BTCONLY == 1
    *current = 0x00;
    current++;
    *current = 0x01;
#elif PRODUCT_BITBOX_BASE == 1 || PRODUCT_BITBOXBASE_FACTORYSETUP == 1
    *current = 0x01;
    current++;
    *current = 0x00;
#endif
    current++;

    // 1 byte locked status
    *current = keystore_is_locked() ? 0x00 : 0x01;
    current++;

    return current - buf;
}

static void _clear_state(void)
{
    _state.blocking_op = HWW_BLOCKING_OP_NONE;
    _state.unlock_wf = NULL;
}

static void _lock(hww_blocking_op_t op)
{
    _state.blocking_op = op;
    usb_processing_lock(usb_processing_hww());
}

/**
 * Clears the current state and unlocks the USB stack (if locked).
 */
static void _unlock(void)
{
    _clear_state();
    if (_state.blocking_op != HWW_BLOCKING_OP_NONE) {
        usb_processing_unlock();
    }
}

static void _unlock_finished_cb(bool result, void* param)
{
    (void)param;
    _state.unlock_result = result;
    _state.unlock_wf = NULL;
}

/**
 * Executes the HWW packet.
 * @param[in] in_packet The incoming HWW packet.
 * @param[in] out_packet The outgoing HWW packet.
 * @param[in] max_out_len The maximum number of bytes that the outgoing HWW packet can hold.
 * @return true if the response has been completed, false otherwise.
 */
static void _process_packet(const in_buffer_t* in_req, hww_packet_rsp_t* out_rsp)
{
    out_rsp->status = HWW_RSP_ACK;
    if (in_req->len >= 1) {
        switch (in_req->data[0]) {
        case OP_ATTESTATION:
            _api_attestation(in_req, &out_rsp->buffer);
            return;
        case OP_UNLOCK:
            if (!memory_is_initialized()) {
                out_rsp->buffer.data[0] = OP_STATUS_FAILURE_UNINITIALIZED;
                out_rsp->buffer.len = 1;
                return;
            } else if (!keystore_is_locked()) {
                out_rsp->buffer.data[0] = OP_STATUS_SUCCESS;
                out_rsp->buffer.len = 1;
                return;
            } else {
                _state.unlock_wf = workflow_unlock(&_unlock_finished_cb, NULL);
                workflow_stack_start_workflow(_state.unlock_wf);
                _lock(HWW_BLOCKING_OP_UNLOCK);
                out_rsp->status = HWW_RSP_NOT_READY;
            }
        default:
            break;
        }
    }

    // No other message than the attestation and unlock calls shall pass until the device is
    // unlocked or ready to be initialized.
    if (memory_is_initialized() && keystore_is_locked()) {
        return;
    }

    // Process protofbuf/noise api calls.
    if (!bb_noise_process_msg(in_req, &out_rsp->buffer, commander)) {
        workflow_status_blocking("Could not\npair with app", false);
    }
    return;
}

/**
 * Aborts the current operation (if any). Reply with an
 * appropriate failure state, depending on what the cancelled
 * operation was.
 *
 * @param[out] response Response data to fill.
 */
static void _cancel_packet(hww_packet_rsp_t* response)
{
    /* First, prepare a response. */
    switch (_state.blocking_op) {
    case HWW_BLOCKING_OP_UNLOCK:
        response->status = HWW_RSP_ACK;
        response->buffer.data[0] = OP_STATUS_FAILURE;
        response->buffer.len = 1;
        break;
    default:
        /* We're not blocking on anything. */
        response->status = HWW_RSP_NACK;
    }
    /* Now, cancel whatever we are doing. */
    hww_abort_outstanding_op();
    _unlock();
}

/**
 * Polls the current operation (if any). If the current operation
 * is finished, respond appropriately.
 *
 * @param[out] response Response data to fill.
 */
static void _retry_packet(hww_packet_rsp_t* response)
{
    response->status = HWW_RSP_NACK;
    switch (_state.blocking_op) {
    case HWW_BLOCKING_OP_UNLOCK:
        if (_state.unlock_wf) {
            response->status = HWW_RSP_NOT_READY;
        } else {
            response->status = HWW_RSP_ACK;
            response->buffer.data[0] = _state.unlock_result ? OP_STATUS_SUCCESS : OP_STATUS_FAILURE;
            response->buffer.len = 1;
            _unlock();
        }
        break;
    default:
        /* We're not blocking on anything. */
        break;
    }
}

static void _msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    if (in_packet->len == 0) {
        out_packet->data_addr[0] = HWW_RSP_NACK;
        out_packet->len = 1;
        return;
    }

    hww_req_t cmd = in_packet->data_addr[0];
    if (cmd == HWW_REQ_INFO) {
        // HWW_REQ_INFO is treated as a special case: it has a direct response without a status
        // code, so it can be called independently of the firmware version and framing protocol.
        // Before v7.0.0, there was no HWW framing layer, and the info call was an api-call using
        // the same 'i' OP_INFO op code byte.
        out_packet->len = _api_info(out_packet->data_addr);
        return;
    }

    const in_buffer_t decoded_buffer = {
        .data = in_packet->data_addr + 1,
        .len = in_packet->len - 1,
    };
    hww_packet_rsp_t response = {
        .status = HWW_RSP_NACK,
        .buffer = {.data = out_packet->data_addr + 1, .len = 0, .max_len = max_out_len - 1}};
    switch (cmd) {
    case HWW_REQ_NEW:
        _process_packet(&decoded_buffer, &response);
        break;
    case HWW_REQ_CANCEL:
        /* We don't have anything to cancel yet. */
        _cancel_packet(&response);
        break;
    case HWW_REQ_RETRY:
        _retry_packet(&response);
        break;
    default:
        break;
    }
    out_packet->data_addr[0] = response.status;
    if (response.status == HWW_RSP_ACK) {
        out_packet->len = response.buffer.len + 1;
    } else {
        out_packet->len = 1;
    }
}

bool hww_blocking_request_can_go_through(const Packet* in_packet)
{
    if (in_packet->len != 1) {
        return false;
    }
    uint8_t cmd = in_packet->data_addr[0];
    return cmd == HWW_REQ_CANCEL || cmd == HWW_REQ_RETRY;
}

void hww_blocked_req_error(Packet* out_packet, const Packet* in_packet)
{
    (void)in_packet;
    out_packet->len = 1;
    out_packet->data_addr[0] = HWW_RSP_BUSY;
}

void hww_abort_outstanding_op(void)
{
    switch (_state.blocking_op) {
    case HWW_BLOCKING_OP_UNLOCK:
        workflow_stack_abort_workflow(_state.unlock_wf);
        break;
    default:
        /* Nothing to do. */
        break;
    }
}

void hww_process(void)
{
    /** Nothing to do. */
}

void hww_setup(void)
{
    const CMD_Callback hww_cmd_callbacks[] = {{HWW_MSG, _msg}};
    usb_processing_register_cmds(
        usb_processing_hww(), hww_cmd_callbacks, sizeof(hww_cmd_callbacks) / sizeof(CMD_Callback));
}
