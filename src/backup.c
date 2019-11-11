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
#include <string.h>
#include <time.h>

#include <backup.h>
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <restore.h>
#include <sd.h>
#include <util.h>
#include <version.h>

const char* backup_error_str(backup_error_t err)
{
    switch (err) {
    case BACKUP_OK:
        return "OK";
    case BACKUP_STALE:
        return "STALE";
    case BACKUP_SEED_INACCESSIBLE:
        return "SEED_INACCESSIBLE";
    case BACKUP_ERR_ENCODE:
        return "ENCODE";
    case BACKUP_ERR_SD_LIST:
        return "SD_LIST";
    case BACKUP_ERR_SD_READ:
        return "SD_READ";
    case BACKUP_ERR_SD_WRITE:
        return "SD_WRITE";
    case BACKUP_ERR_CHECK:
        return "CHECK";
    default:
        return "UNKNOWN";
    }
}

/**
 * Data used during encode.
 */
typedef struct encode_data {
    BackupData* backup_data;
    BackupMode* mode;
} encode_data_t;

void backup_cleanup_backup(Backup* backup)
{
    util_zero(backup, sizeof(Backup));
}

void backup_cleanup_backup_data(BackupData* backup_data)
{
    util_zero(backup_data, sizeof(BackupData));
}

static void _cleanup_backup_bytes(uint8_t** backup_bytes)
{
    util_zero(*backup_bytes, SD_MAX_FILE_SIZE);
}

/**
 * Calculates the checksum of the timestamp, mode and backup data.
 * The checksum is used to verify the integrity of the backup during restore.
 * @param[in] content The backup content.
 * @param[out] hash The SHA256 hash.
 */
void backup_calculate_checksum(
    BackupContent* content,
    BackupData* backup_data,
    uint8_t hash[SHA256_LEN])
{
    // size = all fields in backup data, all fields in backup meta data and the length
    const uint16_t size = sizeof(uint32_t) + sizeof(BackupMode) + sizeof(content->metadata.name) +
                          sizeof(uint32_t) + sizeof(backup_data->seed) + sizeof(uint32_t) +
                          sizeof(backup_data->generator) + sizeof(uint32_t);
    uint16_t start = 0;
    uint8_t bytes[size];
    memcpy(bytes + start, &content->metadata.timestamp, sizeof(uint32_t));
    start += sizeof(uint32_t);
    memcpy(bytes + start, &content->metadata.mode, sizeof(BackupMode));
    start += sizeof(BackupMode);
    memcpy(bytes + start, &content->metadata.name, sizeof(content->metadata.name));
    start += sizeof(content->metadata.name);
    memcpy(bytes + start, &backup_data->seed_length, sizeof(uint32_t));
    start += sizeof(uint32_t);
    memcpy(bytes + start, backup_data->seed, sizeof(backup_data->seed));
    start += sizeof(backup_data->seed);
    memcpy(bytes + start, &backup_data->birthdate, sizeof(uint32_t));
    start += sizeof(uint32_t);
    memcpy(bytes + start, backup_data->generator, sizeof(backup_data->generator));
    start += sizeof(backup_data->generator);
    memcpy(bytes + start, &content->length, sizeof(uint32_t));
    start += sizeof(uint32_t);
    if (size != start) {
        // Just a hint for future developers:
        Abort("Backup Format changed! Check backup_calculate_checksum.");
    }
    wally_sha256(bytes, start, hash, SHA256_LEN);
    util_zero(bytes, start);
}

/**
 * Encodes the backup and returns the number of bytes written, or 0 if encoding failed.
 * @return the number of bytes written, or 0 if encoding failed.
 */
static size_t _encode_backup(Backup* backup, uint32_t max_size, uint8_t* output)
{
    pb_ostream_t out_stream = pb_ostream_from_buffer(output, (unsigned int)max_size);
    bool status_encode = pb_encode(&out_stream, Backup_fields, backup);
    if (!status_encode) {
        return 0;
    }
    return out_stream.bytes_written;
}

/**
 * NanoPB callback to encode the backup data.
 * @param[out] ostream The outgoing stream.
 * @param[in] field The field that is encoded.
 * @param[in] arg The encode/decode data passed to the callback.
 */
static bool _encode_backup_data(pb_ostream_t* ostream, const pb_field_t* field, void* const* arg)
{
    (void)field;
    encode_data_t* encode_data = (encode_data_t*)*arg;
    if (*(encode_data->mode) != BackupMode_PLAINTEXT) {
        return false;
    }
    /* This encodes the header for the field, based on the constant info
     * from pb_field_t. */
    if (!pb_encode_tag_for_field(ostream, field)) {
        return false;
    }
    /* This encodes the data for the field, based on our BackupData structure. */
    if (!pb_encode_submessage(ostream, BackupData_fields, encode_data->backup_data)) {
        return false;
    }
    return true;
}

/**
 * Fills the backup structure with backup data.
 * @param[in] backup_create_timestamp The time at which the backup was created.
 * @param[out] backup The backup structure filled with data.
 * @param[out] backup_data The backup data structure filled with data.
 * @param[out] encode_data Additional data required for encoding/decoding.
 */
static backup_error_t _fill_backup(
    uint32_t backup_create_timestamp,
    uint32_t seed_birthdate_timestamp,
    Backup* backup,
    BackupData* backup_data,
    encode_data_t* encode_data)
{
    BackupV1* backup_v1 = &backup->backup_v1;
    for (int i = 0; i < 3; i++) {
        BackupContent* backup_content = &backup_v1->content;
        BackupMetaData* backup_metadata = &backup_content->metadata;
        backup_metadata->timestamp = backup_create_timestamp;
        backup_metadata->mode = BackupMode_PLAINTEXT;
        if (sizeof(backup_metadata->name) < MEMORY_DEVICE_NAME_MAX_LEN) {
            Abort("Not enough room for device name");
        }
        util_zero(backup_metadata->name, sizeof(backup_metadata->name));
        memory_get_device_name(backup_metadata->name);
        memset(backup_data, 0, sizeof(BackupData));
        const char* firmware_v = DIGITAL_BITBOX_VERSION_SHORT;
        snprintf(backup_data->generator, sizeof(backup_data->generator), "%s", firmware_v);

        backup_data->birthdate = seed_birthdate_timestamp;

        if (!keystore_copy_seed(backup_data->seed, &backup_data->seed_length)) {
            return BACKUP_SEED_INACCESSIBLE;
        }
        encode_data->backup_data = backup_data;
        encode_data->mode = &backup_metadata->mode;

        uint8_t submessage_output[SD_MAX_FILE_SIZE];
        pb_ostream_t submessage_out_stream =
            pb_ostream_from_buffer(submessage_output, (unsigned int)SD_MAX_FILE_SIZE);
        _encode_backup_data(&submessage_out_stream, BackupData_fields, (void* const*)&encode_data);

        backup_content->length = submessage_out_stream.bytes_written;

        backup_content->data.arg = encode_data;
        backup_content->data.funcs.encode = &_encode_backup_data;
        backup_calculate_checksum(backup_content, backup_data, backup_content->checksum);
        util_zero(submessage_output, SD_MAX_FILE_SIZE);
    }
    return BACKUP_OK;
}

/**
 * Get a directory name for the given seed. The directory name is the hash of the seed.
 * @param[in] seed The seed to be backuped.
 * @param[out] dir_name The name of the directory and must be 65 bytes (32 byte hex string + null
 * terminator)..
 */
static void _get_directory_name(const uint8_t seed[32], char* dir_name)
{
    uint8_t hmac_seed[HMAC_SHA256_LEN];
    wally_hmac_sha256(
        (const unsigned char*)"backup", strlens("backup"), seed, 32, hmac_seed, HMAC_SHA256_LEN);
    util_uint8_to_hex(hmac_seed, sizeof(hmac_seed), dir_name);
}

/**
 * Prepares the file name and writes it to file_name.
 * @param[in] backup_create_timestamp The create timestamp from which we create the timestamp.
 * @param[out] file_name The name of the file which includes a timestamp.
 * @param[in] index The index of the backup.
 */
static void _get_file_name(uint32_t backup_create_timestamp, char* file_name, uint8_t index)
{
    time_t local_timestamp = (time_t)backup_create_timestamp;
    struct tm* local_time = localtime(&local_timestamp);
    static char local_timestring[100] = {0};
    strftime(local_timestring, sizeof(local_timestring), "%a_%Y-%m-%dT%H-%M-%SZ", local_time);

    snprintf(file_name, 257, "backup_%s_%d.bin", local_timestring, index);
}

#define CLEANUP_BACKUP_BYTES(var)                                            \
    uint8_t* __attribute__((__cleanup__(_cleanup_backup_bytes))) var##_clean \
        __attribute__((unused)) = var;

/**
 * Checks whether the restore from backup was successful.
 * Compares the backup restored from the given buffer with the passed backup.
 * @return BACKUP_OK if the backup was good, BACKUP_ERR_CHECK if the check failed.
 */
static backup_error_t _check_backup(uint8_t* output, size_t output_length, const Backup* backup)
{
    Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup_check;
    BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data_check;
    restore_error_t res =
        restore_from_buffer(output, output_length, &backup_check, &backup_data_check);
    if (res != RESTORE_OK) {
        return BACKUP_ERR_CHECK;
    }
    if (!MEMEQ(backup_check.backup_v1.content.checksum, backup->backup_v1.content.checksum, 32)) {
        return BACKUP_ERR_CHECK;
    }
    return BACKUP_OK;
}

/**
 * Creates a backup using the given timestamp.
 */
backup_error_t backup_create(uint32_t backup_create_timestamp, uint32_t seed_birthdate_timestamp)
{
    Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup;
    BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data;
    encode_data_t encode_data;
    backup_error_t res = _fill_backup(
        backup_create_timestamp, seed_birthdate_timestamp, &backup, &backup_data, &encode_data);
    if (res != BACKUP_OK) {
        return res;
    }
    uint8_t output[SD_MAX_FILE_SIZE];
    CLEANUP_BACKUP_BYTES(output);
    size_t output_length = _encode_backup(&backup, SD_MAX_FILE_SIZE, output);
    if (output_length == 0) {
        return BACKUP_ERR_ENCODE;
    }

    if (_check_backup(output, output_length, &backup) != BACKUP_OK) {
        return BACKUP_ERR_CHECK;
    }

    char dir_name[65];
    _get_directory_name(backup_data.seed, dir_name);

    sd_list_t files __attribute__((__cleanup__(sd_free_list)));
    if (!sd_list_subdir(&files, dir_name)) {
        return BACKUP_ERR_SD_LIST;
    }

    for (int i = 0; i < 3; i++) {
        char file_name[257];
        _get_file_name(backup_create_timestamp, file_name, i);

        if (!sd_write_bin(file_name, dir_name, (const uint8_t*)output, output_length, true)) {
            return BACKUP_ERR_SD_WRITE;
        }
        // If the backup could not be decoded successfully, we should make sure
        // that the previous backup (if any) isn't erased and that we return
        // BACKUP_ERR_CHECK.
        // The caller could try again.
        uint8_t read_content[SD_MAX_FILE_SIZE];
        size_t read_length;
        if (!sd_load_bin(file_name, dir_name, read_content, &read_length)) {
            return BACKUP_ERR_SD_READ;
        }
        if (_check_backup(read_content, read_length, &backup) != BACKUP_OK) {
            return BACKUP_ERR_CHECK;
        }
    }
    bool is_stale = false;
    for (size_t j = 0; j < files.num_files; j++) {
        if (!sd_erase_file_in_subdir(files.files[j], dir_name)) {
            is_stale = true;
        }
    }
    if (is_stale) {
        return BACKUP_STALE;
    }
    return BACKUP_OK;
}

backup_error_t backup_check(char* id_out, char* name_out, uint32_t* birthdate_out)
{
    Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup;
    BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data;
    encode_data_t encode_data;
    backup_error_t backup_res = _fill_backup(0, 0, &backup, &backup_data, &encode_data);
    if (backup_res != BACKUP_OK) {
        return backup_res;
    }
    char* dir_name = id_out;
    _get_directory_name(backup_data.seed, dir_name);

    Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup_copy;
    BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data_copy;
    restore_error_t restore_res = restore_from_directory(dir_name, &backup_copy, &backup_data_copy);
    if (restore_res != RESTORE_OK) {
        return BACKUP_ERR_CHECK;
    }

    if (!MEMEQ(backup_data.seed, backup_data_copy.seed, 32) ||
        backup_data.seed_length != backup_data_copy.seed_length) {
        return BACKUP_ERR_CHECK;
    }
    if (name_out != NULL) {
        snprintf(
            name_out,
            MEMORY_DEVICE_NAME_MAX_LEN,
            "%s",
            backup_copy.backup_v1.content.metadata.name);
    }
    if (birthdate_out != NULL) {
        *birthdate_out = backup_data_copy.birthdate;
    }
    return BACKUP_OK;
}
