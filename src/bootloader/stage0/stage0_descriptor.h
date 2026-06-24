// SPDX-License-Identifier: Apache-2.0

#ifndef _STAGE0_DESCRIPTOR_H_
#define _STAGE0_DESCRIPTOR_H_

#include <stddef.h>
#include <stdint.h>

#include "bootloader/stage0/stage0_version.h"

#define BB02_STAGE0_DESCRIPTOR_MAGIC (0x30534242U) // "BBS0" in little-endian flash order.
#define BB02_STAGE0_FLAG_DEVELOPMENT (1U << 0)
#define BB02_STAGE0_DESCRIPTOR_ADDR (0x00001ff4U)
#define BB02_STAGE0_DESCRIPTOR_LEN (12U)

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"
typedef struct __attribute__((__packed__)) {
    uint16_t stage0_version;
    uint16_t product_id;
    uint32_t flags;
    uint32_t magic;
} bb02_stage0_descriptor_t;
#pragma GCC diagnostic pop

_Static_assert(
    sizeof(bb02_stage0_descriptor_t) == BB02_STAGE0_DESCRIPTOR_LEN,
    "stage0 descriptor ABI changed");
_Static_assert(
    offsetof(bb02_stage0_descriptor_t, stage0_version) == BB02_STAGE0_DESCRIPTOR_LEN - 12U,
    "stage0 descriptor ABI changed");
_Static_assert(
    offsetof(bb02_stage0_descriptor_t, product_id) == BB02_STAGE0_DESCRIPTOR_LEN - 10U,
    "stage0 descriptor ABI changed");
_Static_assert(
    offsetof(bb02_stage0_descriptor_t, flags) == BB02_STAGE0_DESCRIPTOR_LEN - 8U,
    "stage0 descriptor ABI changed");
_Static_assert(
    offsetof(bb02_stage0_descriptor_t, magic) == BB02_STAGE0_DESCRIPTOR_LEN - 4U,
    "stage0 descriptor ABI changed");

extern const bb02_stage0_descriptor_t bb02_stage0_descriptor;

#endif
