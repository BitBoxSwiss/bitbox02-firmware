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

#include "restore.h"
#include "backup_common.h"

#include <stdio.h>
#include <string.h>

#include <keystore.h>
#include <memory/memory.h>
#include <sd.h>
#include <util.h>

#include <pb_decode.h>
#include <wally_crypto.h>

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
