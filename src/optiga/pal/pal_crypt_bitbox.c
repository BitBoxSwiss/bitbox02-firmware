/**
 * SPDX-FileCopyrightText: 2021-2024 Infineon Technologies AG
 * SPDX-License-Identifier: MIT
 *
 * \author Infineon Technologies AG
 *
 * \file pal_crypt_bitbox.c
 *
 * \brief   This file implements the platform abstraction layer APIs for cryptographic functions.
 *
 * \ingroup  grPAL
 *
 * @{
 */

#include "mbedtls/ccm.h"
#include "mbedtls/version.h"
#include "optiga_lib_common.h"
#include "pal_crypt.h"
#include "pal_os_memory.h"

#include <stddef.h>
#include <string.h>

#define PAL_CRYPT_MAX_LABEL_SEED_LENGTH (96U)

// Copied from external/optiga-trust-m/extras/pal/pal_crypt_mbedtls.c and
// adjusted to use our own SHA256/HMAC implementation for
// pal_crypt_tls_prf_sha256(). AES-128-CCM stays on mbedTLS.
extern void rust_hmac_sha256(
    const void* key,
    size_t key_len,
    const void* data,
    size_t data_len,
    uint8_t* out);

// lint --e{818, 715, 830} suppress "argument "p_pal_crypt" is not used in the implementation but
// kept for future use"
pal_status_t pal_crypt_tls_prf_sha256(
    pal_crypt_t* p_pal_crypt,
    const uint8_t* p_secret,
    uint16_t secret_length,
    const uint8_t* p_label,
    uint16_t label_length,
    const uint8_t* p_seed,
    uint16_t seed_length,
    uint8_t* p_derived_key,
    uint16_t derived_key_length)
{
#define PAL_CRYPT_DIGEST_MAX_SIZE (32U)

    pal_status_t return_value = PAL_STATUS_FAILURE;
    uint16_t derive_key_len_index;
    uint16_t hmac_result_length;
    uint8_t md_hmac_temp_array[PAL_CRYPT_MAX_LABEL_SEED_LENGTH + PAL_CRYPT_DIGEST_MAX_SIZE];
    uint8_t hmac_checksum_result[PAL_CRYPT_DIGEST_MAX_SIZE];
    uint16_t final_seed_length = 0;

    (void)p_pal_crypt;

    do {
#ifdef OPTIGA_LIB_DEBUG_NULL_CHECK
        if ((NULL == p_secret) || (NULL == p_label) || (NULL == p_seed) ||
            (NULL == p_derived_key)) {
            break;
        }
#endif // OPTIGA_LIB_DEBUG_NULL_CHECK

        if (sizeof(md_hmac_temp_array) <
            (uint32_t)(PAL_CRYPT_DIGEST_MAX_SIZE + label_length + seed_length)) {
            return_value = PAL_STATUS_INVALID_INPUT;
            break;
        }

        memcpy(md_hmac_temp_array + PAL_CRYPT_DIGEST_MAX_SIZE, p_label, label_length);
        memcpy(md_hmac_temp_array + PAL_CRYPT_DIGEST_MAX_SIZE + label_length, p_seed, seed_length);
        final_seed_length = label_length + seed_length;

        rust_hmac_sha256(
            p_secret,
            secret_length,
            md_hmac_temp_array + PAL_CRYPT_DIGEST_MAX_SIZE,
            final_seed_length,
            md_hmac_temp_array);

        for (derive_key_len_index = 0; derive_key_len_index < derived_key_length;
             derive_key_len_index += PAL_CRYPT_DIGEST_MAX_SIZE) {
            rust_hmac_sha256(
                p_secret,
                secret_length,
                md_hmac_temp_array,
                PAL_CRYPT_DIGEST_MAX_SIZE + final_seed_length,
                hmac_checksum_result);

            hmac_result_length =
                ((derive_key_len_index + PAL_CRYPT_DIGEST_MAX_SIZE) > derived_key_length)
                    ? (derived_key_length % PAL_CRYPT_DIGEST_MAX_SIZE)
                    : PAL_CRYPT_DIGEST_MAX_SIZE;
            memcpy(p_derived_key + derive_key_len_index, hmac_checksum_result, hmac_result_length);

            if ((derive_key_len_index + PAL_CRYPT_DIGEST_MAX_SIZE) >= derived_key_length) {
                break;
            }

            rust_hmac_sha256(
                p_secret,
                secret_length,
                md_hmac_temp_array,
                PAL_CRYPT_DIGEST_MAX_SIZE,
                md_hmac_temp_array);
        }

        return_value = PAL_STATUS_SUCCESS;
    } while (FALSE);

    memset(md_hmac_temp_array, 0x00, sizeof(md_hmac_temp_array));
    memset(hmac_checksum_result, 0x00, sizeof(hmac_checksum_result));
#undef PAL_CRYPT_DIGEST_MAX_SIZE
    return return_value;
}

// lint --e{818, 715, 830} suppress "argument "p_pal_crypt" is not used in the implementation but
// kept for future use"
pal_status_t pal_crypt_encrypt_aes128_ccm(
    pal_crypt_t* p_pal_crypt,
    const uint8_t* p_plain_text,
    uint16_t plain_text_length,
    const uint8_t* p_encrypt_key,
    const uint8_t* p_nonce,
    uint16_t nonce_length,
    const uint8_t* p_associated_data,
    uint16_t associated_data_length,
    uint8_t mac_size,
    uint8_t* p_cipher_text)
{
#define AES128_KEY_BITS_SIZE (16U)
#define MAC_TAG_BUFFER_SIZE (16U)

    pal_status_t return_status = PAL_STATUS_FAILURE;
    uint8_t mac_output[MAC_TAG_BUFFER_SIZE];
    mbedtls_ccm_context sEncrypt;

    (void)p_pal_crypt;
    mbedtls_ccm_init(&sEncrypt);

    do {
#ifdef OPTIGA_LIB_DEBUG_NULL_CHECK
        if ((NULL == p_cipher_text) || (NULL == p_plain_text) || (NULL == p_nonce) ||
            (NULL == p_associated_data) || (NULL == p_encrypt_key)) {
            break;
        }
#endif

        if (0 != mbedtls_ccm_setkey(
                     &sEncrypt, MBEDTLS_CIPHER_ID_AES, p_encrypt_key, 8 * AES128_KEY_BITS_SIZE)) {
            break;
        }

        if (0 != mbedtls_ccm_encrypt_and_tag(
                     &sEncrypt,
                     plain_text_length,
                     p_nonce,
                     nonce_length,
                     p_associated_data,
                     associated_data_length,
                     p_plain_text,
                     p_cipher_text,
                     mac_output,
                     mac_size)) {
            break;
        }

        memcpy((p_cipher_text + plain_text_length), mac_output, mac_size);
        return_status = PAL_STATUS_SUCCESS;
    } while (FALSE);
    mbedtls_ccm_free(&sEncrypt);
#undef AES128_KEY_BITS_SIZE
#undef MAC_TAG_BUFFER_SIZE
    return return_status;
}

// lint --e{818, 715, 830} suppress "argument "p_pal_crypt" is not used in the implementation but
// kept for future use"
pal_status_t pal_crypt_decrypt_aes128_ccm(
    pal_crypt_t* p_pal_crypt,
    const uint8_t* p_cipher_text,
    uint16_t cipher_text_length,
    const uint8_t* p_decrypt_key,
    const uint8_t* p_nonce,
    uint16_t nonce_length,
    const uint8_t* p_associated_data,
    uint16_t associated_data_length,
    uint8_t mac_size,
    uint8_t* p_plain_text)
{
#define AES128_KEY_BITS_SIZE (16U)
    pal_status_t return_status = PAL_STATUS_FAILURE;
    mbedtls_ccm_context sDecrypt;

    (void)p_pal_crypt;
    mbedtls_ccm_init(&sDecrypt);

    do {
#ifdef OPTIGA_LIB_DEBUG_NULL_CHECK
        if ((NULL == p_plain_text) || (NULL == p_cipher_text) || (NULL == p_nonce) ||
            (NULL == p_associated_data) || (NULL == p_decrypt_key)) {
            break;
        }
#endif

        if (0 != mbedtls_ccm_setkey(
                     &sDecrypt, MBEDTLS_CIPHER_ID_AES, p_decrypt_key, 8 * AES128_KEY_BITS_SIZE)) {
            break;
        }

        if (0 != mbedtls_ccm_auth_decrypt(
                     &sDecrypt,
                     (cipher_text_length - mac_size),
                     p_nonce,
                     nonce_length,
                     p_associated_data,
                     associated_data_length,
                     p_cipher_text,
                     p_plain_text,
                     &p_cipher_text[cipher_text_length - mac_size],
                     mac_size)) {
            break;
        }
        return_status = PAL_STATUS_SUCCESS;
    } while (FALSE);
    mbedtls_ccm_free(&sDecrypt);
#undef AES128_KEY_BITS_SIZE
    return return_status;
}

pal_status_t pal_crypt_version(uint8_t* p_crypt_lib_version_info, uint16_t* length)
{
    pal_status_t return_value = PAL_STATUS_FAILURE;
    uint8_t sizeof_version_number = (uint8_t)strlen(MBEDTLS_VERSION_STRING);

    do {
        if (sizeof_version_number > *length) {
            break;
        }

        pal_os_memcpy(p_crypt_lib_version_info, MBEDTLS_VERSION_STRING, sizeof_version_number);
        *length = sizeof_version_number;

        return_value = PAL_STATUS_SUCCESS;

    } while (0);
    return return_value;
}

/**
 * @}
 */
