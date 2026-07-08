// SPDX-License-Identifier: Apache-2.0

#ifndef _STAGE0_FLASH_H_
#define _STAGE0_FLASH_H_

#include "bootloader_upgrade/bootloader_upgrade.h"
#include "util.h"

#include <stddef.h>
#include <stdint.h>
#include <string.h>

#define STAGE0_FLASH_PAGE_SIZE_BYTES (512U)
#define STAGE0_FLASH_PAGE_WORDS (STAGE0_FLASH_PAGE_SIZE_BYTES / sizeof(uint32_t))
#define STAGE0_FLASH_BLOCK_SIZE_BYTES (8192U)
#define STAGE0_FLASH_REGION_SIZE_BYTES (32768U)
#define STAGE0_STAGE1_PAGE_ADDR BB02_STAGE1_ADDR
#define STAGE0_STAGE1_BLOCK_ADDR BB02_STAGE1_ADDR
#define STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR \
    (BB02_STAGE1_FACTORY_RANDOM_ADDR & ~(STAGE0_FLASH_PAGE_SIZE_BYTES - 1U))
#define STAGE0_STAGE1_FACTORY_RANDOM_BLOCK_ADDR \
    (BB02_STAGE1_FACTORY_RANDOM_ADDR & ~(STAGE0_FLASH_BLOCK_SIZE_BYTES - 1U))

#define STAGE0_FACTORY_RANDOM_BACKUP_MAGIC (0x30524642U) // "BFR0" in little-endian flash order.
#define STAGE0_FACTORY_RANDOM_BACKUP_FORMAT_VERSION (1U)
#define STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_ERASED UINT32_MAX
#define STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_WRITTEN (0U)

typedef struct {
    uint32_t magic;
    uint32_t format_version;
    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint32_t commit;
} stage0_factory_random_backup_t;

typedef enum {
    STAGE0_FACTORY_RANDOM_SOURCE_CURRENT,
    STAGE0_FACTORY_RANDOM_SOURCE_BACKUP,
} stage0_factory_random_source_t;

_Static_assert(
    STAGE0_STAGE1_PAGE_ADDR % STAGE0_FLASH_PAGE_SIZE_BYTES == 0,
    "stage1 header page is unaligned");
_Static_assert(
    BB02_STAGE1_HEADER_LEN % STAGE0_FLASH_PAGE_SIZE_BYTES == 0,
    "stage1 header must occupy whole flash pages");
_Static_assert(
    STAGE0_STAGE1_BLOCK_ADDR % STAGE0_FLASH_BLOCK_SIZE_BYTES == 0,
    "stage1 block is unaligned");
_Static_assert(
    BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR % STAGE0_FLASH_PAGE_SIZE_BYTES == 0,
    "factory randomness backup page is unaligned");
_Static_assert(
    BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR % STAGE0_FLASH_BLOCK_SIZE_BYTES == 0,
    "factory randomness backup block is unaligned");
_Static_assert(
    BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_LEN == STAGE0_FLASH_BLOCK_SIZE_BYTES,
    "factory randomness backup must occupy one erase block");
_Static_assert(
    sizeof(stage0_factory_random_backup_t) <= STAGE0_FLASH_PAGE_SIZE_BYTES,
    "factory randomness backup record does not fit in one page");
_Static_assert(
    STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR + STAGE0_FLASH_PAGE_SIZE_BYTES ==
        BB02_STAGE1_FACTORY_RANDOM_ADDR + BB02_STAGE1_FACTORY_RANDOM_LEN,
    "factory randomness must be at the end of its flash page");
_Static_assert(
    STAGE0_STAGE1_FACTORY_RANDOM_BLOCK_ADDR + STAGE0_FLASH_BLOCK_SIZE_BYTES ==
        BB02_STAGE1_FACTORY_RANDOM_ADDR + BB02_STAGE1_FACTORY_RANDOM_LEN,
    "factory randomness must be at the end of its flash block");

// GCC needs noclone; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
static secbool_u32 __attribute__((noinline, noclone)) stage0_flash_page_addr_ok(
    volatile uint32_t addr)
{
    if ((addr % STAGE0_FLASH_PAGE_SIZE_BYTES) != 0) {
        return secfalse_u32;
    }
    if (addr >= STAGE0_STAGE1_PAGE_ADDR && addr <= STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR) {
        return sectrue_u32;
    }
    if (addr == BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR) {
        return sectrue_u32;
    }
    return secfalse_u32;
}

// GCC needs noclone; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
static secbool_u32 __attribute__((noinline, noclone)) stage0_flash_block_addr_ok(
    volatile uint32_t addr)
{
    if ((addr % STAGE0_FLASH_BLOCK_SIZE_BYTES) != 0) {
        return secfalse_u32;
    }
    if (addr >= STAGE0_STAGE1_BLOCK_ADDR && addr <= STAGE0_STAGE1_FACTORY_RANDOM_BLOCK_ADDR) {
        return sectrue_u32;
    }
    if (addr == BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR) {
        return sectrue_u32;
    }
    return secfalse_u32;
}

static void stage0_flash_make_invalid_header_page(uint32_t page_words[STAGE0_FLASH_PAGE_WORDS])
{
    // Flash programming can only clear bits. Keep all other words at 0xff so
    // they are left unchanged, and clear only the magic word to invalidate.
    memset(page_words, 0xff, STAGE0_FLASH_PAGE_SIZE_BYTES);
    page_words[0] = 0;
}

static bool stage0_flash_page_contains_addr(uint32_t page_addr, uint32_t addr)
{
    return page_addr <= addr && page_addr + STAGE0_FLASH_PAGE_SIZE_BYTES > addr;
}

static void stage0_flash_make_stage1_page(
    uint32_t page_words[STAGE0_FLASH_PAGE_WORDS],
    const uint8_t* image,
    uint32_t image_len,
    uint32_t dst_addr,
    const uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN])
{
    memset(page_words, 0xff, STAGE0_FLASH_PAGE_SIZE_BYTES);
    if (dst_addr >= BB02_STAGE1_ADDR) {
        const uint32_t image_offset = dst_addr - BB02_STAGE1_ADDR;
        if (image_offset < image_len) {
            uint32_t copy_len = image_len - image_offset;
            if (copy_len > STAGE0_FLASH_PAGE_SIZE_BYTES) {
                copy_len = STAGE0_FLASH_PAGE_SIZE_BYTES;
            }
            memcpy(page_words, image + image_offset, copy_len);
        }
    }
    if (factory_random != NULL &&
        stage0_flash_page_contains_addr(dst_addr, BB02_STAGE1_FACTORY_RANDOM_ADDR)) {
        memcpy(
            ((uint8_t*)page_words) + (BB02_STAGE1_FACTORY_RANDOM_ADDR - dst_addr),
            factory_random,
            BB02_STAGE1_FACTORY_RANDOM_LEN);
    }
}

static void stage0_factory_random_backup_make_data_page(
    uint32_t page_words[STAGE0_FLASH_PAGE_WORDS],
    const uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN])
{
    memset(page_words, 0xff, STAGE0_FLASH_PAGE_SIZE_BYTES);
    stage0_factory_random_backup_t* backup = (stage0_factory_random_backup_t*)page_words;
    backup->magic = STAGE0_FACTORY_RANDOM_BACKUP_MAGIC;
    backup->format_version = STAGE0_FACTORY_RANDOM_BACKUP_FORMAT_VERSION;
    memcpy(backup->factory_random, factory_random, BB02_STAGE1_FACTORY_RANDOM_LEN);
    backup->commit = STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_ERASED;
}

static void stage0_factory_random_backup_make_commit_page(
    uint32_t page_words[STAGE0_FLASH_PAGE_WORDS])
{
    memset(page_words, 0xff, STAGE0_FLASH_PAGE_SIZE_BYTES);
    stage0_factory_random_backup_t* backup = (stage0_factory_random_backup_t*)page_words;
    backup->commit = STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_WRITTEN;
}

static secbool_u32 stage0_factory_random_backup_valid(
    const stage0_factory_random_backup_t* backup,
    uint8_t factory_random_out[BB02_STAGE1_FACTORY_RANDOM_LEN])
{
    if (backup->magic != STAGE0_FACTORY_RANDOM_BACKUP_MAGIC ||
        backup->format_version != STAGE0_FACTORY_RANDOM_BACKUP_FORMAT_VERSION ||
        backup->commit != STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_WRITTEN) {
        return secfalse_u32;
    }

    if (factory_random_out != NULL) {
        memcpy(factory_random_out, backup->factory_random, BB02_STAGE1_FACTORY_RANDOM_LEN);
    }
    return sectrue_u32;
}

static stage0_factory_random_source_t stage0_factory_random_source(
    secbool_u32 installed_header_ok,
    secbool_u32 backup_valid,
    secbool_u32 current_matches_backup)
{
    if (backup_valid == sectrue_u32) {
        if (installed_header_ok != sectrue_u32 || current_matches_backup == sectrue_u32) {
            return STAGE0_FACTORY_RANDOM_SOURCE_BACKUP;
        }
    }
    return STAGE0_FACTORY_RANDOM_SOURCE_CURRENT;
}

#endif
