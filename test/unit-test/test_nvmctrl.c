// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include "memory/nvmctrl.h"

#include <driver_init.h>
#include <flags.h>
#include <hri_nvmctrl_d51.h>

fake_nvmctrl_t fake_nvmctrl;
struct flash_descriptor FLASH_0;

static uint8_t _flash[ERASE_BLOCK_SIZE];
static uint32_t _address;
static size_t _operation;
static size_t _fail_operation;
static uint16_t _error_flag;
static uint32_t _error_types;
static bool _return_error;

// Operation order: unlock, erase, sixteen page writes, relock.
USE_RESULT static bool _flash_operation(void)
{
    _operation++;
    if (_operation == _fail_operation) {
        fake_nvmctrl.INTFLAG.reg |= _error_flag;
        fake_nvmctrl.ECCERR.reg |= _error_types;
        return !_return_error;
    }
    return true;
}

int32_t flash_unlock(struct flash_descriptor* flash, uint32_t address, uint32_t pages)
{
    assert_ptr_equal(flash, &FLASH_0);
    assert_int_equal(address, _address & ~(FLASH_REGION_PAGE_NUM * FLASH_PAGE_SIZE - 1U));
    assert_int_equal(pages, FLASH_REGION_PAGE_NUM);
    assert_int_equal(_operation, 0);
    return _flash_operation() ? (int32_t)FLASH_REGION_PAGE_NUM : ERR_DENIED;
}

int32_t flash_lock(struct flash_descriptor* flash, uint32_t address, uint32_t pages)
{
    assert_ptr_equal(flash, &FLASH_0);
    assert_int_equal(address, _address & ~(FLASH_REGION_PAGE_NUM * FLASH_PAGE_SIZE - 1U));
    assert_int_equal(pages, FLASH_REGION_PAGE_NUM);
    assert_int_equal(_operation, 2 + ERASE_BLOCK_SIZE / FLASH_PAGE_SIZE);
    return _flash_operation() ? (int32_t)FLASH_REGION_PAGE_NUM : ERR_DENIED;
}

int32_t flash_erase(struct flash_descriptor* flash, uint32_t address, uint32_t pages)
{
    assert_ptr_equal(flash, &FLASH_0);
    assert_int_equal(address, _address);
    assert_int_equal(pages, FLASH_ERASE_PAGE_NUM);
    assert_int_equal(_operation, 1);
    memset(_flash, UINT8_MAX, sizeof(_flash));
    return _flash_operation() ? ERR_NONE : ERR_DENIED;
}

int32_t flash_append(struct flash_descriptor* flash, uint32_t address, uint8_t* data, uint32_t len)
{
    assert_ptr_equal(flash, &FLASH_0);
    assert_int_equal(len, FLASH_PAGE_SIZE);
    assert_int_equal((uintptr_t)data % sizeof(uint32_t), 0);
    const size_t offset = (_operation - 2) * FLASH_PAGE_SIZE;
    assert_int_equal(address, _address + offset);
    assert_true(offset + len <= ERASE_BLOCK_SIZE);
    memcpy(_flash + offset, data, len);
    return _flash_operation() ? ERR_NONE : ERR_DENIED;
}

static void _reset_nvmctrl(void)
{
    memset(&fake_nvmctrl, 0, sizeof(fake_nvmctrl));
    fake_nvmctrl.STATUS.bit.READY = 1;
    _operation = 0;
    _fail_operation = 0;
    _error_flag = 0;
    _error_types = 0;
    _return_error = false;
    _address = FLASH_BOOTDATA_START;
}

static void test_clear_errors(void** state)
{
    (void)state;
    _reset_nvmctrl();
    const uint16_t error_flags = NVMCTRL_INTFLAG_ADDRE | NVMCTRL_INTFLAG_PROGE |
                                 NVMCTRL_INTFLAG_LOCKE | NVMCTRL_INTFLAG_ECCSE |
                                 NVMCTRL_INTFLAG_ECCDE | NVMCTRL_INTFLAG_NVME;
    fake_nvmctrl.INTFLAG.reg = error_flags | NVMCTRL_INTFLAG_DONE;
    fake_nvmctrl.ECCERR.reg = NVMCTRL_ECCERR_TYPEL_SINGLE | 0x1234U;

    nvmctrl_clear_errors();

    assert_int_equal(fake_nvmctrl.INTFLAG.reg, NVMCTRL_INTFLAG_DONE);
    assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
    assert_int_equal(fake_nvmctrl.eccerr_reads, 1);
}

static void test_check_no_errors(void** state)
{
    (void)state;
    _reset_nvmctrl();

    assert_true(nvmctrl_wait_ready_and_check_errors());
    assert_int_equal(fake_nvmctrl.eccerr_reads, 1);
}

static void test_check_each_error_flag(void** state)
{
    (void)state;
    const uint16_t error_flags[] = {
        NVMCTRL_INTFLAG_ADDRE,
        NVMCTRL_INTFLAG_PROGE,
        NVMCTRL_INTFLAG_LOCKE,
        NVMCTRL_INTFLAG_ECCDE,
        NVMCTRL_INTFLAG_NVME,
    };

    for (size_t i = 0; i < sizeof(error_flags) / sizeof(error_flags[0]); i++) {
        for (size_t corrected = 0; corrected < 2; corrected++) {
            _reset_nvmctrl();
            fake_nvmctrl.INTFLAG.reg = error_flags[i];
            if (corrected) {
                fake_nvmctrl.INTFLAG.reg |= NVMCTRL_INTFLAG_ECCSE;
                fake_nvmctrl.ECCERR.reg = NVMCTRL_ECCERR_TYPEL_SINGLE | NVMCTRL_ECCERR_TYPEH_SINGLE;
            }

            assert_false(nvmctrl_wait_ready_and_check_errors());
            assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
            assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
            assert_int_equal(fake_nvmctrl.eccerr_reads, 1);
        }
    }
}

static void test_check_ecc_types_and_flags(void** state)
{
    (void)state;
    const uint32_t low_types[] = {
        NVMCTRL_ECCERR_TYPEL_NONE,
        NVMCTRL_ECCERR_TYPEL_SINGLE,
        NVMCTRL_ECCERR_TYPEL_DUAL,
        NVMCTRL_ECCERR_TYPEL_Msk, // Unexpected type 3.
    };
    const uint32_t high_types[] = {
        NVMCTRL_ECCERR_TYPEH_NONE,
        NVMCTRL_ECCERR_TYPEH_SINGLE,
        NVMCTRL_ECCERR_TYPEH_DUAL,
        NVMCTRL_ECCERR_TYPEH_Msk, // Unexpected type 3.
    };
    const uint16_t flags[] = {
        0,
        NVMCTRL_INTFLAG_ECCSE,
        NVMCTRL_INTFLAG_ECCDE,
        NVMCTRL_INTFLAG_ECCSE | NVMCTRL_INTFLAG_ECCDE,
    };

    for (size_t low = 0; low < sizeof(low_types) / sizeof(low_types[0]); low++) {
        for (size_t high = 0; high < sizeof(high_types) / sizeof(high_types[0]); high++) {
            for (size_t flag = 0; flag < sizeof(flags) / sizeof(flags[0]); flag++) {
                _reset_nvmctrl();
                fake_nvmctrl.ECCERR.reg = low_types[low] | high_types[high];
                fake_nvmctrl.INTFLAG.reg = flags[flag];
                // Only NONE/SINGLE in both halves, without ECCDE, may succeed.
                assert_int_equal(
                    nvmctrl_wait_ready_and_check_errors(), low < 2 && high < 2 && flag < 2);
                assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
                assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
                assert_int_equal(fake_nvmctrl.eccerr_reads, 1);
            }
        }
    }
}

static void test_check_ignores_non_errors(void** state)
{
    (void)state;
    _reset_nvmctrl();
    fake_nvmctrl.INTFLAG.reg = NVMCTRL_INTFLAG_DONE;
    fake_nvmctrl.ECCERR.reg = 0x00123450U;

    assert_true(nvmctrl_wait_ready_and_check_errors());
    assert_int_equal(fake_nvmctrl.INTFLAG.reg, NVMCTRL_INTFLAG_DONE);
}

static void test_read(void** state)
{
    (void)state;
    _reset_nvmctrl();
    const uint8_t flash[] = {1, 2, 3, 4, 5};
    uint8_t readback[sizeof(flash)] = {0};
    assert_true(nvmctrl_read(flash, readback, sizeof(flash)));
    assert_memory_equal(readback, flash, sizeof(flash));
    // One ECCERR read before the operation, one when checking its result.
    assert_int_equal(fake_nvmctrl.eccerr_reads, 2);
}

static void test_read_errors(void** state)
{
    (void)state;
    const uint16_t flags[] = {
        NVMCTRL_INTFLAG_ADDRE,
        NVMCTRL_INTFLAG_PROGE,
        NVMCTRL_INTFLAG_LOCKE,
        NVMCTRL_INTFLAG_NVME,
        NVMCTRL_INTFLAG_ECCDE,
    };
    const uint8_t flash[] = {1, 2, 3};
    uint8_t readback[sizeof(flash)];
    for (size_t i = 0; i < sizeof(flags) / sizeof(flags[0]); i++) {
        for (size_t corrected = 0; corrected < 2; corrected++) {
            _reset_nvmctrl();
            // Inject an error after stale flags are cleared and the flash read has finished.
            fake_nvmctrl.flags_on_ready = flags[i];
            if (corrected) {
                fake_nvmctrl.flags_on_ready |= NVMCTRL_INTFLAG_ECCSE;
                fake_nvmctrl.eccerr_on_ready =
                    NVMCTRL_ECCERR_TYPEL_SINGLE | NVMCTRL_ECCERR_TYPEH_SINGLE;
            }
            assert_false(nvmctrl_read(flash, readback, sizeof(readback)));
            assert_memory_equal(readback, flash, sizeof(flash));
            assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
            assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
        }
    }
}

static void test_read_correctable_ecc(void** state)
{
    (void)state;
    const uint32_t types[] = {
        NVMCTRL_ECCERR_TYPEL_NONE | NVMCTRL_ECCERR_TYPEH_NONE,
        NVMCTRL_ECCERR_TYPEL_SINGLE,
        NVMCTRL_ECCERR_TYPEH_SINGLE,
        NVMCTRL_ECCERR_TYPEL_SINGLE | NVMCTRL_ECCERR_TYPEH_SINGLE,
    };
    const uint8_t flash[] = {1, 2, 3};
    uint8_t readback[sizeof(flash)];
    for (size_t i = 0; i < sizeof(types) / sizeof(types[0]); i++) {
        for (size_t flagged = 0; flagged < 2; flagged++) {
            _reset_nvmctrl();
            fake_nvmctrl.flags_on_ready = flagged ? NVMCTRL_INTFLAG_ECCSE : 0;
            fake_nvmctrl.eccerr_on_ready = types[i];
            assert_true(nvmctrl_read(flash, readback, sizeof(readback)));
            assert_memory_equal(readback, flash, sizeof(flash));
            assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
            assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
            assert_int_equal(fake_nvmctrl.eccerr_reads, 2);
        }
    }
}

static void test_read_block_checked(void** state)
{
    (void)state;
    _reset_nvmctrl();
    for (size_t i = 0; i < sizeof(_flash); i++) {
        _flash[i] = (uint8_t)i;
    }
    uint8_t readback[ERASE_BLOCK_SIZE];
    assert_true(nvmctrl_read_block_checked((uintptr_t)_flash, readback));
    assert_memory_equal(readback, _flash, sizeof(readback));
    assert_int_equal(fake_nvmctrl.eccerr_reads, 4);
}

static void _read_block_error(void)
{
    _operation++;
    if (_operation == _fail_operation) {
        fake_nvmctrl.INTFLAG.reg |= NVMCTRL_INTFLAG_ECCDE;
        fake_nvmctrl.ECCERR.reg |= NVMCTRL_ECCERR_TYPEL_DUAL;
    }
}

static void test_read_block_checked_errors(void** state)
{
    (void)state;
    uint8_t readback[ERASE_BLOCK_SIZE];
    for (size_t read = 1; read <= 2; read++) {
        _reset_nvmctrl();
        _fail_operation = read;
        fake_nvmctrl.on_ready = _read_block_error;
        assert_false(nvmctrl_read_block_checked((uintptr_t)_flash, readback));
        assert_int_equal(_operation, read);
    }
}

static void _change_flash_after_read(void)
{
    _flash[ERASE_BLOCK_SIZE - 1] ^= 1;
    fake_nvmctrl.on_ready = NULL;
}

static void test_read_block_checked_mismatch(void** state)
{
    (void)state;
    _reset_nvmctrl();
    fake_nvmctrl.on_ready = _change_flash_after_read;
    uint8_t readback[ERASE_BLOCK_SIZE];
    assert_false(nvmctrl_read_block_checked((uintptr_t)_flash, readback));
    assert_int_equal(fake_nvmctrl.eccerr_reads, 4);
}

static void test_write_block(void** state)
{
    (void)state;
    uint8_t chunk[ERASE_BLOCK_SIZE];
    for (size_t i = 0; i < sizeof(chunk); i++) {
        chunk[i] = (uint8_t)i;
    }
    const uint32_t addresses[] = {FLASH_BOOTDATA_START, FLASH_APPDATA_START + 7 * ERASE_BLOCK_SIZE};
    for (size_t i = 0; i < sizeof(addresses) / sizeof(addresses[0]); i++) {
        _reset_nvmctrl();
        _address = addresses[i];
        assert_true(nvmctrl_write_block(_address, chunk));
        assert_int_equal(_operation, 3 + ERASE_BLOCK_SIZE / FLASH_PAGE_SIZE);
        assert_memory_equal(_flash, chunk, sizeof(chunk));
        assert_int_equal(fake_nvmctrl.eccerr_reads, 2 * _operation);
    }
}

static void test_write_block_errors(void** state)
{
    (void)state;
    const uint8_t chunk[ERASE_BLOCK_SIZE] = {0};
    const uint16_t flags[] = {
        0, // HAL error without any NVM flag
        NVMCTRL_INTFLAG_ADDRE,
        NVMCTRL_INTFLAG_PROGE,
        NVMCTRL_INTFLAG_LOCKE,
        NVMCTRL_INTFLAG_NVME,
        NVMCTRL_INTFLAG_ECCDE,
    };
    for (size_t operation = 1; operation <= 3 + ERASE_BLOCK_SIZE / FLASH_PAGE_SIZE; operation++) {
        for (size_t flag = 0; flag < sizeof(flags) / sizeof(flags[0]); flag++) {
            for (size_t corrected = 0; corrected < 2; corrected++) {
                _reset_nvmctrl();
                _fail_operation = operation;
                _return_error = flags[flag] == 0;
                _error_flag = flags[flag];
                if (corrected) {
                    _error_flag |= NVMCTRL_INTFLAG_ECCSE;
                    _error_types = NVMCTRL_ECCERR_TYPEL_SINGLE | NVMCTRL_ECCERR_TYPEH_SINGLE;
                }
                assert_false(nvmctrl_write_block(_address, chunk));
                assert_int_equal(_operation, operation);
                assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
                assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
            }
        }
    }
}

static void test_write_block_correctable_ecc(void** state)
{
    (void)state;
    const uint8_t chunk[ERASE_BLOCK_SIZE] = {0};
    for (size_t operation = 1; operation <= 3 + ERASE_BLOCK_SIZE / FLASH_PAGE_SIZE; operation++) {
        _reset_nvmctrl();
        _fail_operation = operation;
        _error_flag = NVMCTRL_INTFLAG_ECCSE;
        _error_types = NVMCTRL_ECCERR_TYPEL_SINGLE | NVMCTRL_ECCERR_TYPEH_SINGLE;
        assert_true(nvmctrl_write_block(_address, chunk));
        assert_int_equal(_operation, 3 + ERASE_BLOCK_SIZE / FLASH_PAGE_SIZE);
        assert_memory_equal(_flash, chunk, sizeof(chunk));
        assert_int_equal(fake_nvmctrl.INTFLAG.reg, 0);
        assert_int_equal(fake_nvmctrl.ECCERR.reg, 0);
    }
}

static void test_write_block_invalid_arguments(void** state)
{
    (void)state;
    _reset_nvmctrl();
    const uint8_t chunk[ERASE_BLOCK_SIZE] = {0};
    assert_false(nvmctrl_write_block(FLASH_BOOTDATA_START, NULL));
    assert_false(nvmctrl_write_block(FLASH_BOOTDATA_START + 1, chunk));
    assert_false(nvmctrl_write_block(FLASH_SIZE, chunk));
    assert_int_equal(_operation, 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_clear_errors),
        cmocka_unit_test(test_check_no_errors),
        cmocka_unit_test(test_check_each_error_flag),
        cmocka_unit_test(test_check_ecc_types_and_flags),
        cmocka_unit_test(test_check_ignores_non_errors),
        cmocka_unit_test(test_read),
        cmocka_unit_test(test_read_errors),
        cmocka_unit_test(test_read_correctable_ecc),
        cmocka_unit_test(test_read_block_checked),
        cmocka_unit_test(test_read_block_checked_errors),
        cmocka_unit_test(test_read_block_checked_mismatch),
        cmocka_unit_test(test_write_block),
        cmocka_unit_test(test_write_block_errors),
        cmocka_unit_test(test_write_block_correctable_ecc),
        cmocka_unit_test(test_write_block_invalid_arguments),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
