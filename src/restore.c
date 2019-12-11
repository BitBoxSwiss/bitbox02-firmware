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

#include <stdio.h>

#include "restore.h"

#include <backup.h>
#include <keystore.h>
#include <memory/memory.h>
#include <sd.h>
#include <util.h>

#include <pb_decode.h>
#include <pb_encode.h>

/**
 * Data used during decode.
 */
typedef struct decode_data {
    BackupData* backup_data;
    BackupMode* mode;
} decode_data_t;

typedef struct {
    const char* filename;
    uint8_t content[SD_MAX_FILE_SIZE];
    uint32_t file_length;
    bool loaded;
} broken_backup_t;

typedef struct {
    uint8_t count;
    broken_backup_t backups[3];
} broken_backups_t;

typedef struct {
    uint8_t content[SD_MAX_FILE_SIZE];
    uint32_t file_length;
    Backup* backup;
    BackupData* backup_data;
} good_backup_t;

static bool _decode_backup_data(pb_istream_t* istream, const pb_field_t* field, void** arg)
{
    (void)field;
    decode_data_t* decode_data = (decode_data_t*)*arg;
    if (*(decode_data->mode) != BackupMode_PLAINTEXT) {
        return false;
    }
    if (!pb_decode(istream, BackupData_fields, decode_data->backup_data)) {
        return false;
    }
    return true;
}

restore_error_t restore_from_buffer(
    uint8_t* input,
    uint32_t length,
    Backup* backup,
    BackupData* backup_data)
{
    decode_data_t decode_data;
    BackupContent* backup_content = &backup->backup_v1.content;
    decode_data.mode = &backup_content->metadata.mode;
    decode_data.backup_data = backup_data;
    backup->backup_v1.content.data.arg = &decode_data;
    backup->backup_v1.content.data.funcs.decode = &_decode_backup_data;

    pb_istream_t in_stream = pb_istream_from_buffer(input, length);
    bool status = pb_decode(&in_stream, Backup_fields, backup);
    if (!status) {
        return RESTORE_ERR_DECODE;
    }
    return RESTORE_OK;
}

static bool _check_integrity(BackupContent* content, BackupData* data)
{
    uint8_t hash[SHA256_LEN];
    backup_calculate_checksum(content, data, hash);
    return MEMEQ(content->checksum, hash, SHA256_LEN);
}

/**
 * Attempts to error-correct a backup by performing a majority vote over the bits of
 * the 3 backup copies. If, for example, bit 15 of the first backup buffer is 1, but it is
 * 0 of the second and third backup buffers, we flip the bit of the first backup buffer and
 * continue until all backup buffers are equal. Afterwards, we try to restore the backup and
 * perform an integrity check to validate if the error correction was successful.
 * @return RESTORE_OK if the error correction was successful, RESTORE_ERR_DECODE if we couldn't
 * decode, and RESTORE_ERR_CHECK if the integrity check failed.
 */
static restore_error_t _recover_defective_contents(
    broken_backup_t broken_backups[3],
    good_backup_t* good_backup)
{
    uint32_t assumed_file_length = broken_backups[0].file_length;
    for (int i = 1; i < 3; i++) {
        if (assumed_file_length != broken_backups[i].file_length) {
            // Fail to recover, because we do not know how long the file is supposed to be.
            return RESTORE_ERR_RECOVER;
        }
    }

    for (uint32_t i = 0; i < assumed_file_length; i++) {
        for (unsigned int b = 0; b < 8; b++) {
            unsigned int bit_f0 = (broken_backups[0].content[i] >> b) & 1U;
            unsigned int bit_f1 = (broken_backups[1].content[i] >> b) & 1U;
            unsigned int bit_f2 = (broken_backups[2].content[i] >> b) & 1U;

            if (bit_f0 != bit_f1 && bit_f0 != bit_f2) {
                broken_backups[0].content[i] ^= 1U << b;
            } else if (bit_f0 != bit_f1) {
                broken_backups[1].content[i] ^= 1U << b;
            } else if (bit_f0 != bit_f2) {
                broken_backups[2].content[i] ^= 1U << b;
            }
        }
    }
    restore_error_t res = restore_from_buffer(
        broken_backups[0].content,
        assumed_file_length,
        good_backup->backup,
        good_backup->backup_data);
    if (res == RESTORE_OK) {
        BackupContent content = good_backup->backup->backup_v1.content;
        if (_check_integrity(&content, good_backup->backup_data)) {
            memcpy(good_backup->content, broken_backups[0].content, assumed_file_length);
            good_backup->file_length = assumed_file_length;
            return RESTORE_OK;
        }
        return RESTORE_ERR_CHECK;
    }
    return res;
}

/**
 * Attempts to load a backup under the given sub-directory and fills the good_backup and/or
 * broken_backups structures with relevant data.
 * @param[in] files The names of the backup files under the given directory.
 * @param[in] dir The name of the directory.
 * @param[out] good_backup Contains the bytes of the good backup, as well as pointers to the backups
 * and backup data, or is left empty if restoring failed.
 * @param[out] broken_backups Contains the bytes of the bad backup or is left empty if restoring was
 * successful.
 * @return true if at least one backup file could be read and its integrity was successfully
 * checked, false if either no backups could be read or all backups are corrupt.
 */
static bool _load_good_and_bad_backup_contents(
    char** files,
    const char* dir,
    good_backup_t* good_backup,
    broken_backups_t* broken_backups)
{
    for (int i = 0; i < 3; i++) {
        const char* filename = files[i];
        uint8_t input[SD_MAX_FILE_SIZE];
        size_t input_length = 0;
        if (sd_load_bin(filename, dir, input, &input_length)) {
            restore_error_t res = restore_from_buffer(
                input, input_length, good_backup->backup, good_backup->backup_data);
            if (res == RESTORE_OK) {
                BackupContent content = good_backup->backup->backup_v1.content;
                if (_check_integrity(&content, good_backup->backup_data)) {
                    memcpy(good_backup->content, input, input_length);
                    good_backup->file_length = input_length;
                    return true;
                }
            }
            // If we get here, we either failed to restore or the integrity check failed.
            memcpy(&broken_backups->backups[i].content, input, input_length);
            broken_backups->backups[i].loaded = true;
            broken_backups->backups[i].file_length = input_length;
        } else {
            // If we get here, we failed to read the SD card's content.
            broken_backups->backups[i].loaded = false;
        }
        broken_backups->backups[i].filename = filename;
        broken_backups->count++;
    }
    return false;
}

restore_error_t restore_from_directory(const char* dir, Backup* backup, BackupData* backup_data)
{
    sd_list_t list_backups __attribute__((__cleanup__(sd_free_list)));
    if (!sd_list_subdir(&list_backups, dir)) {
        return RESTORE_ERR_SD_READ;
    }
    // sanity check.
    if (list_backups.num_files != 3) {
        return RESTORE_ERR_RECOVER;
    }
    broken_backups_t broken_backups;
    memset(&broken_backups, 0, sizeof(broken_backups));

    good_backup_t good_backup;
    memset(&good_backup, 0, sizeof(good_backup));
    memset(backup, 0, sizeof(Backup));
    memset(backup_data, 0, sizeof(BackupData));
    good_backup.backup = backup;
    good_backup.backup_data = backup_data;

    if (!_load_good_and_bad_backup_contents(
            list_backups.files, dir, &good_backup, &broken_backups)) {
        bool all_loaded = true;
        for (int i = 0; i < 3; i++) {
            all_loaded &= broken_backups.backups[i].loaded;
        }
        if (!all_loaded) {
            return RESTORE_ERR_SD_READ;
        }
        // load successful. but content is corrupt
        restore_error_t res = _recover_defective_contents(broken_backups.backups, &good_backup);
        if (res != RESTORE_OK) {
            return res;
        }
    }

    // at this point we either read a good content or recovered it
    for (int i = 0; i < broken_backups.count; i++) {
        if (!sd_write_bin(
                broken_backups.backups[i].filename,
                dir,
                good_backup.content,
                good_backup.file_length,
                true)) {
            return RESTORE_ERR_SD_WRITE;
        }
    }
    // if we found a file that matches the checksum, overwrite corrupt files.
    // do majority-vote error correction if all files are corrupt. Read file bit by bit and update
    // the bits that are diverging.
    return RESTORE_OK;
}

restore_error_t restore_list_backups(ListBackupsResponse* backups)
{
    sd_list_t list_subdirs __attribute__((__cleanup__(sd_free_list)));
    if (!sd_list(&list_subdirs)) {
        return RESTORE_ERR_SD_LIST;
    }
    backups->info_count = 0;
    for (size_t i = 0; i < list_subdirs.num_files; i++) {
        if (backups->info_count >= sizeof(backups->info) / sizeof(backups->info[0])) {
            return RESTORE_TOO_MANY;
        }
        char* dir = list_subdirs.files[i];
        Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup;
        BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data;
        if (restore_from_directory(dir, &backup, &backup_data) != RESTORE_OK) {
            continue;
        }
        backups->info[backups->info_count].timestamp = backup.backup_v1.content.metadata.timestamp;
        size_t id_size = sizeof(backups->info[backups->info_count].id);
        int snprintf_result = snprintf(backups->info[backups->info_count].id, id_size, "%s", dir);
        if (snprintf_result < 0 || snprintf_result >= (int)id_size) {
            return RESTORE_ERR_SD_LIST;
        }
        size_t name_size = sizeof(backups->info[backups->info_count].name);
        snprintf_result = snprintf(
            backups->info[backups->info_count].name,
            name_size,
            "%s",
            backup.backup_v1.content.metadata.name);
        if (snprintf_result < 0 || snprintf_result >= (int)name_size) {
            return RESTORE_ERR_SD_LIST;
        }
        backups->info_count++;
    }

    return RESTORE_OK;
}

bool restore_seed(const BackupData* backup_data, const char* password)
{
    bool res =
        keystore_encrypt_and_store_seed(backup_data->seed, backup_data->seed_length, password);

    if (res) {
        if (!memory_set_seed_birthdate(backup_data->birthdate)) {
            // Ignore error here. Missing birthdate should not abort an otherwise successful
            // restore.
        }
    }
    return res;
}
