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
#include "reset.h"
#include "salt.h"
#include "securechip/securechip.h"
#include "util.h"
#include <usb/usb_processing.h>

#include <rust/rust.h>
#include <secp256k1_ecdsa_s2c.h>

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

USE_RESULT static keystore_error_t _retain_seed(const uint8_t* seed, size_t seed_len)
{
    if (!rust_keystore_retain_seed(rust_util_bytes(seed, seed_len))) {
        return KEYSTORE_ERR_STRETCH_RETAINED_SEED_KEY;
    }
    return KEYSTORE_OK;
}

keystore_error_t keystore_unlock(
    const char* password,
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
    if (result != KEYSTORE_OK) {
        failed_attempts = bitbox02_smarteeprom_get_unlock_attempts();

        if (failed_attempts >= MAX_UNLOCK_ATTEMPTS) {
            reset_reset(false);
            return KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED;
        }

        return result;
    }

    if (rust_keystore_is_unlocked_device()) {
        // Already unlocked. Fail if the seed changed under our feet (should never happen).
        if (!rust_keystore_check_retained_seed(rust_util_bytes(seed, seed_len))) {
            Abort("Seed has suddenly changed. This should never happen.");
        }
    } else {
        keystore_error_t retain_seed_result = _retain_seed(seed, seed_len);
        if (retain_seed_result != KEYSTORE_OK) {
            return retain_seed_result;
        }
    }
    bitbox02_smarteeprom_reset_unlock_attempts();

    if (seed_out != NULL && seed_len_out != NULL) {
        memcpy(seed_out, seed, seed_len);
        *seed_len_out = seed_len;
    }
    return KEYSTORE_OK;
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
