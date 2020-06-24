#include "backup_common.h"

#include <hardfault.h>
#include <util.h>

#include <wally_crypto.h>

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
