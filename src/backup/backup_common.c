#include "backup_common.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <sd.h>
#include <util.h>
#include <version.h>

#include <pb_encode.h>
#include <wally_crypto.h>

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

void backup_cleanup_backup(Backup* backup)
{
    util_zero(backup, sizeof(Backup));
}

void backup_cleanup_backup_data(BackupData* backup_data)
{
    util_zero(backup_data, sizeof(BackupData));
}

void backup_calculate_checksum(BackupContent* content, BackupData* backup_data, uint8_t* hash)
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

backup_error_t backup_fill(
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
