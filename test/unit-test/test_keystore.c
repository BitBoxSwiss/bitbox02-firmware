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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <cipher/cipher.h>
#include <keystore.h>
#include <memory/bitbox02_smarteeprom.h>
#include <memory/memory.h>
#include <memory/smarteeprom.h>
#include <secp256k1_ecdsa_s2c.h>
#include <secp256k1_recovery.h>
#include <secp256k1_schnorrsig.h>
#include <securechip/securechip.h>
#include <util.h>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define PASSWORD ("password")

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static uint8_t _mock_seed_2[32] = {
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
};

static uint8_t _mock_bip39_seed[64] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

const uint8_t _aes_iv[32] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

static const uint32_t _keypath[] = {
    44 + BIP32_INITIAL_HARDENED_CHILD,
    0 + BIP32_INITIAL_HARDENED_CHILD,
    0 + BIP32_INITIAL_HARDENED_CHILD,
    0,
    5,
};
// seckey at the above keypath with the above bip39 seed.
static const uint8_t _expected_seckey[32] = {
    0x4e, 0x64, 0xdf, 0xd3, 0x3a, 0xae, 0x66, 0xc4, 0xc7, 0x52, 0x6c, 0xf0, 0x2e, 0xe8, 0xae, 0x3f,
    0x58, 0x92, 0x32, 0x9d, 0x67, 0xdf, 0xd4, 0xad, 0x05, 0xe9, 0xc3, 0xd0, 0x6e, 0xdf, 0x74, 0xfb,
};

static uint8_t _password_salted_hashed_stretch_in[32] = {
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
};

static uint8_t _password_salted_hashed_stretch_out[32] = {
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
};

static uint8_t _password_salted_hashed_stretch_out_invalid[32] = {
    0x72, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
};

static uint8_t _kdf_out_1[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
};

static uint8_t _kdf_out_2[32] = {
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
};

static uint8_t _kdf_out_3[32] = {
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
};

// Fixture: hmac.new(_password_salted_hashed_stretch_out, _kdf_out_3,
// hashlib.sha256).hexdigest()
static uint8_t _expected_secret[32] = {
    0x39, 0xa7, 0x4f, 0x75, 0xb6, 0x9d, 0x6c, 0x84, 0x5e, 0x18, 0x91, 0x5b, 0xae, 0x29, 0xd1, 0x06,
    0x12, 0x12, 0x40, 0x37, 0x7a, 0x79, 0x97, 0x55, 0xd7, 0xcc, 0xe9, 0x26, 0x1e, 0x16, 0x91, 0x71,
};

int __real_secp256k1_anti_exfil_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    const unsigned char* msg32,
    const unsigned char* seckey,
    const unsigned char* host_data32,
    int* recid);

static const unsigned char* _sign_expected_msg = NULL;
static const unsigned char* _sign_expected_seckey = NULL;
int __wrap_secp256k1_anti_exfil_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    const unsigned char* msg32,
    const unsigned char* seckey,
    const unsigned char* host_data32,
    int* recid)
{
    if (_sign_expected_msg != NULL) {
        assert_memory_equal(_sign_expected_msg, msg32, 32);
        _sign_expected_msg = NULL;
    }
    if (_sign_expected_seckey != NULL) {
        assert_memory_equal(_sign_expected_seckey, seckey, 32);
        _sign_expected_seckey = NULL;
    }
    return __real_secp256k1_anti_exfil_sign(ctx, sig, msg32, seckey, host_data32, recid);
}

bool __wrap_salt_hash_data(
    const uint8_t* data,
    size_t data_len,
    const char* purpose,
    uint8_t* hash_out)
{
    check_expected(data);
    check_expected(data_len);
    check_expected(purpose);
    memcpy(hash_out, (const void*)mock(), 32);
    return true;
}

bool __real_cipher_aes_hmac_encrypt(
    const unsigned char* in,
    int in_len,
    uint8_t* out,
    int* out_len,
    const uint8_t* secret);

bool __wrap_cipher_aes_hmac_encrypt(
    const unsigned char* in,
    int in_len,
    uint8_t* out,
    int* out_len,
    const uint8_t* secret)
{
    check_expected(secret);
    return __real_cipher_aes_hmac_encrypt(in, in_len, out, out_len, secret);
}

/** Reset the SmartEEPROM configuration. */
static void _smarteeprom_reset(void)
{
    if (smarteeprom_is_enabled()) {
        smarteeprom_disable();
    }
    smarteeprom_bb02_config();
    bitbox02_smarteeprom_init();
}

static bool _reset_reset_called = false;
void __wrap_reset_reset(void)
{
    _reset_reset_called = true;
}

void __wrap_random_32_bytes(uint8_t* buf)
{
    memcpy(buf, (const void*)mock(), 32);
}

static bool _pubkeys_equal(
    const secp256k1_context* ctx,
    const secp256k1_pubkey* pubkey1,
    const secp256k1_pubkey* pubkey2)
{
    uint8_t pubkey1_bytes[33];
    uint8_t pubkey2_bytes[33];
    size_t len = 33;
    assert_true(
        secp256k1_ec_pubkey_serialize(ctx, pubkey1_bytes, &len, pubkey1, SECP256K1_EC_COMPRESSED));
    assert_true(
        secp256k1_ec_pubkey_serialize(ctx, pubkey2_bytes, &len, pubkey2, SECP256K1_EC_COMPRESSED));
    return memcmp(pubkey1_bytes, pubkey2_bytes, len) == 0;
}

static void _test_keystore_get_xpub(void** state)
{
    const secp256k1_context* ctx = wally_get_secp_context();

    struct ext_key xpub = {0};

    keystore_mock_unlocked(NULL, 0, NULL);
    // fails because keystore is locked
    assert_false(keystore_get_xpub(_keypath, sizeof(_keypath) / sizeof(uint32_t), &xpub));

    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);
    assert_true(keystore_get_xpub(_keypath, sizeof(_keypath) / sizeof(uint32_t), &xpub));

    secp256k1_pubkey expected_pubkey;
    assert_true(secp256k1_ec_pubkey_create(ctx, &expected_pubkey, _expected_seckey));

    secp256k1_pubkey pubkey;
    assert_true(secp256k1_ec_pubkey_parse(ctx, &pubkey, xpub.pub_key, sizeof(xpub.pub_key)));

    assert_true(_pubkeys_equal(ctx, &pubkey, &expected_pubkey));

    char* xpub_string;
    // Make sure it's a public key, no
    assert_false(bip32_key_to_base58(&xpub, BIP32_FLAG_KEY_PRIVATE, &xpub_string) == WALLY_OK);
    assert_true(bip32_key_to_base58(&xpub, BIP32_FLAG_KEY_PUBLIC, &xpub_string) == WALLY_OK);
    assert_string_equal(
        xpub_string,
        "xpub6Gmp9vKrJrVbU5JDcPRm6UmJPjTBurWfqow6w3BoK46E6mVyScMfTXd66WFeLfRa7Ug4iGMWDpWLpZAYcuUHyz"
        "cWZCqh8393rbuMoerRK1p");
    wally_free_string(xpub_string);
}

static void _test_keystore_get_root_fingerprint(void** state)
{
    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);
    uint8_t fingerprint[4];
    assert_true(keystore_get_root_fingerprint(fingerprint));
    uint8_t expected_fingerprint[4] = {0x9e, 0x1b, 0x2d, 0x1e};
    assert_memory_equal(fingerprint, expected_fingerprint, 4);
}

static void _test_keystore_secp256k1_nonce_commit(void** state)
{
    uint8_t msg[32] = {0};
    memset(msg, 0x88, sizeof(msg));
    uint8_t commitment[EC_PUBLIC_KEY_LEN] = {0};
    uint8_t host_nonce_commitment[32] = {0};
    memset(host_nonce_commitment, 0xAB, sizeof(host_nonce_commitment));

    {
        keystore_mock_unlocked(NULL, 0, NULL);
        // fails because keystore is locked
        assert_false(keystore_secp256k1_nonce_commit(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce_commitment, commitment));
    }
    {
        keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);
        assert_true(keystore_secp256k1_nonce_commit(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce_commitment, commitment));
        const uint8_t expected_commitment[EC_PUBLIC_KEY_LEN] =
            "\x02\xfd\xcf\x79\xf9\xc0\x3f\x6a\xcc\xc6\x56\x95\xa1\x90\x82\xe3\x0b\xfb\x9e\xdc\x93"
            "\x04\x5a\x03\x05\x8a\x99\x09\xe4\x9b\x1a\x37\x7b";
        assert_memory_equal(expected_commitment, commitment, sizeof(commitment));
    }
}

static void _test_keystore_secp256k1_sign(void** state)
{
    const secp256k1_context* ctx = wally_get_secp_context();

    secp256k1_pubkey expected_pubkey;
    assert_true(secp256k1_ec_pubkey_create(ctx, &expected_pubkey, _expected_seckey));

    uint8_t msg[32] = {0};
    memset(msg, 0x88, sizeof(msg));
    uint8_t sig[64] = {0};

    uint8_t host_nonce[32] = {0};
    memset(host_nonce, 0x56, sizeof(host_nonce));

    {
        keystore_mock_unlocked(NULL, 0, NULL);
        // fails because keystore is locked
        assert_false(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, NULL));
    }
    {
        keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);

        _sign_expected_seckey = _expected_seckey;
        _sign_expected_msg = msg;
        // check sig by verifying it against the msg.
        assert_true(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, NULL));
        secp256k1_ecdsa_signature secp256k1_sig = {0};
        assert_true(secp256k1_ecdsa_signature_parse_compact(ctx, &secp256k1_sig, sig));
        assert_true(secp256k1_ecdsa_verify(ctx, &secp256k1_sig, msg, &expected_pubkey));
    }
    { // test recoverable id (recid)
        int recid;
        assert_true(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, &recid));
        assert_int_equal(recid, 1);

        // Test recid by recovering the public key from the signature and checking against the
        // expected puklic key.
        secp256k1_ecdsa_recoverable_signature recoverable_sig;
        assert_true(
            secp256k1_ecdsa_recoverable_signature_parse_compact(ctx, &recoverable_sig, sig, recid));

        secp256k1_pubkey recovered_pubkey;
        assert_true(secp256k1_ecdsa_recover(ctx, &recovered_pubkey, &recoverable_sig, msg));

        assert_true(_pubkeys_equal(ctx, &recovered_pubkey, &expected_pubkey));
    }
}

static void _expect_stretch(bool valid)
{
    expect_memory(__wrap_salt_hash_data, data, PASSWORD, strlen(PASSWORD));
    expect_value(__wrap_salt_hash_data, data_len, strlen(PASSWORD));
    expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_access_in");
    will_return(__wrap_salt_hash_data, _password_salted_hashed_stretch_in);

    // KDF 1
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_ROLLKEY);
    expect_memory(securechip_kdf, msg, _password_salted_hashed_stretch_in, 32);
    will_return(securechip_kdf, _kdf_out_1);

    // KDF 2
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_KDF);
    expect_memory(securechip_kdf, msg, _kdf_out_1, 32);
    will_return(securechip_kdf, _kdf_out_2);

    // KDF 3
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_KDF);
    expect_memory(securechip_kdf, msg, _kdf_out_2, 32);
    will_return(securechip_kdf, _kdf_out_3);

    expect_memory(__wrap_salt_hash_data, data, PASSWORD, strlen(PASSWORD));
    expect_value(__wrap_salt_hash_data, data_len, strlen(PASSWORD));
    expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_access_out");
    will_return(
        __wrap_salt_hash_data,
        valid ? _password_salted_hashed_stretch_out : _password_salted_hashed_stretch_out_invalid);
}

static void _expect_encrypt_and_store_seed(void)
{
    will_return(__wrap_memory_is_initialized, false);

    _expect_stretch(true); // first stretch to encrypt
    _expect_stretch(true); // second stretch to verify

    expect_memory(__wrap_cipher_aes_hmac_encrypt, secret, _expected_secret, 32);
    // For the AES IV:
    will_return(__wrap_random_32_bytes, _aes_iv);
}

static void _test_keystore_encrypt_and_store_seed(void** state)
{
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, PASSWORD));
}

// this tests that you can create a keystore, unlock it, and then do this again. This is an expected
// workflow for when the wallet setup process is restarted after seeding and unlocking, but before
// creating a backup, in which case a new seed is created.
static void _test_keystore_create_and_unlock_twice(void** state)
{
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, PASSWORD));

    uint8_t remaining_attempts;
    _smarteeprom_reset();

    will_return(__wrap_memory_is_seeded, true);
    _expect_stretch(true);
    assert_int_equal(KEYSTORE_OK, keystore_unlock(PASSWORD, &remaining_attempts, NULL));

    // Create new (different) seed.
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed_2, 32, PASSWORD));

    will_return(__wrap_memory_is_seeded, true);
    _expect_stretch(true);
    assert_int_equal(KEYSTORE_OK, keystore_unlock(PASSWORD, &remaining_attempts, NULL));
}

static void _expect_seeded(bool seeded)
{
    uint8_t seed[KEYSTORE_MAX_SEED_LENGTH];
    uint32_t len;
    assert_int_equal(seeded, keystore_copy_seed(seed, &len));
}

static void _perform_some_unlocks(void)
{
    uint8_t remaining_attempts;
    // Loop to check that unlocking unlocked works while unlocked.
    for (int i = 0; i < 3; i++) {
        _reset_reset_called = false;
        will_return(__wrap_memory_is_seeded, true);
        _expect_stretch(true);
        assert_int_equal(KEYSTORE_OK, keystore_unlock(PASSWORD, &remaining_attempts, NULL));
        assert_int_equal(remaining_attempts, MAX_UNLOCK_ATTEMPTS);
        assert_false(_reset_reset_called);
        _expect_seeded(true);
    }
}

static void _test_keystore_unlock(void** state)
{
    _smarteeprom_reset();
    keystore_mock_unlocked(NULL, 0, NULL); // reset to locked

    uint8_t remaining_attempts;

    will_return(__wrap_memory_is_seeded, false);
    assert_int_equal(KEYSTORE_ERR_UNSEEDED, keystore_unlock(PASSWORD, &remaining_attempts, NULL));
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, PASSWORD));
    _expect_seeded(false);

    _perform_some_unlocks();

    // Invalid passwords until we run out of attempts.
    for (int i = 1; i <= MAX_UNLOCK_ATTEMPTS; i++) {
        _reset_reset_called = false;
        will_return(__wrap_memory_is_seeded, true);
        _expect_stretch(false);
        assert_int_equal(
            i >= MAX_UNLOCK_ATTEMPTS ? KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED
                                     : KEYSTORE_ERR_INCORRECT_PASSWORD,
            keystore_unlock(PASSWORD, &remaining_attempts, NULL));
        assert_int_equal(remaining_attempts, MAX_UNLOCK_ATTEMPTS - i);
        // Wrong password does not lock the keystore again if already unlocked.
        _expect_seeded(true);
        // reset_reset() called in last attempt
        assert_int_equal(i == MAX_UNLOCK_ATTEMPTS, _reset_reset_called);
    }

    // Trying again after max attempts is blocked immediately.
    _reset_reset_called = false;
    will_return(__wrap_memory_is_seeded, true);
    assert_int_equal(
        KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED, keystore_unlock(PASSWORD, &remaining_attempts, NULL));
    assert_int_equal(remaining_attempts, 0);
    assert_true(_reset_reset_called);
}

static void _test_keystore_lock(void** state)
{
    keystore_mock_unlocked(NULL, 0, NULL);
    assert_true(keystore_is_locked());
    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), NULL);
    assert_true(keystore_is_locked());
    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);
    assert_false(keystore_is_locked());
    keystore_lock();
    assert_true(keystore_is_locked());
}

static void _test_keystore_get_bip39_mnemonic(void** state)
{
    char mnemonic[300];
    keystore_mock_unlocked(NULL, 0, NULL);
    assert_false(keystore_get_bip39_mnemonic(mnemonic, sizeof(mnemonic)));

    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), NULL);
    assert_false(keystore_get_bip39_mnemonic(mnemonic, sizeof(mnemonic)));

    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);
    assert_true(keystore_get_bip39_mnemonic(mnemonic, sizeof(mnemonic)));
    const char* expected_mnemonic =
        "baby mass dust captain baby mass mass dust captain baby mass dutch creek office smoke "
        "grid creek olive baby mass dust captain baby length";
    assert_string_equal(mnemonic, expected_mnemonic);

    // Output buffer too short.
    assert_false(keystore_get_bip39_mnemonic(mnemonic, strlen(expected_mnemonic)));
    // Just enough space to fit.
    assert_true(keystore_get_bip39_mnemonic(mnemonic, strlen(expected_mnemonic) + 1));
}

static void _test_keystore_encode_xpub(void** state)
{
    struct ext_key xpub = {0};
    assert_int_equal(
        bip32_key_from_seed(
            (const unsigned char*)"seedseedseedseed",
            BIP32_ENTROPY_LEN_128,
            BIP32_VER_MAIN_PRIVATE,
            BIP32_FLAG_SKIP_HASH,
            &xpub),
        WALLY_OK);
    assert_int_equal(bip32_key_strip_private_key(&xpub), WALLY_OK);
    char out[XPUB_ENCODED_LEN] = {0};
    assert_false(keystore_encode_xpub(&xpub, XPUB, out, 110));

    assert_true(keystore_encode_xpub(&xpub, TPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "tpubD6NzVbkrYhZ4X8SrpdvxUfkKsPg5iSLHQqmQ2e7MGczVsJskvL4uD62ckffe8zi4BVtbZXRCsVDythz1eNN3fN"
        "S2A14YakBkWLBbyJiVFQ9");
    assert_true(keystore_encode_xpub(&xpub, XPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "xpub661MyMwAqRbcFLG1NSwsGkQxYGaRj3qDsDB6g64CviEc82D3r7Dp4eMnWdarcVkpPbMgwwuLLPPwCXVQFWWomv"
        "yj6QKEuDXWvNbCDF98tgM");
    assert_true(keystore_encode_xpub(&xpub, YPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "ypub6QqdH2c5z7966dT8CojVUqWTiEisffpinKhKTUx6JicVB82H6mPNgi1vXqYScQQjoEUVhRVto3kV5p6xyCvpaA"
        "fKxk1fV8M1C6eqbq4wvip");
    assert_true(keystore_encode_xpub(&xpub, ZPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "zpub6jftahH18ngZwveF3AX7gvbxtCsKcHpDhSDYEsqygizNEDqWMRYwJmg4Z3W2cK4fCsbJSu6TFi72y6iXguLqNQ"
        "Lvq5i653AVTpiUzQXvTSr");
    assert_true(keystore_encode_xpub(&xpub, VPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "vpub5SLqN2bLY4WeYjsmhjNcraDxCLHXqorE2z8f7JGSAhUr1pabLntgpX3WUDfgcgSyaK85SziDR4gqRxGGp7gnBT"
        "cXMivPjPtYNvTuS6sWM3J");
    assert_true(keystore_encode_xpub(&xpub, UPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "upub57Wa4MvRPNyAhSgesNazeV8T2N95uBrj7scSKuNYnh6xximN68j8CTPNT1i6cmo4Ag1GhX7exQLHYfei6RGmPD"
        "vvVPDy9V547CQG3b2p2sZ");
    assert_true(keystore_encode_xpub(&xpub, CAPITAL_VPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "Vpub5dEvVGKn7251yK39ePqbgeZkv8Ko4AXpMFnL2ZXyYUKFe19W7CGxuduSGvdAB7fsonC4KaiLJH5LZ7t37LqjKw"
        "jCCC2o8oMYGejn221hXB7");
    assert_true(keystore_encode_xpub(&xpub, CAPITAL_ZPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "Zpub6vZyhw1ShkEwNVocypz6WzwmbzuapeVp1hsDA97X4VpmrQQR7pwDPtXzMkTWAkHZSLfHKV6a8vVY6GLHz8VnWt"
        "TbfYpVUSdVMYzMaJxms8u");
    assert_true(keystore_encode_xpub(&xpub, CAPITAL_UPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "Upub5JQfBberxLXY81r2p33yUZUFkABM7YYKS9G7FAe6ATwNauLGrY7QHaFJFifaBD1xQ95Fa77mqcinfqGUPeRiXi"
        "3bKrLNYtY3zvg8dWPdbfj");
    assert_true(keystore_encode_xpub(&xpub, CAPITAL_YPUB, out, sizeof(out)));
    assert_string_equal(
        out,
        "Ypub6bjiQGLXZ4hTXCcW9UCUJurGS2m8t2WK6bLzNkDdgVStoJbBsAmempsrLYVvAqde2hYUa1W1gG8zCyijGS5mie"
        "mzoD84tXp15pviBjgS4df");
}

static void _test_keystore_create_and_store_seed(void** state)
{
    const uint8_t seed_random[32] =
        "\x98\xef\xa1\xb6\x0a\x83\x39\x16\x61\xa2\x4d\xc7\x4a\x80\x4f\x34\x36\xe8\x33\xe0\xaa\xbe"
        "\x75\xe9\x71\x1e\x5d\xef\x3a\x8f\x9f\x7c";
    const uint8_t host_entropy[32] =
        "\x25\x56\x9b\x9a\x11\xf9\xdb\x65\x60\x45\x9e\x8e\x48\xb4\x72\x7a\x4c\x93\x53\x00\x14\x3d"
        "\x97\x89\x89\xed\x55\xdb\x1d\x1b\x9c\xbe";
    const uint8_t password_salted_hashed[32] =
        "\xad\xee\x84\x29\xf5\xb6\x70\xa9\xd7\x34\x17\x1b\x70\x87\xf3\x8f\x86\x6a\x7e\x26\x5f\x9d"
        "\x7d\x06\xf0\x0e\x6f\xa4\x17\x54\xac\x77";
    // expected_seed = seed_random ^ host_entropy ^ password_salted_hashed
    const uint8_t expected_seed[32] =
        "\x10\x57\xbe\x05\xee\xcc\x92\xda\xd6\xd3\xc4\x52\x72\xb3\xce\xc1\xfc\x11\x1e\xc6\xe1\x1e"
        "\x9f\x66\x08\xfd\x67\x90\x30\xc0\xaf\xb5";

    // Invalid seed lengths.
    assert_false(keystore_create_and_store_seed(PASSWORD, host_entropy, 8));
    assert_false(keystore_create_and_store_seed(PASSWORD, host_entropy, 24));
    assert_false(keystore_create_and_store_seed(PASSWORD, host_entropy, 40));

    size_t test_sizes[2] = {16, 32};
    for (size_t i = 0; i < sizeof(test_sizes) / sizeof(test_sizes[0]); i++) {
        size_t seed_len = test_sizes[i];
        // Seed random is xored with host entropy and the salted/hashed user password.
        will_return(__wrap_random_32_bytes, seed_random);
        expect_memory(__wrap_salt_hash_data, data, PASSWORD, strlen(PASSWORD));
        expect_value(__wrap_salt_hash_data, data_len, strlen(PASSWORD));
        expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_generation");
        will_return(__wrap_salt_hash_data, password_salted_hashed);
        _expect_encrypt_and_store_seed();
        assert_true(keystore_create_and_store_seed(PASSWORD, host_entropy, seed_len));

        // Decrypt and check seed.
        uint8_t encrypted_seed_and_hmac[96] = {0};
        uint8_t len = 0;
        assert_true(memory_get_encrypted_seed_and_hmac(encrypted_seed_and_hmac, &len));
        size_t decrypted_len = len - 48;
        uint8_t out[decrypted_len];
        assert_true(cipher_aes_hmac_decrypt(
            encrypted_seed_and_hmac, len, out, &decrypted_len, _expected_secret));
        assert_true(decrypted_len == seed_len);

        assert_memory_equal(expected_seed, out, seed_len);
    }
}

static void _mock_with_mnemonic(const char* mnemonic, const char* passphrase)
{
    uint8_t seed[32] = {0};
    size_t seed_len;
    assert_true(keystore_bip39_mnemonic_to_seed(mnemonic, seed, &seed_len));

    keystore_mock_unlocked(seed, seed_len, NULL);
    assert_true(keystore_unlock_bip39(passphrase));
}

static void _test_keystore_get_ed25519_seed(void** state)
{
    // Test vectors taken from:
    // https://github.com/cardano-foundation/CIPs/blob/6c249ef48f8f5b32efc0ec768fadf4321f3173f2/CIP-0003/Ledger.md#test-vectors
    // See also: https://github.com/cardano-foundation/CIPs/pull/132

    _mock_with_mnemonic(
        "recall grace sport punch exhibit mad harbor stand obey short width stem awkward used "
        "stairs wool ugly trap season stove worth toward congress jaguar",
        "");

    uint8_t seed[96];
    assert_true(keystore_get_ed25519_seed(seed));
    assert_memory_equal(
        seed,
        "\xa0\x8c\xf8\x5b\x56\x4e\xcf\x3b\x94\x7d\x8d\x43\x21\xfb\x96\xd7\x0e\xe7\xbb\x76\x08\x77"
        "\xe3\x71\x89\x9b\x14\xe2\xcc\xf8\x86\x58\x10\x4b\x88\x46\x82\xb5\x7e\xfd\x97\xde\xcb\xb3"
        "\x18\xa4\x5c\x05\xa5\x27\xb9\xcc\x5c\x2f\x64\xf7\x35\x29\x35\xa0\x49\xce\xea\x60\x68\x0d"
        "\x52\x30\x81\x94\xcc\xef\x2a\x18\xe6\x81\x2b\x45\x2a\x58\x15\xfb\xd7\xf5\xba\xbc\x08\x38"
        "\x56\x91\x9a\xaf\x66\x8f\xe7\xe4",
        sizeof(seed));

    // Multiple loop iterations.
    _mock_with_mnemonic(
        "correct cherry mammal bubble want mandate polar hazard crater better craft exotic choice "
        "fun tourist census gap lottery neglect address glow carry old business",
        "");
    assert_true(keystore_get_ed25519_seed(seed));
    assert_memory_equal(
        seed,
        "\x58\x7c\x67\x74\x35\x7e\xcb\xf8\x40\xd4\xdb\x64\x04\xff\x7a\xf0\x16\xda\xce\x04\x00\x76"
        "\x97\x51\xad\x2a\xbf\xc7\x7b\x9a\x38\x44\xcc\x71\x70\x25\x20\xef\x1a\x4d\x1b\x68\xb9\x11"
        "\x87\x78\x7a\x9b\x8f\xaa\xb0\xa9\xbb\x6b\x16\x0d\xe5\x41\xb6\xee\x62\x46\x99\x01\xfc\x0b"
        "\xed\xa0\x97\x5f\xe4\x76\x3b\xea\xbd\x83\xb7\x05\x1a\x5f\xd5\xcb\xce\x5b\x88\xe8\x2c\x4b"
        "\xba\xca\x26\x50\x14\xe5\x24\xbd",
        sizeof(seed));

    _mock_with_mnemonic(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon "
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon "
        "abandon art",
        "foo");
    assert_true(keystore_get_ed25519_seed(seed));
    assert_memory_equal(
        seed,
        "\xf0\x53\xa1\xe7\x52\xde\x5c\x26\x19\x7b\x60\xf0\x32\xa4\x80\x9f\x08\xbb\x3e\x5d\x90\x48"
        "\x4f\xe4\x20\x24\xbe\x31\xef\xcb\xa7\x57\x8d\x91\x4d\x3f\xf9\x92\xe2\x16\x52\xfe\xe6\xa4"
        "\xd9\x9f\x60\x91\x00\x69\x38\xfa\xc2\xc0\xc0\xf9\xd2\xde\x0b\xa6\x4b\x75\x4e\x92\xa4\xf3"
        "\x72\x3f\x23\x47\x20\x77\xaa\x4c\xd4\xdd\x8a\x8a\x17\x5d\xba\x07\xea\x18\x52\xda\xd1\xcf"
        "\x26\x8c\x61\xa2\x67\x9c\x38\x90",
        sizeof(seed));
}

// This tests that `secp256k1_schnorrsig_sign()` is the correct function to be used for schnorr sigs
// in taproot. It is a separate test because there are test vectors available for this which cannot
// be made to work with `keystore_secp256k1_schnorr_bip86_sign()`.
static void _test_secp256k1_schnorr_sign(void** state)
{
    typedef struct {
        const uint8_t secret_key[32];
        const uint8_t aux_rand[32];
        const uint8_t msg[32];
        const uint8_t expected_sig[64];
    } test_t;

    // Test vectors are the first four rows of
    // https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0340/test-vectors.csv.
    // clang-format off
    const test_t tests[] = {
        {
            .secret_key = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03",
            .aux_rand = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
            .msg = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
            .expected_sig = "\xE9\x07\x83\x1F\x80\x84\x8D\x10\x69\xA5\x37\x1B\x40\x24\x10\x36\x4B\xDF\x1C\x5F\x83\x07\xB0\x08\x4C\x55\xF1\xCE\x2D\xCA\x82\x15\x25\xF6\x6A\x4A\x85\xEA\x8B\x71\xE4\x82\xA7\x4F\x38\x2D\x2C\xE5\xEB\xEE\xE8\xFD\xB2\x17\x2F\x47\x7D\xF4\x90\x0D\x31\x05\x36\xC0",
        },
        {
            .secret_key = "\xB7\xE1\x51\x62\x8A\xED\x2A\x6A\xBF\x71\x58\x80\x9C\xF4\xF3\xC7\x62\xE7\x16\x0F\x38\xB4\xDA\x56\xA7\x84\xD9\x04\x51\x90\xCF\xEF",
            .aux_rand = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01",
            .msg = "\x24\x3F\x6A\x88\x85\xA3\x08\xD3\x13\x19\x8A\x2E\x03\x70\x73\x44\xA4\x09\x38\x22\x29\x9F\x31\xD0\x08\x2E\xFA\x98\xEC\x4E\x6C\x89",
            .expected_sig = "\x68\x96\xBD\x60\xEE\xAE\x29\x6D\xB4\x8A\x22\x9F\xF7\x1D\xFE\x07\x1B\xDE\x41\x3E\x6D\x43\xF9\x17\xDC\x8D\xCF\x8C\x78\xDE\x33\x41\x89\x06\xD1\x1A\xC9\x76\xAB\xCC\xB2\x0B\x09\x12\x92\xBF\xF4\xEA\x89\x7E\xFC\xB6\x39\xEA\x87\x1C\xFA\x95\xF6\xDE\x33\x9E\x4B\x0A",
        },
        {
            .secret_key = "\xC9\x0F\xDA\xA2\x21\x68\xC2\x34\xC4\xC6\x62\x8B\x80\xDC\x1C\xD1\x29\x02\x4E\x08\x8A\x67\xCC\x74\x02\x0B\xBE\xA6\x3B\x14\xE5\xC9",
            .aux_rand = "\xC8\x7A\xA5\x38\x24\xB4\xD7\xAE\x2E\xB0\x35\xA2\xB5\xBB\xBC\xCC\x08\x0E\x76\xCD\xC6\xD1\x69\x2C\x4B\x0B\x62\xD7\x98\xE6\xD9\x06",
            .msg = "\x7E\x2D\x58\xD8\xB3\xBC\xDF\x1A\xBA\xDE\xC7\x82\x90\x54\xF9\x0D\xDA\x98\x05\xAA\xB5\x6C\x77\x33\x30\x24\xB9\xD0\xA5\x08\xB7\x5C",
            .expected_sig = "\x58\x31\xAA\xEE\xD7\xB4\x4B\xB7\x4E\x5E\xAB\x94\xBA\x9D\x42\x94\xC4\x9B\xCF\x2A\x60\x72\x8D\x8B\x4C\x20\x0F\x50\xDD\x31\x3C\x1B\xAB\x74\x58\x79\xA5\xAD\x95\x4A\x72\xC4\x5A\x91\xC3\xA5\x1D\x3C\x7A\xDE\xA9\x8D\x82\xF8\x48\x1E\x0E\x1E\x03\x67\x4A\x6F\x3F\xB7",
        },
        {
            .secret_key = "\x0B\x43\x2B\x26\x77\x93\x73\x81\xAE\xF0\x5B\xB0\x2A\x66\xEC\xD0\x12\x77\x30\x62\xCF\x3F\xA2\x54\x9E\x44\xF5\x8E\xD2\x40\x17\x10",
            .aux_rand = "\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF",
            .msg = "\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF",
            .expected_sig = "\x7E\xB0\x50\x97\x57\xE2\x46\xF1\x94\x49\x88\x56\x51\x61\x1C\xB9\x65\xEC\xC1\xA1\x87\xDD\x51\xB6\x4F\xDA\x1E\xDC\x96\x37\xD5\xEC\x97\x58\x2B\x9C\xB1\x3D\xB3\x93\x37\x05\xB3\x2B\xA9\x82\xAF\x5A\xF2\x5F\xD7\x88\x81\xEB\xB3\x27\x71\xFC\x59\x22\xEF\xC6\x6E\xA3",
        },
    };
    // clang-format on

    for (size_t i = 0; i < sizeof(tests) / sizeof(test_t); i++) {
        const test_t* test = &tests[i];
        uint8_t sig[64] = {0};
        const secp256k1_context* ctx = wally_get_secp_context();
        secp256k1_keypair keypair = {0};
        assert_true(secp256k1_keypair_create(ctx, &keypair, test->secret_key));
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wcast-qual"
        uint8_t* aux_rand_cast = (uint8_t*)test->aux_rand;
#pragma GCC diagnostic pop
        assert_true(secp256k1_schnorrsig_sign(ctx, sig, test->msg, &keypair, aux_rand_cast));
        assert_memory_equal(sig, test->expected_sig, sizeof(sig));
    }
}

static void _test_keystore_secp256k1_schnorr_bip86_pubkey(void** state)
{
    // Test vectors from:
    // https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0086.mediawiki#test-vectors
    // Here we only test the creation of the tweaked pubkkey. See
    // `test_btc_common_address_from_payload()` for the actual address generation, which takes this
    // pubkey as input.
    _mock_with_mnemonic(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon "
        "about",
        "");
    {
        const uint32_t keypath[] = {
            86 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0,
            0,
        };
        uint8_t pubkey[32] = {0};
        assert_true(keystore_secp256k1_schnorr_bip86_pubkey(keypath, 5, pubkey));
        const uint8_t expected_pubkey[32] =
            "\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4"
            "\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c";
        assert_memory_equal(pubkey, expected_pubkey, sizeof(pubkey));
    }
    {
        const uint32_t keypath[] = {
            86 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0,
            1,
        };
        uint8_t pubkey[32] = {0};
        assert_true(keystore_secp256k1_schnorr_bip86_pubkey(keypath, 5, pubkey));
        const uint8_t expected_pubkey[32] =
            "\xa8\x2f\x29\x94\x4d\x65\xb8\x6a\xe6\xb5\xe5\xcc\x75\xe2\x94\xea\xd6\xc5\x93\x91\xa1"
            "\xed\xc5\xe0\x16\xe3\x49\x8c\x67\xfc\x7b\xbb";
        assert_memory_equal(pubkey, expected_pubkey, sizeof(pubkey));
    }
    {
        const uint32_t keypath[] = {
            86 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            0 + BIP32_INITIAL_HARDENED_CHILD,
            1,
            0,
        };
        uint8_t pubkey[32] = {0};
        assert_true(keystore_secp256k1_schnorr_bip86_pubkey(keypath, 5, pubkey));
        const uint8_t expected_pubkey[32] =
            "\x88\x2d\x74\xe5\xd0\x57\x2d\x5a\x81\x6c\xef\x00\x41\xa9\x6b\x6c\x1d\xe8\x32\xf6\xf9"
            "\x67\x6d\x96\x05\xc4\x4d\x5e\x9a\x97\xd3\xdc";
        assert_memory_equal(pubkey, expected_pubkey, sizeof(pubkey));
    }
}

static void _test_keystore_secp256k1_schnorr_bip86_sign(void** state)
{
    _mock_with_mnemonic(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon "
        "about",
        "");
    uint8_t pubkey[32] = {0};
    const uint32_t keypath[] = {
        86 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };
    assert_true(keystore_secp256k1_schnorr_bip86_pubkey(keypath, 5, pubkey));
    uint8_t msg[32] = {0};
    memset(msg, 0x88, sizeof(msg));
    uint8_t sig[64] = {0};
    uint8_t mock_aux_rand[32] = {0};
    will_return(__wrap_random_32_bytes, mock_aux_rand);
    assert_true(keystore_secp256k1_schnorr_bip86_sign(keypath, 5, msg, sig));
    const secp256k1_context* ctx = wally_get_secp_context();
    secp256k1_xonly_pubkey pubkey_deserialized = {0};
    assert_true(secp256k1_xonly_pubkey_parse(ctx, &pubkey_deserialized, pubkey));
    assert_true(secp256k1_schnorrsig_verify(ctx, sig, msg, sizeof(msg), &pubkey_deserialized));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_keystore_get_xpub),
        cmocka_unit_test(_test_keystore_get_root_fingerprint),
        cmocka_unit_test(_test_keystore_secp256k1_nonce_commit),
        cmocka_unit_test(_test_keystore_secp256k1_sign),
        cmocka_unit_test(_test_keystore_encrypt_and_store_seed),
        cmocka_unit_test(_test_keystore_create_and_unlock_twice),
        cmocka_unit_test(_test_keystore_unlock),
        cmocka_unit_test(_test_keystore_lock),
        cmocka_unit_test(_test_keystore_get_bip39_mnemonic),
        cmocka_unit_test(_test_keystore_encode_xpub),
        cmocka_unit_test(_test_keystore_create_and_store_seed),
        cmocka_unit_test(_test_keystore_get_ed25519_seed),
        cmocka_unit_test(_test_secp256k1_schnorr_sign),
        cmocka_unit_test(_test_keystore_secp256k1_schnorr_bip86_pubkey),
        cmocka_unit_test(_test_keystore_secp256k1_schnorr_bip86_sign),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
