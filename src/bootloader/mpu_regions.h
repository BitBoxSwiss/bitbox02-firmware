// SPDX-License-Identifier: Apache-2.0

#ifndef _MPU_REGIONS_H_
#define _MPU_REGIONS_H_
#ifndef TESTING

/**
 * Initializes the memory regions for bootloader mode.
 * The bootloader code is read-only, but the memory
 * region for the firmware code has full access. Bootdata,
 * Appdata, and SRAM are non-excutable.
 */
void mpu_regions_bootloader_init(void);

/**
 * Updates the memory regions previously set in bootloader
 * mode for code run in firmware (app) mode. The memory
 * region for the firmware code is updated to read-only and
 * bootdata is updated to no-access.
 */
void mpu_regions_firmware_init(void);

#endif
#endif
