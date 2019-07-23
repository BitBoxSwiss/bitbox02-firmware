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

#ifndef _CIPHER_H_
#define _CIPHER_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * @param[out] must have inlen + 64 bytes.
 * @param[inout] out_len must be inlen + 64 incoming, and will be the actual
 * length of the output.
 * @param[in] secret 32 bytes secret
 */
bool cipher_aes_hmac_encrypt(
    const uint8_t* in,
    int in_len,
    uint8_t* out,
    int* out_len,
    const uint8_t* secret);

/**
 * @param[in] must have at least 64 bytes.
 * @param[out] must have in_len - 48 bytes.
 * @param[inout] out_len must be inlen - 48 incoming, and will be the actual
 * length of the output.
 */
bool cipher_aes_hmac_decrypt(
    const uint8_t* in,
    size_t in_len,
    uint8_t* out,
    size_t* outlen,
    const uint8_t* key);

#endif
