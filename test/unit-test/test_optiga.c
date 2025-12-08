// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <memory/memory.h>
#include <optiga/optiga.h>
#include <optiga/optiga_ops.h>

#include <common/optiga_lib_return_codes.h>
#include <optiga_crypt.h>
#include <optiga_util.h>
#include <pal/pal_os_timer.h>
#include <rust/rust.h>
#include <salt.h>

#include <stdint.h>
#include <string.h>

//------------------------------------------------------------------------------
// Fixed test vectors / keys (deterministic fakes).

static const uint8_t _salt_root_fixed[32] = {
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
};

static const uint8_t _password_secret_fixed[32] = {
    0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
    0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
};

static const uint8_t _kdf_cmac_key_fixed[32] = {
    0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0,
    0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0,
};

static const uint8_t _kdf_hmac_key_fixed[32] = {
    0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0,
    0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0,
};

static const uint8_t _auth_code_random_fixed[32] = {
    0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
    0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
};

// Expected stretched_out for password "pw" for the V0 algorithm given the above fakes.
//
// Repro script (mirrors optiga_stretch_password() with the unit test fakes):
// ```python
// import hashlib, hmac
//
// def sha256(b: bytes) -> bytes:
//     return hashlib.sha256(b).digest()
//
// def hmac_sha256(key: bytes, msg: bytes) -> bytes:
//     return hmac.new(key, msg, hashlib.sha256).digest()
//
// def salt_hash_data(data: bytes, purpose: bytes, salt_root: bytes) -> bytes:
//     return sha256(salt_root + purpose + data)
//
// def kdf_internal(msg: bytes, cmac_key: bytes) -> bytes:
//     # optiga_ops_crypt_symmetric_encrypt_sync fake: HMAC-SHA256(cmac_key, msg)[:16]
//     return sha256(hmac_sha256(cmac_key, msg)[:16])
//
// def kdf_hmac(msg: bytes, hmac_key: bytes) -> bytes:
//     # optiga_ops_crypt_hmac_sync fake: HMAC-SHA256(hmac_key, msg)
//     return hmac_sha256(hmac_key, msg)
//
// salt_root = bytes([0x42]) * 32
// cmac_key = bytes([0xA0]) * 32
// hmac_key = bytes([0xB0]) * 32
// password_secret = bytes([0x99]) * 32
// password = b"pw"
//
// kdf_in = salt_hash_data(password, b"optiga_password_stretch_in", salt_root)
// stretched = kdf_internal(kdf_in, cmac_key)
// for _ in range(2):
//     stretched = kdf_hmac(stretched, hmac_key)
// stretched = hmac_sha256(password_secret, stretched)
// out_salt = salt_hash_data(password, b"optiga_password_stretch_out", salt_root)
// stretched = hmac_sha256(out_salt, stretched)
// print(stretched.hex())
// ```
static const uint8_t _expected_stretched_out_v0[32] = {
    0xC4, 0x1F, 0x87, 0xB7, 0xC9, 0xF3, 0x16, 0x9C, 0x14, 0xF3, 0xF2, 0x62, 0x87, 0x09, 0x3C, 0x31,
    0x18, 0x19, 0x06, 0x77, 0x76, 0xF6, 0x16, 0x3B, 0x8A, 0x0F, 0xDF, 0x3D, 0xFB, 0x8B, 0x8E, 0xBB,
};

//------------------------------------------------------------------------------
// Minimal securechip interface fakes.

static void _dummy_get_key(uint8_t* key_out)
{
    memset(key_out, 0, 32);
}

static void _fixed_random_32_bytes(uint8_t* buf)
{
    memcpy(buf, _password_secret_fixed, 32);
}

static const securechip_interface_functions_t _ifs = {
    .get_auth_key = _dummy_get_key,
    .get_io_protection_key = _dummy_get_key,
    .get_encryption_key = _dummy_get_key,
    .random_32_bytes = _fixed_random_32_bytes,
};

//------------------------------------------------------------------------------
// Linker-wrapped salt_hash_data: same as salt.rs, but with a fixed salt_root.

bool __wrap_salt_hash_data(
    const uint8_t* data,
    size_t data_len,
    const char* purpose,
    uint8_t* hash_out)
{
    if ((data_len > 0 && data == NULL) || purpose == NULL || hash_out == NULL) {
        return false;
    }

    void* ctx = rust_sha256_new();
    if (ctx == NULL) {
        return false;
    }
    rust_sha256_update(ctx, _salt_root_fixed, sizeof(_salt_root_fixed));
    rust_sha256_update(ctx, purpose, strlen(purpose));
    rust_sha256_update(ctx, data, data_len);
    rust_sha256_finish(&ctx, hash_out);
    return true;
}

//------------------------------------------------------------------------------
// Minimal OPTIGA + PAL stubs required by optiga.c during setup.

static optiga_util_t _fake_util;
static optiga_crypt_t _fake_crypt;

static uint8_t _oid_password[32];
static bool _oid_password_set;

static uint8_t _oid_counter_password_buf[8];

static bool _authorized_password;
static bool _authorized_password_secret;

static void _reset_fakes(void)
{
    memset(_oid_password, 0, sizeof(_oid_password));
    _oid_password_set = false;
    _authorized_password = false;
    _authorized_password_secret = false;
}

static uint32_t _get_counter(uint16_t oid)
{
    switch (oid) {
    case OID_COUNTER_PASSWORD:
        return optiga_common_get_uint32(&_oid_counter_password_buf[0]);
    default:
        fail();
        return 0;
    }
}

static uint32_t _get_threshold(uint16_t oid)
{
    switch (oid) {
    case OID_COUNTER_PASSWORD:
        return optiga_common_get_uint32(&_oid_counter_password_buf[4]);
    default:
        fail();
        return 0;
    }
}

pal_status_t pal_timer_init(void)
{
    return PAL_STATUS_SUCCESS;
}

uint32_t optiga_common_get_uint32(const uint8_t* p_input_buffer)
{
    return ((uint32_t)p_input_buffer[0] << 24) | ((uint32_t)p_input_buffer[1] << 16) |
           ((uint32_t)p_input_buffer[2] << 8) | (uint32_t)p_input_buffer[3];
}

void optiga_common_set_uint32(uint8_t* p_output_buffer, uint32_t four_byte_value)
{
    p_output_buffer[0] = (uint8_t)(four_byte_value >> 24);
    p_output_buffer[1] = (uint8_t)(four_byte_value >> 16);
    p_output_buffer[2] = (uint8_t)(four_byte_value >> 8);
    p_output_buffer[3] = (uint8_t)(four_byte_value);
}

void optiga_util_set_comms_params(optiga_util_t* me, uint8_t parameter_type, uint8_t value)
{
#ifdef OPTIGA_COMMS_SHIELDED_CONNECTION
    if (parameter_type == OPTIGA_COMMS_PROTECTION_LEVEL) {
        me->protection_level = value;
    } else if (parameter_type == OPTIGA_COMMS_PROTOCOL_VERSION) {
        me->protocol_version = value;
    }
#else
    (void)me;
    (void)parameter_type;
    (void)value;
#endif
}

void optiga_crypt_set_comms_params(optiga_crypt_t* me, uint8_t parameter_type, uint8_t value)
{
#ifdef OPTIGA_COMMS_SHIELDED_CONNECTION
    if (parameter_type == OPTIGA_COMMS_PROTECTION_LEVEL) {
        me->protection_level = value;
    } else if (parameter_type == OPTIGA_COMMS_PROTOCOL_VERSION) {
        me->protocol_version = value;
    }
#else
    (void)me;
    (void)parameter_type;
    (void)value;
#endif
}

//------------------------------------------------------------------------------
// Fake optiga_ops API surface (unit-test seam).

optiga_lib_status_t optiga_ops_create(optiga_util_t** util_out, optiga_crypt_t** crypt_out)
{
    // Handler/callback not currently needed in tests, so it is set to NULL.

    memset(&_fake_util, 0, sizeof(_fake_util));
    _fake_util.caller_context = OPTIGA_INSTANCE_ID_0;
    _fake_util.handler = NULL;
    _fake_util.instance_state = 0;
#ifdef OPTIGA_COMMS_SHIELDED_CONNECTION
    _fake_util.protection_level = OPTIGA_COMMS_FULL_PROTECTION;
    _fake_util.protocol_version = OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET;
#endif
    *util_out = &_fake_util;

    memset(&_fake_crypt, 0, sizeof(_fake_crypt));
    _fake_crypt.my_cmd = NULL;
    _fake_crypt.caller_context = OPTIGA_INSTANCE_ID_0;
    _fake_crypt.handler = NULL;
    _fake_crypt.instance_state = 0;
#ifdef OPTIGA_COMMS_SHIELDED_CONNECTION
    _fake_crypt.protection_level = OPTIGA_COMMS_FULL_PROTECTION;
    _fake_crypt.protocol_version = OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET;
#endif
    *crypt_out = &_fake_crypt;

    return 0;
}

optiga_lib_status_t optiga_ops_util_open_application_sync(optiga_util_t* me, bool_t perform_restore)
{
    (void)me;
    (void)perform_restore;
    return OPTIGA_LIB_SUCCESS;
}

optiga_lib_status_t optiga_ops_util_close_application_sync(
    optiga_util_t* me,
    bool_t perform_hibernate)
{
    (void)me;
    (void)perform_hibernate;
    return OPTIGA_UTIL_ERROR;
}

optiga_lib_status_t optiga_ops_util_read_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t* buffer,
    uint16_t* length)
{
    (void)me;
    (void)optiga_oid;
    (void)buffer;
    (void)length;
    return OPTIGA_UTIL_ERROR;
}

optiga_lib_status_t optiga_ops_util_write_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    const uint8_t* buffer,
    uint8_t length)
{
    (void)me;
    (void)optiga_oid;
    (void)buffer;
    (void)length;
    return OPTIGA_UTIL_ERROR;
}

optiga_lib_status_t optiga_ops_util_read_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint16_t offset,
    uint8_t* buffer,
    uint16_t* length)
{
    (void)me;
    (void)offset;

    if (optiga_oid == OID_PASSWORD_SECRET) {
        if (!_authorized_password) {
            return OPTIGA_UTIL_ERROR;
        }
        if (*length < 32) {
            return OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT;
        }
        memcpy(buffer, _password_secret_fixed, 32);
        *length = 32;
        return OPTIGA_UTIL_SUCCESS;
    }
    if (optiga_oid == OID_COUNTER) {
        if (*length < 4) {
            return OPTIGA_UTIL_ERROR_MEMORY_INSUFFICIENT;
        }
        memset(buffer, 0, 4);
        *length = 4;
        return OPTIGA_UTIL_SUCCESS;
    }
    if (optiga_oid == OID_ARBITRARY_DATA) {
        memset(buffer, 0, *length);
        return OPTIGA_UTIL_SUCCESS;
    }

    return OPTIGA_UTIL_ERROR_INVALID_INPUT;
}

optiga_lib_status_t optiga_ops_util_write_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t write_type,
    uint16_t offset,
    const uint8_t* buffer,
    uint16_t length)
{
    (void)me;
    (void)write_type;
    (void)offset;

    if (optiga_oid == OID_PASSWORD) {
        if (!_authorized_password_secret) {
            return OPTIGA_UTIL_ERROR;
        }
        if (length != 32) {
            return OPTIGA_UTIL_ERROR_INVALID_INPUT;
        }
        memcpy(_oid_password, buffer, 32);
        _oid_password_set = true;
        return OPTIGA_UTIL_SUCCESS;
    }
    if (optiga_oid == OID_COUNTER_PASSWORD) {
        if (length != sizeof(_oid_counter_password_buf)) {
            return OPTIGA_UTIL_ERROR_INVALID_INPUT;
        }
        memcpy(_oid_counter_password_buf, buffer, sizeof(_oid_counter_password_buf));
        return OPTIGA_UTIL_SUCCESS;
    }
    // Accept other writes without emulating full semantics (counter reset, hmac key, etc.).
    (void)buffer;
    (void)length;
    return OPTIGA_UTIL_SUCCESS;
}

optiga_lib_status_t optiga_ops_crypt_generate_auth_code_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    const uint8_t* optional_data,
    uint16_t optional_data_length,
    uint8_t* random_data,
    uint16_t random_data_length)
{
    (void)me;
    (void)rng_type;
    (void)optional_data;
    (void)optional_data_length;
    if (random_data_length != sizeof(_auth_code_random_fixed)) {
        return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
    }
    memcpy(random_data, _auth_code_random_fixed, sizeof(_auth_code_random_fixed));
    return OPTIGA_CRYPT_SUCCESS;
}

optiga_lib_status_t optiga_ops_crypt_hmac_verify_sync(
    optiga_crypt_t* me,
    optiga_hmac_type_t type,
    uint16_t secret,
    const uint8_t* input_data,
    uint32_t input_data_length,
    const uint8_t* hmac,
    uint32_t hmac_length)
{
    (void)me;
    (void)type;

    if (hmac_length != 32 || input_data_length != 32) {
        return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
    }

    const uint8_t* key = NULL;
    uintptr_t key_len = 0;
    if (secret == OID_PASSWORD_SECRET) {
        key = _password_secret_fixed;
        key_len = sizeof(_password_secret_fixed);
    } else if (secret == OID_PASSWORD) {
        // Emulate the small monotonic counter that is attached to password authorization.
        // Stored as {counter_be_u32, threshold_be_u32}.
        uint32_t counter = _get_counter(OID_COUNTER_PASSWORD);
        uint32_t threshold = _get_threshold(OID_COUNTER_PASSWORD);
        if (counter >= threshold) {
            return 0x802F;
        }
        counter++;
        optiga_common_set_uint32(&_oid_counter_password_buf[0], counter);

        if (!_oid_password_set) {
            return OPTIGA_CRYPT_ERROR;
        }
        key = _oid_password;
        key_len = sizeof(_oid_password);
    } else {
        return OPTIGA_CRYPT_ERROR_INVALID_INPUT;
    }

    uint8_t computed[32] = {0};
    rust_hmac_sha256(key, key_len, input_data, input_data_length, computed);
    if (memcmp(computed, hmac, 32) != 0) {
        return 0x802F;
    }

    if (secret == OID_PASSWORD) {
        _authorized_password = true;
    } else if (secret == OID_PASSWORD_SECRET) {
        _authorized_password_secret = true;
    }
    return OPTIGA_CRYPT_SUCCESS;
}

optiga_lib_status_t optiga_ops_crypt_clear_auto_state_sync(optiga_crypt_t* me, uint16_t secret)
{
    (void)me;
    if (secret == OID_PASSWORD) {
        _authorized_password = false;
    } else if (secret == OID_PASSWORD_SECRET) {
        _authorized_password_secret = false;
    }
    return OPTIGA_CRYPT_SUCCESS;
}

optiga_lib_status_t optiga_ops_crypt_symmetric_generate_key_sync(
    optiga_crypt_t* me,
    optiga_symmetric_key_type_t key_type,
    uint8_t key_usage,
    bool_t export_symmetric_key,
    void* symmetric_key)
{
    (void)me;
    (void)key_type;
    (void)key_usage;
    (void)export_symmetric_key;
    (void)symmetric_key;
    return OPTIGA_CRYPT_ERROR;
}

optiga_lib_status_t optiga_ops_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length)
{
    (void)me;
    (void)rng_type;
    (void)random_data;
    (void)random_data_length;
    return OPTIGA_CRYPT_ERROR;
}

// Use rust_hmac_sha256 with a fixed key and msg as the value, truncated to 16 bytes.
optiga_lib_status_t optiga_ops_crypt_symmetric_encrypt_sync(
    optiga_crypt_t* me,
    optiga_symmetric_encryption_mode_t encryption_mode,
    optiga_key_id_t symmetric_key_oid,
    const uint8_t* plain_data,
    uint32_t plain_data_length,
    const uint8_t* iv,
    uint16_t iv_length,
    const uint8_t* associated_data,
    uint16_t associated_data_length,
    uint8_t* encrypted_data,
    uint32_t* encrypted_data_length)
{
    (void)me;
    (void)encryption_mode;
    assert_int_equal(symmetric_key_oid, OID_AES_SYMKEY);
    (void)iv;
    (void)iv_length;
    (void)associated_data;
    (void)associated_data_length;

    uint8_t out[32] = {0};
    rust_hmac_sha256(
        _kdf_cmac_key_fixed, sizeof(_kdf_cmac_key_fixed), plain_data, plain_data_length, out);

    if (*encrypted_data_length < 16) {
        return OPTIGA_CRYPT_ERROR_MEMORY_INSUFFICIENT;
    }
    memcpy(encrypted_data, out, 16);
    *encrypted_data_length = 16;
    return OPTIGA_CRYPT_SUCCESS;
}

// Use rust_hmac_sha256 with a different fixed key and msg as the value.
optiga_lib_status_t optiga_ops_crypt_hmac_sync(
    optiga_crypt_t* me,
    optiga_hmac_type_t type,
    uint16_t secret,
    const uint8_t* input_data,
    uint32_t input_data_length,
    uint8_t* mac,
    uint32_t* mac_length)
{
    (void)me;
    (void)type;
    assert_int_equal(secret, OID_HMAC);
    if (*mac_length < 32) {
        return OPTIGA_CRYPT_ERROR_MEMORY_INSUFFICIENT;
    }
    rust_hmac_sha256(
        _kdf_hmac_key_fixed, sizeof(_kdf_hmac_key_fixed), input_data, input_data_length, mac);
    *mac_length = 32;
    return OPTIGA_CRYPT_SUCCESS;
}

// Unused by these tests, but required to satisfy optiga.c link dependencies.
optiga_lib_status_t optiga_ops_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t* me,
    optiga_ecc_curve_t curve_id,
    uint8_t key_usage,
    bool_t export_private_key,
    void* private_key,
    uint8_t* public_key,
    uint16_t* public_key_length)
{
    (void)me;
    (void)curve_id;
    (void)key_usage;
    (void)export_private_key;
    (void)private_key;
    (void)public_key;
    (void)public_key_length;
    return OPTIGA_CRYPT_ERROR;
}

optiga_lib_status_t optiga_ops_crypt_ecdsa_sign_sync(
    optiga_crypt_t* me,
    const uint8_t* digest,
    uint8_t digest_length,
    optiga_key_id_t private_key,
    uint8_t* signature,
    uint16_t* signature_length)
{
    (void)me;
    (void)digest;
    (void)digest_length;
    (void)private_key;
    (void)signature;
    (void)signature_length;
    return OPTIGA_CRYPT_ERROR;
}

//------------------------------------------------------------------------------
// Tests

static void test_optiga_stretch_password_v0_success(void** state)
{
    (void)state;
    _reset_fakes();
    assert_int_equal(optiga_setup(&_ifs), 0);

    // Seed the OID_PASSWORD and OID_PASSWORD_COUNTER objects as if they were provisioned earlier.
    assert_true(salt_hash_data((const uint8_t*)"pw", 2, "optiga_password", _oid_password));
    _oid_password_set = true;
    const uint8_t counter_reset_buf[8] = {0, 0, 0, 0, 0, 0, 0, SMALL_MONOTONIC_COUNTER_MAX_USE};
    memcpy(_oid_counter_password_buf, counter_reset_buf, sizeof(_oid_counter_password_buf));

    uint8_t stretched_out[32] = {0};
    assert_int_equal(
        optiga_stretch_password("pw", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out), 0);
    assert_memory_equal(
        stretched_out, _expected_stretched_out_v0, sizeof(_expected_stretched_out_v0));
    // Successful password verification resets the small monotonic counter/threshold.
    assert_int_equal(_get_counter(OID_COUNTER_PASSWORD), 0);
    assert_int_equal(_get_threshold(OID_COUNTER_PASSWORD), SMALL_MONOTONIC_COUNTER_MAX_USE);
}

static void test_optiga_stretch_password_v0_attempt_counter(void** state)
{
    (void)state;
    _reset_fakes();
    assert_int_equal(optiga_setup(&_ifs), 0);

    // Seed the OID_PASSWORD and OID_PASSWORD_COUNTER objects as if they were provisioned earlier.
    assert_true(salt_hash_data((const uint8_t*)"pw", 2, "optiga_password", _oid_password));
    _oid_password_set = true;
    const uint8_t counter_reset_buf[8] = {0, 0, 0, 0, 0, 0, 0, SMALL_MONOTONIC_COUNTER_MAX_USE};
    memcpy(_oid_counter_password_buf, counter_reset_buf, sizeof(_oid_counter_password_buf));

    uint8_t stretched_out[32] = {0};

    assert_int_equal(
        optiga_stretch_password("wrong", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out),
        SC_ERR_INCORRECT_PASSWORD);
    assert_int_equal(_get_counter(OID_COUNTER_PASSWORD), 1);
    assert_int_equal(_get_threshold(OID_COUNTER_PASSWORD), SMALL_MONOTONIC_COUNTER_MAX_USE);

    assert_int_equal(
        optiga_stretch_password("wrong", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out),
        SC_ERR_INCORRECT_PASSWORD);
    assert_int_equal(_get_counter(OID_COUNTER_PASSWORD), 2);
    assert_int_equal(_get_threshold(OID_COUNTER_PASSWORD), SMALL_MONOTONIC_COUNTER_MAX_USE);

    assert_int_equal(
        optiga_stretch_password("pw", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out), 0);
    assert_int_equal(_get_counter(OID_COUNTER_PASSWORD), 0);
    assert_int_equal(_get_threshold(OID_COUNTER_PASSWORD), SMALL_MONOTONIC_COUNTER_MAX_USE);

    for (int i = 0; i < SMALL_MONOTONIC_COUNTER_MAX_USE; i++) {
        assert_int_equal(
            optiga_stretch_password("wrong", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out),
            SC_ERR_INCORRECT_PASSWORD);
    }
    assert_int_equal(
        optiga_common_get_uint32(&_oid_counter_password_buf[0]), SMALL_MONOTONIC_COUNTER_MAX_USE);

    // After exhausting all allowed attempts, a correct password fails as well.
    assert_int_equal(
        optiga_stretch_password("pw", MEMORY_PASSWORD_STRETCH_ALGO_V0, stretched_out),
        SC_ERR_INCORRECT_PASSWORD);
    assert_int_equal(
        optiga_common_get_uint32(&_oid_counter_password_buf[0]), SMALL_MONOTONIC_COUNTER_MAX_USE);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_optiga_stretch_password_v0_success),
        cmocka_unit_test(test_optiga_stretch_password_v0_attempt_counter),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
