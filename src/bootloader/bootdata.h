// SPDX-License-Identifier: Apache-2.0

#ifndef _BOOTDATA_H_
#define _BOOTDATA_H_

#include <compiler_util.h>
#include <memory/memory_shared.h>
#include <stdbool.h>

/**
 * Checks the checksum protecting the bootdata version floors.
 *
 * @param[in] bootdata Bootdata containing the version floors and stored checksum.
 * @param[out] valid_out Whether the checksum matches; only set on success.
 * @return true if the check completed, or false if an argument is null or SHA-256 fails.
 */
USE_RESULT bool bootdata_hash_check(const boot_data_t* bootdata, bool* valid_out);

/**
 * Computes and stores the checksum protecting the bootdata version floors.
 *
 * The checksum is SHA256(signing_pubkeys_version_le32 || firmware_version_le32). The bootdata is
 * unchanged if SHA-256 fails.
 *
 * @param[in,out] bootdata Bootdata whose checksum should be updated.
 * @return true if the computed checksum was stored, otherwise false.
 */
USE_RESULT bool bootdata_hash_set(boot_data_t* bootdata);

/**
 * Reads and validates bootdata from flash, performing the one-time legacy migration if permitted.
 *
 * @param[out] bootdata_out Initialized, checksummed bootdata matching flash on success;
 *                         must not be used on failure.
 * @return true if bootdata was validated and migration is marked consumed, otherwise false.
 */
USE_RESULT bool bootdata_init(boot_data_t* bootdata_out);

/**
 * Reads initialized, checksummed bootdata with migration marked consumed. Never repairs it.
 *
 * @param[out] bootdata_out Initialized, checksummed bootdata matching flash on success;
 *                         must not be used on failure.
 * @return true if bootdata was read and validated with migration marked consumed, otherwise false.
 */
USE_RESULT bool bootdata_read(boot_data_t* bootdata_out);

/**
 * Updates the checksum and writes bootdata, verifying the complete readback.
 *
 * @param[in,out] bootdata Initialized bootdata to write; its checksum may change even on failure.
 * @return true if bootdata was written and the complete readback verified, otherwise false.
 */
USE_RESULT bool bootdata_write(boot_data_t* bootdata);

#ifdef BOOTLOADER_DEVELOPMENT
/**
 * Resets bootdata to zero version floors with a fresh checksum on development devices.
 *
 * @param[out] bootdata_out Recovered bootdata on success; must not be used on failure.
 * @return true if bootdata was reset, written, and validated, otherwise false.
 */
USE_RESULT bool bootdata_dev_recover(boot_data_t* bootdata_out);
#endif

#endif
