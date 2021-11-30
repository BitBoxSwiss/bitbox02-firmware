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

#ifndef _PUKCC_H_
#define _PUKCC_H_

#include <stdint.h>

#define PUKCC_ECC_PARAM_LEN 32u
#define PUKCC_ECC_TEST_MSG_MAX_LEN 32u

typedef struct {
    uint8_t a[PUKCC_ECC_PARAM_LEN];
    uint8_t b[PUKCC_ECC_PARAM_LEN];
    uint8_t modulo_p[PUKCC_ECC_PARAM_LEN];
    uint8_t base_x[PUKCC_ECC_PARAM_LEN];
    uint8_t base_y[PUKCC_ECC_PARAM_LEN];
    uint8_t base_z[PUKCC_ECC_PARAM_LEN];
    uint8_t order[PUKCC_ECC_PARAM_LEN];
    uint8_t one[PUKCC_ECC_PARAM_LEN];
    uint8_t zero[PUKCC_ECC_PARAM_LEN];
    uint8_t test_message_len;
    char test_message[PUKCC_ECC_TEST_MSG_MAX_LEN];
    uint8_t test_message_hash[PUKCC_ECC_PARAM_LEN];
    uint8_t test_private_key[PUKCC_ECC_PARAM_LEN];
    uint8_t test_public_key[PUKCC_ECC_PARAM_LEN * 2];
    uint8_t test_k[PUKCC_ECC_PARAM_LEN];
    uint8_t test_signature[PUKCC_ECC_PARAM_LEN * 2];
} PUKCC_CURVE_256_X;

uint8_t pukcc_ecdsa_verify(
    const uint8_t* public_key,
    const uint8_t* signature,
    const uint8_t* message,
    uint32_t message_len,
    PUKCC_CURVE_256_X curve);

int32_t pukcc_sha256_compute(const uint8_t* message, uint32_t message_len, uint8_t* hash);

#endif
