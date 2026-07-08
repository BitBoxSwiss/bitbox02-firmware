// SPDX-License-Identifier: Apache-2.0

#ifndef _STAGE1_SIGCHECK_H_
#define _STAGE1_SIGCHECK_H_

#include "bootloader_upgrade/bootloader_upgrade.h"
#include "util.h"
#include <stdint.h>

secbool_u32 stage1_sigcheck_image_ok(
    const bb02_stage1_header_t* header,
    const uint8_t pubkeys[BB02_STAGE1_ROOT_KEY_COUNT][64]);

#endif
