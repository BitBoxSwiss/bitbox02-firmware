// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include "bootloader/stage0/stage1_sigcheck.h"
#include "hal_sha_sync.h"
#include "pukcc/pukcc.h"
#include "rust/rust.h"

#define BODY_LEN (1024u)
#define EXTRA_HEADER_LEN (BB02_STAGE1_HEADER_ALIGNMENT)
#define IMAGE_BUF_LEN (BB02_STAGE1_HEADER_LEN + EXTRA_HEADER_LEN + BODY_LEN)
#define INVALID_DIGEST_BYTE (0xa5u)
#define SHA_UPDATE_COUNT (6u)
#define SIGNED_HEADER_TAIL_OFFSET ((uint32_t)offsetof(bb02_stage1_header_t, header_len))

struct sha_sync_descriptor HASH_ALGORITHM_0;

static uint8_t _image[IMAGE_BUF_LEN] __attribute__((aligned(8)));
static const uint8_t _pubkeys[BB02_STAGE1_ROOT_KEY_COUNT][64] = {{0}, {1}, {2}};
static uint8_t _accepted_digest[BB02_STAGE1_SIGNED_DIGEST_LEN];
static uint8_t _expected_digest[BB02_STAGE1_SIGNED_DIGEST_LEN];
static void* _sha_ctx;
static int32_t _sha_start_status;
static int32_t _sha_update_status[SHA_UPDATE_COUNT];
static int32_t _sha_finish_status;
static bool _sha_finish_writes_digest;
static uint8_t _sha_start_count;
static uint8_t _sha_update_count;
static uint8_t _sha_finish_count;
static bool _accept_any_digest;
static bool _assert_message_not_header_prefix;
static uint8_t _forward_valid_mask;
static uint8_t _reverse_valid_mask;
static uint8_t _call_count;
static uint8_t _call_order[BB02_STAGE1_ROOT_KEY_COUNT * 2u];

/*
Recompute these expected values with:

python3 <<'PY'
import hashlib

MAGIC = b"BBS1"
FLAGS = 0
VERSION = 1
PRODUCT_ID = 1  # stage0_sigcheck defines PRODUCT_BITBOX_MULTI=1.
SIGS_LEN = 64 * 3
BODY_LEN = 1024
MONOTONIC_VERSION = 7
MARKETING_VERSION = b"dev"
MARKETING_VERSION_FIELD_LEN = 37

def compute_digest(header_len):
    image_len = header_len + BODY_LEN
    marketing_version_size = len(MARKETING_VERSION)
    marketing_version_field = MARKETING_VERSION + bytes(
        MARKETING_VERSION_FIELD_LEN - marketing_version_size
    )
    signed_header_len = header_len - SIGS_LEN

    signed_payload = b""
    signed_payload += MAGIC
    signed_payload += int.to_bytes(FLAGS, 4, "little")
    signed_payload += int.to_bytes(VERSION, 2, "little")
    signed_payload += int.to_bytes(PRODUCT_ID, 2, "little")
    signed_payload += int.to_bytes(header_len, 4, "little")
    signed_payload += int.to_bytes(image_len, 8, "little")
    signed_payload += int.to_bytes(MONOTONIC_VERSION, 2, "little")
    signed_payload += int.to_bytes(marketing_version_size, 1, "little")
    signed_payload += marketing_version_field

    reserved_len = signed_header_len - len(signed_payload)
    reserved = bytes(
        (0x90 + i) & 0xff
        for i in range(reserved_len)
    )
    signatures = bytes((0x80 + i) & 0xff for i in range(SIGS_LEN))

    signed_payload += reserved
    assert len(signed_payload) == signed_header_len
    body = bytes((0x40 + i) & 0xff for i in range(BODY_LEN))
    signed_payload += body
    assert len(signed_payload) + len(signatures) == image_len
    return hashlib.sha256(signed_payload).hexdigest()

print(compute_digest(header_len=1024))
print(compute_digest(header_len=2048))
PY
*/
static const uint8_t _digest_fixture_default[BB02_STAGE1_SIGNED_DIGEST_LEN] = {
    0xd4, 0xde, 0xc4, 0x9d, 0xab, 0x5f, 0x53, 0x7a, 0x3b, 0xfd, 0x3b, 0x29, 0xff, 0x34, 0xf3, 0x37,
    0xdf, 0x27, 0x68, 0xc3, 0x0e, 0x65, 0x15, 0x5d, 0x6d, 0xca, 0x6b, 0xb8, 0x41, 0xc6, 0xf9, 0x4f,
};

static const uint8_t _digest_fixture_extended_reserved[BB02_STAGE1_SIGNED_DIGEST_LEN] = {
    0x39, 0xf9, 0xc2, 0x93, 0x12, 0x55, 0xb5, 0x0a, 0x1f, 0x86, 0xc0, 0xd7, 0x5c, 0xd7, 0x7e, 0x6a,
    0xde, 0xda, 0x6b, 0x3f, 0x4b, 0x22, 0x05, 0xdb, 0x01, 0xda, 0x3c, 0x91, 0x53, 0x8e, 0xf4, 0x55,
};

static bb02_stage1_header_t* _header_ptr(void)
{
    return (bb02_stage1_header_t*)_image;
}

#define _header (*_header_ptr())

static uint8_t* _body(void)
{
    return _image + _header.header_len;
}

static uint8_t* _signatures(void)
{
    return (void*)(_image + _header.header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN);
}

static void _fill_signature_fixture(void)
{
    uint8_t* signatures = _signatures();
    for (size_t i = 0; i < BB02_STAGE1_HEADER_SIGNATURES_LEN; i++) {
        signatures[i] = (uint8_t)(0x80u + i);
    }
}

static void _fill_reserved_fixture(void)
{
    const uint32_t signed_header_len = _header.header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN;
    const uint32_t reserved_offset = (uint32_t)offsetof(bb02_stage1_header_t, reserved);
    for (uint32_t offset = reserved_offset; offset < signed_header_len; offset++) {
        _image[offset] = (uint8_t)(0x90u + offset - reserved_offset);
    }
}

static void _fill_body(void)
{
    for (size_t i = 0; i < BODY_LEN; i++) {
        _body()[i] = (uint8_t)(0x40u + i);
    }
}

static void _compute_digest(
    const bb02_stage1_header_t* header,
    uint8_t digest[BB02_STAGE1_SIGNED_DIGEST_LEN])
{
    void* ctx = rust_sha256_new();
    assert_non_null(ctx);
    const uint32_t expected_magic = BB02_STAGE1_HEADER_MAGIC;
    const uint32_t flags = header->flags;
    const uint16_t header_version = header->header_version;
    const uint16_t expected_product_id = BB02_STAGE1_PRODUCT_ID;
    rust_sha256_update(ctx, (const uint8_t*)&expected_magic, sizeof(expected_magic));
    rust_sha256_update(ctx, (const uint8_t*)&flags, sizeof(flags));
    rust_sha256_update(ctx, (const uint8_t*)&header_version, sizeof(header_version));
    rust_sha256_update(ctx, (const uint8_t*)&expected_product_id, sizeof(expected_product_id));
    const uint32_t signed_header_len = header->header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN;
    rust_sha256_update(
        ctx,
        ((const uint8_t*)header) + SIGNED_HEADER_TAIL_OFFSET,
        signed_header_len - SIGNED_HEADER_TAIL_OFFSET);
    rust_sha256_update(
        ctx, ((const uint8_t*)header) + header->header_len, header->image_len - header->header_len);
    rust_sha256_finish(&ctx, digest);
    assert_null(ctx);
}

static void _reset_image(void)
{
    memset(_image, 0, sizeof(_image));
    _header.magic = BB02_STAGE1_HEADER_MAGIC;
    _header.header_version = BB02_STAGE1_HEADER_FORMAT_VERSION;
    _header.product_id = BB02_STAGE1_PRODUCT_ID;
    _header.flags = 0u;
    _header.header_len = BB02_STAGE1_HEADER_LEN;
    _header.image_len = BB02_STAGE1_HEADER_LEN + BODY_LEN;
    _header.monotonic_version = 7;
    _header.stage1_marketing_version_len = 3;
    memcpy(_header.stage1_marketing_version, "dev", 3);
    _fill_body();
}

static void _reset_script(uint8_t forward_valid_mask, uint8_t reverse_valid_mask)
{
    _compute_digest(&_header, _accepted_digest);
    memcpy(_expected_digest, _accepted_digest, sizeof(_expected_digest));
    _sha_ctx = NULL;
    _sha_start_status = 0;
    memset(_sha_update_status, 0, sizeof(_sha_update_status));
    _sha_finish_status = 0;
    _sha_finish_writes_digest = true;
    _sha_start_count = 0;
    _sha_update_count = 0;
    _sha_finish_count = 0;
    _accept_any_digest = false;
    _assert_message_not_header_prefix = false;
    _forward_valid_mask = forward_valid_mask;
    _reverse_valid_mask = reverse_valid_mask;
    _call_count = 0;
    memset(_call_order, 0xff, sizeof(_call_order));
}

static void _use_digest_fixture(const uint8_t digest_fixture[static BB02_STAGE1_SIGNED_DIGEST_LEN])
{
    memcpy(_accepted_digest, digest_fixture, BB02_STAGE1_SIGNED_DIGEST_LEN);
    memcpy(_expected_digest, digest_fixture, BB02_STAGE1_SIGNED_DIGEST_LEN);
}

int32_t __wrap_sha_sync_sha256_start(
    struct sha_sync_descriptor* descr,
    struct sha_context* ctx,
    bool is224)
{
    assert_ptr_equal(descr, &HASH_ALGORITHM_0);
    assert_non_null(ctx);
    assert_false(is224);
    assert_int_equal(_sha_start_count, 0);

    _sha_start_count++;
    _sha_ctx = rust_sha256_new();
    assert_non_null(_sha_ctx);
    return _sha_start_status;
}

int32_t __wrap_sha_sync_sha256_update(
    struct sha_sync_descriptor* descr,
    const uint8_t* input,
    uint32_t length)
{
    assert_ptr_equal(descr, &HASH_ALGORITHM_0);
    assert_true(_sha_update_count < SHA_UPDATE_COUNT);

    switch (_sha_update_count) {
    case 0: {
        const uint32_t expected = BB02_STAGE1_HEADER_MAGIC;
        assert_int_equal(length, sizeof(expected));
        assert_memory_equal(input, &expected, sizeof(expected));
    } break;
    case 1: {
        const uint32_t expected = _header.flags;
        assert_int_equal(length, sizeof(expected));
        assert_memory_equal(input, &expected, sizeof(expected));
    } break;
    case 2: {
        const uint16_t expected = _header.header_version;
        assert_int_equal(length, sizeof(expected));
        assert_memory_equal(input, &expected, sizeof(expected));
    } break;
    case 3: {
        const uint16_t expected = BB02_STAGE1_PRODUCT_ID;
        assert_int_equal(length, sizeof(expected));
        assert_memory_equal(input, &expected, sizeof(expected));
    } break;
    case 4:
        assert_ptr_equal(input, ((const uint8_t*)&_header) + SIGNED_HEADER_TAIL_OFFSET);
        assert_int_equal(
            length,
            _header.header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN - SIGNED_HEADER_TAIL_OFFSET);
        break;
    default:
        assert_int_equal(_sha_update_count, 5u);
        assert_ptr_equal(input, _body());
        assert_int_equal(length, BODY_LEN);
        break;
    }

    assert_non_null(_sha_ctx);
    rust_sha256_update(_sha_ctx, input, length);
    const int32_t status = _sha_update_status[_sha_update_count];
    _sha_update_count++;
    return status;
}

int32_t __wrap_sha_sync_sha256_finish(
    struct sha_sync_descriptor* descr,
    uint8_t output[BB02_STAGE1_SIGNED_DIGEST_LEN])
{
    assert_ptr_equal(descr, &HASH_ALGORITHM_0);
    assert_non_null(output);
    assert_int_equal(_sha_finish_count, 0);
    assert_int_equal(_sha_update_count, SHA_UPDATE_COUNT);

    _sha_finish_count++;
    if (_sha_finish_writes_digest) {
        rust_sha256_finish(&_sha_ctx, output);
    } else {
        uint8_t discarded[BB02_STAGE1_SIGNED_DIGEST_LEN];
        rust_sha256_finish(&_sha_ctx, discarded);
    }
    assert_null(_sha_ctx);
    return _sha_finish_status;
}

static uint8_t _key_index(const uint8_t* public_key)
{
    for (uint8_t i = 0; i < BB02_STAGE1_ROOT_KEY_COUNT; i++) {
        if (public_key == _pubkeys[i]) {
            return i;
        }
    }
    fail_msg("unexpected public key pointer");
    return 0;
}

uint8_t __wrap_pukcc_ecdsa_verify(
    const uint8_t* public_key,
    const uint8_t* signature,
    const uint8_t* message,
    uint32_t message_len,
    PUKCC_CURVE_256_X curve)
{
    (void)curve;
    assert_true(_call_count < sizeof(_call_order));

    const uint8_t key_idx = _key_index(public_key);
    assert_ptr_equal(signature, &_signatures()[(uint32_t)key_idx * BB02_STAGE1_SIGNATURE_LEN]);
    assert_int_equal(message_len, BB02_STAGE1_SIGNED_DIGEST_LEN);
    assert_memory_equal(message, _expected_digest, sizeof(_expected_digest));
    if (_assert_message_not_header_prefix) {
        assert_true(memcmp(message, &_header, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);
    }

    _call_order[_call_count] = key_idx;
    const uint8_t valid_mask =
        _call_count < BB02_STAGE1_ROOT_KEY_COUNT ? _forward_valid_mask : _reverse_valid_mask;
    _call_count++;

    const bool digest_accepted =
        _accept_any_digest || memcmp(message, _accepted_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) == 0;
    return digest_accepted && (valid_mask & (uint8_t)(1u << key_idx)) != 0 ? 0 : 1;
}

static void _assert_call_order(void)
{
    static const uint8_t expected_order[BB02_STAGE1_ROOT_KEY_COUNT * 2u] = {0, 1, 2, 2, 1, 0};

    assert_int_equal(_call_count, sizeof(expected_order));
    assert_memory_equal(_call_order, expected_order, sizeof(expected_order));
}

static void _assert_sha_calls(void)
{
    assert_int_equal(_sha_start_count, 1u);
    assert_int_equal(_sha_update_count, SHA_UPDATE_COUNT);
    assert_int_equal(_sha_finish_count, 1u);
}

static void _assert_no_crypto_calls(void)
{
    assert_int_equal(_sha_start_count, 0);
    assert_int_equal(_sha_update_count, 0);
    assert_int_equal(_sha_finish_count, 0);
    assert_int_equal(_call_count, 0);
}

static void _assert_result(
    uint8_t forward_valid_mask,
    uint8_t reverse_valid_mask,
    secbool_u32 expected)
{
    _reset_image();
    _reset_script(forward_valid_mask, reverse_valid_mask);
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), expected);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_matches_default_digest_fixture(void** state)
{
    (void)state;
    _reset_image();
    _fill_reserved_fixture();
    _fill_signature_fixture();
    _reset_script(0x07, 0x07);
    _use_digest_fixture(_digest_fixture_default);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_matches_extended_reserved_digest_fixture(void** state)
{
    (void)state;
    _reset_image();
    _header.header_len = BB02_STAGE1_HEADER_LEN + EXTRA_HEADER_LEN;
    _header.image_len = _header.header_len + BODY_LEN;
    _fill_body();
    _fill_reserved_fixture();
    _fill_signature_fixture();
    _reset_script(0x07, 0x07);
    _use_digest_fixture(_digest_fixture_extended_reserved);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_invalid_image_len(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.image_len = _header.header_len;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();

    _reset_image();
    _reset_script(0x07, 0x07);
    _header.image_len = BB02_STAGE1_MAX_LEN + 1u;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();
}

static void test_stage1_sigcheck_image_ok_rejects_invalid_header_len(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.header_len = BB02_STAGE1_HEADER_LEN - 1u;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();

    _reset_image();
    _reset_script(0x07, 0x07);
    _header.header_len = BB02_STAGE1_HEADER_LEN + 512u;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();
}

static void test_stage1_sigcheck_image_ok_rejects_invalid_fixed_header_fields(void** state)
{
    (void)state;

    _reset_image();
    _reset_script(0x07, 0x07);
    _header.magic ^= 1u;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();

    _reset_image();
    _reset_script(0x07, 0x07);
    _header.product_id++;
    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();
}

static void test_stage1_sigcheck_image_ok_accepts_signed_header_version(void** state)
{
    (void)state;
    _reset_image();
    _header.header_version++;
    _reset_script(0x07, 0x07);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_zero_signatures(void** state)
{
    (void)state;
    _assert_result(0x00, 0x00, secfalse_u32);
}

static void test_stage1_sigcheck_image_ok_rejects_one_signature(void** state)
{
    (void)state;
    _assert_result(0x01, 0x01, secfalse_u32);
    _assert_result(0x02, 0x02, secfalse_u32);
    _assert_result(0x04, 0x04, secfalse_u32);
}

static void test_stage1_sigcheck_image_ok_accepts_two_signatures(void** state)
{
    (void)state;
    _assert_result(0x03, 0x03, sectrue_u32);
    _assert_result(0x05, 0x05, sectrue_u32);
    _assert_result(0x06, 0x06, sectrue_u32);
}

static void test_stage1_sigcheck_image_ok_accepts_three_signatures(void** state)
{
    (void)state;
    _assert_result(0x07, 0x07, sectrue_u32);
}

static void test_stage1_sigcheck_image_ok_rejects_inconsistent_passes(void** state)
{
    (void)state;
    _assert_result(0x03, 0x07, secfalse_u32);
    _assert_result(0x07, 0x03, secfalse_u32);
}

static void test_stage1_sigcheck_image_ok_rejects_changed_body(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _body()[17] ^= 0x80u;
    _compute_digest(&_header, _expected_digest);
    assert_true(memcmp(_accepted_digest, _expected_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_changed_metadata(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.monotonic_version++;
    _compute_digest(&_header, _expected_digest);
    assert_true(memcmp(_accepted_digest, _expected_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_changed_header_version(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.header_version++;
    _compute_digest(&_header, _expected_digest);
    assert_true(memcmp(_accepted_digest, _expected_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_accepts_signed_unknown_flags(void** state)
{
    (void)state;
    _reset_image();
    _header.flags = 0x80000000u;
    _reset_script(0x07, 0x07);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_changed_unknown_flags(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.flags = 0x80000000u;
    _compute_digest(&_header, _expected_digest);
    assert_true(memcmp(_accepted_digest, _expected_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_development_flag(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _header.flags = BB02_STAGE1_FLAG_DEVELOPMENT;

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    _assert_no_crypto_calls();
}

static void test_stage1_sigcheck_image_ok_accepts_larger_header_len(void** state)
{
    (void)state;
    _reset_image();
    _header.header_len = BB02_STAGE1_HEADER_LEN + EXTRA_HEADER_LEN;
    _header.image_len = _header.header_len + BODY_LEN;
    _fill_body();
    _reset_script(0x07, 0x07);

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_uses_computed_digest_message(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);
    _assert_message_not_header_prefix = true;

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), sectrue_u32);
    _assert_sha_calls();
    _assert_call_order();
}

static void test_stage1_sigcheck_image_ok_rejects_sha_failure_without_stale_digest(void** state)
{
    (void)state;
    _reset_image();
    _reset_script(0x07, 0x07);

    uint8_t previous_digest[BB02_STAGE1_SIGNED_DIGEST_LEN];
    memcpy(previous_digest, _accepted_digest, sizeof(previous_digest));
    memset(_expected_digest, INVALID_DIGEST_BYTE, sizeof(_expected_digest));
    _sha_finish_status = -1;
    _sha_finish_writes_digest = false;
    _accept_any_digest = true;

    assert_int_equal(stage1_sigcheck_image_ok(&_header, _pubkeys), secfalse_u32);
    assert_true(memcmp(previous_digest, _expected_digest, BB02_STAGE1_SIGNED_DIGEST_LEN) != 0);
    _assert_sha_calls();
    _assert_call_order();
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_stage1_sigcheck_image_ok_matches_default_digest_fixture),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_matches_extended_reserved_digest_fixture),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_invalid_image_len),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_invalid_header_len),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_invalid_fixed_header_fields),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_accepts_signed_header_version),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_zero_signatures),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_one_signature),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_accepts_two_signatures),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_accepts_three_signatures),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_inconsistent_passes),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_changed_body),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_changed_metadata),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_changed_header_version),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_accepts_signed_unknown_flags),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_changed_unknown_flags),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_development_flag),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_accepts_larger_header_len),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_uses_computed_digest_message),
        cmocka_unit_test(test_stage1_sigcheck_image_ok_rejects_sha_failure_without_stale_digest),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
