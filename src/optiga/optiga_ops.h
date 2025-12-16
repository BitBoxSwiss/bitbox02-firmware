// SPDX-License-Identifier: Apache-2.0

#ifndef _OPTIGA_OPS_H_
#define _OPTIGA_OPS_H_

#include <optiga_crypt.h>
#include <optiga_util.h>
#include <stdint.h>

optiga_lib_status_t optiga_ops_create(optiga_util_t** util_out, optiga_crypt_t** crypt_out);

optiga_lib_status_t optiga_ops_util_read_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint16_t offset,
    uint8_t* buffer,
    uint16_t* length);

optiga_lib_status_t optiga_ops_util_write_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t write_type,
    uint16_t offset,
    const uint8_t* buffer,
    uint16_t length);

optiga_lib_status_t optiga_ops_util_read_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t* buffer,
    uint16_t* length);

optiga_lib_status_t optiga_ops_util_write_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    const uint8_t* buffer,
    uint8_t length);

optiga_lib_status_t optiga_ops_util_open_application_sync(
    optiga_util_t* me,
    bool_t perform_restore);

optiga_lib_status_t optiga_ops_util_close_application_sync(
    optiga_util_t* me,
    bool_t perform_hibernate);

optiga_lib_status_t optiga_ops_crypt_hmac_sync(
    optiga_crypt_t* me,
    optiga_hmac_type_t type,
    uint16_t secret,
    const uint8_t* input_data,
    uint32_t input_data_length,
    uint8_t* mac,
    uint32_t* mac_length);

optiga_lib_status_t optiga_ops_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t* me,
    optiga_ecc_curve_t curve_id,
    uint8_t key_usage,
    bool_t export_private_key,
    void* private_key,
    uint8_t* public_key,
    uint16_t* public_key_length);

optiga_lib_status_t optiga_ops_crypt_ecdsa_sign_sync(
    optiga_crypt_t* me,
    const uint8_t* digest,
    uint8_t digest_length,
    optiga_key_id_t private_key,
    uint8_t* signature,
    uint16_t* signature_length);

optiga_lib_status_t optiga_ops_crypt_symmetric_encrypt_sync(
    optiga_crypt_t* me,
    optiga_symmetric_encryption_mode_t encryption_mode,
    optiga_key_id_t symmetric_key_oid,
    const uint8_t* plain_data,
    uint32_t plain_data_length,
    const uint8_t* iv,
    uint16_t iv_length,
    const uint8_t* associated_data,
    uint16_t associated_data_length,
    uint8_t* encrypted_data,
    uint32_t* encrypted_data_length);

optiga_lib_status_t optiga_ops_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length);

optiga_lib_status_t optiga_ops_crypt_symmetric_generate_key_sync(
    optiga_crypt_t* me,
    optiga_symmetric_key_type_t key_type,
    uint8_t key_usage,
    bool_t export_symmetric_key,
    void* symmetric_key);

optiga_lib_status_t optiga_ops_crypt_generate_auth_code_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    const uint8_t* optional_data,
    uint16_t optional_data_length,
    uint8_t* random_data,
    uint16_t random_data_length);

optiga_lib_status_t optiga_ops_crypt_clear_auto_state_sync(optiga_crypt_t* me, uint16_t secret);

optiga_lib_status_t optiga_ops_crypt_hmac_verify_sync(
    optiga_crypt_t* me,
    optiga_hmac_type_t type,
    uint16_t secret,
    const uint8_t* input_data,
    uint32_t input_data_length,
    const uint8_t* hmac,
    uint32_t hmac_length);

#endif // _OPTIGA_OPS_H_
