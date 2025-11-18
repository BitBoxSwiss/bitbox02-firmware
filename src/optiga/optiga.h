// SPDX-License-Identifier: Apache-2.0

#ifndef _OPTIGA_H_
#define _OPTIGA_H_

/* Optiga Trust M implementation of the secure chip functions. */
/* See securechip.h for the docstrings of the individual functions. */

#include "compiler_util.h"
#include "securechip/securechip.h"
#include <memory/bitbox02_smarteeprom.h>
#include <memory/memory.h>
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <rust/rust.h> // for MAX_UNLOCK_ATTEMPTS

// The Data Object IDs we use.

// Stores a shared secret used for a shielded connection. Is is used to encrypt
// communications. Read/write disabled.
#define OID_PLATFORM_BINDING 0xE140
// CMAC key slot, read/write prohibited. Used to perform KDF using CMAC. Key is regenerated at
// factory setup and with each device reset. Monotonic counter `OID_COUNTER` attached.
#define OID_AES_SYMKEY 0xE200
// HMAC key slot, read prohibited, write allowed. 32 random bytes are written to it at factory setup
// and with each device reset.
#define OID_HMAC 0xF1D0
// Arbitrary data object, stores up to 140 bytes. Used to store the U2F counter.
#define OID_ARBITRARY_DATA 0xF1D1
// ECC slot used for creating the device attestation key and signing with it. Read/write
// disabled. The Key is internally generated at factory setup and used to sign the device
// attestation host challenge.
#define OID_ATTESTATION 0xE0F1
// Monotonic counter, initialized at 0 and attached to `OID_AES_SYMKEY` - every CMAC generation
// increments the counter. When the threshold `MONOTONIC_COUNTER_MAX_USE` is reached, further CMAC
// computations return an error.
#define OID_COUNTER 0xE120

// The three objects below (`OID_PASSWORD_SECRET`, `OID_PASSWORD`, `OID_COUNTER_PASSWORD`) deal with
// implementing the small monotonic counter that limits the number of unlocks to a small number.

// A random shared key which authorizes updating `OID_PASSWORD` and `OID_COUNTER_PASSWORD`.
// It is also part of the password stretching.
#define OID_PASSWORD_SECRET 0xF1D2
// A hmac digest of the device password.
// Authorizes reading `OID_PASSWORD`. Monotonic counter `OID_COUNTER_PASSWORD` attached to
// authorization, which limits the number of unlock attempts to a small number.
#define OID_PASSWORD 0xF1D3
// Monotonic counter with a small limit. Every password stretch increments the counter. A correct
// password resets the counter. When the threshold `SMALL_MONOTONIC_COUNTER_MAX_USE` is reached,
// further password stretches return an error.
#define OID_COUNTER_PASSWORD 0xE121

// Number of times the password can be entered incorrectly before further password stretching fails.
// The counter is reset when the correct password is entered.
#define SMALL_MONOTONIC_COUNTER_MAX_USE (MAX_UNLOCK_ATTEMPTS)

USE_RESULT int optiga_setup(const securechip_interface_functions_t* ifs);
USE_RESULT int optiga_kdf_external(const uint8_t* msg, size_t len, uint8_t* mac_out);
USE_RESULT int optiga_init_new_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo);
USE_RESULT int optiga_stretch_password(
    const char* password,
    memory_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out);
USE_RESULT bool optiga_reset_keys(void);
USE_RESULT bool optiga_gen_attestation_key(uint8_t* pubkey_out);
USE_RESULT bool optiga_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);
USE_RESULT bool optiga_monotonic_increments_remaining(uint32_t* remaining_out);
USE_RESULT bool optiga_random(uint8_t* rand_out);
#if APP_U2F == 1 || FACTORYSETUP == 1
USE_RESULT bool optiga_u2f_counter_set(uint32_t counter);
#endif
#if APP_U2F == 1
USE_RESULT bool optiga_u2f_counter_inc(uint32_t* counter);
#endif
USE_RESULT bool optiga_model(securechip_model_t* model_out);

#endif // _OPTIGA_H_
