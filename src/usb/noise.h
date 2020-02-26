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

#include <compiler_util.h>
#include <usb/usb_packet.h>
#include <util.h>

#include <stdbool.h>

#define NOISE_PUBKEY_SIZE 32

typedef void (*bb_noise_process_msg_callback)(const in_buffer_t* input, buffer_t* output);

USE_RESULT bool bb_noise_process_msg(
    const in_buffer_t* in_buf,
    buffer_t* out_buf,
    bb_noise_process_msg_callback process_msg);

/**
 * @param[out] private_key_out must be 32 bytes.
 */
USE_RESULT bool bb_noise_generate_static_private_key(uint8_t* private_key_out);

#endif
