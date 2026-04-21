// SPDX-License-Identifier: Apache-2.0

#ifndef _OPTIGA_H_
#define _OPTIGA_H_

/* Optiga Trust M implementation of the secure chip functions. */

#include "compiler_util.h"
#include "securechip/securechip.h"
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stdint.h>

typedef struct optiga_util optiga_util_t;
typedef struct optiga_crypt optiga_crypt_t;

// Keep in sync with MAX_UNLOCK_ATTEMPTS in keystore.rs.
#ifndef MAX_UNLOCK_ATTEMPTS
    #define MAX_UNLOCK_ATTEMPTS 10
#endif

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
// Like the HMAC slot, but writing to it needs authorization by `OID_PASSWORD`, and it is attached
// to monotonic counter `OID_COUNTER_HMAC_WRITEPROTECTED`.
#define OID_HMAC_WRITEPROTECTED 0xF1D4
// Monotonic counter with a small limit.  Every password stretch increments the counter. A correct
// password resets the counter. When the threshold `SMALL_MONOTONIC_COUNTER_MAX_USE` is reached,
// further password stretches return an error.
#define OID_COUNTER_HMAC_WRITEPROTECTED 0xE122
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
// Number of times the first KDF slot can be used over the lifetime of the device. The maximum
// does not seem to be specified, so we use something a little below the endurance indication of
// 600000 updates. See Solution Reference Manual Figure 32.
#define MONOTONIC_COUNTER_MAX_USE (590000)

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

// Values of life cycle states.
// See Table "Life Cycle Status":
// https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#link05d4c12a_5c94_4a05_a05d_102c53684d3d
#define LCSO_STATE_CREATION (0x01)
#define LCSO_STATE_OPERATIONAL (0x07)

// Set the object LcsO flag to Operational. After this, the metadata cannot be changed anymore.
// During development, set this to `LCSO_STATE_CREATION`.
#define FINAL_LCSO_STATE_V0 LCSO_STATE_OPERATIONAL
#define FINAL_LCSO_STATE_V1 LCSO_STATE_OPERATIONAL

// See Solution Reference Manual Table 79 "Data structure arbitrary data object".
#define ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE 140

// Maximum size of metadata. See "Metadata Update Identifier":
// https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linka946a953_def2_41cf_850a_74fb7899fe11
// Two extra bytes for the `0x20 <len>` header bytes.
#define METADATA_MAX_SIZE (44 + 2)

USE_RESULT int optiga_setup(const securechip_interface_functions_t* ifs);
USE_RESULT bool optiga_gen_attestation_key(uint8_t* pubkey_out);
USE_RESULT bool optiga_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);
USE_RESULT optiga_util_t* optiga_util_instance(void);
USE_RESULT optiga_crypt_t* optiga_crypt_instance(void);
USE_RESULT bool optiga_ifs_random_32_bytes(uint8_t* rand_out);
USE_RESULT int optiga_random(uint8_t* rand_out);
#if APP_U2F == 1 || FACTORYSETUP == 1
USE_RESULT bool optiga_u2f_counter_set(uint32_t counter);
#endif
#if APP_U2F == 1
USE_RESULT bool optiga_u2f_counter_inc(uint32_t* counter);
#endif

#endif // _OPTIGA_H_
