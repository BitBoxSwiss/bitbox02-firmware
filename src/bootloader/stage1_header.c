// SPDX-License-Identifier: Apache-2.0

#include "bootloader_upgrade/bootloader_upgrade.h"
#include <bootloader/bootloader_version.h>

_Static_assert(
    BOOTLOADER_VERSION_LEN <= BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN,
    "stage1 marketing version too long for stage1 header");

#ifdef BOOTLOADER_DEVDEVICE
    #define BB02_STAGE1_HEADER_FLAGS BB02_STAGE1_FLAG_DEVELOPMENT
#else
    #define BB02_STAGE1_HEADER_FLAGS 0u
#endif

const bb02_stage1_header_t bb02_stage1_header_placeholder
    __attribute__((used, section(".stage1_header"), aligned(4))) = {
        .magic = BB02_STAGE1_HEADER_MAGIC,
        .flags = BB02_STAGE1_HEADER_FLAGS,
        .header_version = BB02_STAGE1_HEADER_FORMAT_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .header_len = BB02_STAGE1_HEADER_LEN,
        .image_len = 0,
        .monotonic_version = 1,
        .stage1_marketing_version_len = BOOTLOADER_VERSION_LEN,
        .stage1_marketing_version = BOOTLOADER_VERSION,
        .reserved = {0},
        .signatures = {{0}},
};
