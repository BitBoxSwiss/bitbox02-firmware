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

#ifndef _BACKUP_COMMON_H_
#define _BACKUP_COMMON_H_

#include <backup.pb.h>

typedef enum {
    BACKUP_OK,
    // the backup was successfully created, but the old
    // backup couldn't be deleted for some reason.
    BACKUP_STALE,
    BACKUP_SEED_INACCESSIBLE,
    BACKUP_ERR_ENCODE,
    BACKUP_ERR_SD_LIST,
    BACKUP_ERR_SD_READ,
    BACKUP_ERR_SD_WRITE,
    BACKUP_ERR_CHECK,
} backup_error_t;

/**
 * Data used during encode.
 */
typedef struct encode_data {
    BackupData* backup_data;
    BackupMode* mode;
} encode_data_t;

/**
 * enum to string conversion
 */
const char* backup_error_str(backup_error_t err);

void backup_cleanup_backup(Backup* backup);
void backup_cleanup_backup_data(BackupData* backup_data);

/**
 * Calculates the checksum of the timestamp, mode and backup data.
 * The checksum is used to verify the integrity of the backup during restore.
 * @param[in] content The backup content.
 * @param[out] hash The SHA256 hash.
 */
void backup_calculate_checksum(BackupContent* content, BackupData* backup_data, uint8_t* hash);

/**
 * Fills the backup structure with backup data.
 * @param[in] generator a string identifying the creator of the backup, e.g. the firmware version.
 * @param[in] backup_create_timestamp The time at which the backup was created.
 * @param[in] seed_birtdate_timestamp The time at which the seed was created. It is not necessarily
 * the same as backup_create_timestamp, as a backup of the same seed can be re-created (e.g. on a
 * second microSD card).
 * @param[out] backup The backup structure filled with data.
 * @param[out] backup_data The backup data structure filled with data.
 * @param[out] encode_data Additional data required for encoding/decoding.
 */
backup_error_t backup_fill(
    const char* generator,
    uint32_t backup_create_timestamp,
    uint32_t seed_birthdate_timestamp,
    Backup* backup,
    BackupData* backup_data,
    encode_data_t* encode_data);

#endif
