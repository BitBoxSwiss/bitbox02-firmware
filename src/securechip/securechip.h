// Copyright 2019 Shift Cryptosecurity AG
// Copyright 2024 Shift Crypto AG
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

#include "compiler_util.h"
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
    // Errors common to any securechip implementation
    SC_ERR_IFS = -1,
    SC_ERR_INVALID_ARGS = -2,
    SC_ERR_CONFIG_MISMATCH = -3,
    SC_ERR_SALT = -4,
    SC_ERR_HASH = -5,

    // Errors specific to the ATECC
    SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG = -100,
    SC_ATECC_ERR_ZONE_UNLOCKED_DATA = -101,
    SC_ATECC_ERR_SLOT_UNLOCKED_IO = -103,
    SC_ATECC_ERR_SLOT_UNLOCKED_AUTH = -104,
    SC_ATECC_ERR_SLOT_UNLOCKED_ENC = -105,

    // Errors specific to the Optiga
    SC_OPTIGA_ERR_CREATE = -201,
    SC_OPTIGA_ERR_UNEXPECTED_METADATA = -204,
    SC_OPTIGA_ERR_PAL = -205,
    SC_OPTIGA_ERR_UNEXPECTED_LEN = -206,
} securechip_error_t;

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

/**
 * Discovers what secure chip is used and configures the module to communicate with it.
 * @return True if success
 */
USE_RESULT bool securechip_init(void);

/**
 * Initializes the cryptoauthlib communication, by providing a custom i2c chip
 * communication interface/bridge to cryptoauthlib. On first call, the chip
 * is configured and locked.
 * @param[in] ifs Interface functions.
 * @return 0 on success. Values of `securechip_error_t` if negative. If positive, values of
 * `ATCA_STATUS` for ATECC, values of optiga_lib_return_codes.h for Optiga.
 */
USE_RESULT int securechip_setup(const securechip_interface_functions_t* ifs);

/**
 * Updates the two KDF keys (rollkey and kdf key). The previous keys are lost
 * and cannot be restored. Calling this function does not increment the
 * monotonic counter.
 * @return true on success.
 */
USE_RESULT bool securechip_update_keys(void);

/**
 * Perform KDF using the key in kdf slot with the input msg.
 * This must not increment a monotonic counter.
 * @param[in] msg Use this msg as input
 * @param[in] len Must be <= 127.
 * @param[out] kdf_out Must have size 32. Result of the kdf will be stored here.
 * Cannot be the same as `msg`.
 * @return 0 on success. Values of `securechip_error_t` if negative. If positive, values of
 * `ATCA_STATUS` for ATECC, values of optiga_lib_return_codes.h for Optiga.
 */
USE_RESULT int securechip_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out);

/**
 * Stretch password using secrets in the secure chip.
 * Calling this function increments the monotonic counter.
 * @param[in] msg Use this msg as input
 * @param[in] len Must be <= 127.
 * @param[out] kdf_out Must have size 32. Result of the kdf will be stored here.
 * Cannot be the same as `msg`.
 * @return 0 on success. Values of `securechip_error_t` if negative. If positive, values of
 * `ATCA_STATUS` for ATECC, values of optiga_lib_return_codes.h for Optiga.
 */
USE_RESULT int securechip_stretch_password(const char* password, uint8_t* stretched_out);

/**
 * Generates a new attestation device key and outputs the public key.
 * @param[out] pubkey_out
 */
USE_RESULT bool securechip_gen_attestation_key(uint8_t* pubkey_out);

/**
 * @param[in] msg 32 byte message to sign.
 * @param[out] signature_out must be 64 bytes. R/S P256 signature.
 */
USE_RESULT bool securechip_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);

/**
 * Retrieves the number of remaining possible counter increments (max value - Counter).
 * The counter is increment when using `securechip_kdf()` (see its docstring).
 * @param[out] remaining_out current value of the monotonic counter.
 * @return false if there was a communication error with the SC.
 */
USE_RESULT bool securechip_monotonic_increments_remaining(uint32_t* remaining_out);

/**
 * @param[out] rand_out must be 32 bytes.
 */
USE_RESULT bool securechip_random(uint8_t* rand_out);

#if APP_U2F == 1 || FACTORYSETUP == 1
/**
 * Set the u2f counter to `counter`. Should only be used for initialization.
 * @param[in] counter Value to set counter to
 * @return True if success
 */
USE_RESULT bool securechip_u2f_counter_set(uint32_t counter);
#endif

#if APP_U2F == 1
/**
 * Monotonically increase the U2F counter and return the current value
 * @param[out] counter Next counter value
 * @return True if success
 */
USE_RESULT bool securechip_u2f_counter_inc(uint32_t* counter);
#endif

typedef enum {
    ATECC_ATECC608A,
    ATECC_ATECC608B,
    OPTIGA_TRUST_M_V3,
} securechip_model_t;

/**
 * Output the securechip model.
 * @param[out] model_out securechip model
 * @return True if success
 */
USE_RESULT bool securechip_model(securechip_model_t* model_out);

#endif
