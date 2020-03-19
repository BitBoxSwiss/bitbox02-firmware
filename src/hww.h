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

#ifndef _HWW_H_
#define _HWW_H_

#include <stdbool.h>

#include <usb/usb_packet.h>

#define HWW_MSG (HID_VENDOR_FIRST + 0x01) // Hardware wallet command

/**
 * Set up the HWW command.
 */
void hww_setup(void);

/**
 * When the HWW stack is blocking the device, checks if
 * a HWW request is allowed to be processed.
 * HWW requests that are allowed are OP_CANCEL and OP_RETRY.
 */
bool hww_blocking_request_can_go_through(const Packet* in_packet);

/**
 * Create an output packet used to signal to the client
 * that a valid HWW request has been received, but it can't be processed
 * because the device is busy doing something else.
 *
 * We respond to any HWW request that can't be processed by the device
 * with OP_STATUS_FAILURE_DEVICE_BUSY, since we're not technically able
 * to verify user presence at the moment.
 */
void hww_blocked_req_error(Packet* out_packet, const Packet* in_packet);

/**
 * Processes the async operations on the HWW USB stack.
 * This is not doing anything at the moment, as all user operations
 * are handled with blocking operations upon packet reception.
 */
void hww_process(void);

/**
 * Called to abort any operation that blocked the HWW stack.
 */
void hww_abort_outstanding_op(void);

#endif
