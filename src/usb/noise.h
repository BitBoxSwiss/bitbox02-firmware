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

#ifndef _NOISE_H_
#define _NOISE_H_

#include "usb_packet.h"

#include <stdbool.h>

#define NOISE_PUBKEY_SIZE 32

typedef size_t (*bb_noise_process_msg_callback)(
    const uint8_t* input,
    const size_t in_len,
    uint8_t* response,
    const size_t max_out_len);

bool bb_noise_process_msg(
    const Packet* in_packet,
    Packet* out_packet,
    const size_t max_out_len,
    bb_noise_process_msg_callback process_msg);

/**
 * @param[out] private_key_out must be 32 bytes.
 */
bool bb_noise_generate_static_private_key(uint8_t* private_key_out);

#endif
