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
