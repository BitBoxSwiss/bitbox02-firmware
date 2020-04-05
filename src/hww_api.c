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

#include <commander/commander.h>
#include <keystore.h>
#include <memory/memory.h>
#include <usb/noise.h>
#include <workflow/status.h>

void hww_api_process_packet(const in_buffer_t* in_req, buffer_t* out_rsp)
{
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
