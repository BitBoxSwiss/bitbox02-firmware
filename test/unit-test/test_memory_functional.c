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
#include <random.h>

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

void _memory_setup_rand_mock_test_functional(uint8_t* buf_out)
{
    static uint8_t ctr = 0;
    static uint8_t fixtures[][32] = {
        // salt root
        "\xbd\xb9\xca\x49\x75\xe5\x9e\x1b\x61\xd9\x14\x1c\x5e\x79\x68\x8c\xba\x7b\x39\x89\xb5\x2b"
        "\x78\x2d\xe2\xe7\xe4\x9b\x07\xec\x8f\xae",
        // io_protection_key
        "\x28\x30\x9e\x5a\x2e\x3b\xcf\x4a\xac\x94\xc0\xe5\x90\x10\xfa\x34\x92\xe1\x08\x39\xef\xb5"
        "\xb6\x61\x92\xad\x18\xf6\x6a\x80\x51\x0b",
        // io_protection_key_split
        "\xae\x5b\xe4\x4d\x8b\x71\xa6\x04\x1a\x7e\x97\x33\xe5\x5f\x8c\x88\xb7\x9d\xd5\x52\x10\x76"
        "\x24\xe0\xa9\x16\xc1\x0d\x87\x55\xe0\x4e",
        // authorization_key
        "\x62\xc7\x41\xd9\xce\x78\x32\xe8\x56\xec\x06\xf6\x35\x1c\xef\xcd\x9e\x7c\x5c\xa6\x07\x93"
        "\x8a\xbb\x70\x97\x70\xa5\xf2\xdb\xeb\xcb",
        // authorization_key_split
        "\x20\x74\x2d\x5a\x58\x2f\x1f\x25\xb6\xe9\xd1\xc1\xe8\xb1\xef\xfb\x40\xcf\xac\x85\x56\x67"
        "\xea\x7f\x49\x96\x8a\xf7\xf7\xeb\x5c\x19",
        // encryption_key
        "\xed\x18\x37\x84\xcb\xd2\x97\xf9\xc2\xc2\x41\xd0\xdd\x7c\xd1\x6d\x62\x36\x6c\x44\xb8\x33"
        "\xdd\xf2\xc0\x12\xfb\x4b\x49\xe1\xe8\xf3",
        // encryption_key_split
        "\x19\xf6\x0e\xe8\x25\xe7\x52\x15\x0d\x30\x88\x17\x34\x8c\x0f\xa6\xb3\xfe\x4f\x60\x4c\x85"
        "\xc1\x7e\x2e\xb9\x7a\xda\x60\x4a\x47\x6f",
    };
    memcpy(buf_out, fixtures[ctr], 32);
    ctr++;
}

// Test a series of write/read operations
static void _test_functional(void** state)
{
    mock_memory_factoryreset();

    memory_interface_functions_t ifs = {
        .random_32_bytes = _memory_setup_rand_mock_test_functional,
    };
    assert_true(memory_setup(&ifs));

    uint8_t io_protection_key[32];
    const uint8_t expected_io_protection_key[32] =
        "\x86\x6b\x7a\x17\xa5\x4a\x69\x4e\xb6\xea\x57\xd6\x75\x4f\x76\xbc\x25\x7c\xdd\x6b\xff\xc3"
        "\x92\x81\x3b\xbb\xd9\xfb\xed\xd5\xb1\x45";
    memory_get_io_protection_key(io_protection_key);
    assert_memory_equal(io_protection_key, expected_io_protection_key, sizeof(io_protection_key));

    uint8_t authorization_key[32];
    const uint8_t expected_authorization_key[32] =
        "\x42\xb3\x6c\x83\x96\x57\x2d\xcd\xe0\x05\xd7\x37\xdd\xad\x00\x36\xde\xb3\xf0\x23\x51\xf4"
        "\x60\xc4\x39\x01\xfa\x52\x05\x30\xb7\xd2";
    memory_get_authorization_key(authorization_key);
    assert_memory_equal(authorization_key, expected_authorization_key, sizeof(authorization_key));

    uint8_t encryption_key[32];
    const uint8_t expected_encryption_key[32] =
        "\xf4\xee\x39\x6c\xee\x35\xc5\xec\xcf\xf2\xc9\xc7\xe9\xf0\xde\xcb\xd1\xc8\x23\x24\xf4\xb6"
        "\x1c\x8c\xee\xab\x81\x91\x29\xab\xaf\x9c";
    memory_get_encryption_key(encryption_key);
    assert_memory_equal(encryption_key, expected_encryption_key, sizeof(encryption_key));

    // Run again, shouldn't do anything. Secure chip keys unchanged.
    assert_true(memory_setup(&ifs));
    // Other operations modifying the same memory chunk shouldn't change the secure chip keys.
    auto_enter_t autoenter = {.value = secfalse_u8};
    upside_down_t upside_down = {.value = true};
    assert_true(memory_bootloader_set_flags(autoenter, upside_down));

    assert_memory_equal(io_protection_key, expected_io_protection_key, sizeof(io_protection_key));
    assert_memory_equal(authorization_key, expected_authorization_key, sizeof(authorization_key));
    assert_memory_equal(encryption_key, expected_encryption_key, sizeof(encryption_key));
}

static void _test_attestation_bootloader_hash(void** state)
{
    mock_memory_factoryreset();

    memory_interface_functions_t ifs = {
        .random_32_bytes = random_32_bytes_mcu,
    };
    assert_true(memory_setup(&ifs));

    const uint8_t mock1[32] =
        "\x03\x22\xb3\x19\x1a\xab\x5b\xc4\x15\xc5\xba\xfa\xc5\x33\x34\x45\x17\x5b\xe2\xfa\xa8\x33"
        "\x3a\xc3\xab\xee\x4c\xd1\x7e\x49\x08\x2a";
    memory_set_bootloader_hash_mock(mock1);
    uint8_t hash[32];
    memory_bootloader_hash(hash);
    memory_get_attestation_bootloader_hash(hash);
    assert_memory_equal(hash, mock1, sizeof(hash));

    assert_true(memory_set_attestation_bootloader_hash());
    memset(hash, 0x00, sizeof(hash));
    memory_get_attestation_bootloader_hash(hash);
    assert_memory_equal(hash, mock1, sizeof(hash));

    const uint8_t mock2[32] =
        "\x6c\xad\x6a\xbc\x3f\xd4\x47\xa5\x8d\x7a\x26\x2d\x76\x06\xa0\x40\xe4\x9e\x82\xb0\x06\x48"
        "\x62\x36\x25\x88\x3e\x9f\xc0\xfa\xa8\xad";
    memory_set_bootloader_hash_mock(mock2);

    memset(hash, 0x00, sizeof(hash));
    memory_bootloader_hash(hash);
    assert_memory_equal(hash, mock2, sizeof(hash));

    memset(hash, 0x00, sizeof(hash));
    memory_get_attestation_bootloader_hash(hash);
    assert_memory_equal(hash, mock1, sizeof(hash));

    assert_true(memory_reset_hww());

    memset(hash, 0x00, sizeof(hash));
    memory_get_attestation_bootloader_hash(hash);
    assert_memory_equal(hash, mock1, sizeof(hash));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_memory_multisig),
        cmocka_unit_test(_test_memory_multisig_invalid),
        cmocka_unit_test(_test_memory_multisig_full),
        cmocka_unit_test(_test_memory_attestation),
        cmocka_unit_test(_test_functional),
        cmocka_unit_test(_test_attestation_bootloader_hash),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
