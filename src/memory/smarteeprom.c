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

#include "smarteeprom.h"

#include <driver_init.h>
#include <hardfault.h>
#include <inttypes.h>
#include <memory/memory.h>
#include <memory/nvmctrl.h>
#include <screen.h>
#include <stdint.h>
#include <string.h>
#include <workflow/status.h>

#define SMARTEEPROM_WRITE_MODE_UNBUFFERED (0)
#define SMARTEEPROM_WRITE_MODE_BUFFERED (1)

bool smarteeprom_is_enabled(void)
{
    return NVMCTRL->SEESTAT.bit.PSZ == SMARTEEPROM_PSZ_VALUE &&
           NVMCTRL->SEESTAT.bit.SBLK == SMARTEEPROM_SBLK_VALUE;
}

void smarteeprom_disable(void)
{
    /*
     * Wrong config, overwrite it.
     */
    uint32_t config_word;
    config_word = *((uint32_t*)NVMCTRL_FUSES_SEEPSZ_ADDR);
    /* Set the PSZ value first */
    config_word = (config_word & ~NVMCTRL_FUSES_SEEPSZ_Msk) | (NVMCTRL_FUSES_SEEPSZ(0));
    /* Now set the SBLK value as well. */
    config_word = (config_word & ~NVMCTRL_FUSES_SEESBLK_Msk) | (NVMCTRL_FUSES_SEESBLK(0));
    NVMCTRL->CTRLA.bit.WMODE = NVMCTRL_CTRLA_WMODE_MAN; // Manual write
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_EP); // Erase page
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_PBC); // Clear page buffer
    *((uint32_t*)NVMCTRL_FUSES_SEEPSZ_ADDR) = config_word;
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_WQW); // Write a 128-bit word
    while (!NVMCTRL->STATUS.bit.READY)
        ;
    while (NVMCTRL->SEESTAT.bit.BUSY)
        ;
}

void smarteeprom_setup(void)
{
    /*
     * First, erase the last blocks of flash.
     * So we are sure that there is no garbage data
     * in them.
     */
    if (!memory_cleanup_smarteeprom()) {
        /*
         * Something has gone seriously wrong. We don't want to abort
         * as it would brick the device (we are executing this code at boot time...),
         * however, we want to inform the user that there's a big issue.
         */
        for (int i = 0; i < 3; ++i) {
            workflow_status_blocking("Failed to erase SmartEEPROM memory area.", false);
            workflow_status_blocking("We suggest you reset the device and contact support.", false);
        }
    }
    /*
     * Wrong config, overwrite it.
     */
    uint32_t config_word;
    config_word = *((uint32_t*)NVMCTRL_FUSES_SEEPSZ_ADDR);
    /* Set the PSZ value first */
    config_word =
        (config_word & ~NVMCTRL_FUSES_SEEPSZ_Msk) | (NVMCTRL_FUSES_SEEPSZ(SMARTEEPROM_PSZ_VALUE));
    /* Now set the SBLK value as well. */
    config_word = (config_word & ~NVMCTRL_FUSES_SEESBLK_Msk) |
                  (NVMCTRL_FUSES_SEESBLK(SMARTEEPROM_SBLK_VALUE));
    NVMCTRL->CTRLA.bit.WMODE = NVMCTRL_CTRLA_WMODE_MAN; // Manual write
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_EP); // Erase page
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_PBC); // Clear page buffer
    *((uint32_t*)NVMCTRL_FUSES_SEEPSZ_ADDR) = config_word;
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_WQW); // Write a 128-bit word
    while (!NVMCTRL->STATUS.bit.READY)
        ;
    while (NVMCTRL->SEESTAT.bit.BUSY)
        ;
}

void smarteeprom_bb02_config(void)
{
    /*
     * SmartEEPROM configuration is read at setup.
     *
     * Check if the SmartEEPROM is configured correctly - we will reboot
     * into the new configuration if it isn't, and leave it permanently on.
     */
    if (!smarteeprom_is_enabled()) {
        smarteeprom_setup();
        _reset_mcu();
    }
    NVMCTRL->SEECFG.bit.WMODE = SMARTEEPROM_WRITE_MODE_BUFFERED;
    if (NVMCTRL->SEESTAT.bit.LOAD != 0) {
        for (int i = 0; i < 3; ++i) {
            workflow_status_blocking(
                "SmartEEPROM just initialized, but data found in cache!", false);
            workflow_status_blocking("We suggest you reset the device and contact support.", false);
        }
    }
}

void smarteeprom_read(size_t address, size_t bytes, uint8_t* out_buffer)
{
    if (!out_buffer) {
        Abort("NULL output buffer in smarteeprom_read.");
    }

    volatile uint8_t* eeprom = (uint8_t*)SEEPROM_ADDR + address;
    while (NVMCTRL->SEESTAT.bit.BUSY)
        ;
    for (size_t i = 0; i < bytes; ++i) {
        out_buffer[i] = *eeprom;
        eeprom++;
    }
}

void smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer)
{
    if (!buffer) {
        Abort("NULL input buffer in smarteeprom_write.");
    }
    volatile uint8_t* eeprom = (uint8_t*)SEEPROM_ADDR + address;
    while (NVMCTRL->SEESTAT.bit.BUSY)
        ;
    /*
     * Buffered write of multiple bytes.
     * Note that crossing a 32B page will still result in a partial flush
     * being issued.
     */
    for (size_t i = 0; i < bytes; ++i) {
        *eeprom = buffer[i];
        eeprom++;
    }
    /* Now, flush the write we have issued. */
    nvmctrl_exec_cmd(NVMCTRL_CTRLB_CMD_SEEFLUSH);
    while (NVMCTRL->SEESTAT.bit.LOAD != 0)
        ;
    while (NVMCTRL->SEESTAT.bit.BUSY != 0)
        ;
    /*
     * Read back the buffer.
     * Check that it matches what we've just written.
     */
    uint8_t read_buf[bytes];
    smarteeprom_read(address, bytes, read_buf);
    if (memcmp(read_buf, buffer, bytes) != 0) {
        Abort("Write to SmartEEPROM failed to verify data.");
    }
}
