// SPDX-License-Identifier: Apache-2.0

#ifndef _SECURECHIP_H_
#define _SECURECHIP_H_

#include "compiler_util.h"
#include <memory/memory.h>
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Keep in sync with securechip.rs `SECURECHIP_ERRORS`.
typedef enum {
    // Errors common to any securechip implementation
    SC_ERR_IFS = -1,
    SC_ERR_INVALID_ARGS = -2,
    SC_ERR_CONFIG_MISMATCH = -3,
    SC_ERR_SALT = -4,
    // Currently only used by Optiga, but it is in the common errors so that the API of the
    // securechip is consistent and the caller does not need to distinguish between the chips at the
    // callsite.
    SC_ERR_INCORRECT_PASSWORD = -6,
    // The password stretch algo is not supported
    SC_ERR_INVALID_PASSWORD_STRETCH_ALGO = -7,
    SC_ERR_MEMORY = -8,

    // Errors specific to the ATECC
    SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG = -100,
    SC_ATECC_ERR_ZONE_UNLOCKED_DATA = -101,
    SC_ATECC_ERR_SLOT_UNLOCKED_IO = -103,
    SC_ATECC_ERR_SLOT_UNLOCKED_AUTH = -104,
    SC_ATECC_ERR_SLOT_UNLOCKED_ENC = -105,
    SC_ATECC_ERR_RESET_KEYS = -106,

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
 * Prepare the securechip for a new password: re-initialize keys used in the derivation,
 * set up monotonic counters, etc.
 * @param[in] password The user password.
 * @param[in] password_stretch_algo the password stretching algorithm that should be used.
 * @param[out] stretched_out the stretched password. Same as calling `securechip_stretch_password()`
 * with the same stretching algo, but more efficient in terms of securechip operations.
 * @return For ATECC: values of `atecc_error_t` if negative, values of `ATCA_STATUS` if positive, 0
 * on success. For Optiga: values of `optiga_error_t` if negative, values of
 * optiga_lib_return_codes.h if positive, 0 on success.
 */
USE_RESULT int securechip_init_new_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out);

/**
 * Stretch password using secrets in the secure chip.
 * Calling this function increments the monotonic counter.
 * @param[in] password The user password.
 * @param[in] password_stretch_algo the password stretching algorithm that should be used.
 * @param[out] stretched_out the stretched password.
 * @return 0 on success. Values of `securechip_error_t` if negative. If positive, values of
 * `ATCA_STATUS` for ATECC, values of optiga_lib_return_codes.h for Optiga.
 */
USE_RESULT int securechip_stretch_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out);

/**
 * Reset the securechip objects involved in the password stretching.
 * @return true on success, false on failure.
 */
USE_RESULT bool securechip_reset_keys(void);

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
