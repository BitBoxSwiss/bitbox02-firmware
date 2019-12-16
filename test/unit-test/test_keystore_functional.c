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

#include <keystore.h>

#include <apps/btc/btc_common.h>
#include <mock_memory.h>
#include <securechip/securechip.h>
#include <util.h>

#include <wally_crypto.h>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* This file performs some functional keystore tests against fixtures rather than unit tests,
   mocking only low-level functions which are not available on the host. This gives extra assurance
   that seeding/unlocking/derivations work as expected. */

static const char* _some_password = "foo";
static const char* _some_other_password = "bar";
static const uint8_t _xpub_version[4] = {0x04, 0x88, 0xb2, 0x1e};

static uint8_t _seed[KEYSTORE_MAX_SEED_LENGTH] =
    "\xcb\x33\xc2\x0c\xea\x62\xa5\xc2\x77\x52\x7e\x20\x02\xda\x82\xe6\xe2\xb3\x74\x50\xa7\x55\x14"
    "\x3a\x54\x0a\x54\xce\xa8\xda\x90\x44";

static uint8_t _salt_root[KEYSTORE_MAX_SEED_LENGTH] = {
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
};

bool __wrap_securechip_kdf(securechip_slot_t slot, const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    assert_true(slot == SECURECHIP_SLOT_KDF || slot == SECURECHIP_SLOT_ROLLKEY);
    uint8_t key[3] = "key";
    assert_int_equal(WALLY_OK, wally_hmac_sha256(key, sizeof(key), msg, len, kdf_out, SHA256_LEN));
    return true;
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

static void _test_seeds(void** state)
{
    _smarteeprom_reset();
    assert_true(keystore_is_locked());
    uint8_t read_seed[KEYSTORE_MAX_SEED_LENGTH];
    uint32_t read_seed_len;
    assert_false(keystore_copy_seed(read_seed, &read_seed_len));

    will_return(__wrap_memory_is_initialized, true);
    assert_false(keystore_encrypt_and_store_seed(_seed, 32, _some_password));

    uint32_t seed_sizes[3] = {16, 24, 32};
    for (size_t seed_size_idx = 0; seed_size_idx < 3; seed_size_idx++) {
        uint32_t seed_size = seed_sizes[seed_size_idx];
        will_return(__wrap_memory_is_initialized, false);
        assert_true(keystore_encrypt_and_store_seed(_seed, seed_size, _some_password));
        uint8_t remaining_attempts;
        will_return(__wrap_memory_is_seeded, true);
        assert_int_equal(
            KEYSTORE_ERR_INCORRECT_PASSWORD,
            keystore_unlock(_some_other_password, &remaining_attempts));
        // First time: unlock. After unlock, it becomes a password check.
        for (int i = 0; i < 3; i++) {
            will_return(__wrap_memory_is_seeded, true);
            assert_int_equal(KEYSTORE_OK, keystore_unlock(_some_password, &remaining_attempts));
        }
        assert_true(keystore_copy_seed(read_seed, &read_seed_len));
        assert_int_equal(seed_size, read_seed_len);
        keystore_lock();
    }
}

static void _free_string(char** s)
{
    wally_free_string(*s);
}

static void _check_mnemonic(const char* expected)
{
    char* __attribute__((__cleanup__(_free_string))) mnemonic;
    assert_true(keystore_get_bip39_mnemonic(&mnemonic));
    assert_string_equal(mnemonic, expected);
}

static void _assert_equal_memory_hex(const uint8_t* buf, size_t buf_size, const char* expected_hex)
{
    char buf_hex[2 * buf_size + 1];
    util_uint8_to_hex(buf, buf_size, buf_hex);
    assert_string_equal(buf_hex, expected_hex);
}

static void _check_pubs(
    const char* expected_xpub,
    const char* expected_hash160_hex,
    const char* expected_pubkey_uncompressed_hex)
{
    struct ext_key __attribute__((__cleanup__(keystore_zero_xkey))) xpub;
    uint32_t keypath[] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        1,
        2,
    };

    assert_true(keystore_get_xpub(keypath, 3, &xpub));
    char xpub_serialized[120];
    assert_true(
        btc_common_encode_xpub(&xpub, _xpub_version, xpub_serialized, sizeof(xpub_serialized)));
    assert_string_equal(xpub_serialized, expected_xpub);

    uint8_t hash160[20];
    assert_true(keystore_secp256k1_pubkey(
        KEYSTORE_SECP256K1_PUBKEY_HASH160, keypath, 5, hash160, sizeof(hash160)));
    _assert_equal_memory_hex(hash160, sizeof(hash160), expected_hash160_hex);

    uint8_t pubkey_uncompressed[EC_PUBLIC_KEY_UNCOMPRESSED_LEN];
    assert_true(keystore_secp256k1_pubkey(
        KEYSTORE_SECP256K1_PUBKEY_UNCOMPRESSED,
        keypath,
        5,
        pubkey_uncompressed,
        sizeof(pubkey_uncompressed)));
    _assert_equal_memory_hex(
        pubkey_uncompressed, sizeof(pubkey_uncompressed), expected_pubkey_uncompressed_hex);
}

static void _test_combination(
    const char* mnemonic_passphrase,
    uint32_t seed_len,
    const char* expected_mnemonic,
    const char* expected_xpub,
    const char* expected_hash160_hex,
    const char* expected_pubkey_uncompressed_hex,
    const char* expected_u2f_seed_hex)
{
    assert_false(keystore_unlock_bip39(mnemonic_passphrase));

    will_return(__wrap_memory_is_initialized, false);
    assert_true(keystore_encrypt_and_store_seed(_seed, seed_len, _some_password));
    assert_false(keystore_unlock_bip39(mnemonic_passphrase));
    uint8_t remaining_attempts;
    assert_true(keystore_is_locked());
    will_return(__wrap_memory_is_seeded, true);
    assert_int_equal(KEYSTORE_OK, keystore_unlock(_some_password, &remaining_attempts));
    assert_true(keystore_is_locked());
    assert_true(keystore_unlock_bip39(mnemonic_passphrase));
    assert_false(keystore_is_locked());
    _check_mnemonic(expected_mnemonic);
    _check_pubs(expected_xpub, expected_hash160_hex, expected_pubkey_uncompressed_hex);

    uint8_t u2f_seed[32];
    assert_true(keystore_get_u2f_seed(u2f_seed));
    _assert_equal_memory_hex(u2f_seed, sizeof(u2f_seed), expected_u2f_seed_hex);
    keystore_lock();
}

static void _test_fixtures(void** state)
{
    {
        const char* mnemonic_passphrase = "";
        uint32_t seed_len = 32;
        const char* expected_mnemonic =
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot "
            "dream turkey before sport action praise tunnel hood donate man";
        const char* expected_xpub =
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZc"
            "fAC1C75HfR66UbjpAb39f4PnG";
        const char* expected_hash160_hex = "e5f89ab6543744f78f15867c4306ee866bb11df9";
        const char* expected_pubkey_uncompressed_hex =
            "0477a44aa9e8c8fb5105ef5ee2394e8aed89ad73fc74361425f06347ecfe326131e1339367ee3cbe877192"
            "85a07f774b17eb933ecf0b9b82acebc195226d634244";
        const char* expected_u2f_seed_hex =
            "4f464a6667ad88eebcd0f02982761e474ee0dd16253160320f49d1d6681745e9";
        _test_combination(
            mnemonic_passphrase,
            seed_len,
            expected_mnemonic,
            expected_xpub,
            expected_hash160_hex,
            expected_pubkey_uncompressed_hex,
            expected_u2f_seed_hex);
    }
    {
        const char* mnemonic_passphrase = "abc";
        uint32_t seed_len = 32;
        const char* expected_mnemonic =
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot "
            "dream turkey before sport action praise tunnel hood donate man";
        const char* expected_xpub =
            "xpub6DXBP3HhFdhUTafatEULxfTXUUxDVuCxfa9RAiBU5r6aRgKiABbeBDyqwWWjmKPP1BZvpvVNMbVR5LeHzh"
            "QphtLcPZ8jk3MdLBgc2sACJwR";
        const char* expected_hash160_hex = "a2de30da46d3980eccbd73dbd67784da162952b2";
        const char* expected_pubkey_uncompressed_hex =
            "044fb66eeefd352b441c86a6200a1e871928a367f5ab5f46566645d01d0534791ae39ff64a7d14d2427297"
            "61ebd3829e8536b389dba543cbc48b1d86c01559d27b";
        const char* expected_u2f_seed_hex =
            "d599da991ad83baaf449c789e2dff1539dd66983b47a1dec1c00ff3f352cccbc";
        _test_combination(
            mnemonic_passphrase,
            seed_len,
            expected_mnemonic,
            expected_xpub,
            expected_hash160_hex,
            expected_pubkey_uncompressed_hex,
            expected_u2f_seed_hex);
    }
    {
        const char* mnemonic_passphrase = "";
        uint32_t seed_len = 24;
        const char* expected_mnemonic =
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot "
            "dream turkey before subject";
        const char* expected_xpub =
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18B"
            "G81VJhJDXnSpRe5NGKwVpXiAb";
        const char* expected_hash160_hex = "6f4558f6256a3e06437766cd9de36c61538fe3bb";
        const char* expected_pubkey_uncompressed_hex =
            "043113631363e62a07d6a0becafc8063bb311fd1e9e71a6930d995857837642648aba5c743374e19428565"
            "80f565c6b929737af5439f65f5333baf1d63c1f986bf";
        const char* expected_u2f_seed_hex =
            "fb9dc3fb0a17390776df5c3d8f9261bc5fd5df9f00414cee1393e37e0efda7ef";
        _test_combination(
            mnemonic_passphrase,
            seed_len,
            expected_mnemonic,
            expected_xpub,
            expected_hash160_hex,
            expected_pubkey_uncompressed_hex,
            expected_u2f_seed_hex);
    }
    {
        const char* mnemonic_passphrase = "";
        uint32_t seed_len = 16;
        const char* expected_mnemonic =
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff";
        const char* expected_xpub =
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6"
            "gA4ULCBhvwZj29mHCPYSux3YV";
        const char* expected_hash160_hex = "bd4ec9ff7089a093865d1b8f3a12062f607664d4";
        const char* expected_pubkey_uncompressed_hex =
            "04588110a40455d74a3fd439fa2f4c0994cd0dc64644f9e5bc03cc99e7fcfe32eea56cb72d31cb997663b1"
            "f62ad12e9c3a24b717064e8db4cc8ca70ac8a98a46a5";
        const char* expected_u2f_seed_hex =
            "20d68b206aff9667b623a460ce61fc94762de67561d6855ca9a6df7b409b2a54";
        _test_combination(
            mnemonic_passphrase,
            seed_len,
            expected_mnemonic,
            expected_xpub,
            expected_hash160_hex,
            expected_pubkey_uncompressed_hex,
            expected_u2f_seed_hex);
    }
}

int main(void)
{
    mock_memory_set_salt_root(_salt_root);

    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_seeds),
        cmocka_unit_test(_test_fixtures),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
