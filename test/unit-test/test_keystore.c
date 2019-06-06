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
#include <secp256k1.h>
#include <securechip/securechip.h>
#include <util.h>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wbad-function-cast"

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static uint8_t _mock_bip39_seed[64] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static uint8_t _password_salted_hashed_stretch_in[32] = {
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
};

static uint8_t _password_salted_hashed_stretch_out[32] = {
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
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

int __real_secp256k1_ecdsa_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    const unsigned char* msg32,
    const unsigned char* seckey,
    secp256k1_nonce_function noncefp,
    const void* ndata);

int __wrap_secp256k1_ecdsa_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    const unsigned char* msg32,
    const unsigned char* seckey,
    secp256k1_nonce_function noncefp,
    const void* ndata)
{
    check_expected(msg32);
    check_expected(seckey);
    return __real_secp256k1_ecdsa_sign(ctx, sig, msg32, seckey, noncefp, ndata);
}

bool __wrap_salt_hash_data(
    const uint8_t* data,
    size_t data_len,
    const char* purpose,
    uint8_t* hash_out)
{
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

static bool _get_pubkey(const uint32_t* keypath, size_t keypath_len, secp256k1_pubkey* out)
{
    struct ext_key xpub = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &xpub)) {
        return false;
    }
    return secp256k1_ec_pubkey_parse(
        wally_get_secp_context(), out, xpub.pub_key, sizeof(xpub.pub_key));
}

static void _test_keystore_sign_secp256k1(void** state)
{
    uint32_t keypath[] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        5,
    };
    uint8_t msg[32] = {0};
    memset(msg, 0x88, sizeof(msg));
    uint8_t sig[64] = {0};
    secp256k1_context* ctx = wally_get_secp_context();

    {
        // fails because keystore is locked
        assert_false(
            keystore_sign_secp256k1(keypath, sizeof(keypath) / sizeof(uint32_t), msg, sig));
    }
    {
        mock_state(_mock_seed, _mock_bip39_seed);
        // check derivation with a fixture
        const uint8_t expected_seckey[32] = {
            0x4e, 0x64, 0xdf, 0xd3, 0x3a, 0xae, 0x66, 0xc4, 0xc7, 0x52, 0x6c,
            0xf0, 0x2e, 0xe8, 0xae, 0x3f, 0x58, 0x92, 0x32, 0x9d, 0x67, 0xdf,
            0xd4, 0xad, 0x05, 0xe9, 0xc3, 0xd0, 0x6e, 0xdf, 0x74, 0xfb,
        };
        expect_memory(__wrap_secp256k1_ecdsa_sign, seckey, expected_seckey, 32);
        expect_memory(__wrap_secp256k1_ecdsa_sign, msg32, msg, sizeof(msg));
        // check sig by verifying it against the msg.
        assert_true(keystore_sign_secp256k1(keypath, sizeof(keypath) / sizeof(uint32_t), msg, sig));
        secp256k1_pubkey pubkey = {0};
        assert_true(_get_pubkey(keypath, sizeof(keypath) / sizeof(uint32_t), &pubkey));
        secp256k1_ecdsa_signature secp256k1_sig = {0};
        assert_true(secp256k1_ecdsa_signature_parse_compact(ctx, &secp256k1_sig, sig));
        assert_true(secp256k1_ecdsa_verify(ctx, &secp256k1_sig, msg, &pubkey));
    }
}

static void _expect_stretch(void)
{
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

    expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_access_out");
    will_return(__wrap_salt_hash_data, _password_salted_hashed_stretch_out);
}

static void _test_keystore_encrypt_and_store_seed(void** state)
{
    will_return(__wrap_memory_is_initialized, false);
    const char* password = "password";

    _expect_stretch(); // first stretch to encrypt
    _expect_stretch(); // second stretch to verify

    // Fixture: hmac.new(_password_salted_hashed_stretch_out, _kdf_out_3,
    // hashlib.sha256).hexdigest()
    static uint8_t expected_secret[32] = {
        0x39, 0xa7, 0x4f, 0x75, 0xb6, 0x9d, 0x6c, 0x84, 0x5e, 0x18, 0x91,
        0x5b, 0xae, 0x29, 0xd1, 0x06, 0x12, 0x12, 0x40, 0x37, 0x7a, 0x79,
        0x97, 0x55, 0xd7, 0xcc, 0xe9, 0x26, 0x1e, 0x16, 0x91, 0x71,
    };
    expect_memory(__wrap_cipher_aes_hmac_encrypt, secret, expected_secret, 32);

    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, password));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_keystore_sign_secp256k1),
        cmocka_unit_test(_test_keystore_encrypt_and_store_seed),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
