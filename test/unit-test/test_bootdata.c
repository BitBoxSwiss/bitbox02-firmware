// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include "bootloader/bootdata.h"
#include "memory/memory.h"
#include "memory/nvmctrl.h"
#include "pukcc/pukcc.h"
#include "rust/rust.h"

#define VERSIONS_PREIMAGE_LEN 8U
#define MIGRATION_CONSUMED_CHUNK_ADDR \
    (FLASH_APPDATA_START + MEMORY_BOOTDATA_MIGRATION_CONSUMED_CHUNK * CHUNK_SIZE)

static const uint8_t _migration_consumed_magic[MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN] =
    MEMORY_BOOTDATA_MIGRATION_CONSUMED_MAGIC;
static boot_data_t _flash_bootdata;
static uint8_t _migration_consumed_chunk[CHUNK_SIZE];
static size_t _read_calls;
static size_t _write_calls;
static size_t _migration_consumed_writes;
static size_t _bootdata_writes;
static size_t _migration_consumed_readback_calls;
static bool _migration_consumed_needs_verification;
static size_t _fail_read_call;
static size_t _unstable_read_call;
static size_t _fail_write_call;
static size_t _cut_after_bytes;
static size_t _corrupt_write_call;

static void _reset_io(void)
{
    _read_calls = 0;
    _write_calls = 0;
    _migration_consumed_writes = 0;
    _bootdata_writes = 0;
    _migration_consumed_readback_calls = 0;
    _migration_consumed_needs_verification = false;
    _fail_read_call = 0;
    _unstable_read_call = 0;
    _fail_write_call = 0;
    _cut_after_bytes = 0;
    _corrupt_write_call = 0;
}

USE_RESULT bool __wrap_nvmctrl_read(const volatile uint8_t* flash, uint8_t* out, size_t len)
{
    const uintptr_t address = (uintptr_t)flash;
    _read_calls++;
    if (address == FLASH_BOOTDATA_START) {
        assert_int_equal(len, CHUNK_SIZE);
        memcpy(out, _flash_bootdata.bytes, len);
    } else if (address == MIGRATION_CONSUMED_CHUNK_ADDR) {
        assert_int_equal(len, CHUNK_SIZE);
        memcpy(out, _migration_consumed_chunk, len);
        if (_migration_consumed_needs_verification && ++_migration_consumed_readback_calls == 2) {
            _migration_consumed_needs_verification = false;
        }
    } else {
        assert_int_equal(
            address, MIGRATION_CONSUMED_CHUNK_ADDR + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET);
        assert_int_equal(len, MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN);
        memcpy(out, _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET, len);
    }
    if (_read_calls == _unstable_read_call) {
        out[0] ^= 1;
    }
    return _read_calls != _fail_read_call;
}

USE_RESULT bool __wrap_nvmctrl_read_block_checked(uintptr_t address, uint8_t* out)
{
    const volatile uint8_t* flash = (const volatile uint8_t*)address;
    const size_t first_read = _read_calls + 1;
    if (!__wrap_nvmctrl_read(flash, out, CHUNK_SIZE) ||
        !__wrap_nvmctrl_read(flash, out, CHUNK_SIZE)) {
        return false;
    }
    return _unstable_read_call != first_read && _unstable_read_call != first_read + 1;
}

USE_RESULT bool __wrap_nvmctrl_write_block(uint32_t address, const uint8_t* data)
{
    _write_calls++;
    uint8_t* dest;
    if (address == MIGRATION_CONSUMED_CHUNK_ADDR) {
        _migration_consumed_writes++;
        _migration_consumed_needs_verification = true;
        _migration_consumed_readback_calls = 0;
        dest = _migration_consumed_chunk;
    } else {
        assert_int_equal(address, FLASH_BOOTDATA_START);
        assert_memory_equal(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        assert_false(_migration_consumed_needs_verification);
        _bootdata_writes++;
        dest = _flash_bootdata.bytes;
    }
    memset(dest, UINT8_MAX, CHUNK_SIZE);
    if (_write_calls == _fail_write_call) {
        assert_true(_cut_after_bytes <= CHUNK_SIZE);
        memcpy(dest, data, _cut_after_bytes);
        return false;
    }
    memcpy(dest, data, CHUNK_SIZE);
    if (_write_calls == _corrupt_write_call) {
        // Detect mismatches outside the hashed fields through the complete write readback.
        dest[CHUNK_SIZE - 1] ^= 1;
    }
    return true;
}

static bool _sha_succeeds;
static size_t _sha_calls;
static size_t _fail_sha_call;
static uint8_t _sha_input[VERSIONS_PREIMAGE_LEN];

static void _reset_sha(void)
{
    _sha_succeeds = true;
    _sha_calls = 0;
    _fail_sha_call = 0;
    memset(_sha_input, 0, sizeof(_sha_input));
}

USE_RESULT int32_t
__wrap_pukcc_sha256_compute(const uint8_t* message, uint32_t message_len, uint8_t* hash)
{
    assert_int_equal(message_len, VERSIONS_PREIMAGE_LEN);
    memcpy(_sha_input, message, sizeof(_sha_input));
    _sha_calls++;
    if (!_sha_succeeds || _sha_calls == _fail_sha_call) {
        return -1;
    }
    rust_sha256(message, message_len, hash);
    return 0;
}

USE_RESULT static boot_data_t _bootdata(
    version_t signing_pubkeys_version,
    version_t firmware_version)
{
    boot_data_t bootdata = {0};
    memset(bootdata.fields.is_initialized, sectrue_u8, sizeof(bootdata.fields.is_initialized));
    bootdata.fields.signing_pubkeys_version = signing_pubkeys_version;
    bootdata.fields.firmware_version = firmware_version;
    return bootdata;
}

static void _reset_flash(void)
{
    _reset_sha();
    _reset_io();
    _flash_bootdata = _bootdata(3, 42);
    // No valid firmware signatures are provided: first sealing must not depend on the carrier.
    memset(_migration_consumed_chunk, 0xA5, sizeof(_migration_consumed_chunk));
    memset(
        _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        UINT8_MAX,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN);
}

static void test_migration_consumed_persistent_format(void** state)
{
    (void)state;
    const uint8_t expected_magic[] = {0x2F, 0x19, 0x4F, 0x78};
    assert_int_equal(MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET, 36);
    assert_int_equal(MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN, sizeof(expected_magic));
    assert_memory_equal(_migration_consumed_magic, expected_magic, sizeof(expected_magic));
}

static void test_set_known_hash(void** state)
{
    (void)state;
    _reset_sha();
    boot_data_t bootdata = _bootdata(0x01020304U, 0xA1B2C3D4U);
    const uint8_t expected_input[VERSIONS_PREIMAGE_LEN] = {
        0x04,
        0x03,
        0x02,
        0x01,
        0xD4,
        0xC3,
        0xB2,
        0xA1,
    };
    const uint8_t expected_hash[SHA256_DIGEST_LENGTH] = {
        0xF5, 0x46, 0x83, 0xD6, 0x1E, 0x15, 0xBA, 0x25, 0xBE, 0x7B, 0xCD,
        0x9B, 0x83, 0xBD, 0x5A, 0x06, 0xBB, 0xB9, 0x13, 0x04, 0xCB, 0xE4,
        0x78, 0x50, 0x4D, 0xBC, 0x23, 0xB2, 0x0C, 0x3E, 0xC5, 0x7D,
    };

    assert_true(bootdata_hash_set(&bootdata));
    assert_int_equal(_sha_calls, 1);
    assert_memory_equal(_sha_input, expected_input, sizeof(expected_input));
    assert_memory_equal(bootdata.fields.versions_hash, expected_hash, sizeof(expected_hash));
}

static void test_hash_check_zero(void** state)
{
    (void)state;
    _reset_sha();
    boot_data_t bootdata = _bootdata(1, 2);
    bool hash_valid = true;

    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_false(hash_valid);
    assert_int_equal(_sha_calls, 1);
}

static void test_hash_check_valid_and_changed_versions(void** state)
{
    (void)state;
    _reset_sha();
    boot_data_t bootdata = _bootdata(1, 2);
    assert_true(bootdata_hash_set(&bootdata));

    bool hash_valid;
    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_true(hash_valid);

    bootdata.fields.signing_pubkeys_version--;
    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_false(hash_valid);
    bootdata.fields.signing_pubkeys_version++;

    bootdata.fields.firmware_version--;
    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_false(hash_valid);
}

static void test_hash_check_nonzero_mismatches(void** state)
{
    (void)state;
    _reset_sha();
    boot_data_t bootdata = _bootdata(1, 2);
    bool hash_valid;

    memset(bootdata.fields.versions_hash, UINT8_MAX, sizeof(bootdata.fields.versions_hash));
    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_false(hash_valid);

    memset(bootdata.fields.versions_hash, 0, sizeof(bootdata.fields.versions_hash));
    bootdata.fields.versions_hash[0] = 1;
    assert_true(bootdata_hash_check(&bootdata, &hash_valid));
    assert_false(hash_valid);
}

static void test_hash_failures(void** state)
{
    (void)state;
    _reset_sha();
    boot_data_t bootdata = _bootdata(1, 2);
    memset(bootdata.fields.versions_hash, 0x42, sizeof(bootdata.fields.versions_hash));
    boot_data_t original = bootdata;
    bool hash_valid;

    _sha_succeeds = false;
    assert_false(bootdata_hash_set(&bootdata));
    assert_memory_equal(bootdata.bytes, original.bytes, sizeof(bootdata.bytes));
    assert_false(bootdata_hash_check(&bootdata, &hash_valid));
}

static void test_null_arguments(void** state)
{
    (void)state;
    boot_data_t bootdata = _bootdata(1, 2);
    bool hash_valid;

    assert_false(bootdata_hash_set(NULL));
    assert_false(bootdata_hash_check(NULL, &hash_valid));
    assert_false(bootdata_hash_check(&bootdata, NULL));
}

static void test_bootdata_init_legacy_once(void** state)
{
    (void)state;
    _reset_flash();
    boot_data_t expected = _flash_bootdata;
    assert_true(bootdata_hash_set(&expected));
    uint8_t expected_marker_chunk[CHUNK_SIZE];
    memcpy(expected_marker_chunk, _migration_consumed_chunk, sizeof(expected_marker_chunk));
    memcpy(
        expected_marker_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        _migration_consumed_magic,
        sizeof(_migration_consumed_magic));

    boot_data_t result;
    assert_true(bootdata_init(&result));
    assert_int_equal(_migration_consumed_writes, 1);
    assert_int_equal(_bootdata_writes, 1);
    assert_memory_equal(_flash_bootdata.bytes, expected.bytes, sizeof(expected.bytes));
    assert_memory_equal(result.bytes, expected.bytes, sizeof(expected.bytes));
    assert_memory_equal(
        _migration_consumed_chunk, expected_marker_chunk, sizeof(expected_marker_chunk));

    // Repeated boots neither reseal bootdata nor rewrite the consumed marker.
    assert_true(bootdata_init(&result));
    assert_true(bootdata_read(&result));
    assert_int_equal(_write_calls, 2);
}

static void test_bootdata_init_invalid_initialization(void** state)
{
    (void)state;
    const uint8_t invalid[] = {0, 1, UINT8_MAX};
    boot_data_t result;
    for (size_t marker = 0; marker < 2; marker++) {
        for (size_t i = 0; i < sizeof(invalid); i++) {
            _reset_flash();
            _flash_bootdata.fields.is_initialized[marker] = invalid[i];
            assert_false(bootdata_init(&result));
            assert_int_equal(_write_calls, 0);
        }
    }
    _reset_flash();
    memset(_flash_bootdata.bytes, UINT8_MAX, sizeof(_flash_bootdata.bytes));
    assert_false(bootdata_init(&result));
    assert_int_equal(_write_calls, 0);
}

static void test_bootdata_init_accepts_nonmagic_marker(void** state)
{
    (void)state;
    boot_data_t result;
    // Every nonmagic value permits migration, including all-zero and all-0xFF.
    for (uint16_t marker = 0; marker <= UINT8_MAX; marker++) {
        _reset_flash();
        memset(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            marker,
            MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN);
        boot_data_t expected = _flash_bootdata;
        assert_true(bootdata_hash_set(&expected));
        uint8_t expected_marker_chunk[CHUNK_SIZE];
        memcpy(expected_marker_chunk, _migration_consumed_chunk, sizeof(expected_marker_chunk));
        memcpy(
            expected_marker_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        assert_true(bootdata_init(&result));
        assert_int_equal(_migration_consumed_writes, 1);
        assert_int_equal(_bootdata_writes, 1);
        assert_memory_equal(result.bytes, expected.bytes, sizeof(expected.bytes));
        assert_memory_equal(
            _migration_consumed_chunk, expected_marker_chunk, sizeof(expected_marker_chunk));
    }
    // Matching three bytes is not consumed, regardless of which byte differs.
    for (size_t byte = 0; byte < sizeof(_migration_consumed_magic); byte++) {
        _reset_flash();
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        _migration_consumed_chunk[MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + byte] ^= 1;
        assert_true(bootdata_init(&result));
        assert_int_equal(_migration_consumed_writes, 1);
        assert_int_equal(_bootdata_writes, 1);
    }
}

static void test_bootdata_init_never_reseals_consumed_migration(void** state)
{
    (void)state;
    const uint8_t bad_hash_bytes[] = {0, 0xA5, UINT8_MAX};
    boot_data_t result;
    for (size_t i = 0; i < sizeof(bad_hash_bytes); i++) {
        _reset_flash();
        assert_true(bootdata_init(&result));
        _reset_io();
        // Simulate a torn write lowering both floors and leaving a zero/corrupt/erased hash.
        _flash_bootdata.fields.signing_pubkeys_version = 0;
        _flash_bootdata.fields.firmware_version = 0;
        memset(_flash_bootdata.fields.versions_hash, bad_hash_bytes[i], SHA256_DIGEST_LENGTH);
        assert_false(bootdata_init(&result));
        assert_false(bootdata_init(&result));
        assert_false(bootdata_read(&result));
        assert_false(bootdata_write(&result));
        assert_int_equal(_write_calls, 0);
    }
}

static void test_bootdata_init_accepts_any_hash_before_migration(void** state)
{
    (void)state;
    boot_data_t result;
    // The independent marker, not any pattern in the checksum bytes, authorizes first sealing.
    for (uint16_t value = 0; value <= UINT8_MAX; value++) {
        _reset_flash();
        memset(_flash_bootdata.fields.versions_hash, value, SHA256_DIGEST_LENGTH);
        boot_data_t expected = _flash_bootdata;
        assert_true(bootdata_hash_set(&expected));
        assert_false(bootdata_read(&result));
        assert_false(bootdata_write(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_init(&result));
        assert_int_equal(_migration_consumed_writes, 1);
        assert_int_equal(_bootdata_writes, 1);
        assert_memory_equal(result.bytes, expected.bytes, sizeof(expected.bytes));
        assert_memory_equal(_flash_bootdata.bytes, expected.bytes, sizeof(expected.bytes));
        assert_true(bootdata_init(&result));
        assert_true(bootdata_read(&result));
        assert_int_equal(_write_calls, 2);
    }

    // A stale checksum is no different from any other pre-migration value.
    _reset_flash();
    assert_true(bootdata_hash_set(&_flash_bootdata));
    _flash_bootdata.fields.firmware_version--;
    boot_data_t expected = _flash_bootdata;
    assert_true(bootdata_hash_set(&expected));
    assert_true(bootdata_init(&result));
    assert_int_equal(_migration_consumed_writes, 1);
    assert_int_equal(_bootdata_writes, 1);
    assert_memory_equal(result.bytes, expected.bytes, sizeof(expected.bytes));
    assert_memory_equal(_flash_bootdata.bytes, expected.bytes, sizeof(expected.bytes));

    // The same mismatch is fatal once migration is marked consumed, without any repair writes.
    _reset_io();
    _flash_bootdata.fields.firmware_version--;
    assert_false(bootdata_init(&result));
    assert_false(bootdata_init(&result));
    assert_false(bootdata_read(&result));
    assert_false(bootdata_write(&result));
    assert_int_equal(_write_calls, 0);
}

static void test_bootdata_init_restores_consumed_marker_before_writes(void** state)
{
    (void)state;
    _reset_flash();
    boot_data_t result;
    assert_true(bootdata_init(&result));
    boot_data_t original = _flash_bootdata;

    // A later interrupted PERMANENT-chunk migration can change the marker, but must leave valid
    // bootdata untouched. Neither a read for USB nor a write may bypass restoring the marker on
    // boot.
    uint8_t markers[3 + MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN]
                   [MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN] = {
                       {0}, {0xFF, 0xFF, 0xFF, 0xFF}, {0xA5, 0x5A, 0x12, 0x34}};
    for (size_t byte = 0; byte < sizeof(_migration_consumed_magic); byte++) {
        memcpy(markers[3 + byte], _migration_consumed_magic, sizeof(_migration_consumed_magic));
        markers[3 + byte][byte] ^= 1;
    }
    for (size_t i = 0; i < sizeof(markers) / sizeof(markers[0]); i++) {
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            markers[i],
            sizeof(markers[i]));
        _reset_io();
        assert_false(bootdata_read(&result));
        assert_false(bootdata_write(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_init(&result));
        assert_true(bootdata_read(&result));
        assert_int_equal(_migration_consumed_writes, 1);
        assert_int_equal(_bootdata_writes, 0);
        assert_memory_equal(_flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
    }

    result.fields.firmware_version++;
    assert_true(bootdata_write(&result));
    assert_int_equal(_migration_consumed_writes, 1);
    assert_int_equal(_bootdata_writes, 1);
    assert_true(bootdata_read(&result));
    assert_int_equal(result.fields.firmware_version, original.fields.firmware_version + 1);
}

static void test_bootdata_init_migration_consumed_power_cuts(void** state)
{
    (void)state;
    const size_t cuts[] = {
        0,
        1,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + 1,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + 2,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + 3,
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN,
        FLASH_PAGE_SIZE,
        CHUNK_SIZE};
    for (size_t valid_hash = 0; valid_hash < 2; valid_hash++) {
        for (size_t i = 0; i < sizeof(cuts) / sizeof(cuts[0]); i++) {
            _reset_flash();
            if (valid_hash) {
                assert_true(bootdata_hash_set(&_flash_bootdata));
            }
            boot_data_t original = _flash_bootdata;
            _fail_write_call = 1;
            _cut_after_bytes = cuts[i];
            boot_data_t result;
            assert_false(bootdata_init(&result));
            assert_int_equal(_bootdata_writes, 0);
            assert_memory_equal(_flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
            _reset_io(); // Reboot without changing flash.
            const bool partial_marker = cuts[i] < MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET +
                                                      MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN;
            if (partial_marker || valid_hash) {
                // A partial marker authorizes retry: bootdata has never been touched. Valid
                // bootdata never needs resealing, even if writing/restoring the marker was
                // interrupted.
                assert_true(bootdata_init(&result));
                assert_int_equal(_migration_consumed_writes, partial_marker ? 1 : 0);
                assert_int_equal(_bootdata_writes, valid_hash ? 0 : 1);
                if (valid_hash) {
                    assert_memory_equal(
                        _flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
                }
            } else {
                // Includes a cut after the full marker but before the checksum write.
                assert_false(bootdata_init(&result));
                assert_false(bootdata_init(&result));
                assert_int_equal(_write_calls, 0);
            }
        }
    }
}

static void test_bootdata_init_bootdata_power_cuts(void** state)
{
    (void)state;
    // Exercise erase, every partial checksum length, and every page boundary. A cut after all
    // protected fields have been written may leave valid floors, which is safe to retain.
    const size_t hash_end = BOOTDATA_VERSIONS_HASH_OFFSET + SHA256_DIGEST_LENGTH;
    for (size_t cut = 0; cut <= CHUNK_SIZE; cut++) {
        if (cut % FLASH_PAGE_SIZE != 0 && (cut < BOOTDATA_VERSIONS_HASH_OFFSET || cut > hash_end)) {
            continue;
        }
        _reset_flash();
        _fail_write_call = 2;
        _cut_after_bytes = cut;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_migration_consumed_writes, 1);
        assert_int_equal(_bootdata_writes, 1);
        assert_memory_equal(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        _reset_io();
        assert_int_equal(bootdata_init(&result), cut >= hash_end);
        assert_int_equal(bootdata_init(&result), cut >= hash_end);
        assert_int_equal(_write_calls, 0);
    }
}

static void test_bootdata_init_read_failures(void** state)
{
    (void)state;
    // Legacy migration has ten reads: bootdata twice, consumed marker twice, marker chunk twice,
    // marker write readback twice, bootdata write readback twice.
    for (size_t read = 1; read <= 10; read++) {
        _reset_flash();
        _fail_read_call = read;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        if (read <= 8) {
            assert_int_equal(_bootdata_writes, 0);
        }
    }
    for (size_t read = 2; read <= 10; read += 2) {
        _reset_flash();
        _unstable_read_call = read;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        if (read <= 8) {
            assert_int_equal(_bootdata_writes, 0);
        }
    }
}

static void test_bootdata_init_checks_full_readback(void** state)
{
    (void)state;
    for (size_t write = 1; write <= 2; write++) {
        _reset_flash();
        _corrupt_write_call = write;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_write_calls, write);
    }
}

static void test_bootdata_write_power_cut(void** state)
{
    (void)state;
    _reset_flash();
    boot_data_t result;
    assert_true(bootdata_init(&result));
    _reset_io();
    result.fields.firmware_version++;
    _fail_write_call = 1;
    // Erase the whole block while trying to raise the version floor.
    assert_false(bootdata_write(&result));
    _reset_io();
    assert_false(bootdata_init(&result));
    assert_false(bootdata_init(&result));
    assert_int_equal(_write_calls, 0);
}

static void test_bootdata_init_hash_failure(void** state)
{
    (void)state;
    // Fail initial checksum check, checksum generation, or readback validation.
    for (size_t fail_call = 1; fail_call <= 3; fail_call++) {
        _reset_flash();
        _fail_sha_call = fail_call;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_migration_consumed_writes, fail_call == 1 ? 0 : 1);
        assert_int_equal(_bootdata_writes, fail_call == 3 ? 1 : 0);
        _reset_sha();
        _reset_io();
        // Only failure between marking migration consumed and writing the checksum is permanent.
        // Before consumption migration can retry; after the write bootdata is already valid.
        assert_int_equal(bootdata_init(&result), fail_call != 2);
        assert_int_equal(_write_calls, fail_call == 1 ? 2 : 0);
    }
}

static void test_bootdata_preserves_valid_data(void** state)
{
    (void)state;
    const uint8_t markers[][MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN] = {
        {0},
        {0xFF, 0xFF, 0xFF, 0xFF},
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_MAGIC,
        {0xA5, 0x5A, 0x12, 0x34}};
    for (size_t i = 0; i < sizeof(markers) / sizeof(markers[0]); i++) {
        _reset_flash();
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            markers[i],
            sizeof(markers[i]));
        _flash_bootdata.fields.show_firmware_hash = 1;
        _flash_bootdata.bytes[CHUNK_SIZE - 1] = 0x42;
        assert_true(bootdata_hash_set(&_flash_bootdata));
        boot_data_t original = _flash_bootdata;
        boot_data_t result;
        assert_true(bootdata_init(&result));
        assert_true(bootdata_read(&result));
        assert_memory_equal(result.bytes, original.bytes, sizeof(original.bytes));
        assert_memory_equal(_flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
        assert_int_equal(_bootdata_writes, 0);
        assert_int_equal(
            _migration_consumed_writes,
            memcmp(markers[i], _migration_consumed_magic, sizeof(_migration_consumed_magic)) != 0);
    }
}

#ifdef BOOTLOADER_DEVELOPMENT
static void _assert_dev_reset(const boot_data_t* result)
{
    boot_data_t expected = _bootdata(0, 0);
    assert_true(bootdata_hash_set(&expected));
    assert_memory_equal(result->bytes, expected.bytes, sizeof(expected.bytes));
    assert_memory_equal(_flash_bootdata.bytes, expected.bytes, sizeof(expected.bytes));
    assert_memory_equal(
        _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        _migration_consumed_magic,
        sizeof(_migration_consumed_magic));
}

static void test_bootdata_dev_recovers_corrupt_floors(void** state)
{
    (void)state;
    const version_t corrupt_versions[] = {0, UINT32_MAX};
    const uint8_t markers[][MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN] = {
        {0},
        {0xFF, 0xFF, 0xFF, 0xFF},
        MEMORY_BOOTDATA_MIGRATION_CONSUMED_MAGIC,
        {0xA5, 0x5A, 0x12, 0x34}};
    for (size_t field = 0; field < 2; field++) {
        for (size_t i = 0; i < sizeof(corrupt_versions) / sizeof(corrupt_versions[0]); i++) {
            for (size_t marker = 0; marker < sizeof(markers) / sizeof(markers[0]); marker++) {
                _reset_flash();
                memcpy(
                    _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
                    markers[marker],
                    sizeof(markers[marker]));
                assert_true(bootdata_hash_set(&_flash_bootdata));
                if (field == 0) {
                    _flash_bootdata.fields.signing_pubkeys_version = corrupt_versions[i];
                } else {
                    _flash_bootdata.fields.firmware_version = corrupt_versions[i];
                }
                uint8_t expected_marker_chunk[CHUNK_SIZE];
                memcpy(
                    expected_marker_chunk,
                    _migration_consumed_chunk,
                    sizeof(expected_marker_chunk));
                memcpy(
                    expected_marker_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
                    _migration_consumed_magic,
                    sizeof(_migration_consumed_magic));
                boot_data_t result;
                assert_false(bootdata_read(&result));
                assert_int_equal(_write_calls, 0);
                assert_true(bootdata_dev_recover(&result));
                _assert_dev_reset(&result);
                assert_memory_equal(
                    _migration_consumed_chunk,
                    expected_marker_chunk,
                    sizeof(expected_marker_chunk));
                assert_int_equal(
                    _migration_consumed_writes,
                    memcmp(
                        markers[marker],
                        _migration_consumed_magic,
                        sizeof(_migration_consumed_magic)) != 0);
                assert_int_equal(_bootdata_writes, 1);
                assert_true(bootdata_init(&result));
                assert_int_equal(_bootdata_writes, 1);
            }
        }
    }
}

static void test_bootdata_dev_recovers_missing_or_invalid_hash(void** state)
{
    (void)state;
    const uint8_t hashes[] = {0, 0xA5, UINT8_MAX};
    for (size_t i = 0; i < sizeof(hashes); i++) {
        _reset_flash();
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        _flash_bootdata.fields.signing_pubkeys_version = UINT32_MAX;
        _flash_bootdata.fields.firmware_version = UINT32_MAX;
        memset(_flash_bootdata.fields.versions_hash, hashes[i], SHA256_DIGEST_LENGTH);
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_dev_recover(&result));
        _assert_dev_reset(&result);
        assert_int_equal(_migration_consumed_writes, 0);
        assert_int_equal(_bootdata_writes, 1);
    }
}

static void test_bootdata_dev_recovers_erased_or_uninitialized_data(void** state)
{
    (void)state;
    const uint8_t fills[] = {0, 0xA5, UINT8_MAX};
    for (size_t i = 0; i < sizeof(fills); i++) {
        _reset_flash();
        memset(_flash_bootdata.bytes, fills[i], sizeof(_flash_bootdata.bytes));
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_dev_recover(&result));
        _assert_dev_reset(&result);
    }
    for (size_t marker = 0; marker < 2; marker++) {
        _reset_flash();
        assert_true(bootdata_hash_set(&_flash_bootdata));
        _flash_bootdata.fields.is_initialized[marker] = 0;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_dev_recover(&result));
        _assert_dev_reset(&result);
    }
}

static void test_bootdata_dev_recovers_read_errors(void** state)
{
    (void)state;
    for (size_t read = 1; read <= 2; read++) {
        _reset_flash();
        assert_true(bootdata_hash_set(&_flash_bootdata));
        _fail_read_call = read;
        boot_data_t result;
        assert_false(bootdata_init(&result));
        assert_int_equal(_write_calls, 0);
        assert_true(bootdata_dev_recover(&result));
        _assert_dev_reset(&result);
    }
    _reset_flash();
    assert_true(bootdata_hash_set(&_flash_bootdata));
    _unstable_read_call = 2;
    boot_data_t result;
    assert_false(bootdata_init(&result));
    assert_int_equal(_write_calls, 0);
    assert_true(bootdata_dev_recover(&result));
    _assert_dev_reset(&result);
}

static void test_bootdata_dev_recovers_on_api_read(void** state)
{
    (void)state;
    _reset_flash();
    assert_true(bootdata_hash_set(&_flash_bootdata));
    boot_data_t result;
    assert_true(bootdata_init(&result));
    _reset_io();
    _flash_bootdata.fields.firmware_version = UINT32_MAX;
    assert_false(bootdata_read(&result));
    assert_int_equal(_write_calls, 0);
    assert_true(bootdata_dev_recover(&result));
    _assert_dev_reset(&result);

    // The replacement firmware's normal version checks now see zero floors, not corrupt highs.
    result.fields.signing_pubkeys_version = 1;
    result.fields.firmware_version = 1;
    assert_true(bootdata_write(&result));
    assert_true(bootdata_read(&result));
    assert_int_equal(result.fields.signing_pubkeys_version, 1);
    assert_int_equal(result.fields.firmware_version, 1);
    assert_int_equal(_migration_consumed_writes, 0);
}

static void test_bootdata_dev_interrupted_recovery(void** state)
{
    (void)state;
    const size_t cuts[] = {
        0,
        FLASH_PAGE_SIZE,
        BOOTDATA_VERSIONS_HASH_OFFSET,
        BOOTDATA_VERSIONS_HASH_OFFSET + SHA256_DIGEST_LENGTH - 1,
        CHUNK_SIZE};
    for (size_t i = 0; i < sizeof(cuts) / sizeof(cuts[0]); i++) {
        _reset_flash();
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        _fail_write_call = 1;
        _cut_after_bytes = cuts[i];
        boot_data_t result;
        assert_false(bootdata_dev_recover(&result));
        assert_int_equal(_bootdata_writes, 1);
        _reset_io();
        // No permanent panic latch: recovery can be retried after another power cut.
        if (!bootdata_init(&result)) {
            assert_true(bootdata_dev_recover(&result));
        }
        _assert_dev_reset(&result);
        assert_true(bootdata_read(&result));
    }
}

static void test_bootdata_dev_checks_recovery_write(void** state)
{
    (void)state;
    _reset_flash();
    memcpy(
        _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        _migration_consumed_magic,
        sizeof(_migration_consumed_magic));
    _corrupt_write_call = 1;
    boot_data_t result;
    assert_false(bootdata_dev_recover(&result));

    // Check both complete write readback and the final common bootdata_read() validation.
    for (size_t read = 3; read <= 8; read++) {
        _reset_flash();
        memcpy(
            _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
            _migration_consumed_magic,
            sizeof(_migration_consumed_magic));
        _fail_read_call = read;
        assert_false(bootdata_dev_recover(&result));
    }

    _reset_flash();
    memcpy(
        _migration_consumed_chunk + MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET,
        _migration_consumed_magic,
        sizeof(_migration_consumed_magic));
    _sha_succeeds = false;
    assert_false(bootdata_dev_recover(&result));
    assert_int_equal(_write_calls, 0);
}

static void test_bootdata_dev_checks_recovery_migration_consumed(void** state)
{
    (void)state;
    boot_data_t result;
    for (size_t read = 1; read <= 6; read++) {
        _reset_flash();
        boot_data_t original = _flash_bootdata;
        _fail_read_call = read;
        assert_false(bootdata_dev_recover(&result));
        assert_int_equal(_bootdata_writes, 0);
        assert_memory_equal(_flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
    }
    _reset_flash();
    _corrupt_write_call = 1;
    assert_false(bootdata_dev_recover(&result));
    assert_int_equal(_bootdata_writes, 0);

    // Cuts anywhere in the marker must remain retryable without reopening migration permission.
    for (size_t cut = 0; cut <= MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN; cut++) {
        _reset_flash();
        boot_data_t original = _flash_bootdata;
        _fail_write_call = 1;
        _cut_after_bytes = MEMORY_BOOTDATA_MIGRATION_CONSUMED_OFFSET + cut;
        assert_false(bootdata_dev_recover(&result));
        assert_int_equal(_bootdata_writes, 0);
        assert_memory_equal(_flash_bootdata.bytes, original.bytes, sizeof(original.bytes));
        _reset_io();
        assert_true(bootdata_dev_recover(&result));
        _assert_dev_reset(&result);
        assert_int_equal(_migration_consumed_writes, cut < MEMORY_BOOTDATA_MIGRATION_CONSUMED_LEN);
        assert_true(bootdata_read(&result));
    }
}
#endif

static void test_bootdata_io_null_arguments(void** state)
{
    (void)state;
    _reset_flash();
    assert_false(bootdata_init(NULL));
    assert_false(bootdata_read(NULL));
    assert_false(bootdata_write(NULL));
#ifdef BOOTLOADER_DEVELOPMENT
    assert_false(bootdata_dev_recover(NULL));
#endif
    assert_int_equal(_read_calls, 0);
    assert_int_equal(_write_calls, 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_migration_consumed_persistent_format),
        cmocka_unit_test(test_set_known_hash),
        cmocka_unit_test(test_hash_check_zero),
        cmocka_unit_test(test_hash_check_valid_and_changed_versions),
        cmocka_unit_test(test_hash_check_nonzero_mismatches),
        cmocka_unit_test(test_hash_failures),
        cmocka_unit_test(test_null_arguments),
        cmocka_unit_test(test_bootdata_init_legacy_once),
        cmocka_unit_test(test_bootdata_init_invalid_initialization),
        cmocka_unit_test(test_bootdata_init_accepts_nonmagic_marker),
        cmocka_unit_test(test_bootdata_init_never_reseals_consumed_migration),
        cmocka_unit_test(test_bootdata_init_accepts_any_hash_before_migration),
        cmocka_unit_test(test_bootdata_init_restores_consumed_marker_before_writes),
        cmocka_unit_test(test_bootdata_init_migration_consumed_power_cuts),
        cmocka_unit_test(test_bootdata_init_bootdata_power_cuts),
        cmocka_unit_test(test_bootdata_init_read_failures),
        cmocka_unit_test(test_bootdata_init_checks_full_readback),
        cmocka_unit_test(test_bootdata_write_power_cut),
        cmocka_unit_test(test_bootdata_init_hash_failure),
        cmocka_unit_test(test_bootdata_preserves_valid_data),
#ifdef BOOTLOADER_DEVELOPMENT
        cmocka_unit_test(test_bootdata_dev_recovers_corrupt_floors),
        cmocka_unit_test(test_bootdata_dev_recovers_missing_or_invalid_hash),
        cmocka_unit_test(test_bootdata_dev_recovers_erased_or_uninitialized_data),
        cmocka_unit_test(test_bootdata_dev_recovers_read_errors),
        cmocka_unit_test(test_bootdata_dev_recovers_on_api_read),
        cmocka_unit_test(test_bootdata_dev_interrupted_recovery),
        cmocka_unit_test(test_bootdata_dev_checks_recovery_write),
        cmocka_unit_test(test_bootdata_dev_checks_recovery_migration_consumed),
#endif
        cmocka_unit_test(test_bootdata_io_null_arguments),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
