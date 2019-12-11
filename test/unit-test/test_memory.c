// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <memory/memory.h>
#include <usb/noise.h>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define CHUNK_SIZE (16 * 512) // 8kB.

#define FLASH_APP_DATA_LEN (0x000010000)
#define FLASH_SHARED_DATA_START (0xe000)

// chunk 0
static const int _addr_factory_setup_done = 0;
static const uint8_t _factory_setup_done = sectrue_u8;
static const int _addr_iokey = 4;
static const int _addr_authkey = _addr_iokey + 32;
static const int _addr_enckey = _addr_authkey + 32;
static const int _addr_attestation_pubkey = _addr_enckey + 32;
static const int _addr_attestation_certificate = _addr_attestation_pubkey + 64;
static const int _addr_attestation_root_pubkey_identifier = _addr_attestation_certificate + 64;

// chunk 1
static const int _addr_bitmask = 0;
static const int _addr_failed_unlock_attempts = 1;
static const int _addr_noise_static_private_key = 4;
static const int _addr_noise_remote_static_pubkeys = _addr_noise_static_private_key + 32;
static const int _addr_salt_root = _addr_noise_remote_static_pubkeys + 5 * NOISE_PUBKEY_SIZE;
static const int _addr_device_name = _addr_salt_root + 32;
static const int _addr_seed_birthdate = _addr_device_name + MEMORY_DEVICE_NAME_MAX_LEN + 1 + 96;
static const uint8_t _bitmask_seeded = (1 << 0);
static const uint8_t _bitmask_initialized = (1 << 1);
static const uint8_t _bitmask_mnemonic_passphrase_enabled = (1 << 2);

// shared chunk
static const int _addr_iokey_split = 4;
static const int _addr_authkey_split = _addr_iokey_split + 32;
static const int _addr_enckey_split = _addr_authkey_split + 32;

__extension__ static uint8_t _io_key[] = {[0 ... 32 - 1] = 0x78};
__extension__ static uint8_t _io_key_split[] = {[0 ... 32 - 1] = 0x79};
__extension__ static uint8_t _auth_key[] = {[0 ... 32 - 1] = 0x80};
__extension__ static uint8_t _auth_key_split[] = {[0 ... 32 - 1] = 0x81};
__extension__ static uint8_t _enc_key[] = {[0 ... 32 - 1] = 0x82};
__extension__ static uint8_t _enc_key_split[] = {[0 ... 32 - 1] = 0x83};

__extension__ static uint8_t _salt_root[] = {[0 ... 32 - 1] = 0x84};

#define EMPTYCHUNK(var) __extension__ uint8_t var[] = {[0 ... CHUNK_SIZE - 1] = 0xFF};

void __wrap_memory_read_chunk_mock(uint32_t chunk_num, uint8_t* chunk_out)
{
    check_expected(chunk_num);
    memcpy(chunk_out, (uint8_t*)mock(), CHUNK_SIZE);
}

void __wrap_memory_read_shared_bootdata_mock(uint8_t* chunk_out)
{
    memcpy(chunk_out, (uint8_t*)mock(), CHUNK_SIZE);
}

bool __wrap_memory_write_chunk_mock(uint32_t chunk_num, uint8_t* chunk)
{
    check_expected(chunk_num);
    check_expected(chunk);
    return mock();
}

bool __wrap_memory_write_to_address_mock(uint32_t addr, uint8_t* chunk)
{
    check_expected(addr);
    check_expected(chunk);
    return true;
}

static uint8_t _noise_static_private_key[32] = {
    0x46, 0xf5, 0xd6, 0x76, 0x2a, 0xbc, 0xbd, 0xce, 0xe7, 0xe0, 0xa0, 0x2f, 0xe8, 0xa3, 0x8b, 0xe8,
    0x2a, 0xad, 0x11, 0x32, 0xb0, 0xe4, 0xd7, 0xed, 0xed, 0x1b, 0xd0, 0x2d, 0x9f, 0xa0, 0x37, 0xed,
};

bool __wrap_bb_noise_generate_static_private_key(uint8_t* private_key_out)
{
    memcpy(private_key_out, _noise_static_private_key, 32);
    return true;
}

static void _mock_random_32_bytes(uint8_t* buf)
{
    memcpy(buf, (uint8_t*)mock(), 32);
}

static memory_interface_functions_t _ifs = {
    .random_32_bytes = _mock_random_32_bytes,
};

static void _expect_reset(uint8_t* empty_chunk1, uint8_t* empty_chunk2)
{
    // Reset all
    for (uint32_t write_calls = 0; write_calls < FLASH_APP_DATA_LEN / CHUNK_SIZE - 1;
         write_calls++) {
        expect_value(__wrap_memory_write_chunk_mock, chunk_num, write_calls + 1);
        expect_value(__wrap_memory_write_chunk_mock, chunk, NULL);
        will_return(__wrap_memory_write_chunk_mock, true);
    }

    will_return(_mock_random_32_bytes, _salt_root);

    // Initialize chunk 1
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk1);

    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
    memcpy(&empty_chunk2[_addr_noise_static_private_key], _noise_static_private_key, 32);
    memcpy(&empty_chunk2[_addr_salt_root], _salt_root, 32);
    expect_memory(__wrap_memory_write_chunk_mock, chunk, empty_chunk2, CHUNK_SIZE);
    will_return(__wrap_memory_write_chunk_mock, true);
}

#define EXPECT_RESET          \
    EMPTYCHUNK(reset_chunk1); \
    EMPTYCHUNK(reset_chunk2); \
    _expect_reset(reset_chunk1, reset_chunk2);

static void _expect_setup(uint8_t* expected_chunk, uint8_t* expected_shared_chunk)
{
    memcpy(expected_chunk + _addr_authkey, _auth_key, 32);
    memcpy(expected_shared_chunk + _addr_authkey_split, _auth_key_split, 32);
    memcpy(expected_chunk + _addr_iokey, _io_key, 32);
    memcpy(expected_shared_chunk + _addr_iokey_split, _io_key_split, 32);
    memcpy(expected_chunk + _addr_enckey, _enc_key, 32);
    memcpy(expected_shared_chunk + _addr_enckey_split, _enc_key_split, 32);

    expect_value(__wrap_memory_write_to_address_mock, addr, FLASH_SHARED_DATA_START);
    expect_memory(__wrap_memory_write_to_address_mock, chunk, expected_shared_chunk, CHUNK_SIZE);

    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 0);
    expected_chunk[_addr_factory_setup_done] = _factory_setup_done;
    expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
}

static void _expect_keys(void)
{
    will_return(_mock_random_32_bytes, _io_key);
    will_return(_mock_random_32_bytes, _io_key_split);
    will_return(_mock_random_32_bytes, _auth_key);
    will_return(_mock_random_32_bytes, _auth_key_split);
    will_return(_mock_random_32_bytes, _enc_key);
    will_return(_mock_random_32_bytes, _enc_key_split);
}

static void _test_memory_setup(void** state)
{
    // Success if setup not yet done.
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    EMPTYCHUNK(empty_chunk);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk);
    EXPECT_RESET;

    _expect_keys();

    EMPTYCHUNK(empty_shared_chunk);
    will_return(__wrap_memory_read_shared_bootdata_mock, empty_shared_chunk);

    EMPTYCHUNK(setup_chunk);
    EMPTYCHUNK(shared_chunk);
    _expect_setup(setup_chunk, shared_chunk);
    will_return(__wrap_memory_write_chunk_mock, true);
    assert_true(memory_setup(&_ifs));

    // Success if setup already done before.
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    EMPTYCHUNK(chunk0_setupdone);
    chunk0_setupdone[_addr_factory_setup_done] = _factory_setup_done;
    will_return(__wrap_memory_read_chunk_mock, chunk0_setupdone);
    assert_true(memory_setup(&_ifs));
}

static void _fail_reset(void)
{
    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
    expect_value(__wrap_memory_write_chunk_mock, chunk, NULL);
    will_return(__wrap_memory_write_chunk_mock, false);
}

static void _test_memory_setup_failreset(void** state)
{
    // Fail because resetting failed.
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    EMPTYCHUNK(chunk0);
    will_return(__wrap_memory_read_chunk_mock, chunk0);
    _fail_reset();
    assert_false(memory_setup(&_ifs));
}

static void _test_memory_setup_failpersist(void** state)
{
    // Failing because perisisting the factory setup data fails.
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    EMPTYCHUNK(chunk0);
    will_return(__wrap_memory_read_chunk_mock, chunk0);

    EXPECT_RESET;

    _expect_keys();

    EMPTYCHUNK(empty_shared_chunk);
    will_return(__wrap_memory_read_shared_bootdata_mock, empty_shared_chunk);

    EMPTYCHUNK(setup_chunk);
    EMPTYCHUNK(shared_chunk);
    _expect_setup(setup_chunk, shared_chunk);
    will_return(__wrap_memory_write_chunk_mock, false);
    assert_false(memory_setup(&_ifs));
}

static void _expect_bitmask(uint8_t* chunk, uint8_t bitmask)
{
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    chunk[_addr_bitmask] = bitmask;
    will_return(__wrap_memory_read_chunk_mock, chunk);
}

static void _test_memory_is_seeded(void** state)
{
    for (uint8_t i = 0; i < 255; i++) {
        EMPTYCHUNK(chunk);
        _expect_bitmask(chunk, i);
        assert_int_equal(memory_is_seeded(), (~i & _bitmask_seeded) != 0);
    }
}

static void _test_memory_is_initialized(void** state)
{
    for (uint8_t i = 0; i < 255; i++) {
        EMPTYCHUNK(chunk);
        _expect_bitmask(chunk, i);
        assert_int_equal(memory_is_initialized(), (~i & _bitmask_initialized) != 0);
    }
}

static void _test_memory_set_initialized(void** state)
{
    for (uint8_t i = 0; i < 255; i++) {
        EMPTYCHUNK(chunk);
        _expect_bitmask(chunk, i);
        // if is seeded fails, the whole function fails.
        bool is_seeded = ~i & _bitmask_seeded;
        if (!is_seeded) {
            assert_false(memory_set_initialized());
            continue;
        }

        _expect_bitmask(chunk, i);
        EMPTYCHUNK(expected_chunk);
        expected_chunk[_addr_bitmask] = ~(~i | _bitmask_initialized);
        expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
        expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
        will_return(__wrap_memory_write_chunk_mock, true);
        assert_true(memory_set_initialized());
    }
}

static void _test_memory_get_failed_unlock_attempts(void** state)
{
    for (uint8_t attempts = 0; attempts < 0xFF; attempts++) {
        EMPTYCHUNK(empty_chunk);
        empty_chunk[_addr_failed_unlock_attempts] = 0xFF - attempts;
        expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
        will_return(__wrap_memory_read_chunk_mock, empty_chunk);
        assert_int_equal(attempts, memory_get_failed_unlock_attempts());
    }
}

static void _expect_failed_unlock_attempts(uint8_t* chunk, uint8_t attempts)
{
    chunk[_addr_failed_unlock_attempts] = 0xFF - attempts;
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, chunk);
}

static void _test_memory_increment_failed_unlock_attempts(void** state)
{
    for (uint8_t attempts = 0; attempts < 0xFF; attempts++) {
        EMPTYCHUNK(chunk);
        _expect_failed_unlock_attempts(chunk, attempts);

        EMPTYCHUNK(expected_chunk);
        expected_chunk[_addr_failed_unlock_attempts] = 0xFF - (attempts + 1);
        expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
        expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
        will_return(__wrap_memory_write_chunk_mock, true);
        assert_true(memory_increment_failed_unlock_attempts());
    }
}

static void _test_memory_increment_failed_unlock_attempts_overflow(void** state)
{
    EMPTYCHUNK(chunk);
    _expect_failed_unlock_attempts(chunk, 0xFF);
    assert_false(memory_increment_failed_unlock_attempts());
}

static void _test_memory_reset_failed_unlock_attempts(void** state)
{
    uint8_t attempts = 0xFF;
    do {
        EMPTYCHUNK(chunk);
        _expect_failed_unlock_attempts(chunk, attempts);
        if (attempts != 0) { // no write if already reset.
            EMPTYCHUNK(expected_chunk);
            expected_chunk[_addr_failed_unlock_attempts] = 0xFF;
            expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
            expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
            will_return(__wrap_memory_write_chunk_mock, true);
        }
        assert_true(memory_reset_failed_unlock_attempts());
    } while (attempts--);
}

static void _test_memory_is_mnemonic_passphrase_enabled(void** state)
{
    for (uint8_t i = 0; i < 255; i++) {
        EMPTYCHUNK(chunk);
        _expect_bitmask(chunk, i);
        assert_int_equal(
            memory_is_mnemonic_passphrase_enabled(),
            (~i & _bitmask_mnemonic_passphrase_enabled) != 0);
    }
}

static void _test_memory_set_mnemonic_passphrase_enabled(void** state)
{
    for (uint8_t enable = 0; enable <= 1; enable++) {
        for (uint8_t i = 0; i < 255; i++) {
            EMPTYCHUNK(chunk);
            _expect_bitmask(chunk, i);

            EMPTYCHUNK(expected_chunk);
            if (enable) {
                // expect enabled
                expected_chunk[_addr_bitmask] = ~(~i | _bitmask_mnemonic_passphrase_enabled);
            } else {
                // expect disabled
                expected_chunk[_addr_bitmask] = ~(~i & ~_bitmask_mnemonic_passphrase_enabled);
            }
            expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
            expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
            will_return(__wrap_memory_write_chunk_mock, true);
            assert_true(memory_set_mnemonic_passphrase_enabled(enable));
        }
    }
}

static void _test_memory_reset_hww(void** state)
{
    EXPECT_RESET;
    assert_true(memory_reset_hww());

    _fail_reset();
    assert_false(memory_reset_hww());
}

static void _test_memory_get_device_name_default(void** state)
{
    char name_out[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    EMPTYCHUNK(empty_chunk);
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk);
    memory_get_device_name(name_out);
    assert_string_equal(MEMORY_DEFAULT_DEVICE_NAME, name_out);
}

static void _test_memory_get_device_name(void** state)
{
    char name_out[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    EMPTYCHUNK(chunk);
    memset(chunk + _addr_device_name, 0, MEMORY_DEVICE_NAME_MAX_LEN);
    const char* device_name = "Äxxxxxxxxxxxxxx xxxxxxxxxxxxxxxxxxxxxx 漢字xxxxxxxxxxxxxxxxx";
    snprintf((char*)chunk + _addr_device_name, MEMORY_DEVICE_NAME_MAX_LEN, "%s", device_name);

    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, chunk);
    memory_get_device_name(name_out);
    assert_string_equal(device_name, name_out);
}

static void _set_device_name(const char* device_name)
{
    EMPTYCHUNK(empty_chunk);
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk);

    EMPTYCHUNK(expected_chunk);
    memset(expected_chunk + _addr_device_name, 0, MEMORY_DEVICE_NAME_MAX_LEN);
    snprintf(
        (char*)expected_chunk + _addr_device_name, MEMORY_DEVICE_NAME_MAX_LEN, "%s", device_name);
    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
    expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
    will_return(__wrap_memory_write_chunk_mock, true);
    assert_true(memory_set_device_name(device_name));
}

static void _test_memory_device_name(void** state)
{
    const char invalid_name[] = "\xff";
    assert_false(memory_set_device_name(invalid_name));

    const char* device_name = "test name";
    _set_device_name(device_name);
    const char* device_name2 = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    _set_device_name(device_name2);
}

static void _test_memory_set_seed_birthdate(void** state)
{
    EMPTYCHUNK(empty_chunk);
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 1);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk);

    EMPTYCHUNK(expected_chunk);
    uint32_t* timestamp = (uint32_t*)&expected_chunk[_addr_seed_birthdate];
    *timestamp = 0xabcdef11;
    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 1);
    expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
    will_return(__wrap_memory_write_chunk_mock, true);
    assert_true(memory_set_seed_birthdate(*timestamp));
}

static void _test_memory_set_attestation_device_pubkey(void** state)
{
    EMPTYCHUNK(empty_chunk);
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    will_return(__wrap_memory_read_chunk_mock, empty_chunk);

    const uint8_t pubkey[64] = {
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
    };
    EMPTYCHUNK(expected_chunk);
    memcpy(&expected_chunk[_addr_attestation_pubkey], pubkey, 64);
    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 0);
    expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
    will_return(__wrap_memory_write_chunk_mock, true);
    assert_true(memory_set_attestation_device_pubkey(pubkey));
}

static void _test_memory_set_attestation_certificate(void** state)
{
    EMPTYCHUNK(chunk);

    const uint8_t pubkey[64] = {
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
    };
    const uint8_t certificate[64] = {
        0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x44, 0x44,
        0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x11, 0x11, 0x11, 0x11,
        0x11, 0x11, 0x11, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    };
    uint8_t root_pubkey_identifier[32];
    memset(root_pubkey_identifier, 0x67, sizeof(root_pubkey_identifier));
    { // fail if the pubkey does not match
        expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
        will_return(__wrap_memory_read_chunk_mock, chunk);

        assert_false(
            memory_set_attestation_certificate(pubkey, certificate, root_pubkey_identifier));
    }

    memcpy(&chunk[_addr_attestation_pubkey], pubkey, 64);
    // 1st call to check if setup is done, 2nd for read/write.
    expect_value(__wrap_memory_read_chunk_mock, chunk_num, 0);
    will_return(__wrap_memory_read_chunk_mock, chunk);

    EMPTYCHUNK(expected_chunk);
    memcpy(&expected_chunk[_addr_attestation_pubkey], pubkey, 64);
    memcpy(&expected_chunk[_addr_attestation_certificate], certificate, 64);
    memcpy(&expected_chunk[_addr_attestation_root_pubkey_identifier], root_pubkey_identifier, 32);
    expect_value(__wrap_memory_write_chunk_mock, chunk_num, 0);
    expect_memory(__wrap_memory_write_chunk_mock, chunk, expected_chunk, CHUNK_SIZE);
    will_return(__wrap_memory_write_chunk_mock, true);
    assert_true(memory_set_attestation_certificate(pubkey, certificate, root_pubkey_identifier));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_memory_setup),
        cmocka_unit_test(_test_memory_setup_failreset),
        cmocka_unit_test(_test_memory_setup_failpersist),
        cmocka_unit_test(_test_memory_is_seeded),
        cmocka_unit_test(_test_memory_is_initialized),
        cmocka_unit_test(_test_memory_set_initialized),
        cmocka_unit_test(_test_memory_get_failed_unlock_attempts),
        cmocka_unit_test(_test_memory_increment_failed_unlock_attempts),
        cmocka_unit_test(_test_memory_increment_failed_unlock_attempts_overflow),
        cmocka_unit_test(_test_memory_reset_failed_unlock_attempts),
        cmocka_unit_test(_test_memory_is_mnemonic_passphrase_enabled),
        cmocka_unit_test(_test_memory_set_mnemonic_passphrase_enabled),
        cmocka_unit_test(_test_memory_reset_hww),
        cmocka_unit_test(_test_memory_get_device_name_default),
        cmocka_unit_test(_test_memory_get_device_name),
        cmocka_unit_test(_test_memory_device_name),
        cmocka_unit_test(_test_memory_set_seed_birthdate),
        cmocka_unit_test(_test_memory_set_attestation_device_pubkey),
        cmocka_unit_test(_test_memory_set_attestation_certificate),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
