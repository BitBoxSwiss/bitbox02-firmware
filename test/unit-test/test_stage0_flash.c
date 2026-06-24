// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include "bootloader/stage0/stage0_flash.h"

static void test_stage0_flash_page_addr_ok(void** state)
{
    (void)state;

    assert_int_equal(stage0_flash_page_addr_ok(STAGE0_STAGE1_PAGE_ADDR), sectrue_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(STAGE0_STAGE1_PAGE_ADDR + STAGE0_FLASH_PAGE_SIZE_BYTES),
        sectrue_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR), sectrue_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR), sectrue_u32);

    assert_int_equal(stage0_flash_page_addr_ok(STAGE0_STAGE1_PAGE_ADDR - 1u), secfalse_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(STAGE0_STAGE1_PAGE_ADDR - STAGE0_FLASH_PAGE_SIZE_BYTES),
        secfalse_u32);
    assert_int_equal(stage0_flash_page_addr_ok(STAGE0_STAGE1_PAGE_ADDR + 1u), secfalse_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(
            STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR + STAGE0_FLASH_PAGE_SIZE_BYTES),
        secfalse_u32);
    assert_int_equal(
        stage0_flash_page_addr_ok(
            BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR + STAGE0_FLASH_PAGE_SIZE_BYTES),
        secfalse_u32);
}

static void test_stage0_flash_block_addr_ok(void** state)
{
    (void)state;

    assert_int_equal(stage0_flash_block_addr_ok(STAGE0_STAGE1_BLOCK_ADDR), sectrue_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(STAGE0_STAGE1_BLOCK_ADDR + STAGE0_FLASH_BLOCK_SIZE_BYTES),
        sectrue_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(STAGE0_STAGE1_FACTORY_RANDOM_BLOCK_ADDR), sectrue_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR), sectrue_u32);

    assert_int_equal(stage0_flash_block_addr_ok(STAGE0_STAGE1_BLOCK_ADDR - 1u), secfalse_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(STAGE0_STAGE1_BLOCK_ADDR - STAGE0_FLASH_BLOCK_SIZE_BYTES),
        secfalse_u32);
    assert_int_equal(stage0_flash_block_addr_ok(STAGE0_STAGE1_BLOCK_ADDR + 1u), secfalse_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(
            STAGE0_STAGE1_FACTORY_RANDOM_BLOCK_ADDR + STAGE0_FLASH_BLOCK_SIZE_BYTES),
        secfalse_u32);
    assert_int_equal(
        stage0_flash_block_addr_ok(
            BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR + STAGE0_FLASH_BLOCK_SIZE_BYTES),
        secfalse_u32);
}

static void test_stage0_flash_make_invalid_header_page(void** state)
{
    (void)state;

    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    stage0_flash_make_invalid_header_page(page);

    assert_int_equal(page[0], 0);
    for (uint32_t i = 1; i < STAGE0_FLASH_PAGE_WORDS; i++) {
        assert_int_equal(page[i], UINT32_MAX);
    }
}

static void _program_page(
    uint32_t dst[STAGE0_FLASH_PAGE_WORDS],
    uint32_t src[STAGE0_FLASH_PAGE_WORDS])
{
    for (uint32_t i = 0; i < STAGE0_FLASH_PAGE_WORDS; i++) {
        dst[i] &= src[i];
    }
}

static void _factory_random(uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN])
{
    for (uint32_t i = 0; i < BB02_STAGE1_FACTORY_RANDOM_LEN; i++) {
        factory_random[i] = (uint8_t)(0xa5u ^ i);
    }
}

static void test_stage0_flash_make_stage1_header_page(void** state)
{
    (void)state;

    uint8_t image[STAGE0_FLASH_PAGE_SIZE_BYTES];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    for (uint32_t i = 0; i < sizeof(image); i++) {
        image[i] = (uint8_t)i;
    }

    stage0_flash_make_stage1_page(page, image, sizeof(image), BB02_STAGE1_HEADER_ADDR, NULL);

    assert_memory_equal(page, image, sizeof(image));
}

static void test_stage0_flash_make_stage1_second_header_page(void** state)
{
    (void)state;

    uint8_t image[BB02_STAGE1_HEADER_LEN];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    for (uint32_t i = 0; i < sizeof(image); i++) {
        image[i] = (uint8_t)i;
    }

    stage0_flash_make_stage1_page(
        page, image, sizeof(image), BB02_STAGE1_HEADER_ADDR + STAGE0_FLASH_PAGE_SIZE_BYTES, NULL);

    assert_memory_equal(page, image + STAGE0_FLASH_PAGE_SIZE_BYTES, STAGE0_FLASH_PAGE_SIZE_BYTES);
}

static void test_stage0_flash_make_stage1_factory_random_page(void** state)
{
    (void)state;

    static uint8_t image[BB02_STAGE1_MAX_LEN];
    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    const uint32_t image_offset = STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR - BB02_STAGE1_ADDR;
    const uint32_t random_page_offset =
        BB02_STAGE1_FACTORY_RANDOM_ADDR - STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR;

    memset(image, 0x42, sizeof(image));
    for (uint32_t i = 0; i < random_page_offset; i++) {
        image[image_offset + i] = (uint8_t)i;
    }
    _factory_random(factory_random);

    stage0_flash_make_stage1_page(
        page, image, BB02_STAGE1_MAX_LEN, STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR, factory_random);

    assert_memory_equal(page, &image[image_offset], random_page_offset);
    assert_memory_equal(
        ((uint8_t*)page) + random_page_offset, factory_random, sizeof(factory_random));
}

static void test_stage0_flash_make_stage1_factory_random_page_without_image_bytes(void** state)
{
    (void)state;

    uint8_t image[STAGE0_FLASH_PAGE_SIZE_BYTES];
    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    const uint32_t image_offset = STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR - BB02_STAGE1_ADDR;
    const uint32_t random_page_offset =
        BB02_STAGE1_FACTORY_RANDOM_ADDR - STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR;

    memset(image, 0x42, sizeof(image));
    _factory_random(factory_random);

    stage0_flash_make_stage1_page(
        page, image, image_offset, STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR, factory_random);

    for (uint32_t i = 0; i < random_page_offset; i++) {
        assert_int_equal(((uint8_t*)page)[i], 0xff);
    }
    assert_memory_equal(
        ((uint8_t*)page) + random_page_offset, factory_random, sizeof(factory_random));
}

static void test_factory_random_source_policy(void** state)
{
    (void)state;

    assert_int_equal(
        stage0_factory_random_source(sectrue_u32, sectrue_u32, sectrue_u32),
        STAGE0_FACTORY_RANDOM_SOURCE_BACKUP);
    assert_int_equal(
        stage0_factory_random_source(sectrue_u32, sectrue_u32, secfalse_u32),
        STAGE0_FACTORY_RANDOM_SOURCE_CURRENT);
    assert_int_equal(
        stage0_factory_random_source(secfalse_u32, sectrue_u32, secfalse_u32),
        STAGE0_FACTORY_RANDOM_SOURCE_BACKUP);
    assert_int_equal(
        stage0_factory_random_source(sectrue_u32, secfalse_u32, secfalse_u32),
        STAGE0_FACTORY_RANDOM_SOURCE_CURRENT);
}

static void test_factory_random_backup_valid_record(void** state)
{
    (void)state;

    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint8_t factory_random_out[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint32_t flash[STAGE0_FLASH_PAGE_WORDS];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];

    _factory_random(factory_random);
    memset(flash, 0xff, sizeof(flash));
    stage0_factory_random_backup_make_data_page(page, factory_random);
    _program_page(flash, page);
    stage0_factory_random_backup_make_commit_page(page);
    _program_page(flash, page);

    assert_int_equal(
        stage0_factory_random_backup_valid(
            (const stage0_factory_random_backup_t*)flash, factory_random_out),
        sectrue_u32);
    assert_memory_equal(factory_random_out, factory_random, sizeof(factory_random));
}

static void test_factory_random_backup_erased_page_invalid(void** state)
{
    (void)state;

    uint32_t flash[STAGE0_FLASH_PAGE_WORDS];
    memset(flash, 0xff, sizeof(flash));

    assert_int_equal(
        stage0_factory_random_backup_valid((const stage0_factory_random_backup_t*)flash, NULL),
        secfalse_u32);
}

static void test_factory_random_backup_missing_commit_invalid(void** state)
{
    (void)state;

    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];

    _factory_random(factory_random);
    stage0_factory_random_backup_make_data_page(page, factory_random);

    assert_int_equal(
        stage0_factory_random_backup_valid((const stage0_factory_random_backup_t*)page, NULL),
        secfalse_u32);
    assert_int_equal(
        ((const stage0_factory_random_backup_t*)page)->commit,
        STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_ERASED);
}

static void test_factory_random_backup_commit_page_only_clears_commit(void** state)
{
    (void)state;

    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    stage0_factory_random_backup_make_commit_page(page);

    const stage0_factory_random_backup_t* backup = (const stage0_factory_random_backup_t*)page;
    assert_int_equal(backup->commit, STAGE0_FACTORY_RANDOM_BACKUP_COMMIT_WRITTEN);

    const uint32_t commit_word =
        (uint32_t)(offsetof(stage0_factory_random_backup_t, commit) / sizeof(uint32_t));
    for (uint32_t i = 0; i < STAGE0_FLASH_PAGE_WORDS; i++) {
        if (i == commit_word) {
            continue;
        }
        assert_int_equal(page[i], UINT32_MAX);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_stage0_flash_page_addr_ok),
        cmocka_unit_test(test_stage0_flash_block_addr_ok),
        cmocka_unit_test(test_stage0_flash_make_invalid_header_page),
        cmocka_unit_test(test_stage0_flash_make_stage1_header_page),
        cmocka_unit_test(test_stage0_flash_make_stage1_second_header_page),
        cmocka_unit_test(test_stage0_flash_make_stage1_factory_random_page),
        cmocka_unit_test(test_stage0_flash_make_stage1_factory_random_page_without_image_bytes),
        cmocka_unit_test(test_factory_random_source_policy),
        cmocka_unit_test(test_factory_random_backup_valid_record),
        cmocka_unit_test(test_factory_random_backup_erased_page_invalid),
        cmocka_unit_test(test_factory_random_backup_missing_commit_invalid),
        cmocka_unit_test(test_factory_random_backup_commit_page_only_clears_commit),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
