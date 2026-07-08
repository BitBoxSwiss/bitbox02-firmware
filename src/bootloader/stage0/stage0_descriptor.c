// SPDX-License-Identifier: Apache-2.0

#include "stage0_descriptor.h"
#include "bootloader_upgrade/bootloader_upgrade.h"

#ifdef BB02_STAGE0_DEVELOPMENT
    #define BB02_STAGE0_DESCRIPTOR_FLAGS BB02_STAGE0_FLAG_DEVELOPMENT
#else
    #define BB02_STAGE0_DESCRIPTOR_FLAGS 0u
#endif

const bb02_stage0_descriptor_t bb02_stage0_descriptor
    __attribute__((used, section(".stage0_descriptor"), aligned(4))) = {
        .stage0_version = BB02_STAGE0_IMAGE_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .flags = BB02_STAGE0_DESCRIPTOR_FLAGS,
        .magic = BB02_STAGE0_DESCRIPTOR_MAGIC,
};
