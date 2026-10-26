// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <cmocka.h>

#include "bootloader/stage0/stage0_install.h"

static void test_stage0_install_auth_versions(void** state)
{
    (void)state;

    const uint16_t versions[] = {0, 1, 2, 0x7fff, 0x8000, UINT16_MAX};
    for (size_t i = 0; i < COUNT_OF(versions); i++) {
        for (size_t j = 0; j < COUNT_OF(versions); j++) {
            const secbool_u32 expected = versions[j] >= versions[i] ? sectrue_u32 : secfalse_u32;
            assert_int_equal(
                stage0_install_auth(
                    sectrue_u32, sectrue_u32, sectrue_u32, versions[i], versions[j]),
                expected);
        }
    }
}

static void test_stage0_install_auth_recovery(void** state)
{
    (void)state;

    // A damaged header's version is not a trustworthy recovery floor.
    assert_int_equal(
        stage0_install_auth(sectrue_u32, secfalse_u32, secfalse_u32, UINT16_MAX, 0), sectrue_u32);
    assert_int_equal(
        stage0_install_auth(sectrue_u32, secfalse_u32, secfalse_u32, 0, UINT16_MAX), sectrue_u32);
}

static void test_stage0_install_auth_verification_results(void** state)
{
    (void)state;

    const secbool_u32 results[] = {
        secfalse_u32, sectrue_u32, 1, UINT32_MAX, sectrue_u32 ^ 1U, sectrue_u32 ^ 2U};
    for (size_t u = 0; u < COUNT_OF(results); u++) {
        for (size_t i = 0; i < COUNT_OF(results); i++) {
            for (size_t r = 0; r < COUNT_OF(results); r++) {
                const bool allowed = results[u] == sectrue_u32 && results[i] == results[r] &&
                                     (results[i] == sectrue_u32 || results[i] == secfalse_u32);
                assert_int_equal(
                    stage0_install_auth(results[u], results[i], results[r], 1, 2),
                    allowed ? sectrue_u32 : secfalse_u32);
            }
        }
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_stage0_install_auth_versions),
        cmocka_unit_test(test_stage0_install_auth_recovery),
        cmocka_unit_test(test_stage0_install_auth_verification_results),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
