// SPDX-License-Identifier: Apache-2.0

#ifndef _ATECC_H_
#define _ATECC_H_

/* ATECC implementation of the secure chip functions. */

#include "compiler_util.h"
#include "securechip/securechip.h"
#include <platform/platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
    ATECC_SLOT_IO_PROTECTION_KEY = 0,
    ATECC_SLOT_AUTHKEY = 1,
    ATECC_SLOT_ENCRYPTION_KEY = 2,
    ATECC_SLOT_ROLLKEY = 3,
    ATECC_SLOT_KDF = 4,
    ATECC_SLOT_ATTESTATION = 5,
    // Deprecated as the equivalent does not exist in the Optiga chip.
    ATECC_SLOT_ECC_UNSAFE_SIGN_DEPRECATED = 6,
    ATECC_SLOT_DATA0 = 9,
    // The other slots are currently not in use.
} atecc_slot_t;

USE_RESULT int atecc_setup(
    const uint8_t* io_protection_key,
    const uint8_t* auth_key,
    const uint8_t* encryption_key);
USE_RESULT bool atecc_gen_attestation_key(const uint8_t* auth_key, uint8_t* pubkey_out);

const uint8_t* atecc_serial_number(void);
bool atecc_serial_number_is_cached(void);

#define ATECC_OPS_STATUS_BUSY 0x100

int atecc_ops_get_status(void);
uint32_t atecc_ops_get_poll_delay_ms(void);
void atecc_ops_poll(void);

USE_RESULT int atecc_cmd_start_nonce_rand(const uint8_t* num_in);
USE_RESULT int atecc_cmd_start_checkmac(const uint8_t* response);
USE_RESULT int atecc_cmd_start_random(void);
USE_RESULT int atecc_cmd_start_counter_read(void);
USE_RESULT int atecc_cmd_start_info_revision(void);
USE_RESULT int atecc_cmd_start_kdf(atecc_slot_t slot, const uint8_t* msg, size_t len);
USE_RESULT int atecc_cmd_start_derivekey_rollkey(void);
USE_RESULT int atecc_cmd_start_nonce_load_msgdigest(const uint8_t* msg);
USE_RESULT int atecc_cmd_start_sign_attestation(void);
USE_RESULT int atecc_cmd_start_gendig_encryption_key(void);
USE_RESULT int atecc_cmd_start_read_block(uint16_t slot, uint8_t block);
USE_RESULT int atecc_cmd_start_write_encrypted_block(
    uint16_t slot,
    uint8_t block,
    const uint8_t* value,
    const uint8_t* mac);

USE_RESULT int atecc_cmd_read_random_response(uint8_t* out);
USE_RESULT int atecc_cmd_read_counter_response(uint32_t* counter_out);
USE_RESULT int atecc_cmd_read_info_response(uint8_t* out);
USE_RESULT int atecc_cmd_read_kdf_response(uint8_t* out_data, uint8_t* out_nonce);
USE_RESULT int atecc_cmd_read_sign_response(uint8_t* signature_out);
USE_RESULT int atecc_cmd_read_block_response(uint8_t* out);

USE_RESULT int atecc_auth_compute_response(
    const uint8_t* num_in,
    const uint8_t* rand_out,
    const uint8_t* auth_key,
    uint8_t* response_out);
USE_RESULT int atecc_kdf_decrypt(
    const uint8_t* io_protection_key,
    const uint8_t* nonce_out,
    uint8_t* data,
    size_t data_size);
USE_RESULT int atecc_io_prepare_tempkey(const uint8_t* num_in, const uint8_t* rand_out);
USE_RESULT int atecc_io_apply_gendig(const uint8_t* encryption_key);
USE_RESULT int atecc_io_prepare_encrypted_write(
    uint16_t key_id,
    uint8_t block,
    const uint8_t* input_data,
    uint8_t* encrypted_out,
    uint8_t* mac_out);
USE_RESULT int atecc_io_decrypt_block(uint8_t* data, size_t len);

#endif
