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
#include <cmocka.h>

#include <ff.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <wally_crypto.h>

#include <backup/backup.h>
#include <backup/restore.h>
#include <sd.h>
#include <util.h>
#include <version.h>

#define DEVICE_NAME "TestDeviceName"
static const uint32_t _current_timestamp = 1553098951;

// Based on _current_timestamp
static const char* _file_name_0 = "backup_Wed_2019-03-20T16-22-31Z_0.bin";
static const char* _file_name_1 = "backup_Wed_2019-03-20T16-22-31Z_1.bin";
static const char* _file_name_2 = "backup_Wed_2019-03-20T16-22-31Z_2.bin";

static const uint32_t _mock_seed_birthdate = 1552553498;
static const uint8_t _mock_seed[32] = {0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                                       11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                                       22, 23, 24, 25, 26, 27, 28, 29, 30, 31};
static const uint8_t _mock_seed_length = 32;

// based on _mock_seed.
static const char* _dir_name = "064bc03053f0d86068fd35b6ae0e0371abab9a4fa111b7f17b58f70701e1a495";

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

bool __wrap_keystore_encrypt_and_store_seed(uint8_t* seed, uint32_t length, const char* password)
{
    assert_int_equal(length, _mock_seed_length);
    assert_memory_equal(seed, _mock_seed, length);
    return true;
}

static Backup _backup;
static BackupData _backup_data;
static encode_data_t _encode_data;

static uint8_t _backup_bytes[SD_MAX_FILE_SIZE];
static size_t _num_backup_bytes;

static void _will_mock_backup_queries(const uint32_t seed_birthdate, const uint8_t* seed)
{
    will_return(__wrap_memory_get_device_name, DEVICE_NAME);
    will_return(__wrap_keystore_copy_seed, _mock_seed_length);
    will_return(__wrap_keystore_copy_seed, cast_ptr_to_largest_integral_type(seed));
}

static bool dir_exists(const char* dir)
{
    sd_list_t files __attribute__((__cleanup__(sd_free_list)));
    return sd_list(&files) && files.num_files > 0;
}

static int test_setup(void** state)
{
    assert_true(sd_format());

    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);

    assert_int_equal(
        backup_fill(
            DIGITAL_BITBOX_VERSION_SHORT,
            _current_timestamp,
            _mock_seed_birthdate,
            &_backup,
            &_backup_data,
            &_encode_data),
        BACKUP_OK);

    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);

    assert_int_equal(backup_create(_current_timestamp, _mock_seed_birthdate), BACKUP_OK);

    // assert directory name is salted hash of seed
    assert_true(dir_exists(_dir_name));

    if (!sd_load_bin(_file_name_0, _dir_name, _backup_bytes, &_num_backup_bytes)) {
        fail_msg("Failed to load backup");
        return false;
    }

    return 0;
}

static int test_teardown(void** state)
{
    return 0;
}

static void _assert_backup_correct(Backup* copy_backup, BackupData* copy_backup_data)
{
    // note: can't compare backup directly, because encode callback and argument point to a
    // different address
    assert_memory_equal(
        &_backup.backup_v1.content.checksum, &copy_backup->backup_v1.content.checksum, SHA256_LEN);
    assert_memory_equal(
        &_backup.backup_v1.content.metadata,
        &copy_backup->backup_v1.content.metadata,
        sizeof(BackupMetaData));
    assert_memory_equal(&_backup_data, copy_backup_data, sizeof(BackupData));
}

static void _create_and_store_corrupt_backup(Backup* backup, int file_index)
{
    uint8_t output[SD_MAX_FILE_SIZE];
    memset(output, 0, SD_MAX_FILE_SIZE);
    size_t output_length = backup_encode(backup, SD_MAX_FILE_SIZE, output);

    // starting at the byte 0, 1 or 2...
    for (size_t i = file_index; i < _num_backup_bytes; i = i + 3) {
        // bit-wise complement every 3rd byte to corrupt the content
        output[i] = ~output[i];
    }

    // store corrupt backup
    switch (file_index) {
    case 0:
        assert_true(
            sd_write_bin(_file_name_0, _dir_name, (const uint8_t*)output, output_length, true));
        break;
    case 1:
        assert_true(
            sd_write_bin(_file_name_1, _dir_name, (const uint8_t*)output, output_length, true));
        break;
    case 2:
        assert_true(
            sd_write_bin(_file_name_2, _dir_name, (const uint8_t*)output, output_length, true));
        break;
    default:
        fail_msg("Only index 0 - 2 allowed in backup.", file_index);
        break;
    }
}

static void _copy_backup_file(const char* from, const char* to)
{
    uint8_t buffer[SD_MAX_FILE_SIZE];
    size_t buffer_length;
    assert_true(sd_load_bin(from, _dir_name, buffer, &buffer_length));
    assert_true(sd_write_bin(to, _dir_name, buffer, buffer_length, true));
}

static bool _restore_backup_file(const char* file_name)
{
    uint8_t input[SD_MAX_FILE_SIZE];
    size_t input_length;
    if (!sd_load_bin(file_name, _dir_name, input, &input_length)) {
        return false;
    }
    Backup backup;
    BackupData backup_data;
    bool res = restore_from_buffer(input, input_length, &backup, &backup_data) == RESTORE_OK;
    if (res) {
        uint8_t hash[SHA256_LEN];
        backup_calculate_checksum(&backup.backup_v1.content, &backup_data, hash);
        res = memcmp(hash, backup.backup_v1.content.checksum, SHA256_LEN) == 0;
    }
    return res;
}

/**
 * Test if backup bytes, which were read during test_setup, can be restored successfully.
 */
static void test_restore_from_buffer(void** state)
{
    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_buffer(_backup_bytes, _num_backup_bytes, &copy_backup, &copy_backup_data),
        RESTORE_OK);

    _assert_backup_correct(&copy_backup, &copy_backup_data);
}

/**
 * Test list backups for a single seed.
 * We do another backup at a later point, but always expect exactly one item in the list
 * (corresponding to one seed).
 */
static void test_restore_list_backups_single_seed(void** state)
{
    ListBackupsResponse list_backups_response;
    assert_int_equal(restore_list_backups(&list_backups_response), RESTORE_OK);
    assert_int_equal(list_backups_response.info_count, 1);
    assert_string_equal(list_backups_response.info[0].id, _dir_name);
    assert_int_equal(list_backups_response.info[0].timestamp, _current_timestamp);
    assert_memory_equal(list_backups_response.info[0].name, DEVICE_NAME, sizeof(DEVICE_NAME));

    _will_mock_backup_queries(_mock_seed_birthdate, _mock_seed);

    // now let's make another backup
    const uint32_t newer_timestamp = _current_timestamp + 2 + 24 * 60 * 60 * 1000;
    assert_int_equal(backup_create(newer_timestamp, _mock_seed_birthdate), BACKUP_OK);

    assert_int_equal(restore_list_backups(&list_backups_response), RESTORE_OK);
    assert_int_equal(list_backups_response.info_count, 1);
    assert_string_equal(list_backups_response.info[0].id, _dir_name);
    assert_int_equal(list_backups_response.info[0].timestamp, newer_timestamp);
    assert_memory_equal(list_backups_response.info[0].name, DEVICE_NAME, sizeof(DEVICE_NAME));
}

/**
 * Test list backups for multiple seeds.
 */
static void test_restore_list_backups_multiple_seeds(void** state)
{
    ListBackupsResponse list_backups_response = {0};
    assert_int_equal(restore_list_backups(&list_backups_response), RESTORE_OK);
    assert_int_equal(list_backups_response.info_count, 1);
    assert_string_equal(list_backups_response.info[0].id, _dir_name);
    assert_int_equal(list_backups_response.info[0].timestamp, _current_timestamp);
    assert_memory_equal(list_backups_response.info[0].name, DEVICE_NAME, sizeof(DEVICE_NAME));

    const uint32_t new_seed_birthdate = _mock_seed_birthdate + 24 * 60 * 60 * 1000;
    const uint8_t new_seed[32] = {16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                                  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15};

    _will_mock_backup_queries(new_seed_birthdate, new_seed);

    // now let's make another backup
    const uint32_t newer_timestamp = _current_timestamp + 2 + 24 * 60 * 60 * 1000;
    assert_int_equal(backup_create(newer_timestamp, _mock_seed_birthdate), BACKUP_OK);

    assert_int_equal(restore_list_backups(&list_backups_response), RESTORE_OK);
    assert_int_equal(list_backups_response.info_count, 2);

    // based on new_seed
    const char* new_dir_name = "76fd7d926f970a55c1997dfe9c804e5c42f3dcd456f0096be6814b12d0da7c0a";

    int old_index, new_index;
    if (list_backups_response.info[0].timestamp == _current_timestamp) {
        old_index = 0;
        new_index = 1;
    } else {
        old_index = 1;
        new_index = 0;
    }
    assert_string_equal(list_backups_response.info[old_index].id, _dir_name);
    assert_int_equal(list_backups_response.info[old_index].timestamp, _current_timestamp);
    assert_memory_equal(
        list_backups_response.info[old_index].name, DEVICE_NAME, sizeof(DEVICE_NAME));
    assert_string_equal(list_backups_response.info[new_index].id, new_dir_name);
    assert_int_equal(list_backups_response.info[new_index].timestamp, newer_timestamp);
    assert_memory_equal(
        list_backups_response.info[new_index].name, DEVICE_NAME, sizeof(DEVICE_NAME));
}

/**
 * Test restore from directory.
 */
static void test_restore_good_backup_from_directory(void** state)
{
    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_directory(_dir_name, &copy_backup, &copy_backup_data), RESTORE_OK);
    _assert_backup_correct(&copy_backup, &copy_backup_data);
}

/**
 * Test restore one corrupt backup.
 */
static void test_restore_1_corrupt_backup_from_directory(void** state)
{
    _create_and_store_corrupt_backup(&_backup, 0);

    assert_false(_restore_backup_file(_file_name_0));

    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_directory(_dir_name, &copy_backup, &copy_backup_data), RESTORE_OK);

    assert_true(_restore_backup_file(_file_name_0));
    _assert_backup_correct(&copy_backup, &copy_backup_data);
}

/**
 * Test restore two corrupt backups.
 */
static void test_restore_2_corrupt_backups_from_directory(void** state)
{
    _create_and_store_corrupt_backup(&_backup, 0);
    _create_and_store_corrupt_backup(&_backup, 1);

    assert_false(_restore_backup_file(_file_name_0));
    assert_false(_restore_backup_file(_file_name_1));

    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_directory(_dir_name, &copy_backup, &copy_backup_data), RESTORE_OK);

    assert_true(_restore_backup_file(_file_name_0));
    assert_true(_restore_backup_file(_file_name_1));
    _assert_backup_correct(&copy_backup, &copy_backup_data);
}

/**
 * Test restore three corrupt backups.
 */
static void test_restore_3_corrupt_backups_from_directory(void** state)
{
    _create_and_store_corrupt_backup(&_backup, 0);
    _create_and_store_corrupt_backup(&_backup, 1);
    _create_and_store_corrupt_backup(&_backup, 2);

    assert_false(_restore_backup_file(_file_name_0));
    assert_false(_restore_backup_file(_file_name_1));
    assert_false(_restore_backup_file(_file_name_2));

    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_directory(_dir_name, &copy_backup, &copy_backup_data), RESTORE_OK);

    assert_true(_restore_backup_file(_file_name_0));
    assert_true(_restore_backup_file(_file_name_1));
    _assert_backup_correct(&copy_backup, &copy_backup_data);
}

/**
 * Test fail to restore when majority vote isn't possible.
 */
static void test_fail_restore_if_majority_vote_fails(void** state)
{
    _create_and_store_corrupt_backup(&_backup, 0);
    _create_and_store_corrupt_backup(&_backup, 1);
    _copy_backup_file(_file_name_0, _file_name_2);

    assert_false(_restore_backup_file(_file_name_0));
    assert_false(_restore_backup_file(_file_name_1));
    assert_false(_restore_backup_file(_file_name_2));

    Backup copy_backup;
    BackupData copy_backup_data;
    assert_int_equal(
        restore_from_directory(_dir_name, &copy_backup, &copy_backup_data), RESTORE_ERR_DECODE);
}

/**
 * Test restore seed.
 */
static void test_restore_seed(void** state)
{
    assert_true(restore_seed(&_backup_data, "secret"));
}

// TODO: test that repeated backup should override existing one

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_restore_from_buffer, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_list_backups_single_seed, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_list_backups_multiple_seeds, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_good_backup_from_directory, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_1_corrupt_backup_from_directory, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_2_corrupt_backups_from_directory, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_restore_3_corrupt_backups_from_directory, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(
            test_fail_restore_if_majority_vote_fails, test_setup, test_teardown),
        cmocka_unit_test_setup_teardown(test_restore_seed, test_setup, test_teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
