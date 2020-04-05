// Copyright 2020 Shift Cryptosecurity AG
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

#include "hww_api.h"

#include <attestation.h>
#include <commander/commander.h>
#include <keystore.h>
#include <memory/memory.h>
#include <usb/noise.h>
#include <workflow/status.h>

#include <stdint.h>
#include <string.h>

#define OP_ATTESTATION ((uint8_t)'a')

#define OP_STATUS_SUCCESS ((uint8_t)0)
#define OP_STATUS_FAILURE ((uint8_t)1)
#define OP_STATUS_FAILURE_UNINITIALIZED ((uint8_t)2)

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

void hww_api_process_packet(const in_buffer_t* in_req, buffer_t* out_rsp)
{
    if (in_req->len >= 1) {
        switch (in_req->data[0]) {
        case OP_ATTESTATION:
            _api_attestation(in_req, out_rsp);
            return;
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
    if (!bb_noise_process_msg(in_req, out_rsp, commander)) {
        workflow_status_blocking("Could not\npair with app", false);
    }
}
