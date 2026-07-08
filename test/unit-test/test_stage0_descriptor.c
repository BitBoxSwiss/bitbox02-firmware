// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <cmocka.h>

#include "bootloader/stage0/stage0_descriptor.h"
#include "bootloader_upgrade/bootloader_upgrade.h"

static void test_stage0_descriptor_abi(void** state)
{
    (void)state;

    assert_int_equal(BB02_STAGE0_DESCRIPTOR_ADDR, 0x00001ff4u);
    assert_int_equal(BB02_STAGE0_DESCRIPTOR_LEN, 12u);
    assert_int_equal(sizeof(bb02_stage0_descriptor_t), BB02_STAGE0_DESCRIPTOR_LEN);
    assert_int_equal(
        offsetof(bb02_stage0_descriptor_t, stage0_version), BB02_STAGE0_DESCRIPTOR_LEN - 12u);
    assert_int_equal(
        offsetof(bb02_stage0_descriptor_t, product_id), BB02_STAGE0_DESCRIPTOR_LEN - 10u);
    assert_int_equal(offsetof(bb02_stage0_descriptor_t, flags), BB02_STAGE0_DESCRIPTOR_LEN - 8u);
    assert_int_equal(offsetof(bb02_stage0_descriptor_t, magic), BB02_STAGE0_DESCRIPTOR_LEN - 4u);
}

static void test_stage0_descriptor_value(void** state)
{
    (void)state;

    assert_int_equal(bb02_stage0_descriptor.magic, BB02_STAGE0_DESCRIPTOR_MAGIC);
    assert_int_equal(bb02_stage0_descriptor.stage0_version, BB02_STAGE0_IMAGE_VERSION);
    assert_int_equal(bb02_stage0_descriptor.product_id, BB02_STAGE1_PRODUCT_ID);
    assert_int_equal(bb02_stage0_descriptor.flags, 0u);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_stage0_descriptor_abi),
        cmocka_unit_test(test_stage0_descriptor_value),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
