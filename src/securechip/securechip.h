// SPDX-License-Identifier: Apache-2.0

#ifndef _SECURECHIP_H_
#define _SECURECHIP_H_

#include "compiler_util.h"
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

typedef enum {
    // Legacy/initial value for BitBox02 and BitBox02 Nova using the initial stretch algo in
    // ATECC/Optiga.
    SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
    // Currently used only by Optiga.
    SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
} securechip_password_stretch_algo_t;

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

/* The common securechip ABI is implemented in Rust and declared in rust/rust.h. */

typedef enum {
    ATECC_ATECC608A,
    ATECC_ATECC608B,
    OPTIGA_TRUST_M_V3,
} securechip_model_t;

#endif
