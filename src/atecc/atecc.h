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

#define ATECC_KDF_ASYNC_CONTEXT_SIZE 1024u
#define ATECC_RESET_KEYS_ASYNC_CONTEXT_SIZE 1536u
#define ATECC_STRETCH_PASSWORD_ASYNC_CONTEXT_SIZE 1536u
#define ATECC_INIT_NEW_PASSWORD_ASYNC_CONTEXT_SIZE 3072u

typedef struct {
    uint8_t opaque[ATECC_KDF_ASYNC_CONTEXT_SIZE];
} atecc_kdf_async_ctx_t;

typedef struct {
    uint8_t opaque[ATECC_RESET_KEYS_ASYNC_CONTEXT_SIZE];
} atecc_reset_keys_async_ctx_t;

typedef struct {
    uint8_t opaque[ATECC_STRETCH_PASSWORD_ASYNC_CONTEXT_SIZE];
} atecc_stretch_password_async_ctx_t;

typedef struct {
    uint8_t opaque[ATECC_INIT_NEW_PASSWORD_ASYNC_CONTEXT_SIZE];
} atecc_init_new_password_async_ctx_t;

USE_RESULT int atecc_setup(const securechip_interface_functions_t* ifs);
USE_RESULT int atecc_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out);
USE_RESULT int atecc_kdf_async_start(
    atecc_kdf_async_ctx_t* ctx,
    const uint8_t* msg,
    size_t len,
    uint8_t* kdf_out,
    uint16_t* wait_ms_out);
USE_RESULT int atecc_kdf_async_poll(atecc_kdf_async_ctx_t* ctx, uint16_t* wait_ms_out);
void atecc_kdf_async_abort(atecc_kdf_async_ctx_t* ctx);
USE_RESULT int atecc_init_new_password(
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out);
USE_RESULT int atecc_init_new_password_async_start(
    atecc_init_new_password_async_ctx_t* ctx,
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out,
    uint16_t* wait_ms_out);
USE_RESULT int atecc_init_new_password_async_poll(
    atecc_init_new_password_async_ctx_t* ctx,
    uint16_t* wait_ms_out);
void atecc_init_new_password_async_abort(atecc_init_new_password_async_ctx_t* ctx);
USE_RESULT int atecc_stretch_password(
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out);
USE_RESULT int atecc_stretch_password_async_start(
    atecc_stretch_password_async_ctx_t* ctx,
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out,
    uint16_t* wait_ms_out);
USE_RESULT int atecc_stretch_password_async_poll(
    atecc_stretch_password_async_ctx_t* ctx,
    uint16_t* wait_ms_out);
void atecc_stretch_password_async_abort(atecc_stretch_password_async_ctx_t* ctx);
USE_RESULT bool atecc_reset_keys(void);
USE_RESULT int atecc_reset_keys_async_start(
    atecc_reset_keys_async_ctx_t* ctx,
    uint16_t* wait_ms_out);
USE_RESULT int atecc_reset_keys_async_poll(
    atecc_reset_keys_async_ctx_t* ctx,
    uint16_t* wait_ms_out);
void atecc_reset_keys_async_abort(atecc_reset_keys_async_ctx_t* ctx);
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
