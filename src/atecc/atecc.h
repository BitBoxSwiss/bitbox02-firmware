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

#ifndef _ATECC_H_
#define _ATECC_H_

#include "compiler_util.h"
#include "securechip/securechip.h"
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
    ATECC_ERR_ZONE_UNLOCKED_CONFIG = -1,
    ATECC_ERR_ZONE_UNLOCKED_DATA = -2,
    ATECC_ERR_CONFIG_MISMATCH = -3,
    ATECC_ERR_SLOT_UNLOCKED_IO = -4,
    ATECC_ERR_SLOT_UNLOCKED_AUTH = -5,
    ATECC_ERR_SLOT_UNLOCKED_ENC = -6,
    ATECC_ERR_IFS = -7,
    ATECC_ERR_INVALID_ARGS = -8,
} atecc_error_t;

/**
 * Initializes the cryptoauthlib communication, by providing a custom i2c chip
 * communication interface/bridge to cryptoauthlib. On first call, the chip
 * is configured and locked.
 * @param[in] ifs Interface functions.
 * @return values of `atecc_error_t` if negative, values of `ATCA_STATUS` if positive, 0 on
 * success.
 */
USE_RESULT int atecc_setup(const securechip_interface_functions_t* ifs);

/**
 * Updates the two KDF keys (rollkey and kdf key). The previous keys are lost
 * and cannot be restored. Calling this function does not increment the
 * monotonic counter Counter0.
 * @return true on success.
 */
USE_RESULT bool atecc_update_keys(void);

/**
 * Perform HMAC using the key in KDF slot with the input msg.
 * @param[in] msg Use this msg as input
 * @param[in] len Must be <= 127.
 * @param[out] kdf_out Must have size 32. Result of the kdf will be stored here.
 * Cannot be the same as `msg`.
 * @return values of `atecc_error_t` if negative, values of `ATCA_STATUS` if positive, 0 on
 */
USE_RESULT int atecc_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out);

/**
 * Perform KDF using the key in rollkey slot with the input msg.
 * Calling this function increments the
 * monotonic counter Counter0.
 * @param[in] msg Use this msg as input
 * @param[in] len Must be <= 127.
 * @param[out] kdf_out Must have size 32. Result of the kdf will be stored here.
 * Cannot be the same as `msg`.
 * @return values of `securechip_error_t` if negative, values of `ATCA_STATUS` if positive, 0 on
 */
USE_RESULT int atecc_kdf_rollkey(const uint8_t* msg, size_t len, uint8_t* kdf_out);

/**
 * Generates a new attestation device key and outputs the public key.
 * @param[out] pubkey_out
 */
USE_RESULT bool atecc_gen_attestation_key(uint8_t* pubkey_out);

/**
 * @param[in] msg 32 byte message to sign.
 * @param[out] signature_out must be 64 bytes. R/S P256 signature.
 */
USE_RESULT bool atecc_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);

/**
 * Retrieves the number of remaining possible counter increments (max value - Counter0).
 * The counter is increment when using `atecc_kdf()` (see its docstring).
 * @param[out] remaining_out current value of the monotonic counter.
 * @return false if there was a communication error with the SC.
 */
USE_RESULT bool atecc_monotonic_increments_remaining(uint32_t* remaining_out);

/**
 * @param[out] rand_out must be 32 bytes.
 */
USE_RESULT bool atecc_random(uint8_t* rand_out);

#if APP_U2F == 1 || FACTORYSETUP == 1
/**
 * Set the u2f counter to `counter`. Should only be used for initialization.
 * @param[in] counter Value to set counter to
 * @return True if success
 */
USE_RESULT bool atecc_u2f_counter_set(uint32_t counter);
#endif

#if APP_U2F == 1
/**
 * Monotonically increase the U2F counter and return the current value
 * @param[out] counter Next counter value
 * @return True if success
 */
USE_RESULT bool atecc_u2f_counter_inc(uint32_t* counter);
#endif

/**
 * Output the atecc model.
 * @param[out] model_out atecc model
 * @return True if success
 */
USE_RESULT bool atecc_model(securechip_model_t* model_out);

#endif