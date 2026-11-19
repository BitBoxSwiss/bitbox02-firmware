// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <pal_crypt.h>

static void test_pal_crypt_tls_prf_sha256(void** state)
{
    (void)state;
    // TLS 1.2 PRF vector from Mbed TLS's test_suite_ssl.data.
    const uint8_t secret[] = {
        0x9b,
        0xbe,
        0x43,
        0x6b,
        0xa9,
        0x40,
        0xf0,
        0x17,
        0xb1,
        0x76,
        0x52,
        0x84,
        0x9a,
        0x71,
        0xdb,
        0x35};
    const uint8_t seed[] = {
        0xa0,
        0xba,
        0x9f,
        0x93,
        0x6c,
        0xda,
        0x31,
        0x18,
        0x27,
        0xa6,
        0xf7,
        0x96,
        0xff,
        0xd5,
        0x19,
        0x8c};
    const uint8_t label[] = "test label";
    const uint8_t expected[] = {0xe3, 0xf2, 0x29, 0xba, 0x72, 0x7b, 0xe1, 0x7b, 0x8d, 0x12,
                                0x26, 0x20, 0x55, 0x7c, 0xd4, 0x53, 0xc2, 0xaa, 0xb2, 0x1d,
                                0x07, 0xc3, 0xd4, 0x95, 0x32, 0x9b, 0x52, 0xd4, 0xe6, 0x1e,
                                0xdb, 0x5a, 0x6b, 0x30, 0x17, 0x91, 0xe9, 0x0d, 0x35, 0xc9};
    uint8_t derived[sizeof(expected) + 1] = {0};
    derived[sizeof(expected)] = 0xa5;
    for (int i = 0; i < 3; i++) {
        assert_int_equal(
            pal_crypt_tls_prf_sha256(
                NULL,
                secret,
                sizeof(secret),
                label,
                sizeof(label) - 1,
                seed,
                sizeof(seed),
                derived,
                sizeof(expected)),
            PAL_STATUS_SUCCESS);
        assert_memory_equal(derived, expected, sizeof(expected));
        assert_int_equal(derived[sizeof(expected)], 0xa5);
    }
}

static void test_pal_crypt_tls_prf_sha256_failure(void** state)
{
    (void)state;
    const uint8_t secret[32] = {0};
    const uint8_t seed[32] = {0};
    const uint8_t label[97] = {0};
    uint8_t derived[40];
    const uint8_t zero[40] = {0};
    memset(derived, 0xa5, sizeof(derived));
    assert_int_equal(
        pal_crypt_tls_prf_sha256(
            NULL,
            secret,
            sizeof(secret),
            label,
            sizeof(label),
            seed,
            sizeof(seed),
            derived,
            sizeof(derived)),
        PAL_STATUS_FAILURE);
    assert_memory_equal(derived, zero, sizeof(derived));
}

static void test_pal_crypt_aes128_ccm(void** state)
{
    (void)state;
    // Checked independently with Python cryptography's AESCCM (8-byte nonce and tag).
    const uint8_t key[] = {
        0xc0,
        0xc1,
        0xc2,
        0xc3,
        0xc4,
        0xc5,
        0xc6,
        0xc7,
        0xc8,
        0xc9,
        0xca,
        0xcb,
        0xcc,
        0xcd,
        0xce,
        0xcf};
    const uint8_t nonce[] = {0x00, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0xa0};
    const uint8_t aad[] = {0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07};
    const uint8_t plain[] = {0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
                             0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e};
    const uint8_t expected[] = {0x0a, 0x8a, 0xb9, 0x76, 0x77, 0xc1, 0xa8, 0x2c, 0xcf, 0x49, 0xdc,
                                0x93, 0x6e, 0x75, 0xa9, 0x65, 0xd4, 0xdd, 0xf2, 0x08, 0x8d, 0x6b,
                                0x94, 0xab, 0x81, 0x39, 0xa9, 0xcb, 0xf2, 0x96, 0x7a};
    uint8_t cipher[sizeof(expected) + 1] = {0};
    uint8_t decrypted[sizeof(plain) + 1] = {0};
    cipher[sizeof(expected)] = 0xa5;
    decrypted[sizeof(plain)] = 0xa5;
    for (int i = 0; i < 3; i++) {
        // OPTIGA encrypts and decrypts its communication buffer in place.
        memcpy(cipher, plain, sizeof(plain));
        assert_int_equal(
            pal_crypt_encrypt_aes128_ccm(
                NULL,
                i == 0 ? plain : cipher,
                sizeof(plain),
                key,
                nonce,
                sizeof(nonce),
                aad,
                sizeof(aad),
                8,
                cipher),
            PAL_STATUS_SUCCESS);
        assert_memory_equal(cipher, expected, sizeof(expected));
        assert_int_equal(cipher[sizeof(expected)], 0xa5);
        assert_int_equal(
            pal_crypt_decrypt_aes128_ccm(
                NULL,
                cipher,
                sizeof(expected),
                key,
                nonce,
                sizeof(nonce),
                aad,
                sizeof(aad),
                8,
                decrypted),
            PAL_STATUS_SUCCESS);
        assert_memory_equal(decrypted, plain, sizeof(plain));
        assert_int_equal(decrypted[sizeof(plain)], 0xa5);
        assert_int_equal(
            pal_crypt_decrypt_aes128_ccm(
                NULL,
                cipher,
                sizeof(expected),
                key,
                nonce,
                sizeof(nonce),
                aad,
                sizeof(aad),
                8,
                cipher),
            PAL_STATUS_SUCCESS);
        assert_memory_equal(cipher, plain, sizeof(plain));
        assert_int_equal(cipher[sizeof(expected)], 0xa5);
    }

    memcpy(cipher, expected, sizeof(expected));
    cipher[sizeof(expected) - 1] ^= 1;
    assert_int_equal(
        pal_crypt_decrypt_aes128_ccm(
            NULL,
            cipher,
            sizeof(expected),
            key,
            nonce,
            sizeof(nonce),
            aad,
            sizeof(aad),
            8,
            decrypted),
        PAL_STATUS_FAILURE);
    const uint8_t zero[sizeof(plain)] = {0};
    assert_memory_equal(decrypted, zero, sizeof(zero));
    assert_int_equal(decrypted[sizeof(plain)], 0xa5);

    memset(decrypted, 0xa5, sizeof(decrypted));
    assert_int_equal(
        pal_crypt_decrypt_aes128_ccm(
            NULL, cipher, 7, key, nonce, sizeof(nonce), aad, sizeof(aad), 8, decrypted),
        PAL_STATUS_FAILURE);
    assert_int_equal(decrypted[0], 0xa5);
}

static void test_pal_crypt_aes128_ccm_empty_and_invalid(void** state)
{
    (void)state;
    const uint8_t key[16] = {0};
    const uint8_t nonce[8] = {0};
    uint8_t cipher[9] = {0};
    uint8_t plain[1] = {0xa5};
    cipher[8] = 0xa5;
    assert_int_equal(
        pal_crypt_encrypt_aes128_ccm(NULL, NULL, 0, key, nonce, sizeof(nonce), NULL, 0, 8, cipher),
        PAL_STATUS_SUCCESS);
    assert_int_equal(cipher[8], 0xa5);
    assert_int_equal(
        pal_crypt_decrypt_aes128_ccm(NULL, cipher, 8, key, nonce, sizeof(nonce), NULL, 0, 8, plain),
        PAL_STATUS_SUCCESS);
    assert_int_equal(plain[0], 0xa5);

    assert_int_equal(
        pal_crypt_encrypt_aes128_ccm(
            NULL, plain, 1, key, nonce, sizeof(nonce) - 1, NULL, 0, 8, cipher),
        PAL_STATUS_FAILURE);
    assert_int_equal(
        pal_crypt_encrypt_aes128_ccm(NULL, plain, 1, key, nonce, sizeof(nonce), NULL, 0, 4, cipher),
        PAL_STATUS_FAILURE);
    assert_int_equal(
        pal_crypt_decrypt_aes128_ccm(
            NULL, cipher, 8, NULL, nonce, sizeof(nonce), NULL, 0, 8, plain),
        PAL_STATUS_FAILURE);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_pal_crypt_tls_prf_sha256),
        cmocka_unit_test(test_pal_crypt_tls_prf_sha256_failure),
        cmocka_unit_test(test_pal_crypt_aes128_ccm),
        cmocka_unit_test(test_pal_crypt_aes128_ccm_empty_and_invalid),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
