// SPDX-License-Identifier: Apache-2.0

#define _POSIX_C_SOURCE 200809L

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <flags.h>
#include <memory/bitbox02_smarteeprom.h>
#include <memory/memory.h>
#include <memory/smarteeprom.h>
#include <rust/rust.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static int _writes_before_interrupt = -1;
static size_t _bytes_before_interrupt;
static size_t _eeprom_writes;
static const char* _test_executable;
static uint8_t _legacy_chunk[CHUNK_SIZE];
static bool _allow_legacy_reads;

#define LEGACY_DEVICE_NAME_OFFSET 228
#define EEPROM_IMAGE_SIZE 160

void __real_smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer);

void __wrap_smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer)
{
    if (_writes_before_interrupt == 0) {
        if (_bytes_before_interrupt > 0) {
            assert_true(_bytes_before_interrupt < bytes);
            __real_smarteeprom_write(address, _bytes_before_interrupt, buffer);
        }
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
    assert_true(_allow_legacy_reads);
    assert_int_equal(chunk_num, 1);
    memcpy(chunk, _legacy_chunk, sizeof(_legacy_chunk));
}

bool __wrap_memory_write_chunk_fake(uint32_t chunk_num, const uint8_t* chunk)
{
    fail_msg("SmartEEPROM must not write appdata");
    return false;
}

void __wrap_AbortAutoenter(const char* msg)
{
    check_expected(msg);
    mock_assert(false, "AbortAutoenter", __FILE__, __LINE__);
}

static int _setup(void** state)
{
    (void)state;
    smarteeprom_setup();
    _writes_before_interrupt = -1;
    _bytes_before_interrupt = 0;
    _eeprom_writes = 0;
    memset(_legacy_chunk, 0xff, sizeof(_legacy_chunk));
    _allow_legacy_reads = true;
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

static void _test_init_migrates(uint8_t version)
{
    const uint8_t previous_values[] = {0, 1, 0xff, 0xa5};
    const char* names[] = {
        "BïtBöx Zürich",
        "DeviceName_ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxy",
        NULL,
        "invalid\nname",
    };
    for (size_t i = 0; i < sizeof(previous_values); i++) {
        memset(_legacy_chunk, 0xff, sizeof(_legacy_chunk));
        const bool passphrase_enabled = i % 2 != 0;
        if (passphrase_enabled) {
            _legacy_chunk[0] &= ~(1u << 2);
        }
        if (names[i] != NULL) {
            memset(_legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, 0, MEMORY_DEVICE_MAX_LEN_WITH_NULL);
            memcpy(_legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, names[i], strlen(names[i]));
        }
        _allow_legacy_reads = true;
        uint8_t image[EEPROM_IMAGE_SIZE];
        memset(image, 0xa5, sizeof(image));
        image[0] = version;
        image[32] = MAX_UNLOCK_ATTEMPTS;
        // Unused v1 bytes are not assumed to be erased, even if they match a language code.
        image[64] = previous_values[i];
        __real_smarteeprom_write(0, sizeof(image), image);

        bitbox02_smarteeprom_init();
        _allow_legacy_reads = false;

        if (version == 1) {
            image[64] = 0;
        }
        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(),
            image[64] == 1 ? BITBOX02_SMARTEEPROM_LANGUAGE_DE : BITBOX02_SMARTEEPROM_LANGUAGE_EN);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), MAX_UNLOCK_ATTEMPTS);
        assert_int_equal(memory_is_mnemonic_passphrase_enabled(), passphrase_enabled);
        image[0] = 3;
        image[65] = passphrase_enabled;
        memcpy(
            image + 96, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, MEMORY_DEVICE_MAX_LEN_WITH_NULL);
        uint8_t actual[sizeof(image)];
        smarteeprom_read(0, sizeof(actual), actual);
        assert_memory_equal(actual, image, sizeof(image));
    }
}

static void test_init_migrates_v1(void** state)
{
    (void)state;
    _test_init_migrates(1);
}

static void test_init_migrates_v2(void** state)
{
    (void)state;
    _test_init_migrates(2);
}

static void test_init_uninitialized(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    assert_int_equal(_read_byte(0), 3);
    assert_int_equal(_read_byte(64), 0);
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 0);
    assert_false(memory_is_mnemonic_passphrase_enabled());
    uint8_t name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
    bitbox02_smarteeprom_get_device_name(name);
    assert_memory_equal(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name));
}

static void test_init_preserves_v3(void** state)
{
    (void)state;
    uint8_t image[EEPROM_IMAGE_SIZE];
    memset(image, 0xa5, sizeof(image));
    image[0] = 3;
    image[32] = 5;
    image[64] = 1;
    image[65] = 1;
    memset(image + 96, 0, MEMORY_DEVICE_MAX_LEN_WITH_NULL);
    memcpy(image + 96, "Stored name", sizeof("Stored name"));
    __real_smarteeprom_write(0, sizeof(image), image);

    _allow_legacy_reads = false;
    bitbox02_smarteeprom_init();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_DE);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 5);
    assert_true(memory_is_mnemonic_passphrase_enabled());
    char name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
    memory_get_device_name(name);
    assert_string_equal(name, "Stored name");
    assert_int_equal(_eeprom_writes, 0);
}

static void _test_init_retries_interrupted_migration(uint8_t version)
{
    _legacy_chunk[0] &= ~(1u << 2);
    memset(_legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, 'x', MEMORY_DEVICE_MAX_LEN_WITH_NULL - 1);
    _legacy_chunk[LEGACY_DEVICE_NAME_OFFSET + MEMORY_DEVICE_MAX_LEN_WITH_NULL - 1] = 0;
    const int writes = version == 1 ? 5 : 3;
    for (volatile int writes_before_interrupt = 0; writes_before_interrupt < writes;
         writes_before_interrupt++) {
        uint8_t image[EEPROM_IMAGE_SIZE];
        memset(image, 0xa5, sizeof(image));
        image[0] = version;
        image[32] = 7;
        image[64] = 1;
        __real_smarteeprom_write(0, sizeof(image), image);

        _writes_before_interrupt = writes_before_interrupt;
        expect_assert_failure(bitbox02_smarteeprom_init());
        assert_int_equal(_read_byte(0), version == 1 && writes_before_interrupt < 2 ? 1 : 2);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 7);

        _writes_before_interrupt = -1;
        bitbox02_smarteeprom_init();
        assert_int_equal(_read_byte(0), 3);
        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(),
            version == 1 ? BITBOX02_SMARTEEPROM_LANGUAGE_EN : BITBOX02_SMARTEEPROM_LANGUAGE_DE);
        assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 7);
        assert_true(memory_is_mnemonic_passphrase_enabled());
        uint8_t name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
        bitbox02_smarteeprom_get_device_name(name);
        assert_memory_equal(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name));
    }
}

static void test_init_retries_interrupted_v1_migration(void** state)
{
    (void)state;
    _test_init_retries_interrupted_migration(1);
}

static void test_init_retries_interrupted_v2_migration(void** state)
{
    (void)state;
    _test_init_retries_interrupted_migration(2);
}

static void test_init_retries_partial_name_migration(void** state)
{
    (void)state;
    uint8_t image[EEPROM_IMAGE_SIZE];
    memset(image, 0xa5, sizeof(image));
    image[0] = 2;
    image[32] = 7;
    image[64] = 1;
    __real_smarteeprom_write(0, sizeof(image), image);
    _legacy_chunk[0] &= ~(1u << 2);
    memset(_legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, 'x', MEMORY_DEVICE_MAX_LEN_WITH_NULL - 1);
    _legacy_chunk[LEGACY_DEVICE_NAME_OFFSET + MEMORY_DEVICE_MAX_LEN_WITH_NULL - 1] = 0;

    // The name spans two pages; interruption after flushing the first must be recoverable.
    _writes_before_interrupt = 1;
    _bytes_before_interrupt = 32;
    expect_assert_failure(bitbox02_smarteeprom_init());
    assert_int_equal(_read_byte(0), 2);
    assert_int_equal(_read_byte(96), 'x');
    assert_int_equal(_read_byte(128), 0xa5);

    _writes_before_interrupt = -1;
    bitbox02_smarteeprom_init();
    assert_int_equal(_read_byte(0), 3);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 7);
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_DE);
    assert_true(memory_is_mnemonic_passphrase_enabled());
    uint8_t name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
    bitbox02_smarteeprom_get_device_name(name);
    assert_memory_equal(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name));
}

static void test_init_rejects_unknown_version(void** state)
{
    (void)state;
    const uint8_t version = 4;
    __real_smarteeprom_write(0, sizeof(version), &version);
    expect_string(__wrap_AbortAutoenter, msg, "Unrecognized SmartEEPROM version.\nGot 4");
    expect_assert_failure(bitbox02_smarteeprom_init());
    assert_int_equal(_eeprom_writes, 0);
}

static void test_device_language_storage(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    _allow_legacy_reads = false;
    bitbox02_smarteeprom_increment_unlock_attempts();
    uint8_t before[EEPROM_IMAGE_SIZE];
    smarteeprom_read(0, sizeof(before), before);

    const bitbox02_smarteeprom_language_t languages[] = {
        BITBOX02_SMARTEEPROM_LANGUAGE_DE,
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

static void test_device_settings_storage(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    _allow_legacy_reads = false;
    bitbox02_smarteeprom_increment_unlock_attempts();
    assert_true(bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE));
    uint8_t expected[EEPROM_IMAGE_SIZE];
    smarteeprom_read(0, sizeof(expected), expected);

    const char* device_name = "BïtBöx Zürich";
    assert_true(memory_set_device_name(device_name, strlen(device_name)));
    memset(expected + 96, 0, MEMORY_DEVICE_MAX_LEN_WITH_NULL);
    memcpy(expected + 96, device_name, strlen(device_name));
    char name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
    memory_get_device_name(name);
    assert_string_equal(name, device_name);

    for (uint8_t enabled = 0; enabled <= 1; enabled++) {
        assert_true(memory_set_mnemonic_passphrase_enabled(enabled));
        assert_int_equal(memory_is_mnemonic_passphrase_enabled(), enabled);
        expected[65] = enabled;
        uint8_t actual[sizeof(expected)];
        smarteeprom_read(0, sizeof(actual), actual);
        assert_memory_equal(actual, expected, sizeof(expected));
    }
}

static void test_reset_device_settings_before_migration(void** state)
{
    (void)state;
    for (uint8_t version = 1; version <= 2; version++) {
        __real_smarteeprom_write(0, sizeof(version), &version);
        bitbox02_smarteeprom_reset_device_settings();
        assert_int_equal(_eeprom_writes, 0);
        assert_int_equal(_read_byte(0), version);
    }
}

static void test_device_language_invalid_stored_value(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    const uint8_t invalid_values[] = {2, 0xff};
    for (size_t i = 0; i < sizeof(invalid_values); i++) {
        __real_smarteeprom_write(64, 1, &invalid_values[i]);
        assert_int_equal(
            bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    }
}

static void test_device_settings_reset(void** state)
{
    (void)state;
    bitbox02_smarteeprom_init();
    assert_true(bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE));
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_DE);
    assert_true(memory_set_mnemonic_passphrase_enabled(true));
    assert_true(memory_set_device_name("Custom name", strlen("Custom name")));

    // A device reset disables SmartEEPROM until the next boot.
    smarteeprom_disable();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_false(bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE));
    assert_false(memory_is_mnemonic_passphrase_enabled());
    assert_false(memory_set_mnemonic_passphrase_enabled(true));
    assert_false(memory_set_device_name("Custom name", strlen("Custom name")));
    uint8_t name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
    bitbox02_smarteeprom_get_device_name(name);
    assert_memory_equal(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name));
    bitbox02_smarteeprom_reset_device_settings();

    smarteeprom_bb02_config();
    bitbox02_smarteeprom_init();
    assert_int_equal(bitbox02_smarteeprom_get_device_language(), BITBOX02_SMARTEEPROM_LANGUAGE_EN);
    assert_int_equal(bitbox02_smarteeprom_get_unlock_attempts(), 0);
    assert_false(memory_is_mnemonic_passphrase_enabled());
    bitbox02_smarteeprom_get_device_name(name);
    assert_memory_equal(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name));
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

static void test_device_settings_persistence(void** state)
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
        memset(_legacy_chunk, 0xff, sizeof(_legacy_chunk));
        _allow_legacy_reads = true;
        smarteeprom_setup();
        bitbox02_smarteeprom_init();
        _allow_legacy_reads = false;
        if (strcmp(argv[1], "write") == 0) {
            bitbox02_smarteeprom_increment_unlock_attempts();
            return bitbox02_smarteeprom_set_device_language(BITBOX02_SMARTEEPROM_LANGUAGE_DE) &&
                           memory_set_mnemonic_passphrase_enabled(true) &&
                           memory_set_device_name("Persistent name", strlen("Persistent name"))
                       ? 0
                       : 1;
        }
        if (strcmp(argv[1], "read") == 0) {
            char name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
            memory_get_device_name(name);
            return bitbox02_smarteeprom_get_device_language() == BITBOX02_SMARTEEPROM_LANGUAGE_DE &&
                           bitbox02_smarteeprom_get_unlock_attempts() == 1 &&
                           memory_is_mnemonic_passphrase_enabled() &&
                           strcmp(name, "Persistent name") == 0
                       ? 0
                       : 1;
        }
        if (strcmp(argv[1], "reset") == 0) {
            smarteeprom_disable();
            return 0;
        }
        if (strcmp(argv[1], "read-reset") == 0) {
            uint8_t name[MEMORY_DEVICE_MAX_LEN_WITH_NULL];
            bitbox02_smarteeprom_get_device_name(name);
            return bitbox02_smarteeprom_get_device_language() == BITBOX02_SMARTEEPROM_LANGUAGE_EN &&
                           bitbox02_smarteeprom_get_unlock_attempts() == 0 &&
                           !memory_is_mnemonic_passphrase_enabled() &&
                           memcmp(name, _legacy_chunk + LEGACY_DEVICE_NAME_OFFSET, sizeof(name)) ==
                               0
                       ? 0
                       : 1;
        }
        return 1;
    }
    const struct CMUnitTest tests[] = {
        cmocka_unit_test_setup_teardown(test_init_migrates_v1, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_migrates_v2, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_uninitialized, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_preserves_v3, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_init_retries_interrupted_v1_migration, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_init_retries_interrupted_v2_migration, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_init_retries_partial_name_migration, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_init_rejects_unknown_version, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_language_storage, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_settings_storage, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_reset_device_settings_before_migration, _setup, _teardown),
        cmocka_unit_test_setup_teardown(
            test_device_language_invalid_stored_value, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_settings_reset, _setup, _teardown),
        cmocka_unit_test_setup_teardown(test_device_settings_persistence, _setup, _teardown),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
