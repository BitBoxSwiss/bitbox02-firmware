// Copyright 2020 Shift Crypto AG
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
#include <keystore/keystore_antiklepto.h>

#include <secp256k1_ecdsa_s2c.h>
#include <wally_bip32.h>
#include <wally_crypto.h>

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wnested-externs"

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

bool __wrap_keystore_secp256k1_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    check_expected(msg32);
    check_expected(host_nonce32);
    check_expected(sig_compact_out);
    check_expected(recid_out);
    return __real_keystore_secp256k1_sign(
        keypath, keypath_len, msg32, host_nonce32, sig_compact_out, recid_out);
}

bool __wrap_keystore_secp256k1_nonce_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    check_expected(msg32);
    check_expected(host_commitment);
    check_expected(signer_commitment_out);
    return __real_keystore_secp256k1_nonce_commit(
        keypath, keypath_len, msg32, host_commitment, signer_commitment_out);
}

static void _test_keystore_antiklepto(void** state)
{
    mock_state(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);

    uint32_t keypath[] = {
        84 + BIP32_INITIAL_HARDENED_CHILD,
        1 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };

    uint8_t msg[32];
    memset(msg, 0x23, sizeof(msg));

    uint8_t host_nonce[32];
    memset(host_nonce, 0x55, sizeof(host_nonce));

    uint8_t signer_commitment[33] = {0};

    uint8_t sig[64];
    int recid;

    // No cached data yet.
    assert_false(keystore_antiklepto_secp256k1_sign(host_nonce, sig, &recid));

    // Run multiple times to make sure the signing cache works a expected.
    for (int i = 0; i < 3; i++) {
        // Modify params to check that the right thing is cached.
        keypath[4] = i;
        msg[0] = i;
        host_nonce[0] = i;
        uint8_t host_nonce_commitment[32];

        // Protocol steps are described in secp256k1/include/secp256k1_ecdsa_s2c.h under "ECDSA
        // Anti-Klepto Protocol".

        // Protocol step 1.
        assert_true(secp256k1_ecdsa_anti_exfil_host_commit(
            wally_get_secp_context(), host_nonce_commitment, host_nonce));

        { // Commit - protocol step 2.
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit, keypath, keypath, sizeof(keypath));
            expect_value(__wrap_keystore_secp256k1_nonce_commit, keypath_len, 5);
            expect_memory(__wrap_keystore_secp256k1_nonce_commit, msg32, msg, sizeof(msg));
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit,
                host_commitment,
                host_nonce_commitment,
                sizeof(host_nonce_commitment));
            expect_value(
                __wrap_keystore_secp256k1_nonce_commit, signer_commitment_out, signer_commitment);
            assert_true(keystore_antiklepto_secp256k1_commit(
                keypath, 5, msg, host_nonce_commitment, signer_commitment));

            // Can't commit again, already has cached data
            assert_false(keystore_antiklepto_secp256k1_commit(
                keypath, 5, msg, host_nonce_commitment, signer_commitment));

            // After clearing, we can commit again.
            keystore_antiklepto_clear();
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit, keypath, keypath, sizeof(keypath));
            expect_value(__wrap_keystore_secp256k1_nonce_commit, keypath_len, 5);
            expect_memory(__wrap_keystore_secp256k1_nonce_commit, msg32, msg, sizeof(msg));
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit,
                host_commitment,
                host_nonce_commitment,
                sizeof(host_nonce_commitment));
            expect_value(
                __wrap_keystore_secp256k1_nonce_commit, signer_commitment_out, signer_commitment);
            assert_true(keystore_antiklepto_secp256k1_commit(
                keypath, 5, msg, host_nonce_commitment, signer_commitment));
        }
        // Protocol step 3: host_nonce sent from host to signer to be used in step 4
        { // Sign - protocol step 4.
            expect_memory(__wrap_keystore_secp256k1_sign, keypath, keypath, sizeof(keypath));
            expect_value(__wrap_keystore_secp256k1_sign, keypath_len, 5);
            expect_memory(__wrap_keystore_secp256k1_sign, msg32, msg, sizeof(msg));
            expect_memory(
                __wrap_keystore_secp256k1_sign, host_nonce32, host_nonce, sizeof(host_nonce));
            expect_value(__wrap_keystore_secp256k1_sign, sig_compact_out, sig);
            expect_value(__wrap_keystore_secp256k1_sign, recid_out, &recid);
            assert_true(keystore_antiklepto_secp256k1_sign(host_nonce, sig, &recid));
        }

        // Protocol step 5: host verification.
        secp256k1_ecdsa_signature parsed_signature;
        assert_true(secp256k1_ecdsa_signature_parse_compact(
            wally_get_secp_context(), &parsed_signature, sig));
        uint8_t pubkey[EC_PUBLIC_KEY_UNCOMPRESSED_LEN];
        assert_true(keystore_secp256k1_pubkey_uncompressed(keypath, 5, pubkey));
        secp256k1_pubkey parsed_pubkey;
        assert_true(secp256k1_ec_pubkey_parse(
            wally_get_secp_context(), &parsed_pubkey, pubkey, sizeof(pubkey)));
        secp256k1_ecdsa_s2c_opening opening;
        assert_true(secp256k1_ecdsa_s2c_opening_parse(
            wally_get_secp_context(), &opening, signer_commitment));
        assert_true(secp256k1_anti_exfil_host_verify(
            wally_get_secp_context(),
            &parsed_signature,
            msg,
            &parsed_pubkey,
            host_nonce,
            &opening));
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_keystore_antiklepto),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
