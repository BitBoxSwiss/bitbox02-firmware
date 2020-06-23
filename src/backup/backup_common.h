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

void backup_cleanup_backup(Backup* backup);
void backup_cleanup_backup_data(BackupData* backup_data);

/**
 * Calculates the checksum of the timestamp, mode and backup data.
 * The checksum is used to verify the integrity of the backup during restore.
 * @param[in] content The backup content.
 * @param[out] hash The SHA256 hash.
 */
void backup_calculate_checksum(BackupContent* content, BackupData* backup_data, uint8_t* hash);

#endif
