// Copyright 2019 Shift Cryptosecurity AG
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
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <time.h>
#include <cmocka.h>

#include <backup/backup.h>
#include <backup/backup_common.h>
#include <backup/restore.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <util.h>

#include <ff.h>
#include <sd.h>
#include <wally_crypto.h>

#define DEVICE_NAME "TestDeviceName"
static const uint32_t _current_timestamp = 1553098951;

static const uint32_t _mock_seed_birthdate = 1552553498;
static const uint8_t _mock_seed[32] = {0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                                       11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                                       22, 23, 24, 25, 26, 27, 28, 29, 30, 31};
static const uint8_t _mock_seed_length = 32;

void __wrap_memory_get_seed_birthdate(uint32_t* timestamp_out)
{
    // caller should specify will_return(__wrap_memory_get_seed_birthdate, _mock_seed_birthdate);
    uint32_t expected = mock_type(uint32_t);
    *timestamp_out = expected;
}

bool __wrap_keystore_copy_seed(uint8_t* seed, uint32_t* length)
{
    // caller should specify will_return(__wrap_keystore_copy_seed,
    // cast_ptr_to_largest_integral_type(seed)); caller should specify
    // will_return(__wrap_keystore_copy_seed, _mock_seed_length);
    uint8_t expected_seed_length = mock_type(uint8_t);
    uint8_t* expected_seed = mock_ptr_type(uint8_t*);
    *length = expected_seed_length;
    memcpy(seed, (void*)(long)expected_seed, *length);
    return true;
}

static void _will_mock_backup_queries(const uint32_t seed_birthdate, const uint8_t* seed)
{
    will_return(__wrap_memory_get_device_name, DEVICE_NAME);
    will_return(__wrap_keystore_copy_seed, _mock_seed_length);
    will_return(__wrap_keystore_copy_seed, cast_ptr_to_largest_integral_type(seed));
}

static void _load_first_backup(Backup* backup, BackupData* backup_data)
{
    const char* dir_name = "064bc03053f0d86068fd35b6ae0e0371abab9a4fa111b7f17b58f70701e1a495";

    const char* first_file_name = "backup_Wed_2019-03-20T16-22-31Z_0.bin";
    uint8_t read_content[SD_MAX_FILE_SIZE];
    size_t read_length;
    assert_true(sd_load_bin(first_file_name, dir_name, read_content, &read_length));

    memset(backup, 0, sizeof(Backup));
    memset(backup_data, 0, sizeof(BackupData));
    assert_int_equal(
        restore_from_buffer(read_content, read_length, backup, backup_data), RESTORE_OK);
}

static int test_setup(void** state)
{
    assert_true(sd_format());
    return 0;
}

static int test_teardown(void** state)
{
    return 0;
}

/**
 * Test calculation of checksum and compare with value stored in backup.
 */
static void test_backup_calculate_checksum(void** state)
{
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);

    Backup backup;
    BackupData backup_data;
    _load_first_backup(&backup, &backup_data);

    uint8_t hash[SHA256_LEN] = {0};
    backup_calculate_checksum(&backup.backup_v1.content, &backup_data, hash);

    uint8_t zeros[SHA256_LEN] = {0};
    assert_memory_equal(backup.backup_v1.content.checksum, hash, SHA256_LEN);
    assert_memory_not_equal(backup.backup_v1.content.checksum, zeros, SHA256_LEN);
}

static bool dir_exists(const char* dir)
{
    sd_list_t files __attribute__((__cleanup__(sd_free_list)));
    return sd_list(&files) && files.num_files > 0;
}

/**
 * Test Backup Create.
 */
static void test_backup_create(void** state)
{
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);

    const char* dir_name = "064bc03053f0d86068fd35b6ae0e0371abab9a4fa111b7f17b58f70701e1a495";
    assert_true(dir_exists(dir_name));

    // assert 3 files in directory, with correct name
    sd_list_t list;
    assert_true(sd_list_subdir(&list, dir_name));
    assert_int_equal(list.num_files, 3);
    sd_free_list(&list);
    for (uint8_t i = 0; i < 3; i++) {
        char file_name[257];
        snprintf(file_name, sizeof(file_name), "backup_Wed_2019-03-20T16-22-31Z_%d.bin", i);
        uint8_t buff[1000] = {0};
        size_t written;
        assert_true(sd_load_bin(file_name, dir_name, buff, &written));
    }

    // test if decode works
    Backup backup;
    BackupData backup_data;
    _load_first_backup(&backup, &backup_data);

    uint8_t zeros_backup[sizeof(Backup)] = {0};
    uint8_t zeros_backup_data[sizeof(BackupData)] = {0};

    assert_memory_not_equal(&backup, zeros_backup, sizeof(Backup));
    assert_memory_not_equal(&backup_data, zeros_backup_data, sizeof(BackupData));

    assert_memory_equal(backup_data.seed, _mock_seed, _mock_seed_length);
    assert_int_equal(backup_data.birthdate, _mock_seed_birthdate);
    assert_int_equal(backup.backup_v1.content.metadata.timestamp, _current_timestamp);
    assert_memory_equal(backup.backup_v1.content.metadata.name, DEVICE_NAME, sizeof(DEVICE_NAME));
}

static void test_backup_check(void** state)
{
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    char id[256];
    char name[64];
    uint32_t birthdate;
    assert_int_equal(backup_check(id, name, &birthdate), BACKUP_OK);
    assert_int_equal(birthdate, _mock_seed_birthdate);
}

static void test_backup_check_fail(void** state)
{
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);

    const uint32_t new_seed_birthdate = _mock_seed_birthdate + 24 * 60 * 60 * 1000;
    const uint8_t new_seed[32] = {16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15};

    _will_mock_backup_queries(new_seed_birthdate, new_seed);
    char id[256];
    char name[64];
    assert_int_not_equal(backup_check(id, name, NULL), BACKUP_OK);
}

// Check the encoding of the backup agains a fixture to ensure no accidental format changes.
static void test_backup_fixture(void** state)
{
    Backup backup;
    BackupData backup_data;
    encode_data_t encode_data;
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(
        backup_fill(
            "v9.0.0",
            _current_timestamp,
            _mock_seed_birthdate,
            &backup,
            &backup_data,
            &encode_data),
        BACKUP_OK);

    uint8_t encoded[1000] = {0};
    size_t len = backup_encode(&backup, sizeof(encoded), encoded);
    const uint8_t expected_encoded[] =
        "\x0a\x70\x0a\x6e\x0a\x20\x67\x1a\xc5\xf7\xe2\x8b\x86\x71\xcf\x98\xa0\xc8\xa5\xca\x40\x1e"
        "\xa8\x88\x66\x69\xad\x24\x5d\x95\xac\x09\xa7\xae\xdc\x73\xe6\x2f\x12\x16\x08\xc7\xd1\xc9"
        "\xe4\x05\x12\x0e\x54\x65\x73\x74\x44\x65\x76\x69\x63\x65\x4e\x61\x6d\x65\x22\x32\x08\x20"
        "\x12\x20\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13"
        "\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x18\x9a\xac\xa8\xe4\x05\x22\x06\x76\x39"
        "\x2e\x30\x2e\x30";
    assert_int_equal(len, sizeof(expected_encoded) - 1);
    assert_memory_equal(encoded, expected_encoded, len);
}

// TODO: test that repeated backup should override existing one

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_backup_create, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_calculate_checksum, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_check, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_check_fail, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_fixture, test_setup, test_teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
