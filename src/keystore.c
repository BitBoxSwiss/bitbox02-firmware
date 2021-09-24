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

#include <secp256k1_ecdsa_s2c.h>

// This number of KDF iterations on the 2nd kdf slot when stretching the device
// password.
#define KDF_NUM_ITERATIONS (2)

// change this ONLY via keystore_unlock() or keystore_lock()
static bool _is_unlocked_device = false;
// seed length defaults to 32 bytes, but we could accept smaller seeds in the future.
static uint8_t _seed_length = KEYSTORE_MAX_SEED_LENGTH;
// must be defined if is_unlocked is true. ONLY ACCESS THIS WITH _get_seed()
static uint8_t _retained_seed[KEYSTORE_MAX_SEED_LENGTH] = {0};

// change this ONLY via keystore_unlock_bip39().
static bool _is_unlocked_bip39 = false;
// must be defined if _is_unlocked is true. ONLY ACCESS THIS WITH _get_bip39_seed().
static uint8_t _retained_bip39_seed[64] = {0};

#ifdef TESTING
void mock_state(const uint8_t* retained_seed, const uint8_t* retained_bip39_seed)
{
    _is_unlocked_device = retained_seed != NULL;
    if (retained_seed != NULL) {
        _seed_length = KEYSTORE_MAX_SEED_LENGTH;
        memcpy(_retained_seed, retained_seed, sizeof(_retained_seed));
    }
    _is_unlocked_bip39 = retained_bip39_seed != NULL;
    if (retained_bip39_seed != NULL) {
        memcpy(_retained_bip39_seed, retained_bip39_seed, sizeof(_retained_bip39_seed));
    }
}
#endif

/**
 * We allow seeds of 16, 24 or 32 bytes.
 */
static bool _validate_seed_length(size_t seed_len)
{
    return seed_len == 16 || seed_len == 24 || seed_len == 32;
}

static uint8_t* _get_seed(void)
{
    if (!_is_unlocked_device) {
        return NULL;
    }
    return _retained_seed;
}

bool keystore_copy_seed(uint8_t* seed_out, uint32_t* length_out)
{
    if (_get_seed() == NULL) {
        return false;
    }
    memcpy(seed_out, _get_seed(), _seed_length);
    *length_out = _seed_length;
    return true;
}

/**
 * @return the pointer ot the static bip39 seed on success. returns NULL if the
 * keystore is locked.
 */
static uint8_t* _get_bip39_seed(void)
{
    if (!_is_unlocked_bip39) {
        return NULL;
    }
    // sanity check
    uint8_t zero[64] = {0};
    util_zero(zero, 64);
    if (MEMEQ(_retained_bip39_seed, zero, sizeof(_retained_bip39_seed))) {
        return NULL;
    }
    return _retained_bip39_seed;
}

/**
 * Stretch the user password using the securechip, putting the result in `kdf_out`, which must be 32
 * bytes. `securechip_result_out`, if not NULL, will contain the error code from `securechip_kdf()`
 * if there was a secure chip error, and 0 otherwise.
 */
static keystore_error_t _stretch_password(
    const char* password,
    uint8_t* kdf_out,
    int* securechip_result_out)
{
    if (securechip_result_out != NULL) {
        *securechip_result_out = 0;
    }
    uint8_t password_salted_hashed[32] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "keystore_seed_access_in",
            password_salted_hashed)) {
        return KEYSTORE_ERR_SALT;
    }

    uint8_t kdf_in[32] = {0};
    UTIL_CLEANUP_32(kdf_in);
    memcpy(kdf_in, password_salted_hashed, 32);

    // First KDF on SECURECHIP_SLOT_ROLLKEY increments the monotonic
    // counter. Call only once!
    int securechip_result = securechip_kdf(SECURECHIP_SLOT_ROLLKEY, kdf_in, 32, kdf_out);
    if (securechip_result) {
        if (securechip_result_out != NULL) {
            *securechip_result_out = securechip_result;
        }
        return KEYSTORE_ERR_SC_KDF;
    }
    // Second KDF does not use the counter and we call it multiple times.
    for (int i = 0; i < KDF_NUM_ITERATIONS; i++) {
        memcpy(kdf_in, kdf_out, 32);
        securechip_result = securechip_kdf(SECURECHIP_SLOT_KDF, kdf_in, 32, kdf_out);
        if (securechip_result) {
            if (securechip_result_out != NULL) {
                *securechip_result_out = securechip_result;
            }
            return KEYSTORE_ERR_SC_KDF;
        }
    }

    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "keystore_seed_access_out",
            password_salted_hashed)) {
        return KEYSTORE_ERR_SALT;
    }
    if (wally_hmac_sha256(
            password_salted_hashed, sizeof(password_salted_hashed), kdf_out, 32, kdf_out, 32) !=
        WALLY_OK) {
        return KEYSTORE_ERR_HASH;
    }

    return KEYSTORE_OK;
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
    keystore_error_t result = _stretch_password(password, secret, securechip_result_out);
    if (result != KEYSTORE_OK) {
        return result;
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
    const char* password,
    const uint8_t* expected_seed,
    size_t expected_seed_len)
{
    uint8_t decrypted_seed[KEYSTORE_MAX_SEED_LENGTH] = {0};
    size_t seed_len;
    UTIL_CLEANUP_32(decrypted_seed);
    if (_get_and_decrypt_seed(password, decrypted_seed, &seed_len, NULL) != KEYSTORE_OK) {
        return false;
    }
    if (expected_seed_len != seed_len) {
        return false;
    }
    if (!MEMEQ(expected_seed, decrypted_seed, seed_len)) {
        return false;
    }
    return true;
}

bool keystore_encrypt_and_store_seed(
    const uint8_t* seed,
    uint32_t seed_length,
    const char* password)
{
    if (memory_is_initialized()) {
        return false;
    }
    keystore_lock();
    if (!_validate_seed_length(seed_length)) {
        return false;
    }
    // Update the two kdf keys before setting a new password. This already
    // happens on a device reset, but we do it here again anyway so the keys are
    // initialized also on first use, reducing trust in the factory setup.
    if (!securechip_update_keys()) {
        return false;
    }
    uint8_t secret[32] = {0};
    UTIL_CLEANUP_32(secret);
    if (_stretch_password(password, secret, NULL) != KEYSTORE_OK) {
        return false;
    }

    size_t encrypted_seed_len = seed_length + 64;
    uint8_t encrypted_seed[encrypted_seed_len];
    UTIL_CLEANUP_32(encrypted_seed);
    if (!cipher_aes_hmac_encrypt(seed, seed_length, encrypted_seed, &encrypted_seed_len, secret)) {
        return false;
    }
    if (encrypted_seed_len > 255) { // sanity check, can't happen
        Abort("keystore_encrypt_and_store_seed");
    }
    if (!memory_set_encrypted_seed_and_hmac(encrypted_seed, encrypted_seed_len)) {
        return false;
    }
    if (!_verify_seed(password, seed, seed_length)) {
        if (!memory_reset_hww()) {
            return false;
        }
        return false;
    }
    return true;
}

bool keystore_create_and_store_seed(
    const char* password,
    const uint8_t* host_entropy,
    size_t host_entropy_size)
{
    if (host_entropy_size != 16 && host_entropy_size != 32) {
        return false;
    }
    if (KEYSTORE_MAX_SEED_LENGTH != RANDOM_NUM_SIZE) {
        Abort("keystore create: size mismatch");
    }
    uint8_t seed[KEYSTORE_MAX_SEED_LENGTH];
    UTIL_CLEANUP_32(seed);
    random_32_bytes(seed);

    // Mix in Host entropy.
    for (size_t i = 0; i < host_entropy_size; i++) {
        seed[i] ^= host_entropy[i];
    }

    // Mix in entropy derived from the user password.
    uint8_t password_salted_hashed[KEYSTORE_MAX_SEED_LENGTH] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "keystore_seed_generation",
            password_salted_hashed)) {
        return false;
    }

    for (size_t i = 0; i < host_entropy_size; i++) {
        seed[i] ^= password_salted_hashed[i];
    }
    return keystore_encrypt_and_store_seed(seed, host_entropy_size, password);
}

static void _free_string(char** str)
{
    wally_free_string(*str);
}

keystore_error_t keystore_unlock(
    const char* password,
    uint8_t* remaining_attempts_out,
    int* securechip_result_out)
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
            if (seed_len != _seed_length || !MEMEQ(_retained_seed, seed, _seed_length)) {
                Abort("Seed has suddenly changed. This should never happen.");
            }
        } else {
            memcpy(_retained_seed, seed, seed_len);
            _seed_length = seed_len;
            _is_unlocked_device = true;
        }
        bitbox02_smarteeprom_reset_unlock_attempts();
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

bool keystore_unlock_bip39(const char* mnemonic_passphrase)
{
    if (!_is_unlocked_device) {
        return false;
    }
    uint8_t* seed = _get_seed();
    if (seed == NULL) {
        return false;
    }
    char* mnemonic __attribute__((__cleanup__(_free_string))) = NULL;
    if (bip39_mnemonic_from_bytes(NULL, seed, _seed_length, &mnemonic) != WALLY_OK) {
        return false;
    }
    uint8_t bip39_seed[BIP39_SEED_LEN_512] = {0};
    UTIL_CLEANUP_64(bip39_seed);
    if (bip39_mnemonic_to_seed(
            mnemonic, mnemonic_passphrase, bip39_seed, sizeof(bip39_seed), NULL) != WALLY_OK) {
        return false;
    }
    memcpy(_retained_bip39_seed, bip39_seed, sizeof(bip39_seed));
    _is_unlocked_bip39 = true;
    return true;
}

void keystore_lock(void)
{
    _is_unlocked_device = false;
    _is_unlocked_bip39 = false;
    _seed_length = KEYSTORE_MAX_SEED_LENGTH;
    util_zero(_retained_seed, sizeof(_retained_seed));
    util_zero(_retained_bip39_seed, sizeof(_retained_bip39_seed));
}

bool keystore_is_locked(void)
{
    bool unlocked = _is_unlocked_device && _is_unlocked_bip39;
    return !unlocked;
}

bool keystore_get_bip39_mnemonic(char* mnemonic_out, size_t mnemonic_out_size)
{
    if (keystore_is_locked()) {
        return false;
    }
    uint8_t* seed = _get_seed();
    if (seed == NULL) {
        return false;
    }
    char* mnemonic = NULL;
    if (bip39_mnemonic_from_bytes(NULL, seed, _seed_length, &mnemonic) != WALLY_OK) {
        return false;
    }
    int snprintf_result = snprintf(mnemonic_out, mnemonic_out_size, "%s", mnemonic);
    util_cleanup_str(&mnemonic);
    free(mnemonic);
    return snprintf_result >= 0 && snprintf_result < (int)mnemonic_out_size;
}

bool keystore_bip39_mnemonic_to_seed(const char* mnemonic, uint8_t* seed_out, size_t* seed_len_out)
{
    return bip39_mnemonic_to_bytes(NULL, mnemonic, seed_out, 32, seed_len_out) == WALLY_OK;
}

static bool _get_xprv(const uint32_t* keypath, const size_t keypath_len, struct ext_key* xprv_out)
{
    if (keystore_is_locked()) {
        return false;
    }
    uint8_t* bip39_seed = _get_bip39_seed();
    if (bip39_seed == NULL) {
        return false;
    }
    struct ext_key xprv_master __attribute__((__cleanup__(keystore_zero_xkey))) = {0};

    if (bip32_key_from_seed(
            bip39_seed, BIP32_ENTROPY_LEN_512, BIP32_VER_MAIN_PRIVATE, 0, &xprv_master) !=
        WALLY_OK) {
        return false;
    }
    if (keypath_len == 0) {
        *xprv_out = xprv_master;
    } else if (
        bip32_key_from_parent_path(
            &xprv_master, keypath, keypath_len, BIP32_FLAG_KEY_PRIVATE, xprv_out) != WALLY_OK) {
        keystore_zero_xkey(xprv_out);
        return false;
    }
    return true;
}

bool keystore_get_root_fingerprint(uint8_t* fingerprint)
{
    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(NULL, 0, &derived_xpub)) {
        return false;
    }
    if (bip32_key_get_fingerprint(&derived_xpub, fingerprint, 4) != WALLY_OK) {
        return false;
    }
    return true;
}

static bool _ext_key_equal(struct ext_key* one, struct ext_key* two)
{
    if (!MEMEQ(one->chain_code, two->chain_code, sizeof(one->chain_code))) {
        return false;
    }
    if (!MEMEQ(one->parent160, two->parent160, sizeof(one->parent160))) {
        return false;
    }
    if (one->depth != two->depth) {
        return false;
    }
    if (!MEMEQ(one->priv_key, two->priv_key, sizeof(one->priv_key))) {
        return false;
    }
    if (one->child_num != two->child_num) {
        return false;
    }
    if (!MEMEQ(one->hash160, two->hash160, sizeof(one->hash160))) {
        return false;
    }
    if (one->version != two->version) {
        return false;
    }
    if (!MEMEQ(one->pub_key, two->pub_key, sizeof(one->pub_key))) {
        return false;
    }
    return true;
}

static bool _get_xprv_twice(
    const uint32_t* keypath,
    const size_t keypath_len,
    struct ext_key* xprv_out)
{
    struct ext_key one __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv(keypath, keypath_len, &one)) {
        return false;
    }
    if (!_get_xprv(keypath, keypath_len, xprv_out)) {
        return false;
    }
    if (!_ext_key_equal(&one, xprv_out)) {
        keystore_zero_xkey(xprv_out);
        return false;
    }
    return true;
}

bool keystore_get_xpub(
    const uint32_t* keypath,
    const size_t keypath_len,
    struct ext_key* hdkey_neutered_out)
{
    struct ext_key xprv __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv_twice(keypath, keypath_len, &xprv)) {
        return false;
    }
    bip32_key_strip_private_key(&xprv); // neuter
    *hdkey_neutered_out = xprv;
    return true;
}

void keystore_zero_xkey(struct ext_key* xkey)
{
    util_zero(xkey, sizeof(struct ext_key));
}

bool keystore_get_bip39_word(uint16_t idx, char** word_out)
{
    return bip39_get_word(NULL, idx, word_out) == WALLY_OK;
}

// Reformats xpub from compressed 33 bytes to uncompressed 65 bytes (<0x04><64 bytes X><64 bytes
// Y>),
// pubkey must be 33 bytes
// uncompressed_out must be 65 bytes.
static bool _compressed_to_uncompressed(const uint8_t* pubkey_bytes, uint8_t* uncompressed_out)
{
    secp256k1_context* ctx = wally_get_secp_context();
    secp256k1_pubkey pubkey;
    if (!secp256k1_ec_pubkey_parse(ctx, &pubkey, pubkey_bytes, 33)) {
        return false;
    }
    size_t len = 65;
    if (!secp256k1_ec_pubkey_serialize(
            ctx, uncompressed_out, &len, &pubkey, SECP256K1_EC_UNCOMPRESSED)) {
        return false;
    }
    return true;
}

bool keystore_secp256k1_pubkey_hash160(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash160_out)
{
    if (keystore_is_locked()) {
        return false;
    }
    struct ext_key xprv __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv_twice(keypath, keypath_len, &xprv)) {
        return false;
    }
    memcpy(hash160_out, xprv.hash160, sizeof(xprv.hash160));
    return true;
}

bool keystore_secp256k1_pubkey_uncompressed(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* pubkey_out)
{
    if (keystore_is_locked()) {
        return false;
    }
    struct ext_key xprv __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv_twice(keypath, keypath_len, &xprv)) {
        return false;
    }
    return _compressed_to_uncompressed(xprv.pub_key, pubkey_out);
}

bool keystore_secp256k1_nonce_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    struct ext_key xprv __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv_twice(keypath, keypath_len, &xprv)) {
        return false;
    }
    secp256k1_context* ctx = wally_get_secp_context();
    secp256k1_ecdsa_s2c_opening signer_commitment;
    if (!secp256k1_ecdsa_anti_klepto_signer_commit(
            ctx,
            &signer_commitment,
            msg32,
            xprv.priv_key + 1, // first byte is 0,
            host_commitment)) {
        return false;
    }

    if (!secp256k1_ecdsa_s2c_opening_serialize(ctx, signer_commitment_out, &signer_commitment)) {
        return false;
    }
    return true;
}

bool keystore_secp256k1_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    if (keystore_is_locked()) {
        return false;
    }
    struct ext_key xprv __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!_get_xprv_twice(keypath, keypath_len, &xprv)) {
        return false;
    }
    secp256k1_context* ctx = wally_get_secp_context();
    secp256k1_ecdsa_signature secp256k1_sig = {0};
    if (!secp256k1_anti_klepto_sign(
            ctx,
            &secp256k1_sig,
            msg32,
            xprv.priv_key + 1, // first byte is 0
            host_nonce32,
            recid_out)) {
        return false;
    }
    if (!secp256k1_ecdsa_signature_serialize_compact(ctx, sig_compact_out, &secp256k1_sig)) {
        return false;
    }
    return true;
}

bool keystore_get_u2f_seed(uint8_t* seed_out)
{
    if (keystore_is_locked()) {
        return false;
    }
    uint8_t* bip39_seed = _get_bip39_seed();
    if (bip39_seed == NULL) {
        return false;
    }
    const uint8_t message[] = "u2f";
    if (wally_hmac_sha256(bip39_seed, 64, message, sizeof(message), seed_out, SHA256_LEN) !=
        WALLY_OK) {
        return false;
    }
    return true;
}

static const uint8_t _xpub_version[4] = {0x04, 0x88, 0xb2, 0x1e};
static const uint8_t _ypub_version[4] = {0x04, 0x9d, 0x7c, 0xb2};
static const uint8_t _zpub_version[4] = {0x04, 0xb2, 0x47, 0x46};
static const uint8_t _tpub_version[4] = {0x04, 0x35, 0x87, 0xcf};
static const uint8_t _vpub_version[4] = {0x04, 0x5f, 0x1c, 0xf6};
static const uint8_t _upub_version[4] = {0x04, 0x4a, 0x52, 0x62};
static const uint8_t _capital_vpub_version[4] = {0x02, 0x57, 0x54, 0x83};
static const uint8_t _capital_zpub_version[4] = {0x02, 0xaa, 0x7e, 0xd3};
static const uint8_t _capital_upub_version[4] = {0x02, 0x42, 0x89, 0xef};
static const uint8_t _capital_ypub_version[4] = {0x02, 0x95, 0xb4, 0x3f};

bool keystore_encode_xpub(
    const struct ext_key* xpub,
    xpub_type_t xpub_type,
    char* out,
    size_t out_len)
{
    char* xpub_string = NULL;
    uint8_t bytes[BIP32_SERIALIZED_LEN] = {0};
    if (bip32_key_serialize(xpub, BIP32_FLAG_KEY_PUBLIC, bytes, sizeof(bytes)) != WALLY_OK) {
        return false;
    }

    const uint8_t* version;
    switch (xpub_type) {
    case XPUB:
        version = _xpub_version;
        break;
    case YPUB:
        version = _ypub_version;
        break;
    case ZPUB:
        version = _zpub_version;
        break;
    case TPUB:
        version = _tpub_version;
        break;
    case VPUB:
        version = _vpub_version;
        break;
    case UPUB:
        version = _upub_version;
        break;
    case CAPITAL_VPUB:
        version = _capital_vpub_version;
        break;
    case CAPITAL_ZPUB:
        version = _capital_zpub_version;
        break;
    case CAPITAL_UPUB:
        version = _capital_upub_version;
        break;
    case CAPITAL_YPUB:
        version = _capital_ypub_version;
        break;
    default:
        return false;
    }

    // Overwrite bip32 version (libwally doesn't give the option to provide a
    // different one)
    memcpy(bytes, version, 4);
    int ret =
        wally_base58_from_bytes(bytes, BIP32_SERIALIZED_LEN, BASE58_FLAG_CHECKSUM, &xpub_string);
    util_zero(bytes, sizeof(bytes));
    if (ret != WALLY_OK) {
        return false;
    }
    int sprintf_result = snprintf(out, out_len, "%s", xpub_string);
    wally_free_string(xpub_string);
    return sprintf_result >= 0 && sprintf_result < (int)out_len;
}

USE_RESULT bool keystore_encode_xpub_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    xpub_type_t xpub_type,
    char* out,
    size_t out_len)
{
    struct ext_key derived_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &derived_xpub)) {
        return false;
    }
    return keystore_encode_xpub(&derived_xpub, xpub_type, out, out_len);
}
