// SPDX-License-Identifier: Apache-2.0

#define _POSIX_C_SOURCE 200809L

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <memory/bitbox02_smarteeprom.h>
#include <memory/smarteeprom.h>
#include <rust/rust.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static int _writes_before_interrupt = -1;
static size_t _eeprom_writes;
static const char* _test_executable;

void __real_smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer);

void __wrap_smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer)
{
    if (_writes_before_interrupt == 0) {
        mock_assert(false, "Interrupted EEPROM write", __FILE__, __LINE__);
    }
    if (_writes_before_interrupt > 0) {
        _writes_before_interrupt--;
    }
    _eeprom_writes++;
    __real_smarteeprom_write(address, bytes, buffer);
}

void __wrap_memory_read_chunk_fake(uint32_t chunk_num, uint8_t* chunk)
{
    fail_msg("SmartEEPROM must not read appdata");
}

bool __wrap_memory_write_chunk_fake(uint32_t chunk_num, const uint8_t* chunk)
{
    fail_msg("SmartEEPROM must not write appdata");
    return false;
}

void __wrap_AbortAutoenter(const char* msg)
{
    check_expected(msg);
    mock_assert(false, msg, __FILE__, __LINE__);
}

static int _setup(void** state)
{
    (void)state;
    smarteeprom_setup();
    _writes_before_interrupt = -1;
    _eeprom_writes = 0;
    return 0;
}

static int _teardown(void** state)
{
    (void)state;
    if (smarteeprom_is_enabled()) {
        smarteeprom_disable();
    }
    return 0;
}

static uint8_t _read_byte(size_t address)
{
    uint8_t value;
    smarteeprom_read(address, sizeof(value), &value);
    return value;
}

static void test_init_migrates_v1(void** state)
{
    (void)state;
    const uint8_t previous_values[] = {0, 1, 0xff, 0xa5};
    for (size_t i = 0; i < sizeof(previous_values); i++) {
        uint8_t image[96];
        memset(image, 0xa5, sizeof(image));
        image[0] = 1;
        image[32] = MAX_UNLOCK_ATTEMPTS;
        // Unused v1 bytes are not assumed to be erased, even if they match a language code.
        image[64] = previous_values[i];
        __real_smarteeprom_write(0, sizeof(image), image);

        bitbox02_smarteeprom_init();

        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), MAX_UNLOCK_ATTEMPTS);
        image[0] = 2;
        image[64] = 0;
        uint8_t actual[sizeof(image)];
        smarteeprom_read(0, sizeof(actual), actual);
        assert_memory_equal(actual, image, sizeof(image));
    }
}

static void test_init_uninitialized(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    assert_int_equal(_read_byte(0), 2);
    assert_int_equal(_read_byte(64), 0);
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 0);
}

static void test_init_preserves_v2(void** state)
{
    (void)state;
    uint8_t image[96];
    memset(image, 0xa5, sizeof(image));
    image[0] = 2;
    image[32] = 5;
    image[64] = 1;
    __real_smarteeprom_write(0, sizeof(image), image);

    bitbox02_smarteeprom_init();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_DE);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 5);
    assert_int_equal(_eeprom_writes, 0);
}

static void test_init_retries_interrupted_migration(void** state)
{
    (void)state;
    for (volatile int writes_before_interrupt = 0; writes_before_interrupt <= 1;
         writes_before_interrupt++) {
        uint8_t image[96];
        memset(image, 0xa5, sizeof(image));
        image[0] = 1;
        image[32] = 7;
        __real_smarteeprom_write(0, sizeof(image), image);

        _writes_before_interrupt = writes_before_interrupt;
        expect_assert_failure(bitbox02_smarteeprom_init());
        assert_int_equal(_read_byte(0), 1);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 7);

        _writes_before_interrupt = -1;
        bitbox02_smarteeprom_init();
        assert_int_equal(_read_byte(0), 2);
        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 7);
    }
}

static void test_init_rejects_unknown_version(void** state)
{
    (void)state;
    const uint8_t version = 3;
    __real_smarteeprom_write(0, sizeof(version), &version);
    expect_string(__wrap_AbortAutoenter, msg, "Unrecognized SmartEEPROM version.\nGot 3");
    expect_assert_failure(bitbox02_smarteeprom_init());
    assert_int_equal(_eeprom_writes, 0);
}

static void test_device_language_storage(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    bitbox02_smarteeprom_increment_unlock_attempts();
    uint8_t before[96];
    smarteeprom_read(0, sizeof(before), before);

    const bitbox02_smarteeprom_language_t languages[] = {
        BITBOX02_SMARTEEPROM_LANGUAGE_DE,
        BITBOX02_SMARTEEPROM_LANGUAGE_IT,
        BITBOX02_SMARTEEPROM_LANGUAGE_ES,
        BITBOX02_SMARTEEPROM_LANGUAGE_EN,
    };
    for (size_t i = 0; i < sizeof(languages) / sizeof(languages[0]); i++) {
        assert_true(bitbox02_smarteeprom_set_device_language(languages[i]));
        assert_int_equal(bitbox02_smarteeprom_get_device_language(), languages[i]);
        before[64] = (uint8_t)languages[i];
        uint8_t after[sizeof(before)];
        smarteeprom_read(0, sizeof(after), after);
        assert_memory_equal(after, before, sizeof(before));
    }
    _eeprom_writes = 0;
    assert_false(bitbox02_smarteeprom_set_device_language((bitbox02_smarteeprom_language_t)0xff));
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_int_equal(_eeprom_writes, 0);
}

static void test_device_language_invalid_stored_value(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    const uint8_t invalid_values[] = {4, 0xff};
    for (size_t i = 0; i < sizeof(invalid_values); i++) {
        __real_smarteeprom_write(64, 1, &invalid_values[i]);
        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    }
}

static void test_device_language_reset(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    assert_true(bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE));
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_DE);

    // A device reset disables SmartEEPROM until the next boot.
    smarteeprom_disable();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_false(bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE));

    smarteeprom_bb02_config();
    bitbox02_smarteeprom_init();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 0);
}

static void _run_persistence_process(const char* mode, const char* path)
{
    pid_t pid = fork();
    assert_true(pid >= 0);
    if (pid == 0) {
        if (setenv("FAKE_MEMORY_FILEPATH", path, 1) != 0) {
            _exit(127);
        }
        // Under Valgrind, /proc/self/exe points to Valgrind instead of this test binary.
        execlp(_test_executable, _test_executable, mode, NULL);
        _exit(127);
    }
    int status;
    assert_int_equal(waitpid(pid, &status, 0), pid);
    assert_true(WIFEXITED(status));
    assert_int_equal(WEXITSTATUS(status), 0);
}

static void test_device_language_persistence(void** state)
{
    (void)state;
    char directory[] = "/tmp/bitbox02-smarteeprom-XXXXXX";
    assert_non_null(mkdtemp(directory));
    char path[128];
    int written = snprintf(path, sizeof(path), "%s/memory", directory);
    assert_true(written > 0 && (size_t)written < sizeof(path));

    _run_persistence_process("write", path);
    _run_persistence_process("read", path);
    _run_persistence_process("reset", path);
    _run_persistence_process("read-reset", path);

    char image_path[160];
    written = snprintf(image_path, sizeof(image_path), "%s_smarteeprom", path);
    assert_true(written > 0 && (size_t)written < sizeof(image_path));
    assert_int_equal(unlink(image_path), 0);
    assert_int_equal(rmdir(directory), 0);
}

int main(int argc, char** argv)
{
    _test_executable = argv[0];
    if (argc == 2) {
        smarteeprom_setup();
        bitbox02_smarteeprom_init();
        if (strcmp(argv[1], "write") == 0) {
            bitbox02_smarteeprom_increment_unlock_attempts();
            return bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE) ? 0
                                                                                              : 1;
        }
        if (strcmp(argv[1], "read") == 0) {
            return bitbox02_smarteeprom_get_device_language() == BITBOX02_SMARTEEPROM_LANGUAGE_DE &&
                           bitbox02_smarteeprom_get_unlock_attempts() == 1
                       ? 0
                       : 1;
        }
        if (strcmp(argv[1], "reset") == 0) {
            smarteeprom_disable();
            return 0;
        }
        if (strcmp(argv[1], "read-reset") == 0) {
            return bitbox02_smarteeprom_get_device_language() == BITBOX02_SMARTEEPROM_LANGUAGE_EN &&
                           bitbox02_smarteeprom_get_unlock_attempts() == 0
                       ? 0
                       : 1;
        }
        return 1;
    }
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_init_migrates_v1, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_uninitialized, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_preserves_v2, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_retries_interrupted_migration, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_rejects_unknown_version, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_language_storage, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_device_language_invalid_stored_value, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_language_reset, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_language_persistence, _setup, _teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
