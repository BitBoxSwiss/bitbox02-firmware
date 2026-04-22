// SPDX-License-Identifier: Apache-2.0

#ifndef _OPTIGA_OPS_H_
#define _OPTIGA_OPS_H_

#include <optiga_crypt.h>
#include <optiga_util.h>
#include <stdint.h>

optiga_lib_status_t optiga_ops_create(optiga_util_t** util_out, optiga_crypt_t** crypt_out);
optiga_lib_status_t optiga_ops_get_status(void);
void optiga_ops_set_status_busy(void);

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

optiga_lib_status_t optiga_ops_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t* me,
    optiga_ecc_curve_t curve_id,
    optiga_key_usage_t key_usage,
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

optiga_lib_status_t optiga_ops_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length);

#endif // _OPTIGA_OPS_H_
