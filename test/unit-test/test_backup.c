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
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <cmocka.h>

#include <backup.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <restore.h>
#include <rust/bitbox02_rust.h>
#include <util.h>

#include <FatFs/source/ff.h>
#include <assert_sd.h>
#include <sd.h>

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
    for (int i = 0; i < 3; i++) {
        will_return(__wrap_memory_get_device_name, DEVICE_NAME);
        will_return(__wrap_keystore_copy_seed, _mock_seed_length);
        will_return(__wrap_keystore_copy_seed, cast_ptr_to_largest_integral_type(seed));
    }
}

/**
 * Get a directory name for the mock seed. The directory name is the salted hash of the seed.
 * @param[out] dir_name The name of the directory.
 */
static void _get_directory_name(char* dir_name)
{
    uint8_t hmac_seed[HMAC_SHA256_LEN];
    wally_hmac_sha256(
        (const unsigned char*)"backup",
        strlens("backup"),
        _mock_seed,
        _mock_seed_length,
        hmac_seed,
        HMAC_SHA256_LEN);
    rust_util_uint8_to_hex(hmac_seed, sizeof(hmac_seed), dir_name);
}

/**
 * Returns the file name.
 * @param[out] file_name The name of the file which includes a timestamp.
 * @param[in] index The index of the backup.
 */
static void _get_file_name(char* file_name, uint8_t index)
{
    time_t local_timestamp = (time_t)(_current_timestamp);
    struct tm* local_time = localtime(&local_timestamp);
    static char local_timestring[100] = {0};
    strftime(local_timestring, sizeof(local_timestring), "%a_%Y-%m-%dT%H-%M-%SZ", local_time);

    snprintf(file_name, 257, "backup_%s_%d.bin", local_timestring, index);
}

static void _load_first_backup(Backup* backup, BackupData* backup_data)
{
    char dir_name[257];
    _get_directory_name(dir_name);

    char first_file_name[257];
    _get_file_name(first_file_name, 0);

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
    return 0;
}

static int test_teardown(void** state)
{
    reset_sd();
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

/**
 * Test Backup Create.
 */
static void test_backup_create(void** state)
{
    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);
    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);

    // assert directory name is salted hash of seed
    char dir_name[257];
    _get_directory_name(dir_name);
    assert_true(dir_exists(dir_name));

    // assert 3 files in directory, with correct name
    sd_list_t list;
    assert_true(sd_list_subdir(&list, dir_name));
    assert_int_equal(list.num_files, 3);
    sd_free_list(&list);
    for (uint8_t i = 0; i < 3; i++) {
        char file_name[257];
        _get_file_name(file_name, i);
        assert_true(file_exists(file_name, dir_name));
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

// TODO: test that repeated backup should override existing one

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_backup_create, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_calculate_checksum, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_check, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_backup_check_fail, test_setup, test_teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
