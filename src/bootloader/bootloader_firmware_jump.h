#ifndef __BOOTLOADER_FIRMWARE_JUMP_H
#define __BOOTLOADER_FIRMWARE_JUMP_H

#include <util.h>

#include "bootloader_data.h"

secbool_u32 bootloader_firmware_jump_verified(const boot_data_t* data, secbool_u32 jump);

void bootloader_firmware_jump_exec(void);

#endif // __BOOTLOADER_FIRMWARE_JUMP_H
