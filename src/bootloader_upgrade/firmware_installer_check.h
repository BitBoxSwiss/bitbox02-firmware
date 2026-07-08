// SPDX-License-Identifier: Apache-2.0

#ifndef _BOOTLOADER_UPGRADE_FIRMWARE_INSTALLER_CHECK_H_
#define _BOOTLOADER_UPGRADE_FIRMWARE_INSTALLER_CHECK_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "bootloader/stage0/stage0_descriptor.h"
#include "bootloader_upgrade.h"

bool bootloader_upgrade_has_legacy_development_markers(const uint8_t* bootloader, size_t len);
bool bootloader_upgrade_is_development_bootloader(
    const bb02_stage0_descriptor_t* stage0_descriptor,
    const bb02_stage1_header_t* stage1_header,
    const uint8_t* legacy_bootloader,
    size_t legacy_bootloader_len);

#endif
