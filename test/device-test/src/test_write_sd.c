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

#include <backup.pb.h>
#include <driver_init.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <screen.h>
#include <sd.h>
#include <string.h>
#include <ui/screen_stack.h>
#include <usb/usb.h>

#include "qtouch.h"
#include "random.h"
#include "util.h"
#include <hardfault.h>

#include "wally_crypto.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

typedef struct encode_decode_data {
    BackupData* backup_data;
    BackupMode* mode;
} encode_decode_data_t;

static void _calculate_checksum(
    uint32_t* timestamp,
    uint8_t* mode,
    uint32_t* length,
    uint8_t* data,
    uint8_t hash[SHA256_LEN])
{
    const uint16_t bytes_size = 4 + 1 + 4 + *length;
    uint8_t bytes[bytes_size];
    memcpy(bytes, timestamp, 4);
    memcpy(bytes + 4, mode, 1);
    memcpy(bytes + 5, length, 4);
    memcpy(bytes + 9, data, *length);
    wally_sha256(bytes, bytes_size, hash, SHA256_LEN);
}

static bool _decode_backup_data(pb_istream_t* istream, const pb_field_t* field, void** arg)
{
    (void)field;
    encode_decode_data_t* encode_decode_data = (encode_decode_data_t*)*arg;
    if (*(encode_decode_data->mode) != BackupMode_PLAINTEXT) {
        screen_sprintf_debug(
            1000, "(decode) mode is not plaintext %d", *(encode_decode_data->mode));
        return false;
    }
    if (!pb_decode(istream, BackupData_fields, encode_decode_data->backup_data)) return false;
    return true;
}

static bool _encode_backup_data(pb_ostream_t* ostream, const pb_field_t* field, void* const* arg)
{
    (void)field;
    encode_decode_data_t* encode_decode_data = (encode_decode_data_t*)*arg;
    if (*(encode_decode_data->mode) != BackupMode_PLAINTEXT) {
        screen_sprintf_debug(1000, "mode is not plaintext %d", *(encode_decode_data->mode));
        return false;
    }
    /* This encodes the header for the field, based on the constant info
     * from pb_field_t. */
    if (!pb_encode_tag_for_field(ostream, field)) return false;
    /* This encodes the data for the field, based on our BackupData structure. */
    if (!pb_encode_submessage(ostream, BackupData_fields, encode_decode_data->backup_data))
        return false;
    return true;
}

static void _fill_backup(
    Backup* backup,
    BackupData* backup_data,
    encode_decode_data_t* encode_decode_data)
{
    BackupV1* backup_v1 = &backup->backup_v1;
    BackupContent* backup_content = &backup_v1->content;
    BackupMetaData* backup_metadata = &backup_content->metadata;
    backup_metadata->timestamp = 666;
    backup_metadata->mode = BackupMode_PLAINTEXT;
    memset(backup_data, 0, sizeof(BackupData));
    const char* firmware_v = "firmware v1.0.0";
    memcpy(backup_data->generator, firmware_v, strlens(firmware_v) + 1);

    encode_decode_data->backup_data = backup_data;
    encode_decode_data->mode = &backup_metadata->mode;

    uint8_t submessage_output[SD_MAX_FILE_SIZE];
    pb_ostream_t submessage_out_stream =
        pb_ostream_from_buffer(submessage_output, (unsigned int)SD_MAX_FILE_SIZE);
    _encode_backup_data(
        &submessage_out_stream, BackupData_fields, (void* const*)&encode_decode_data);

    backup_content->length = submessage_out_stream.bytes_written;

    backup_content->data.arg = encode_decode_data;
    backup_content->data.funcs.encode = &_encode_backup_data;
    _calculate_checksum(
        &backup_metadata->timestamp,
        &backup_metadata->mode,
        &backup_content->length,
        submessage_output,
        backup_content->checksum);
}

static size_t _encode_backup(Backup* backup, uint32_t max_size, uint8_t* output)
{
    pb_ostream_t out_stream = pb_ostream_from_buffer(output, (unsigned int)max_size);
    bool status_encode = pb_encode(&out_stream, Backup_fields, backup);
    if (!status_encode) {
        screen_sprintf_debug(1000, "Failed to encode: %s", PB_GET_ERROR(&out_stream));
        while (1)
            ;
    }
    return out_stream.bytes_written;
}

static void _decode_backup(uint8_t* input, uint32_t length, Backup* backup, BackupData backup_data)
{
    BackupContent* backup_content = &backup->backup_v1.content;
    encode_decode_data_t encode_decode_data;
    encode_decode_data.mode = &backup_content->metadata.mode;
    encode_decode_data.backup_data = &backup_data;
    backup->backup_v1.content.data.arg = &encode_decode_data;
    backup->backup_v1.content.data.funcs.decode = &_decode_backup_data;

    pb_istream_t in_stream = pb_istream_from_buffer(input, length);
    bool status = pb_decode(&in_stream, Backup_fields, backup);
    if (!status) {
        screen_sprintf_debug(1000, "Failed to decode: %s", PB_GET_ERROR(&in_stream));
        while (1)
            ;
    }
}

static void _get_directory_name(uint8_t seed[32], char* hmac_seed_hex)
{
    uint8_t hmac_seed[HMAC_SHA256_LEN];
    wally_hmac_sha256(
        (const unsigned char*)"backup", strlens("backup"), seed, 32, hmac_seed, HMAC_SHA256_LEN);
    util_uint8_to_hex(hmac_seed, sizeof(hmac_seed), hmac_seed_hex);
}

static bool _check_hash(Backup* backup, BackupData* backup_data)
{
    BackupContent backup_content = backup->backup_v1.content;
    BackupMetaData backup_metadata = backup_content.metadata;
    encode_decode_data_t encode_decode_data;
    encode_decode_data.backup_data = backup_data;
    encode_decode_data.mode = &backup_metadata.mode;
    encode_decode_data_t* encode_decode_data_ptr = &encode_decode_data;

    uint8_t submessage_output[SD_MAX_FILE_SIZE];
    pb_ostream_t submessage_out_stream =
        pb_ostream_from_buffer(submessage_output, (unsigned int)SD_MAX_FILE_SIZE);
    _encode_backup_data(
        &submessage_out_stream, BackupData_fields, (void* const*)&encode_decode_data_ptr);

    if (backup_content.length != submessage_out_stream.bytes_written) {
        screen_sprintf_debug(
            0,
            "check hash length mismatch %ld != %ld",
            backup_content.length,
            submessage_out_stream.bytes_written);
        while (1)
            ;
    }

    uint8_t hash[SHA256_LEN];
    _calculate_checksum(
        &backup_metadata.timestamp,
        &backup_metadata.mode,
        &backup_content.length,
        submessage_output,
        hash);

    return memcmp(backup_content.checksum, hash, SHA256_LEN) == 0;
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();

    uint8_t seed[32] = {6};

    Backup backup;
    BackupData backup_data;
    encode_decode_data_t encode_decode_data;
    _fill_backup(&backup, &backup_data, &encode_decode_data);
    uint8_t output[SD_MAX_FILE_SIZE];
    size_t output_length = _encode_backup(&backup, SD_MAX_FILE_SIZE, output);

    char hmac_seed_hex[65];
    _get_directory_name(seed, hmac_seed_hex);

    sd_list_t files;
    if (!sd_list_subdir(&files, hmac_seed_hex)) {
        Abort("Failed to list files");
    }
    char to_delete[257];
    if (files.num_files > 0) {
        snprintf(to_delete, 257, ".%s.bkp", files.files[0]);
        sd_file_rename(files.files[0], to_delete, hmac_seed_hex);
    }
    const char* filename = "backup_Thu_2019-03-07_18-26.bin";
    if (!sd_write_bin(filename, hmac_seed_hex, (const uint8_t*)output, output_length, true)) {
        if (files.num_files > 0) {
            sd_file_rename(to_delete, files.files[0], hmac_seed_hex);
        }
        Abort("Write failed");
    }
    if (files.num_files > 0) {
        sd_erase_file_in_subdir(to_delete, hmac_seed_hex);
    }
    sd_free_list(&files);

    // restore...
    uint8_t read_content[SD_MAX_FILE_SIZE];
    uint32_t length;
    sd_load_bin(filename, hmac_seed_hex, read_content, &length);

    BackupData parsed_backup_data;
    Backup parsed_backup;

    _decode_backup(read_content, length, &parsed_backup, parsed_backup_data);
    screen_sprintf_debug(1000, "content: %ld", parsed_backup.backup_v1.content.metadata.timestamp);
    screen_sprintf_debug(1000, "generator: %s", parsed_backup_data.generator);

    if (!_check_hash(&parsed_backup, &parsed_backup_data)) {
        screen_sprintf_debug(1000, "checksum doesn't match");
    } else {
        screen_sprintf_debug(1000, "checksum matches. Test complete");
    }
}

#pragma GCC diagnostic pop
