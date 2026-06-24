// SPDX-License-Identifier: Apache-2.0

#ifndef _BOOTLOADER_UPGRADE_H_
#define _BOOTLOADER_UPGRADE_H_

#include <bootloader/bootloader_product.h>
#include <stddef.h>
#include <stdint.h>

#define BB02_BOOTLOADER_UPGRADE_STAGE0_ADDR (0x00000000U)
#define BB02_BOOTLOADER_UPGRADE_STAGE0_LEN (0x00002000U)

#define BB02_STAGE1_ADDR (0x00002000U)
#define BB02_STAGE1_HEADER_ADDR BB02_STAGE1_ADDR
#define BB02_STAGE1_HEADER_ALIGNMENT (1024U)
#define BB02_STAGE1_VECTOR_OFFSET (0x00000400U)
#define BB02_STAGE1_VECTOR_ADDR (BB02_STAGE1_ADDR + BB02_STAGE1_VECTOR_OFFSET)
#define BB02_STAGE1_MAX_LEN (0x0000BFE0U)

#define BB02_STAGE1_FACTORY_RANDOM_ADDR (0x0000DFE0U)
#define BB02_STAGE1_FACTORY_RANDOM_LEN (32U)

#define BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR (0x000D8000U)
#define BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_LEN (0x00002000U)
#define BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR

#define BB02_BOOTLOADER_UPGRADE_STAGE1_UPDATE_ADDR (0x000DB000U)

#define BB02_STAGE1_HEADER_MAGIC (0x31534242U) // "BBS1" in little-endian flash order.
#define BB02_STAGE1_HEADER_FORMAT_VERSION (1U)
#define BB02_STAGE1_ROOT_KEY_COUNT (3U)
#define BB02_STAGE1_SIGNATURE_THRESHOLD (2U)
#define BB02_STAGE1_SIGNATURE_LEN (64U)
#define BB02_STAGE1_HEADER_SIGNATURES_LEN (BB02_STAGE1_ROOT_KEY_COUNT * BB02_STAGE1_SIGNATURE_LEN)
#define BB02_STAGE1_SIGNED_DIGEST_LEN (32U)
#define BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN (37U)
#define BB02_STAGE1_FLAG_DEVELOPMENT (1U << 0)
#define BB02_STAGE1_HEADER_RESERVED_LEN (768U)

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"
typedef struct __attribute__((__packed__)) {
    uint32_t magic;
    uint32_t flags;
    uint16_t header_version;
    uint16_t product_id;
    // Total header length in bytes. Must be at least this struct size, at most
    // BB02_STAGE1_MAX_LEN, and a multiple of BB02_STAGE1_HEADER_ALIGNMENT.
    // image_len must be greater than it. Stage1 vectors start at this offset.
    uint32_t header_len;
    // Total stage1 image length, including this header.
    uint64_t image_len;
    uint16_t monotonic_version;
    uint8_t stage1_marketing_version_len;
    uint8_t stage1_marketing_version[BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN];
    uint8_t reserved[BB02_STAGE1_HEADER_RESERVED_LEN];
    // Signatures must remain the last 64*3 bytes of the header. Stage0 derives
    // their location from header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN.
    uint8_t signatures[BB02_STAGE1_ROOT_KEY_COUNT][BB02_STAGE1_SIGNATURE_LEN];
} bb02_stage1_header_t;
#pragma GCC diagnostic pop

#define BB02_STAGE1_HEADER_LEN ((uint32_t)sizeof(bb02_stage1_header_t))
#define BB02_STAGE1_HEADER_SIGNED_LEN ((uint32_t)offsetof(bb02_stage1_header_t, signatures))

_Static_assert(sizeof(bb02_stage1_header_t) == 1024, "header ABI changed");
_Static_assert(
    BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR + BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_LEN <=
        BB02_BOOTLOADER_UPGRADE_STAGE1_UPDATE_ADDR,
    "factory randomness backup overlaps stage1 update slot");
_Static_assert(
    BB02_STAGE1_HEADER_SIGNED_LEN + BB02_STAGE1_HEADER_SIGNATURES_LEN == BB02_STAGE1_HEADER_LEN,
    "header ABI changed");
_Static_assert(BB02_STAGE1_VECTOR_OFFSET == 1024U, "stage1 vector table offset changed");
_Static_assert(
    BB02_STAGE1_VECTOR_OFFSET == BB02_STAGE1_HEADER_LEN,
    "stage1 header must fill the space before vector table");
_Static_assert(
    (BB02_STAGE1_VECTOR_OFFSET % BB02_STAGE1_HEADER_ALIGNMENT) == 0,
    "stage1 vector table must be 1kB aligned");
_Static_assert(BB02_STAGE1_HEADER_SIGNED_LEN == 832U, "signed header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, magic) == 0U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, flags) == 4U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, header_version) == 8U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, product_id) == 10U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, header_len) == 12U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, image_len) == 16U, "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, monotonic_version) == 24U, "header ABI changed");
_Static_assert(
    offsetof(bb02_stage1_header_t, stage1_marketing_version_len) == 26U,
    "header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, reserved) == 64U, "header ABI changed");
_Static_assert(
    offsetof(bb02_stage1_header_t, signatures) == BB02_STAGE1_HEADER_SIGNED_LEN,
    "signed header ABI changed");

static inline const bb02_stage1_header_t* bb02_stage1_update_header(void)
{
    return (const bb02_stage1_header_t*)BB02_BOOTLOADER_UPGRADE_STAGE1_UPDATE_ADDR;
}

static inline const bb02_stage1_header_t* bb02_stage1_installed_header(void)
{
    return (const bb02_stage1_header_t*)BB02_STAGE1_HEADER_ADDR;
}

#endif
