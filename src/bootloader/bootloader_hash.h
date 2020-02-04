#ifndef __BOOTLOADER_HASH_H
#define __BOOTLOADER_HASH_H

#include <util.h>

#include "bootloader_data.h"

void bootloader_hash_firmware(const boot_data_t* data, uint8_t* hash_out);

secbool_u32 bootloader_hash_pubkeys_verified(const boot_data_t* data);

void bootloader_hash_signing_keys(const boot_data_t* data, uint8_t* hash_out);

/**
 * Checks if the given signature buffer is empty (i.e. contains all zeros).
 *
 * @param sig Buffer to check. Must be BOOT_SIG_LEN bytes long.
 */
bool bootloader_hash_is_empty_sig(const uint8_t* sig);

#endif // __BOOTLOADER_HASH_H
