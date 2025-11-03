// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <string.h>

#include "cipher/cipher.h"
#include "hardfault.h"
#include "keystore.h"
#include "memory/bitbox02_smarteeprom.h"
#include "memory/memory.h"
#include "random.h"
#include "reset.h"
#include "salt.h"
#include "securechip/securechip.h"
#include "util.h"
#include <usb/usb_processing.h>

#include <rust/rust.h>
#include <secp256k1_ecdsa_s2c.h>

// Change this ONLY via keystore_unlock() or keystore_lock()
static bool _is_unlocked_device = false;
// Stores a random key after unlock which, after stretching, is used to encrypt the retained seed.
static uint8_t _unstretched_retained_seed_encryption_key[32] = {0};
// Must be defined if is_unlocked is true. ONLY ACCESS THIS WITH keystore_copy_seed().
// Stores the encrypted seed after unlock.
static uint8_t _retained_seed_encrypted[KEYSTORE_MAX_SEED_LENGTH + 64] = {0};
static size_t _retained_seed_encrypted_len = 0;
// A hash of the unencrypted retained seed, used for comparing seeds without knowing their
// plaintext.
static uint8_t _retained_seed_hash[32] = {0};

// Change this ONLY via keystore_unlock_bip39_finalize().
static bool _is_unlocked_bip39 = false;
// Stores a random key after bip39-unlock which, after stretching, is used to encrypt the retained
// bip39 seed.
static uint8_t _unstretched_retained_bip39_seed_encryption_key[32] = {0};
// Must be defined if _is_unlocked is true. ONLY ACCESS THIS WITH keystore_copy_bip39_seed().
// Stores the encrypted BIP-39 seed after bip39-unlock.
static uint8_t _retained_bip39_seed_encrypted[64 + 64] = {0};
static size_t _retained_bip39_seed_encrypted_len = 0;

// Unlocking the keystore take longer than the 500ms watchdog we have setup. Reset the watchdog
// counter to (~7s) to avoid incorrectly assuming we lost communication with the app.
#define LONG_TIMEOUT (-70)

/**
 * We allow seeds of 16, 24 or 32 bytes.
 */
static bool _validate_seed_length(size_t seed_len)
{
    return seed_len == 16 || seed_len == 24 || seed_len == 32;
}

bool keystore_copy_seed(uint8_t* seed_out, size_t* length_out)
{
    if (!_is_unlocked_device) {
        return false;
    }

    uint8_t retained_seed_encryption_key[32] = {0};
    UTIL_CLEANUP_32(retained_seed_encryption_key);
    if (!rust_keystore_stretch_retained_seed_encryption_key(
            rust_util_bytes(
                _unstretched_retained_seed_encryption_key,
                sizeof(_unstretched_retained_seed_encryption_key)),
            "keystore_retained_seed_access_in",
            "keystore_retained_seed_access_out",
            rust_util_bytes_mut(
                retained_seed_encryption_key, sizeof(retained_seed_encryption_key)))) {
        return false;
    }
    size_t len = _retained_seed_encrypted_len - 48;
    bool password_correct = cipher_aes_hmac_decrypt(
        _retained_seed_encrypted,
        _retained_seed_encrypted_len,
        seed_out,
        &len,
        retained_seed_encryption_key);
    if (!password_correct) {
        // Should never happen.
        return false;
    }
    *length_out = len;
    return true;
}

bool keystore_copy_bip39_seed(uint8_t* bip39_seed_out)
{
    if (!_is_unlocked_bip39) {
        return false;
    }

    uint8_t retained_bip39_seed_encryption_key[32] = {0};
    UTIL_CLEANUP_32(retained_bip39_seed_encryption_key);
    if (!rust_keystore_stretch_retained_seed_encryption_key(
            rust_util_bytes(
                _unstretched_retained_bip39_seed_encryption_key,
                sizeof(_unstretched_retained_bip39_seed_encryption_key)),
            "keystore_retained_bip39_seed_access_in",
            "keystore_retained_bip39_seed_access_out",
            rust_util_bytes_mut(
                retained_bip39_seed_encryption_key, sizeof(retained_bip39_seed_encryption_key)))) {
        return false;
    }
    size_t len = _retained_bip39_seed_encrypted_len - 48;
    bool password_correct = cipher_aes_hmac_decrypt(
        _retained_bip39_seed_encrypted,
        _retained_bip39_seed_encrypted_len,
        bip39_seed_out,
        &len,
        retained_bip39_seed_encryption_key);
    if (!password_correct) {
        // Should never happen.
        return false;
    }
    if (len != 64) {
        // Should never happen.
        return false;
    }
    // sanity check
    uint8_t zero[64] = {0};
    util_zero(zero, 64);
    if (MEMEQ(bip39_seed_out, zero, 64)) {
        return false;
    }
    return true;
}

/**
 * Retrieves the encrypted seed and attempts to decrypt it using the password.
 *
 * `securechip_result_out`, if not NULL, will contain the error code from `securechip_kdf()` if
 * there was a secure chip error, and 0 otherwise.
 */
static keystore_error_t _get_and_decrypt_seed(
    const char* password,
    uint8_t* decrypted_seed_out,
    size_t* decrypted_seed_len_out,
    int* securechip_result_out)
{
    uint8_t encrypted_seed_and_hmac[96];
    UTIL_CLEANUP_32(encrypted_seed_and_hmac);
    uint8_t encrypted_len;
    if (!memory_get_encrypted_seed_and_hmac(encrypted_seed_and_hmac, &encrypted_len)) {
        return KEYSTORE_ERR_MEMORY;
    }
    uint8_t secret[32];
    UTIL_CLEANUP_32(secret);
    int stretch_result = securechip_stretch_password(password, secret);
    if (stretch_result) {
        if (stretch_result == SC_ERR_INCORRECT_PASSWORD) {
            // Our Optiga securechip implementation fails password stretching if the password is
            // wrong, so we can early-abort here. The ATECC stretches the password without checking
            // if the password is correct, and we determine if it is correct in the seed decryption
            // step below.
            return KEYSTORE_ERR_INCORRECT_PASSWORD;
        }
        if (securechip_result_out != NULL) {
            *securechip_result_out = stretch_result;
        }
        return KEYSTORE_ERR_SECURECHIP;
    }
    if (encrypted_len < 49) {
        Abort("_get_and_decrypt_seed: underflow / zero size");
    }
    size_t decrypted_len = encrypted_len - 48;
    uint8_t decrypted[decrypted_len];
    bool password_correct = cipher_aes_hmac_decrypt(
        encrypted_seed_and_hmac, encrypted_len, decrypted, &decrypted_len, secret);
    if (!password_correct) {
        return KEYSTORE_ERR_INCORRECT_PASSWORD;
    }
    if (!_validate_seed_length(decrypted_len)) {
        util_zero(decrypted, sizeof(decrypted));
        return KEYSTORE_ERR_SEED_SIZE;
    }
    *decrypted_seed_len_out = decrypted_len;
    memcpy(decrypted_seed_out, decrypted, decrypted_len);

    return KEYSTORE_OK;
}

static bool _verify_seed(
    const uint8_t* encryption_key,
    const uint8_t* expected_seed,
    size_t expected_seed_len)
{
    uint8_t encrypted_seed_and_hmac[96];
    UTIL_CLEANUP_32(encrypted_seed_and_hmac);
    uint8_t encrypted_len;
    if (!memory_get_encrypted_seed_and_hmac(encrypted_seed_and_hmac, &encrypted_len)) {
        return false;
    }
    if (encrypted_len < 49) {
        Abort("_verify_seed: underflow / zero size");
    }
    size_t decrypted_len = encrypted_len - 48;
    uint8_t decrypted[decrypted_len];
    bool password_correct = cipher_aes_hmac_decrypt(
        encrypted_seed_and_hmac, encrypted_len, decrypted, &decrypted_len, encryption_key);
    if (!password_correct) {
        return false;
    }
    if (expected_seed_len != decrypted_len) {
        util_zero(decrypted, sizeof(decrypted));
        return false;
    }
    if (!MEMEQ(expected_seed, decrypted, expected_seed_len)) {
        util_zero(decrypted, sizeof(decrypted));
        return false;
    }
    util_zero(decrypted, sizeof(decrypted));
    return true;
}

static keystore_error_t _hash_seed(const uint8_t* seed, size_t seed_len, uint8_t* out)
{
    uint8_t salted_key[32] = {0};
    if (!salt_hash_data(NULL, 0, "keystore_retain_seed_hash", salted_key)) {
        return KEYSTORE_ERR_SALT;
    }

    rust_hmac_sha256(salted_key, sizeof(salted_key), seed, seed_len, out);
    return KEYSTORE_OK;
}

USE_RESULT static keystore_error_t _retain_seed(const uint8_t* seed, size_t seed_len)
{
#ifdef TESTING
    const uint8_t test_unstretched_retained_seed_encryption_key[32] =
        "\xfe\x09\x76\x01\x14\x52\xa7\x22\x12\xe4\xb8\xbd\x57\x2b\x5b\xe3\x01\x41\xa3\x56\xf1\x13"
        "\x37\xd2\x9d\x35\xea\x8f\xf9\x97\xbe\xfc";
    memcpy(
        _unstretched_retained_seed_encryption_key,
        test_unstretched_retained_seed_encryption_key,
        32);
#else
    random_32_bytes(_unstretched_retained_seed_encryption_key);
#endif
    uint8_t retained_seed_encryption_key[32] = {0};
    UTIL_CLEANUP_32(retained_seed_encryption_key);
    bool stretched = rust_keystore_stretch_retained_seed_encryption_key(
        rust_util_bytes(
            _unstretched_retained_seed_encryption_key,
            sizeof(_unstretched_retained_seed_encryption_key)),
        "keystore_retained_seed_access_in",
        "keystore_retained_seed_access_out",
        rust_util_bytes_mut(retained_seed_encryption_key, sizeof(retained_seed_encryption_key)));
    if (!stretched) {
        return KEYSTORE_ERR_STRETCH_RETAINED_SEED_KEY;
    }
    size_t len = seed_len + 64;
    if (!cipher_aes_hmac_encrypt(
            seed, seed_len, _retained_seed_encrypted, &len, retained_seed_encryption_key)) {
        return KEYSTORE_ERR_ENCRYPT;
    }
    _retained_seed_encrypted_len = len;

    return _hash_seed(seed, seed_len, _retained_seed_hash);
}

USE_RESULT static bool _retain_bip39_seed(const uint8_t* bip39_seed)
{
#ifdef TESTING
    const uint8_t test_unstretched_retained_bip39_seed_encryption_key[32] =
        "\x9b\x44\xc7\x04\x88\x93\xfa\xaf\x6e\x2d\x76\x25\xd1\x3d\x8f\x1c\xab\x07\x65\xfd\x61\xf1"
        "\x59\xd9\x71\x3e\x08\x15\x5d\x06\x71\x7c";
    memcpy(
        _unstretched_retained_bip39_seed_encryption_key,
        test_unstretched_retained_bip39_seed_encryption_key,
        32);
#else
    random_32_bytes(_unstretched_retained_bip39_seed_encryption_key);
#endif
    uint8_t retained_bip39_seed_encryption_key[32] = {0};
    UTIL_CLEANUP_32(retained_bip39_seed_encryption_key);
    if (!rust_keystore_stretch_retained_seed_encryption_key(
            rust_util_bytes(
                _unstretched_retained_bip39_seed_encryption_key,
                sizeof(_unstretched_retained_bip39_seed_encryption_key)),
            "keystore_retained_bip39_seed_access_in",
            "keystore_retained_bip39_seed_access_out",
            rust_util_bytes_mut(
                retained_bip39_seed_encryption_key, sizeof(retained_bip39_seed_encryption_key)))) {
        return false;
    }
    size_t len = sizeof(_retained_bip39_seed_encrypted);
    if (!cipher_aes_hmac_encrypt(
            bip39_seed,
            64,
            _retained_bip39_seed_encrypted,
            &len,
            retained_bip39_seed_encryption_key)) {
        return false;
    }
    _retained_bip39_seed_encrypted_len = len;
    return true;
}

static void _delete_retained_seeds(void)
{
    util_zero(
        _unstretched_retained_seed_encryption_key,
        sizeof(_unstretched_retained_seed_encryption_key));
    util_zero(_retained_seed_encrypted, sizeof(_retained_seed_encrypted));
    _retained_seed_encrypted_len = 0;
    util_zero(_retained_seed_hash, sizeof(_retained_seed_hash));

    util_zero(
        _unstretched_retained_bip39_seed_encryption_key,
        sizeof(_unstretched_retained_seed_encryption_key));
    util_zero(_retained_bip39_seed_encrypted, sizeof(_retained_bip39_seed_encrypted));
    _retained_bip39_seed_encrypted_len = 0;
}

keystore_error_t keystore_encrypt_and_store_seed(
    const uint8_t* seed,
    size_t seed_length,
    const char* password)
{
    if (memory_is_initialized()) {
        return KEYSTORE_ERR_MEMORY;
    }
    rust_keystore_lock();
    if (!_validate_seed_length(seed_length)) {
        return KEYSTORE_ERR_SEED_SIZE;
    }

    usb_processing_timeout_reset(LONG_TIMEOUT);

    if (securechip_init_new_password(password)) {
        return KEYSTORE_ERR_SECURECHIP;
    }
    uint8_t secret[32] = {0};
    UTIL_CLEANUP_32(secret);
    if (securechip_stretch_password(password, secret)) {
        return KEYSTORE_ERR_SECURECHIP;
    }

    size_t encrypted_seed_len = seed_length + 64;
    uint8_t encrypted_seed[encrypted_seed_len];
    UTIL_CLEANUP_32(encrypted_seed);
    if (!cipher_aes_hmac_encrypt(seed, seed_length, encrypted_seed, &encrypted_seed_len, secret)) {
        return KEYSTORE_ERR_ENCRYPT;
    }
    if (encrypted_seed_len > 255) { // sanity check, can't happen
        Abort("keystore_encrypt_and_store_seed");
    }
    uint8_t encrypted_seed_len_u8 = (uint8_t)encrypted_seed_len;
    if (!memory_set_encrypted_seed_and_hmac(encrypted_seed, encrypted_seed_len_u8)) {
        return KEYSTORE_ERR_MEMORY;
    }
    if (!_verify_seed(secret, seed, seed_length)) {
        if (!memory_reset_hww()) {
            return KEYSTORE_ERR_MEMORY;
        }
        return KEYSTORE_ERR_MEMORY;
    }

    keystore_error_t retain_seed_result = _retain_seed(seed, seed_length);
    if (retain_seed_result != KEYSTORE_OK) {
        return retain_seed_result;
    }
    _is_unlocked_device = true;

    return KEYSTORE_OK;
}

// Checks if the retained seed matches the passed seed.
static bool _check_retained_seed(const uint8_t* seed, size_t seed_length)
{
    if (!_is_unlocked_device) {
        return false;
    }
    uint8_t seed_hashed[32] = {0};
    UTIL_CLEANUP_32(seed_hashed);
    if (_hash_seed(seed, seed_length, seed_hashed) != KEYSTORE_OK) {
        return false;
    }
    if (!MEMEQ(seed_hashed, _retained_seed_hash, sizeof(_retained_seed_hash))) {
        return false;
    }
    return true;
}

keystore_error_t keystore_unlock(
    const char* password,
    uint8_t* remaining_attempts_out,
    int* securechip_result_out,
    uint8_t* seed_out,
    size_t* seed_len_out)
{
    if (!memory_is_seeded()) {
        return KEYSTORE_ERR_UNSEEDED;
    }
    uint8_t failed_attempts = bitbox02_smarteeprom_get_unlock_attempts();
    if (failed_attempts >= MAX_UNLOCK_ATTEMPTS) {
        /*
         * We reset the device as soon as the MAX_UNLOCK_ATTEMPTSth attempt
         * is made. So we should never enter this branch...
         * This is just an extraordinary measure for added resilience.
         */
        *remaining_attempts_out = 0;
        reset_reset(false);
        return KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED;
    }
    usb_processing_timeout_reset(LONG_TIMEOUT);

    bitbox02_smarteeprom_increment_unlock_attempts();
    uint8_t seed[KEYSTORE_MAX_SEED_LENGTH] = {0};
    UTIL_CLEANUP_32(seed);
    size_t seed_len;
    keystore_error_t result =
        _get_and_decrypt_seed(password, seed, &seed_len, securechip_result_out);
    if (result != KEYSTORE_OK && result != KEYSTORE_ERR_INCORRECT_PASSWORD) {
        return result;
    }
    if (result == KEYSTORE_OK) {
        if (_is_unlocked_device) {
            // Already unlocked. Fail if the seed changed under our feet (should never happen).
            if (!_check_retained_seed(seed, seed_len)) {
                Abort("Seed has suddenly changed. This should never happen.");
            }
        } else {
            keystore_error_t retain_seed_result = _retain_seed(seed, seed_len);
            if (retain_seed_result != KEYSTORE_OK) {
                return retain_seed_result;
            }
            _is_unlocked_device = true;
        }
        bitbox02_smarteeprom_reset_unlock_attempts();

        if (seed_out != NULL && seed_len_out != NULL) {
            memcpy(seed_out, seed, seed_len);
            *seed_len_out = seed_len;
        }
    }
    // Compute remaining attempts
    failed_attempts = bitbox02_smarteeprom_get_unlock_attempts();

    if (failed_attempts >= MAX_UNLOCK_ATTEMPTS) {
        *remaining_attempts_out = 0;
        reset_reset(false);
        return KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED;
    }

    *remaining_attempts_out = MAX_UNLOCK_ATTEMPTS - failed_attempts;
    return result;
}

bool keystore_unlock_bip39_check(const uint8_t* seed, size_t seed_length)
{
    if (!_is_unlocked_device) {
        return false;
    }

    if (!_check_retained_seed(seed, seed_length)) {
        return false;
    }

    return true;
}

bool keystore_unlock_bip39_finalize(const uint8_t* bip39_seed)
{
    if (!_retain_bip39_seed(bip39_seed)) {
        return false;
    }
    _is_unlocked_bip39 = true;
    return true;
}

void keystore_lock(void)
{
    _is_unlocked_device = false;
    _is_unlocked_bip39 = false;
    _delete_retained_seeds();
}

bool keystore_is_locked(void)
{
    bool unlocked = _is_unlocked_device && _is_unlocked_bip39;
    return !unlocked;
}

bool keystore_get_bip39_word_stack(uint16_t idx, char* word_out, size_t word_out_size)
{
    return rust_get_bip39_word(idx, rust_util_bytes_mut((uint8_t*)word_out, word_out_size));
}

bool keystore_secp256k1_nonce_commit(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    secp256k1_ecdsa_s2c_opening signer_commitment;
    if (!secp256k1_ecdsa_anti_exfil_signer_commit(
            ctx, &signer_commitment, msg32, private_key, host_commitment)) {
        return false;
    }

    if (!secp256k1_ecdsa_s2c_opening_serialize(ctx, signer_commitment_out, &signer_commitment)) {
        return false;
    }
    return true;
}

bool keystore_secp256k1_sign(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    secp256k1_ecdsa_signature secp256k1_sig = {0};
    if (!secp256k1_anti_exfil_sign(
            ctx, &secp256k1_sig, msg32, private_key, host_nonce32, recid_out)) {
        return false;
    }
    if (!secp256k1_ecdsa_signature_serialize_compact(ctx, sig_compact_out, &secp256k1_sig)) {
        return false;
    }
    return true;
}

#ifdef TESTING
void keystore_mock_unlocked(const uint8_t* seed, size_t seed_len)
{
    _is_unlocked_device = seed != NULL;
    if (seed != NULL) {
        if (_retain_seed(seed, seed_len) != KEYSTORE_OK) {
            Abort("couldn't retain seed");
        }
    }
    _is_unlocked_bip39 = false;
}

const uint8_t* keystore_test_get_retained_seed_encrypted(size_t* len_out)
{
    *len_out = _retained_seed_encrypted_len;
    return _retained_seed_encrypted;
}

const uint8_t* keystore_test_get_retained_bip39_seed_encrypted(size_t* len_out)
{
    *len_out = _retained_bip39_seed_encrypted_len;
    return _retained_bip39_seed_encrypted;
}
#endif
