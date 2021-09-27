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

#ifndef _BACKUP_RESTORE_H_
#define _BACKUP_RESTORE_H_

#include <backup.pb.h>
#include <hww.pb.h>

typedef enum {
    RESTORE_OK,
    RESTORE_TOO_MANY,
    RESTORE_ERR_DECODE,
    RESTORE_ERR_SD_LIST,
    RESTORE_ERR_SD_READ,
    RESTORE_ERR_SD_WRITE,
    RESTORE_ERR_RECOVER,
    RESTORE_ERR_CHECK,
} restore_error_t;

restore_error_t restore_from_buffer(
    uint8_t* input,
    uint32_t length,
    Backup* backup,
    BackupData* backup_data);

/**
 * Returns a list of backup information to the caller.
 */
restore_error_t restore_list_backups(ListBackupsResponse* backups);

/**
 * Restore a backup from directory.
 * @param[in] dir The directory from which we want to restore the backup.
 * @param[out] backup_data The restored backup_data.
 * @return RESTORE_OK if the error correction was successful, RESTORE_ERR_DECODE if we couldn't
 * decode, RESTORE_ERR_CHECK if the integrity check failed, and RESTORE_ERR_RECOVER if we couldn't
 * recover for other reasons.
 */
restore_error_t restore_from_directory(const char* dir, Backup* backup, BackupData* backup_data);

#endif
