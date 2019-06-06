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

#ifndef _USB_PROCESSING_H_
#define _USB_PROCESSING_H_

#include "usb_frame.h"
#include "usb_packet.h"

/**
 * Register a command callback that is executed when a USB frame with
 * a specific cmd id is received.
 * @param[in] cmd_callbacks The available callbacks for incoming commands.
 * @param[in] num_cmds The number of registered commands.
 */
void usb_processing_register_cmds(const CMD_Callback* cmd_callbacks, int num_cmds);

/**
 * Enqueues a usb packet for processing. Ownership is transferred, and the
 * memory will be freed in `usb_processing_dequeue`.
 * @param[in] in_state The packet is built from in_state and queued.
 * @param[in] function to be called to send the response. This is passed as it
 * could be one of multiple USB interfaces.
 * @return false if there is already a packet in the queue (need to process it
 * first).
 */
bool usb_processing_enqueue(const State* in_state, void (*send)(void));
void usb_processing_process(void);

#endif
