// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include "bootloader/stage0/stage0_descriptor.h"
#include "bootloader_upgrade/bootloader_upgrade.h"
#include "bootloader_upgrade/firmware_installer_check.h"

static void _put_bytes(uint8_t* dst, size_t dst_len, size_t offset, const char* src)
{
    const size_t src_len = strlen(src);
    assert_true(offset <= dst_len);
    assert_true(src_len <= dst_len - offset);
    memcpy(&dst[offset], src, src_len);
}

static void test_legacy_development_markers(void** state)
{
    (void)state;
    uint8_t bootloader[256] = {0};

    _put_bytes(bootloader, sizeof(bootloader), 10, "DEV DEVICE");
    _put_bytes(bootloader, sizeof(bootloader), 100, "NOT FOR VALUE");

    assert_true(bootloader_upgrade_has_legacy_development_markers(bootloader, sizeof(bootloader)));
    assert_true(
        bootloader_upgrade_is_development_bootloader(NULL, NULL, bootloader, sizeof(bootloader)));
}

static void test_legacy_development_markers_need_both(void** state)
{
    (void)state;
    uint8_t bootloader[256] = {0};

    _put_bytes(bootloader, sizeof(bootloader), 10, "DEV DEVICE");
    assert_false(bootloader_upgrade_has_legacy_development_markers(bootloader, sizeof(bootloader)));

    memset(bootloader, 0, sizeof(bootloader));
    _put_bytes(bootloader, sizeof(bootloader), 100, "NOT FOR VALUE");
    assert_false(bootloader_upgrade_has_legacy_development_markers(bootloader, sizeof(bootloader)));
}

static void test_legacy_development_markers_absent(void** state)
{
    (void)state;
    uint8_t bootloader[256] = {0};

    assert_false(bootloader_upgrade_has_legacy_development_markers(bootloader, sizeof(bootloader)));
    assert_false(
        bootloader_upgrade_is_development_bootloader(NULL, NULL, bootloader, sizeof(bootloader)));
}

static void test_development_stage0_descriptor(void** state)
{
    (void)state;
    const bb02_stage0_descriptor_t descriptor = {
        .stage0_version = BB02_STAGE0_IMAGE_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .flags = BB02_STAGE0_FLAG_DEVELOPMENT,
        .magic = BB02_STAGE0_DESCRIPTOR_MAGIC,
    };
    uint8_t legacy_bootloader[256] = {0};

    assert_true(bootloader_upgrade_is_development_bootloader(
        &descriptor, NULL, legacy_bootloader, sizeof(legacy_bootloader)));
}

static void test_development_stage1_header(void** state)
{
    (void)state;
    const bb02_stage0_descriptor_t stage0_descriptor = {
        .stage0_version = BB02_STAGE0_IMAGE_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .flags = 0,
        .magic = BB02_STAGE0_DESCRIPTOR_MAGIC,
    };
    const bb02_stage1_header_t stage1_header = {
        .magic = BB02_STAGE1_HEADER_MAGIC,
        .header_version = BB02_STAGE1_HEADER_FORMAT_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .header_len = BB02_STAGE1_HEADER_LEN,
        .image_len = BB02_STAGE1_HEADER_LEN + 512u,
        .flags = BB02_STAGE1_FLAG_DEVELOPMENT,
    };
    uint8_t legacy_bootloader[256] = {0};

    assert_true(bootloader_upgrade_is_development_bootloader(
        &stage0_descriptor, &stage1_header, legacy_bootloader, sizeof(legacy_bootloader)));
}

static void test_development_stage1_header_future_header_version(void** state)
{
    (void)state;
    const bb02_stage0_descriptor_t stage0_descriptor = {
        .stage0_version = BB02_STAGE0_IMAGE_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .flags = 0,
        .magic = BB02_STAGE0_DESCRIPTOR_MAGIC,
    };
    const bb02_stage1_header_t stage1_header = {
        .magic = BB02_STAGE1_HEADER_MAGIC,
        .header_version = BB02_STAGE1_HEADER_FORMAT_VERSION + 1u,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .header_len = BB02_STAGE1_HEADER_LEN,
        .image_len = BB02_STAGE1_HEADER_LEN + 512u,
        .flags = BB02_STAGE1_FLAG_DEVELOPMENT,
    };
    uint8_t legacy_bootloader[256] = {0};

    assert_true(bootloader_upgrade_is_development_bootloader(
        &stage0_descriptor, &stage1_header, legacy_bootloader, sizeof(legacy_bootloader)));
}

static void test_production_stage0_descriptor_skips_legacy_markers(void** state)
{
    (void)state;
    const bb02_stage0_descriptor_t stage0_descriptor = {
        .stage0_version = BB02_STAGE0_IMAGE_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .flags = 0,
        .magic = BB02_STAGE0_DESCRIPTOR_MAGIC,
    };
    const bb02_stage1_header_t stage1_header = {
        .magic = BB02_STAGE1_HEADER_MAGIC,
        .header_version = BB02_STAGE1_HEADER_FORMAT_VERSION,
        .product_id = BB02_STAGE1_PRODUCT_ID,
        .header_len = BB02_STAGE1_HEADER_LEN,
        .image_len = BB02_STAGE1_HEADER_LEN + 512u,
        .flags = 0,
    };
    uint8_t legacy_bootloader[256] = {0};

    _put_bytes(legacy_bootloader, sizeof(legacy_bootloader), 10, "DEV DEVICE");
    _put_bytes(legacy_bootloader, sizeof(legacy_bootloader), 100, "NOT FOR VALUE");

    assert_false(bootloader_upgrade_is_development_bootloader(
        &stage0_descriptor, &stage1_header, legacy_bootloader, sizeof(legacy_bootloader)));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_legacy_development_markers),
        cmocka_unit_test(test_legacy_development_markers_need_both),
        cmocka_unit_test(test_legacy_development_markers_absent),
        cmocka_unit_test(test_development_stage0_descriptor),
        cmocka_unit_test(test_development_stage1_header),
        cmocka_unit_test(test_development_stage1_header_future_header_version),
        cmocka_unit_test(test_production_stage0_descriptor_skips_legacy_markers),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
