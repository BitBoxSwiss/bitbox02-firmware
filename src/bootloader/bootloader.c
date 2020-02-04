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

#include "bootloader.h"
#include "bootloader_chunks.h"
#include "bootloader_data.h"
#include "bootloader_firmware_jump.h"
#include "bootloader_graphics.h"
#include "bootloader_usb.h"
#include "bootloader_version.h"
#include "leds.h"

#include <driver_init.h>
#include <stdint.h>
#include <string.h>
#ifdef BOOTLOADER_DEVDEVICE
#include <qtouch/qtouch.h>
#endif
#include <flags.h>
#include <memory/nvmctrl.h>
#include <screen.h>
#include <util.h>

extern volatile uint8_t measurement_done_touch;

// Be sure to not overflow boot data area
#if (                                                                            \
    10 + /* hardawre_version + is_initialized + pad + signing_pubkeys_version */ \
        BOOT_PUBKEY_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS +                       \
        BOOT_SIG_LEN * BOOT_NUM_ROOT_SIGNING_KEYS +                              \
        BOOT_SIG_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS + 4 /* firware_version */  \
    > FLASH_BOOTDATA_LEN)
#error "incompatible bootloader data macro"
#endif
// Be sure signing pubkey data fits within a single chunk
#if (                                                                               \
    1 + 4 + /* op code + signing_pubkeys_version */                                 \
        BOOT_PUBKEY_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS +                          \
        BOOT_SIG_LEN * BOOT_NUM_ROOT_SIGNING_KEYS + 4 /* firmware data version */ + \
        BOOT_SIG_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS >                             \
    FIRMWARE_CHUNK_LEN)
#error "incompatible bootloader data macro"
#endif

// clang-format on

static void _check_init(boot_data_t* data)
{
#ifdef BOOTLOADER_PRODUCTION
    // Enable boot protection if not already enabled. The
    // BOOTPROT fuse is persistent and not erased on chip erase.
    while (NVMCTRL->STATUS.bit.READY == 0) {
    }
    if (NVMCTRL->STATUS.bit.BOOTPROT != FLASH_BOOTPROTECTION) {
        // Update BOOTPROT fuse
        uint32_t fuses[2];
        fuses[0] = *((uint32_t*)NVMCTRL_FUSES_BOOTPROT_ADDR);
        fuses[1] = *(((uint32_t*)NVMCTRL_FUSES_BOOTPROT_ADDR) + 1);
        fuses[0] = (fuses[0] & ~NVMCTRL_FUSES_BOOTPROT_Msk) |
                   (FLASH_BOOTPROTECTION << NVMCTRL_FUSES_BOOTPROT_Pos);
        // Write fuses
        NVMCTRL->CTRLA.bit.WMODE = NVMCTRL_CTRLA_WMODE_MAN; // Manual write
        nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_EP); // Erase page
        nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_PBC); // Clear page buffer
        *((uint32_t*)NVMCTRL_FUSES_BOOTPROT_ADDR) = fuses[0];
        *(((uint32_t*)NVMCTRL_FUSES_BOOTPROT_ADDR) + 1) = fuses[1];
        nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_WQW); // Write a 128-bit word
        // Reboot for changes to take effect
        _reset_mcu();
    }

    // Hard lock the Device Service Unit, i.e., set the PAC write-protection bit,
    // which can only be cleared by a hardware reset. Because the DSU is soft
    // locked by default on reset, an unlock is required before a hard lock.
    periph_unlock(DSU);
    periph_lock_hard(DSU);

    // Set the security bit to disable hardware debug access if not already set.
    // The security bit is persistent and erased on chip erase. A chip erase is
    // disabled by hard locking the DSU.
    if (!DSU->STATUSB.bit.PROT) {
        while (NVMCTRL->STATUS.bit.READY == 0) {
        }
        do {
            NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMD_SSB | NVMCTRL_CTRLB_CMDEX_KEY;
            while (NVMCTRL->INTFLAG.bit.DONE == 0 || NVMCTRL->STATUS.bit.READY == 0) {
            }
        } while (NVMCTRL->INTFLAG.bit.PROGE); // Program Error flag
        // Software reset the NVMCTRL peripheral to have correct NVM state output
        NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMD_SWRST | NVMCTRL_CTRLB_CMDEX_KEY;
        while (NVMCTRL->STATUS.bit.READY == 0) {
        }
    }
#endif

    // Check bootdata initialized
    const uint8_t expected_initialized[2] = {sectrue_u8, sectrue_u8};
    if (!MEMEQ(data->fields.is_initialized, expected_initialized, 2)) {
        memset(data->bytes, 0, sizeof(data->bytes));
        memcpy(data->fields.is_initialized, expected_initialized, 2);
        bootloader_chunks_write_chunk(FLASH_BOOTDATA_START, data->bytes);
    }
}

#ifdef BOOTLOADER_DEVDEVICE
#if PLATFORM_BITBOX02 == 1
static bool _devdevice_enter(secbool_u32 firmware_verified)
{
    UG_ClearBuffer();
    UG_PutString(0, 0, "    <Enter bootloader>", false);
    UG_PutString(0, SCREEN_HEIGHT / 2 - 11, "DEV DEVICE", false);
    UG_PutString(0, SCREEN_HEIGHT / 2 + 2, "NOT FOR VALUE", false);
    UG_PutString(0, SCREEN_HEIGHT - 9, "        <Continue>", false);
    uint16_t ypos = SCREEN_HEIGHT / 2 - 4;
    uint16_t xpos = SCREEN_WIDTH - 10;
    if (firmware_verified != sectrue_u32) {
        // Draw cross
        UG_DrawLine(xpos, ypos, xpos + 5, ypos + 5, C_WHITE);
        UG_DrawLine(xpos + 5, ypos, xpos, ypos + 5, C_WHITE);
    } else {
        // Draw checkmark
        UG_DrawLine(xpos + 5, ypos, xpos, ypos + 5, C_WHITE);
        UG_DrawLine(xpos - 2, ypos + 3, xpos, ypos + 5, C_WHITE);
    }
    UG_SendBuffer();
    while (true) {
        do {
            qtouch_process();
        } while (!measurement_done_touch);

        if (qtouch_is_scroller_active(top_slider)) {
            return true;
        }
        if (qtouch_is_scroller_active(bottom_slider)) {
            return false;
        }
    }
}
#endif
#endif

void bootloader_jump(void)
{
    boot_data_t bootdata;
    shared_data_t shared_data;

    memcpy(bootdata.bytes, (uint8_t*)(FLASH_BOOTDATA_START), FLASH_BOOTDATA_LEN);
    memcpy(shared_data.bytes, (uint8_t*)(FLASH_SHARED_DATA_START), FLASH_SHARED_DATA_LEN);

    _check_init(&bootdata);

#if PLATFORM_BITBOX02 == 1
    if (shared_data.fields.upside_down) {
        screen_rotate();
    }
#endif

    if (shared_data.fields.auto_enter != sectrue_u8) {
#ifdef BOOTLOADER_DEVDEVICE
#if PLATFORM_BITBOXBASE == 1
        // We don't have touch on base dev bootloader yet
        bootloader_firmware_jump_exec();
#elif PLATFORM_BITBOX02 == 1
        if (!_devdevice_enter(bootloader_firmware_jump_verified(&bootdata, secfalse_u32))) {
            bootloader_firmware_jump_exec();
            /* no return */
        }
#endif
#else
        bootloader_firmware_jump_verified(&bootdata, sectrue_u32); // no return if firmware is valid
        bootloader_graphics_render_message("Firmware\ninvalid\n \nEntering bootloader", 3000);
#endif
    }

    // App not entered. Start USB API to receive boot commands
    bootloader_graphics_render_default_screen();
    if (!bootloader_usb_start()) {
        bootloader_graphics_render_message("Failed to initialize USB", 0);
    }
}
