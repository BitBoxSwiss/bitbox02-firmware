// SPDX-License-Identifier: Apache-2.0

#include "bootdata.h"

#include <memory/memory.h>
#include <memory/nvmctrl.h>
#include <pukcc/pukcc.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <util.h>

#define VERSION_LE32_LEN 4U
#define VERSIONS_PREIMAGE_LEN (2U * VERSION_LE32_LEN)

#define MIGRATION_CONSUMED_CHUNK_ADDR \
    (FLASH_APPDATA_START + MEMORY_BOOTDATA_MIGRATION_CONSUMED_CHUNK * CHUNK_SIZE)
static_assert(FLASH_BOOTDATA_LEN == CHUNK_SIZE, "bootdata must occupy one erase block");
static_assert(
    MIGRATION_CONSUMED_CHUNK_ADDR + CHUNK_SIZE <= FLASH_BOOTDATA_START,
    "migration consumed marker must be independent");
static_assert(
    MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN <=
        CHUNK_SIZE,
    "migration consumed marker must fit inside its chunk");

static const uint8_t _migration_consumed_magic[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN] =
    MEMORY_BOOTDATA_MIGRATION_CONSUMED_MAGIC;

USE_RESULT static bool _compute_versions_hash(
    const boot_data_t* bootdata,
    uint8_t hash_out[SHA256_DIGEST_LENGTH])
{
    uint8_t preimage[VERSIONS_PREIMAGE_LEN];
    util_write_le32(preimage, bootdata->fields.signing_pubkeys_version);
    util_write_le32(preimage + VERSION_LE32_LEN, bootdata->fields.firmware_version);

    return pukcc_sha256_compute(preimage, sizeof(preimage), hash_out) == 0;
}

bool bootdata_hash_check(const boot_data_t* bootdata, bool* valid_out)
{
    if (bootdata == NULL || valid_out == NULL) {
        return false;
    }
    uint8_t expected[SHA256_DIGEST_LENGTH];
    if (!_compute_versions_hash(bootdata, expected)) {
        return false;
    }
    *valid_out = MEMEQ(expected, bootdata->fields.versions_hash, sizeof(expected));
    return true;
}

bool bootdata_hash_set(boot_data_t* bootdata)
{
    if (bootdata == NULL) {
        return false;
    }

    uint8_t hash[SHA256_DIGEST_LENGTH];
    if (!_compute_versions_hash(bootdata, hash)) {
        return false;
    }
    memcpy(bootdata->fields.versions_hash, hash, sizeof(hash));
    return true;
}

USE_RESULT static bool _write_verify_chunk(uint32_t address, const uint8_t* data)
{
    if (!nvmctrl_write_block(address, data)) {
        return false;
    }
    uint8_t readback[CHUNK_SIZE];
    return nvmctrl_read_block_checked(address, readback) && MEMEQ(readback, data, CHUNK_SIZE);
}

USE_RESULT static bool _initialized(const boot_data_t* bootdata)
{
    return bootdata->fields.is_initialized[0] == sectrue_u8 &&
           bootdata->fields.is_initialized[1] == sectrue_u8;
}

/**
 * The caller must supply initialized data and ensure migration is durably marked consumed.
 * The RAM checksum and flash may have changed even on failure.
 */
USE_RESULT static bool _write_bootdata(boot_data_t* bootdata)
{
    if (!bootdata_hash_set(bootdata) ||
        !nvmctrl_write_block(FLASH_BOOTDATA_START, bootdata->bytes)) {
        return false;
    }
    boot_data_t readback;
    bool hash_valid;
    return nvmctrl_read_block_checked(FLASH_BOOTDATA_START, readback.bytes) &&
           MEMEQ(readback.bytes, bootdata->bytes, sizeof(readback.bytes)) &&
           _initialized(&readback) && bootdata_hash_check(&readback, &hash_valid) && hash_valid;
}

USE_RESULT static bool _read_migration_consumed(
    uint8_t migration_consumed_out[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN])
{
    const volatile uint8_t* flash =
        (const volatile uint8_t*)(uintptr_t)(MIGRATION_CONSUMED_CHUNK_ADDR +
                                             MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET);
    uint8_t confirm[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN];
    return nvmctrl_read(flash, migration_consumed_out, sizeof(confirm)) &&
           nvmctrl_read(flash, confirm, sizeof(confirm)) &&
           MEMEQ(migration_consumed_out, confirm, sizeof(confirm));
}

/**
 * Writes the consumed magic, preserving and verifying the entire PERMANENT chunk. Returns false
 * if a read fails, the reread marker already matches, or the checked write/readback fails.
 * Never modifies bootdata; migration may write it only after success.
 * On failure, the nonmatching marker might already be marked consumed, so device might not boot.
 */
USE_RESULT static bool _mark_migration_consumed(void)
{
    uint8_t chunk[CHUNK_SIZE];
    if (!nvmctrl_read_block_checked(MIGRATION_CONSUMED_CHUNK_ADDR, chunk) ||
        MEMEQ(
            chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic))) {
        return false;
    }
    memcpy(
        chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        _migration_consumed_magic,
        sizeof(_migration_consumed_magic));
    // Preserve the existing attestation hash and all other bytes. A cut during this write cannot
    // affect bootdata: it is not touched until the whole marker chunk has been verified.
    return _write_verify_chunk(MIGRATION_CONSUMED_CHUNK_ADDR, chunk);
}

bool bootdata_init(boot_data_t* bootdata_out)
{
    if (bootdata_out == NULL ||
        !nvmctrl_read_block_checked(FLASH_BOOTDATA_START, bootdata_out->bytes) ||
        !_initialized(bootdata_out)) {
        return false;
    }
    bool hash_valid;
    uint8_t migration_consumed[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN];
    if (!bootdata_hash_check(bootdata_out, &hash_valid) ||
        !_read_migration_consumed(migration_consumed)) {
        return false;
    }
    if (MEMEQ(migration_consumed, _migration_consumed_magic, sizeof(migration_consumed))) {
        // In particular, zero must never reopen migration after a torn bootdata write.
        return hash_valid;
    }
    if (!_mark_migration_consumed()) {
        return false;
    }
    if (hash_valid) {
        // A cut while rewriting the marker chunk may leave a nonmatching marker. Restore it before
        // allowing any bootdata write, even when no checksum migration is needed.
        return true;
    }
    // Trust the inherited legacy state once, regardless of the old checksum bytes. A marker that
    // does not match the magic permits migration. This does not bypass normal authentication before
    // running firmware or accepting an update, and also works with an incomplete carrier image.
    return _write_bootdata(bootdata_out);
}

bool bootdata_read(boot_data_t* bootdata_out)
{
    if (bootdata_out == NULL) {
        return false;
    }
    bool hash_valid;
    if (nvmctrl_read_block_checked(FLASH_BOOTDATA_START, bootdata_out->bytes) &&
        _initialized(bootdata_out) && bootdata_hash_check(bootdata_out, &hash_valid) &&
        hash_valid) {
        uint8_t migration_consumed[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN];
        return _read_migration_consumed(migration_consumed) &&
               MEMEQ(migration_consumed, _migration_consumed_magic, sizeof(migration_consumed));
    }
    return false;
}

#ifdef BOOTLOADER_DEVELOPMENT
bool bootdata_dev_recover(boot_data_t* bootdata_out)
{
    uint8_t migration_consumed[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN];
    if (bootdata_out == NULL || !_read_migration_consumed(migration_consumed) ||
        (!MEMEQ(migration_consumed, _migration_consumed_magic, sizeof(migration_consumed)) &&
         !_mark_migration_consumed())) {
        return false;
    }
    // Development devices must remain usable after corrupt/erased bootdata, including uncorrectable
    // ECC/other NVM read errors. Reset both floors rather than sealing inflated versions that would
    // prevent reflashing. Never clear the consumed marker to reopen migration; preserve the
    // PERMANENT chunk.
    memset(bootdata_out->bytes, 0, sizeof(bootdata_out->bytes));
    memset(
        bootdata_out->fields.is_initialized,
        sectrue_u8,
        sizeof(bootdata_out->fields.is_initialized));
    return _write_bootdata(bootdata_out) && bootdata_read(bootdata_out);
}
#endif

bool bootdata_write(boot_data_t* bootdata)
{
    boot_data_t current;
    if (bootdata == NULL || !_initialized(bootdata) || !bootdata_read(&current)) {
        return false;
    }
    return _write_bootdata(bootdata);
}
