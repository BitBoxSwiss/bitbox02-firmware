// SPDX-License-Identifier: Apache-2.0

#include "stage1_sigcheck.h"
#include "pukcc/curve_p256.h"
#include "pukcc/pukcc.h"
#include <hal_sha_sync.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>

#define STAGE1_SIGCHECK_ALL_KEYS_MASK ((uint8_t)((1U << BB02_STAGE1_ROOT_KEY_COUNT) - 1U))
#define STAGE1_SIGCHECK_INVALID_DIGEST_BYTE (0xa5U)
#define STAGE1_SIGCHECK_SIGNED_HEADER_TAIL_OFFSET \
    ((uint32_t)offsetof(bb02_stage1_header_t, header_len))

_Static_assert(BB02_STAGE1_ROOT_KEY_COUNT == 3U, "stage1 signature mask expects 3 keys");
_Static_assert(BB02_STAGE1_SIGNATURE_THRESHOLD == 2U, "stage1 signature threshold changed");
_Static_assert(offsetof(bb02_stage1_header_t, magic) == 0U, "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, flags) == 4U, "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, header_version) == 8U, "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, product_id) == 10U, "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, header_len) == 12U, "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, image_len) == 16U, "stage1 header ABI changed");
_Static_assert(
    offsetof(bb02_stage1_header_t, monotonic_version) == 24U,
    "stage1 header ABI changed");
_Static_assert(
    offsetof(bb02_stage1_header_t, stage1_marketing_version_len) == 26U,
    "stage1 header ABI changed");
_Static_assert(offsetof(bb02_stage1_header_t, reserved) == 64U, "stage1 header ABI changed");

extern struct sha_sync_descriptor HASH_ALGORITHM_0;

__attribute__((aligned(128))) static struct sha_context _sha_context;

typedef struct {
    uint8_t valid_mask;
    uint8_t invalid_mask;
} sigcheck_masks_t;

static uint8_t _is_zero_u8(uint8_t value)
{
    const uint32_t v = value;
    return (uint8_t)(1U ^ ((v | (0U - v)) >> 31));
}

static secbool_u32 _secbool_from_bit(uint8_t bit)
{
    return (uint32_t)(bit & 1U) * sectrue_u32;
}

static secbool_u32 _secbool_u8_eq(uint8_t a, uint8_t b)
{
    return _secbool_from_bit(_is_zero_u8((uint8_t)(a ^ b)));
}

static secbool_u32 _secbool_u8_ne(uint8_t a, uint8_t b)
{
    return sectrue_u32 ^ _secbool_u8_eq(a, b);
}

static secbool_u32 _secbool_u32_eq(uint32_t a, uint32_t b)
{
    const uint32_t v = a ^ b;
    return _secbool_from_bit((uint8_t)(1U ^ ((v | (0U - v)) >> 31)));
}

static secbool_u32 _secbool_i32_eq(int32_t a, int32_t b)
{
    return _secbool_u32_eq((uint32_t)a, (uint32_t)b);
}

static int32_t _canonical_signed_digest(
    const bb02_stage1_header_t* header,
    uint32_t image_body_len,
    uint8_t digest[BB02_STAGE1_SIGNED_DIGEST_LEN])
{
    memset(digest, STAGE1_SIGCHECK_INVALID_DIGEST_BYTE, BB02_STAGE1_SIGNED_DIGEST_LEN);

    const uint32_t expected_magic = BB02_STAGE1_HEADER_MAGIC;
    const uint32_t flags = header->flags;
    const uint16_t header_version = header->header_version;
    const uint16_t expected_product_id = BB02_STAGE1_PRODUCT_ID;
    const uint8_t* header_tail =
        ((const uint8_t*)header) + STAGE1_SIGCHECK_SIGNED_HEADER_TAIL_OFFSET;
    const uint32_t header_len = (uint32_t)header->header_len;
    const uint32_t signed_header_len = header_len - BB02_STAGE1_HEADER_SIGNATURES_LEN;
    const uint32_t header_tail_len = signed_header_len - STAGE1_SIGCHECK_SIGNED_HEADER_TAIL_OFFSET;
    const uint8_t* image_body = ((const uint8_t*)header) + header_len;

    int32_t status = sha_sync_sha256_start(&HASH_ALGORITHM_0, &_sha_context, false);
    status |= sha_sync_sha256_update(
        &HASH_ALGORITHM_0, (const uint8_t*)&expected_magic, sizeof(expected_magic));
    status |= sha_sync_sha256_update(&HASH_ALGORITHM_0, (const uint8_t*)&flags, sizeof(flags));
    status |= sha_sync_sha256_update(
        &HASH_ALGORITHM_0, (const uint8_t*)&header_version, sizeof(header_version));
    status |= sha_sync_sha256_update(
        &HASH_ALGORITHM_0, (const uint8_t*)&expected_product_id, sizeof(expected_product_id));
    status |= sha_sync_sha256_update(&HASH_ALGORITHM_0, header_tail, header_tail_len);
    status |= sha_sync_sha256_update(&HASH_ALGORITHM_0, image_body, image_body_len);
    status |= sha_sync_sha256_finish(&HASH_ALGORITHM_0, digest);
    return status;
}

static secbool_u32 _threshold_from_valid_table(uint8_t valid_mask)
{
    static const secbool_u32 threshold_ok[8] = {
        secfalse_u32, // 000
        secfalse_u32, // 001
        secfalse_u32, // 010
        sectrue_u32, // 011
        secfalse_u32, // 100
        sectrue_u32, // 101
        sectrue_u32, // 110
        sectrue_u32, // 111
    };
    return threshold_ok[valid_mask & STAGE1_SIGCHECK_ALL_KEYS_MASK];
}

static secbool_u32 _threshold_from_valid_pairs(uint8_t valid_mask)
{
    const uint8_t mask = valid_mask & STAGE1_SIGCHECK_ALL_KEYS_MASK;
    secbool_u32 ok = _secbool_u8_eq((uint8_t)(mask & 0x03U), 0x03U);
    ok |= _secbool_u8_eq((uint8_t)(mask & 0x05U), 0x05U);
    ok |= _secbool_u8_eq((uint8_t)(mask & 0x06U), 0x06U);
    return ok;
}

static secbool_u32 _threshold_from_invalid_table(uint8_t invalid_mask)
{
    static const secbool_u32 threshold_ok[8] = {
        sectrue_u32, // 000
        sectrue_u32, // 001
        sectrue_u32, // 010
        secfalse_u32, // 011
        sectrue_u32, // 100
        secfalse_u32, // 101
        secfalse_u32, // 110
        secfalse_u32, // 111
    };
    return threshold_ok[invalid_mask & STAGE1_SIGCHECK_ALL_KEYS_MASK];
}

static secbool_u32 _threshold_from_invalid_pairs(uint8_t invalid_mask)
{
    const uint8_t mask = invalid_mask & STAGE1_SIGCHECK_ALL_KEYS_MASK;
    secbool_u32 ok = _secbool_u8_ne((uint8_t)(mask & 0x03U), 0x03U);
    ok &= _secbool_u8_ne((uint8_t)(mask & 0x05U), 0x05U);
    ok &= _secbool_u8_ne((uint8_t)(mask & 0x06U), 0x06U);
    return ok;
}

static sigcheck_masks_t _check_signatures(
    const uint8_t* signatures,
    const uint8_t digest[BB02_STAGE1_SIGNED_DIGEST_LEN],
    const uint8_t pubkeys[BB02_STAGE1_ROOT_KEY_COUNT][64],
    bool reverse)
{
    sigcheck_masks_t masks = {0};
    for (uint8_t i = 0; i < BB02_STAGE1_ROOT_KEY_COUNT; i++) {
        const uint8_t key_idx = reverse ? (uint8_t)(BB02_STAGE1_ROOT_KEY_COUNT - 1U - i) : i;
        const uint8_t key_bit = (uint8_t)(1U << key_idx);
        const size_t signature_offset = (size_t)key_idx * BB02_STAGE1_SIGNATURE_LEN;
        const uint8_t valid = _is_zero_u8(pukcc_ecdsa_verify(
            pubkeys[key_idx],
            &signatures[signature_offset],
            digest,
            BB02_STAGE1_SIGNED_DIGEST_LEN,
            curve_p256));
        const uint8_t invalid = (uint8_t)(valid ^ 1U);
        masks.valid_mask |= (uint8_t)((uint8_t)(0U - valid) & key_bit);
        masks.invalid_mask |= (uint8_t)((uint8_t)(0U - invalid) & key_bit);
    }
    return masks;
}

static secbool_u32 _stage1_header_len_ok(uint32_t header_len)
{
    if (header_len < BB02_STAGE1_HEADER_LEN || header_len > BB02_STAGE1_MAX_LEN) {
        return secfalse_u32;
    }
    if ((header_len % BB02_STAGE1_HEADER_ALIGNMENT) != 0) {
        return secfalse_u32;
    }
    return sectrue_u32;
}

static const uint8_t* _stage1_signatures(const bb02_stage1_header_t* header)
{
    const uint32_t header_len = (uint32_t)header->header_len;
    const size_t signatures_len = (size_t)BB02_STAGE1_ROOT_KEY_COUNT * BB02_STAGE1_SIGNATURE_LEN;
    return ((const uint8_t*)header) + (size_t)header_len - signatures_len;
}

secbool_u32 stage1_sigcheck_image_ok(
    const bb02_stage1_header_t* header,
    const uint8_t pubkeys[BB02_STAGE1_ROOT_KEY_COUNT][64])
{
    const uint32_t header_len = (uint32_t)header->header_len;
    if (header->magic != BB02_STAGE1_HEADER_MAGIC || header->product_id != BB02_STAGE1_PRODUCT_ID ||
        (header->flags & BB02_STAGE1_FLAG_DEVELOPMENT) != 0 ||
        _stage1_header_len_ok(header_len) != sectrue_u32 || header->image_len <= header_len ||
        header->image_len > BB02_STAGE1_MAX_LEN) {
        return secfalse_u32;
    }
    const uint32_t image_body_len = (uint32_t)(header->image_len - header_len);
    const uint8_t* signatures = _stage1_signatures(header);

    uint8_t digest[BB02_STAGE1_SIGNED_DIGEST_LEN];
    const int32_t digest_status = _canonical_signed_digest(header, image_body_len, digest);
    const sigcheck_masks_t forward = _check_signatures(signatures, digest, pubkeys, false);
    const sigcheck_masks_t reverse = _check_signatures(signatures, digest, pubkeys, true);

    secbool_u32 ok = _secbool_i32_eq(digest_status, 0);
    ok &= _threshold_from_valid_table(forward.valid_mask);
    ok &= _threshold_from_valid_pairs(reverse.valid_mask);
    ok &= _threshold_from_invalid_pairs(forward.invalid_mask);
    ok &= _threshold_from_invalid_table(reverse.invalid_mask);
    ok &= _secbool_u8_eq(forward.valid_mask, reverse.valid_mask);
    ok &= _secbool_u8_eq(forward.invalid_mask, reverse.invalid_mask);
    ok &= _secbool_u8_eq(
        (uint8_t)(forward.valid_mask | forward.invalid_mask), STAGE1_SIGCHECK_ALL_KEYS_MASK);
    ok &= _secbool_u8_eq(
        (uint8_t)(reverse.valid_mask | reverse.invalid_mask), STAGE1_SIGCHECK_ALL_KEYS_MASK);
    ok &= _secbool_u8_eq((uint8_t)(forward.valid_mask & forward.invalid_mask), 0U);
    ok &= _secbool_u8_eq((uint8_t)(reverse.valid_mask & reverse.invalid_mask), 0U);
    return ok;
}
