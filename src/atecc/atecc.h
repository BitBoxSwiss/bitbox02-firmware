// SPDX-License-Identifier: Apache-2.0

#ifndef _ATECC_H_
#define _ATECC_H_

/* ATECC implementation of the secure chip functions. */
/* See securechip.h for the docstrings of the individual functions. */

#include "compiler_util.h"
#include "securechip/securechip.h"
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

USE_RESULT int atecc_setup(const securechip_interface_functions_t* ifs);
USE_RESULT int atecc_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out);
USE_RESULT int atecc_init_new_password(const char* password);
USE_RESULT int atecc_stretch_password(const char* password, uint8_t* stretched_out);
USE_RESULT bool atecc_reset_keys(void);
USE_RESULT bool atecc_gen_attestation_key(uint8_t* pubkey_out);
USE_RESULT bool atecc_attestation_sign(const uint8_t* challenge, uint8_t* signature_out);
USE_RESULT bool atecc_monotonic_increments_remaining(uint32_t* remaining_out);
USE_RESULT bool atecc_random(uint8_t* rand_out);
#if APP_U2F == 1 || FACTORYSETUP == 1
USE_RESULT bool atecc_u2f_counter_set(uint32_t counter);
#endif
#if APP_U2F == 1
USE_RESULT bool atecc_u2f_counter_inc(uint32_t* counter);
#endif
USE_RESULT bool atecc_model(securechip_model_t* model_out);

#endif
