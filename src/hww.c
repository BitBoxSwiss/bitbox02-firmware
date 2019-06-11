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
#include <keystore.h>
#include <memory.h>
#include <usb/noise.h>
#include <usb/usb_packet.h>
#include <usb/usb_processing.h>
#include <workflow/status.h>
#include <workflow/unlock.h>

#define OP_ATTESTATION ((uint8_t)'a')
#define OP_UNLOCK ((uint8_t)'u')

#define OP_STATUS_SUCCESS ((uint8_t)0);
#define OP_STATUS_FAILURE ((uint8_t)1);

// in: 'a' + 32 bytes host challenge
// out: bootloader_hash 32 | device_pubkey 64 | certificate 64 | root_pubkey_identifier 32 |
// challenge_signature 64
static void _api_attestation(const Packet* in_packet, Packet* out_packet)
{
    if (in_packet->len != 33) {
        out_packet->len = 1;
        out_packet->data_addr[0] = OP_STATUS_FAILURE;
        return;
    }
    PerformAttestationResponse result;
    if (!attestation_perform(in_packet->data_addr + 1, &result)) {
        out_packet->len = 1;
        out_packet->data_addr[0] = OP_STATUS_FAILURE;
        return;
    }
    out_packet->len = 1 + sizeof(result.bootloader_hash) + sizeof(result.device_pubkey) +
                      sizeof(result.certificate) + sizeof(result.root_pubkey_identifier) +
                      sizeof(result.challenge_signature);

    uint8_t* data = out_packet->data_addr;

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
    data += sizeof(result.challenge_signature);
}

/**
 * Executes the HWW packet.
 * @param[in] in_packet The incoming HWW packet.
 * @param[in] out_packet The outgoing HWW packet.
 * @param[in] max_out_len The maximum number of bytes that the outgoing HWW packet can hold.
 */
static void _msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    if (in_packet->len >= 1) {
        switch (in_packet->data_addr[0]) {
        case OP_ATTESTATION:
            _api_attestation(in_packet, out_packet);
            return;
        case OP_UNLOCK:
            workflow_unlock();
            out_packet->len = 0;
            return;
        default:
            break;
        }
    }

    // No other message than the attestation and unlock calls shall pass until the device is
    // unlocked or ready to be initialized.
    if (memory_is_initialized() && !keystore_is_unlocked()) {
        return;
    }

    // Process protofbuf/noise api calls.
    if (!bb_noise_process_msg(in_packet, out_packet, max_out_len, commander)) {
        workflow_status_create("Could not\npair with app");
    }
}

void hww_setup(void)
{
    const CMD_Callback hww_cmd_callbacks[] = {{HWW_MSG, _msg}};
    usb_processing_register_cmds(
        hww_cmd_callbacks, sizeof(hww_cmd_callbacks) / sizeof(CMD_Callback));
}
