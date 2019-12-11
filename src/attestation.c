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

#include "attestation.h"
#include "hardfault.h"
#include "memory/memory.h"
#ifndef TESTING
#include "securechip/securechip.h"
#endif
#include <wally_crypto.h>

bool attestation_perform(const uint8_t* host_challenge, PerformAttestationResponse* result_out)
{
#ifdef TESTING
    (void)host_challenge;
    (void)result_out;
    return false;
#else
    // Check assumptions
    if (sizeof(result_out->bootloader_hash) != 32 || sizeof(result_out->device_pubkey) != 64 ||
        sizeof(result_out->certificate) != 64 || sizeof(result_out->challenge_signature) != 64 ||
        sizeof(result_out->root_pubkey_identifier) != 32) {
        return false;
    }
    if (!memory_get_attestation_pubkey_and_certificate(
            result_out->device_pubkey,
            result_out->certificate,
            result_out->root_pubkey_identifier)) {
        return false;
    }

    uint8_t hash[SHA256_LEN] = {0};
    if (wally_sha256(host_challenge, 32, hash, sizeof(hash)) != WALLY_OK) {
        Abort("wally_sha256 failed");
    }
    memory_bootloader_hash(result_out->bootloader_hash);
    return securechip_attestation_sign(hash, result_out->challenge_signature);
#endif
}
