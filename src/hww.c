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

#include <commander/commander.h>
#include <usb/noise.h>
#include <usb/usb_packet.h>
#include <usb/usb_processing.h>
#include <workflow/status.h>

/**
 * Executes the HWW packet.
 * @param[in] in_packet The incoming HWW packet.
 * @param[in] out_packet The outgoing HWW packet.
 * @param[in] max_out_len The maximum number of bytes that the outgoing HWW packet can hold.
 */
static void _msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    if (!bb_noise_process_msg(in_packet, out_packet, max_out_len, commander)) {
        workflow_status_create("Could not\npair with app");
    }
}

/**
 * Set up the HWW command.
 */
void hww_setup(void)
{
    const CMD_Callback hww_cmd_callbacks[] = {{HWW_MSG, _msg}};
    usb_processing_register_cmds(
        hww_cmd_callbacks, sizeof(hww_cmd_callbacks) / sizeof(CMD_Callback));
}
