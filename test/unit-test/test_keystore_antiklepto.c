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

static void _test_keystore_antiklepto(void** state)
{
    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);

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

    for (int i = 0; i < 3; i++) {
        keypath[4] = i;
        msg[0] = i;
        host_nonce[0] = i;
        uint8_t host_nonce_commitment[32];

        // Protocol steps are described in secp256k1/include/secp256k1_ecdsa_s2c.h under "ECDSA
        // Anti-Klepto Protocol".

        // Protocol step 1.
        assert_true(secp256k1_ecdsa_anti_exfil_host_commit(
            wally_get_secp_context(), host_nonce_commitment, host_nonce));

        // Commit - protocol step 2.
        assert_true(keystore_secp256k1_nonce_commit(
            keypath, 5, msg, host_nonce_commitment, signer_commitment));
        // Protocol step 3: host_nonce sent from host to signer to be used in step 4
        // Sign - protocol step 4.
        assert_true(keystore_secp256k1_sign(keypath, 5, msg, host_nonce, sig, &recid));

        // Protocol step 5: host verification.
        secp256k1_ecdsa_signature parsed_signature;
        assert_true(secp256k1_ecdsa_signature_parse_compact(
            wally_get_secp_context(), &parsed_signature, sig));
        struct ext_key xpub;
        assert_true(keystore_get_xpub(keypath, 5, &xpub));
        secp256k1_pubkey parsed_pubkey;
        assert_true(secp256k1_ec_pubkey_parse(
            wally_get_secp_context(), &parsed_pubkey, xpub.pub_key, sizeof(xpub.pub_key)));
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
