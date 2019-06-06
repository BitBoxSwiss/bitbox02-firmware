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

#ifndef _SECURECHIP_H_
#define _SECURECHIP_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
    /**
     * @param[out] key_out must be of size 32
     */
    void (*const get_auth_key)(uint8_t* key_out);
    /**
     * @param[out] key_out must be of size 32
     */
    void (*const get_io_protection_key)(uint8_t* key_out);
    /**
     * @param[out] key_out must be of size 32
     */
    void (*const get_encryption_key)(uint8_t* key_out);

    void (*const random_32_bytes)(uint8_t* buf);
} securechip_interface_functions_t;

typedef enum {
    SECURECHIP_SLOT_IO_PROTECTION_KEY = 0,
    SECURECHIP_SLOT_AUTHKEY = 1,
    SECURECHIP_SLOT_ENCRYPTION_KEY = 2,
    SECURECHIP_SLOT_ROLLKEY = 3,
    SECURECHIP_SLOT_KDF = 4,
    SECURECHIP_SLOT_ATTESTATION = 5,
} securechip_slot_t;

/**
 * Initializes the cryptoauthlib communication, by providing a custom i2c chip
 * communication interface/bridge to cryptoauthlib. On first call, the chip
 * is configured and locked.
 * @param[in] ifs Interface functions.
 */
bool securechip_setup(securechip_interface_functions_t* ifs);

/**
 * Updates the two KDF keys (rollkey and kdf key). The previous keys are lost
 * and cannot be restored. Calling this function does not increment the
 * monotonic counter Counter0.
 * @return true on success.
 */
bool securechip_update_keys(void);

/**
 * Perform KDF using the key in predefined slot with the input msg.
 * Calling this function for SECURECHIP_SLOT_ROLLKEY also increments the
 * monotonic counter Counter0.
 * @param[in] slot should be one of SECURECHIP_SLOT_ROLLKEY and
 *            SECURECHIP_SLOT_KDF.
 * @param[in] msg Use this msg as input
 * @param[in] len Must be <= 127.
 * @param[out] kdf_out Must have size 32. Result of the kdf will be stored here.
 * Cannot be the same as `msg`.
 * @return true on success.
 */
bool securechip_kdf(securechip_slot_t slot, const uint8_t* msg, size_t len, uint8_t* kdf_out);

/**
 * Generates a new attestation device key and outputs the public key.
 * @param[out] pubkey_out
 */
bool securechip_gen_attestation_key(uint8_t* pubkey_out);

/**
 * @param[in] msg 32 byte message to sign.
 * @param[out] signature_out must be 64 bytes. R/S P256 signature.
 */
bool securechip_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);

/**
 * Retrieves the number of remaining possible counter increments (max value - Counter0).
 * The counter is increment when using `securechip_kdf()` (see its docstring).
 * @param[out] remaining_out current value of the monotonic counter.
 * @return false if there was a communication error with the SC.
 */
bool securechip_monotonic_increments_remaining(uint32_t* remaining_out);

/**
 * @param[out] rand_out must be 32 bytes.
 */
bool securechip_random(uint8_t* rand_out);

#endif
