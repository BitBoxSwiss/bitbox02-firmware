#ifndef __BOOTLOADER_DATA_H
#define __BOOTLOADER_DATA_H

#include <stdint.h>

#include <flags.h>

#define BOOT_NUM_FIRMWARE_SIGNING_KEYS 3u
#define BOOT_NUM_ROOT_SIGNING_KEYS 3u
#define BOOT_FIRMWARE_SIG_M 2u
#define BOOT_ROOT_SIG_M 2u
#define BOOT_PUBKEY_LEN 64u
#define BOOT_SIG_LEN 64u

// Packed to make the layout more explicit.
// Total size equals min erase granularity
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"

typedef uint32_t version_t;
typedef union {
    struct __attribute__((__packed__)) {
        uint16_t hardware_version;
        uint8_t is_initialized[2];
        version_t signing_pubkeys_version;
        uint8_t signing_pubkeys
            [BOOT_PUBKEY_LEN *
             BOOT_NUM_FIRMWARE_SIGNING_KEYS]; // Keep after signing_pubkeys_version
        uint8_t root_signatures_of_signing_pubkeys[BOOT_SIG_LEN * BOOT_NUM_ROOT_SIGNING_KEYS];
        version_t firmware_version;
        uint8_t
            firmware_signatures[BOOT_SIG_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS]; // Keep after
                                                                                // firmware_version
        uint8_t show_firmware_hash;
    } fields;
    uint8_t bytes[FLASH_BOOTDATA_LEN];
} boot_data_t;

typedef union {
    // If changed, also need to change memory.c
    struct __attribute__((__packed__)) {
        uint8_t auto_enter;
        uint8_t upside_down;
    } fields;
    uint8_t bytes[FLASH_SHARED_DATA_LEN];
} shared_data_t;
#pragma GCC diagnostic pop

#endif // __BOOTLOADER_DATA_H
