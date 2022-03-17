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

#include <memory/memory.h>
#include <mock_memory.h>

static void _test_memory_multisig(void** state)
{
    mock_memory_factoryreset();

    const uint8_t hashes[][32] = {
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "cccccccccccccccccccccccccccccccc",
        "dddddddddddddddddddddddddddddddd",
        "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        "ffffffffffffffffffffffffffffffff",
    };
    const char* names[] = {
        "name1",
        "name2",
        "name3",
        "name4",
        "name5",
        "name6",
    };

    char name[31] = {0};
    assert_false(memory_multisig_get_by_hash(hashes[0], name));

    // set
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[0], names[0]));
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[1], names[1]));
    // overwrite with the same is possible
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[1], names[1]));

    // get
    assert_true(memory_multisig_get_by_hash(hashes[0], NULL));
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, names[0]);
    assert_true(memory_multisig_get_by_hash(hashes[1], name));
    assert_string_equal(name, names[1]);
    // rename
    const char* name0_renamed = "name 1 renamed";
    assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[0], name0_renamed));
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, name0_renamed);

    // rename to a name which already exists fails (duplicate name).
    assert_int_equal(MEMORY_ERR_DUPLICATE_NAME, memory_multisig_set_by_hash(hashes[0], names[1]));
    // was in fact not renamed
    assert_true(memory_multisig_get_by_hash(hashes[0], name));
    assert_string_equal(name, name0_renamed);
}

static void _test_memory_multisig_invalid(void** state)
{
    // invalid hash
    uint8_t empty[32];
    memset(empty, 0xFF, sizeof(empty));
    assert_int_equal(MEMORY_ERR_INVALID_INPUT, memory_multisig_set_by_hash(empty, "foo"));

    // invalid name
    uint8_t hash[32] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    assert_int_equal(MEMORY_ERR_INVALID_INPUT, memory_multisig_set_by_hash(hash, ""));
}

static void _test_memory_multisig_full(void** state)
{
    mock_memory_factoryreset();
    // Only 25 slots available.
    const size_t limit = 25;
    uint8_t hashes[limit + 1][32];
    char names[limit + 1][10];
    for (size_t i = 0; i < limit + 1; i++) {
        memset(hashes[i], (int)(i + i), 32);
        snprintf(names[i], sizeof(names[i]), "name%lu", i);
    }

    for (size_t i = 0; i < limit; i++) {
        assert_int_equal(MEMORY_OK, memory_multisig_set_by_hash(hashes[i], names[i]));
    }
    assert_int_equal(MEMORY_ERR_FULL, memory_multisig_set_by_hash(hashes[limit], names[limit]));
}

static void _test_memory_attestation(void** state)
{
    mock_memory_factoryreset();

    uint8_t expected_pubkey[64];
    memset(expected_pubkey, 0x55, sizeof(expected_pubkey));
    uint8_t expected_certificate[64];
    memset(expected_pubkey, 0x66, sizeof(expected_certificate));
    uint8_t expected_root_pubkey_identifier[32];
    memset(expected_pubkey, 0x77, sizeof(expected_root_pubkey_identifier));

    uint8_t pubkey[64];
    uint8_t certificate[64];
    uint8_t root_pubkey_identifier[32];
    // Setup not done yet.
    assert_false(
        memory_get_attestation_pubkey_and_certificate(pubkey, certificate, root_pubkey_identifier));

    assert_true(memory_set_attestation_device_pubkey(expected_pubkey));

    // Setup not done yet.
    assert_false(
        memory_get_attestation_pubkey_and_certificate(pubkey, certificate, root_pubkey_identifier));

    uint8_t wrong_pubkey[64];
    memset(wrong_pubkey, 0x11, sizeof(wrong_pubkey));
    // Pubkey has to match the previously stored pubkey.
    assert_false(memory_set_attestation_certificate(
        wrong_pubkey, expected_certificate, expected_root_pubkey_identifier));

    assert_true(memory_set_attestation_certificate(
        expected_pubkey, expected_certificate, expected_root_pubkey_identifier));

    // Setup done.
    assert_true(
        memory_get_attestation_pubkey_and_certificate(pubkey, certificate, root_pubkey_identifier));
    assert_memory_equal(pubkey, expected_pubkey, sizeof(pubkey));
    assert_memory_equal(certificate, expected_certificate, sizeof(certificate));
    assert_memory_equal(
        root_pubkey_identifier, expected_root_pubkey_identifier, sizeof(root_pubkey_identifier));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_memory_multisig),
        cmocka_unit_test(_test_memory_multisig_invalid),
        cmocka_unit_test(_test_memory_multisig_full),
        cmocka_unit_test(_test_memory_attestation),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
