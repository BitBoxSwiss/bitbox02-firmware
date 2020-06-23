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

#ifndef _BACKUP_BACKUP_H_
#define _BACKUP_BACKUP_H_

#include <time.h>

#include <backup.pb.h>
#include <wally_crypto.h>

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
 * enum to string conversion
 */

const char* backup_error_str(backup_error_t err);

void backup_cleanup_backup(Backup* backup);
void backup_cleanup_backup_data(BackupData* backup_data);

backup_error_t backup_create(uint32_t backup_create_timestamp, uint32_t seed_birthdate_timestamp);

/**
 * id_out must have max 256 bytes in size; hww.options BackupInfo.id
 * @param[out] name_out must have max MEMORY_DEVICE_NAME_MAX_LEN (64) bytes in size;
 // hww.options BackupInfo.name; can be NULL.
 * @param[out] birthdate_out can be NULL.
 */
backup_error_t backup_check(char* id_out, char* name_out, uint32_t* birthdate_out);

void backup_calculate_checksum(
    BackupContent* content,
    BackupData* backup_data,
    uint8_t hash[SHA256_LEN]);

#endif
