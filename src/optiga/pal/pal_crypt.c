// SPDX-License-Identifier: Apache-2.0

#include "pal_crypt.h"
#include <rust/rust.h>
#include <string.h>

#define AES128_KEY_LENGTH 16
#define NONCE_LENGTH 8
#define TAG_LENGTH 8

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
    (void)p_pal_crypt;
    if (p_secret == NULL || p_label == NULL || p_seed == NULL || p_derived_key == NULL) {
        return PAL_STATUS_FAILURE;
    }
    memset(p_derived_key, 0, derived_key_length);
    return rust_optiga_tls_prf_sha256(
               rust_util_bytes(p_secret, secret_length),
               rust_util_bytes(p_label, label_length),
               rust_util_bytes(p_seed, seed_length),
               rust_util_bytes_mut(p_derived_key, derived_key_length))
               ? PAL_STATUS_SUCCESS
               : PAL_STATUS_FAILURE;
}

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
    (void)p_pal_crypt;
    if ((p_plain_text == NULL && plain_text_length != 0) || p_encrypt_key == NULL ||
        p_nonce == NULL || (p_associated_data == NULL && associated_data_length != 0) ||
        p_cipher_text == NULL || nonce_length != NONCE_LENGTH || mac_size != TAG_LENGTH) {
        return PAL_STATUS_FAILURE;
    }
    // OPTIGA can use the same buffer for input and output.
    if (plain_text_length != 0) {
        memmove(p_cipher_text, p_plain_text, plain_text_length);
    }
    memset(p_cipher_text + plain_text_length, 0, TAG_LENGTH);
    return rust_optiga_encrypt_aes128_ccm(
               rust_util_bytes(p_encrypt_key, AES128_KEY_LENGTH),
               rust_util_bytes(p_nonce, nonce_length),
               rust_util_bytes(p_associated_data, associated_data_length),
               rust_util_bytes_mut(p_cipher_text, (size_t)plain_text_length + TAG_LENGTH))
               ? PAL_STATUS_SUCCESS
               : PAL_STATUS_FAILURE;
}

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
    (void)p_pal_crypt;
    if (p_cipher_text == NULL || p_decrypt_key == NULL || p_nonce == NULL ||
        (p_associated_data == NULL && associated_data_length != 0) || p_plain_text == NULL ||
        nonce_length != NONCE_LENGTH || mac_size != TAG_LENGTH || cipher_text_length < TAG_LENGTH) {
        return PAL_STATUS_FAILURE;
    }
    size_t plain_text_length = cipher_text_length - TAG_LENGTH;
    uint8_t tag[TAG_LENGTH] = {0};
    memcpy(tag, p_cipher_text + plain_text_length, TAG_LENGTH);
    memmove(p_plain_text, p_cipher_text, plain_text_length);
    return rust_optiga_decrypt_aes128_ccm(
               rust_util_bytes(p_decrypt_key, AES128_KEY_LENGTH),
               rust_util_bytes(p_nonce, nonce_length),
               rust_util_bytes(p_associated_data, associated_data_length),
               rust_util_bytes(tag, sizeof(tag)),
               rust_util_bytes_mut(p_plain_text, plain_text_length))
               ? PAL_STATUS_SUCCESS
               : PAL_STATUS_FAILURE;
}
