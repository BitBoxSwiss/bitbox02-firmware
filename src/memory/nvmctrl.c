// SPDX-License-Identifier: Apache-2.0

#include "nvmctrl.h"

#include <driver_init.h>
#include <flags.h>
#include <hri_nvmctrl_d51.h>
#include <string.h>

#define NVMCTRL_NON_ECC_ERROR_FLAGS \
    (NVMCTRL_INTFLAG_ADDRE | NVMCTRL_INTFLAG_PROGE | NVMCTRL_INTFLAG_LOCKE | NVMCTRL_INTFLAG_NVME)
#define NVMCTRL_ERROR_FLAGS (NVMCTRL_NON_ECC_ERROR_FLAGS | NVMCTRL_INTFLAG_ECCDE)

void nvmctrl_exec_cmd(uint16_t cmd)
{
    /* Wait until the NVM is ready to accept a new command. */
    while (NVMCTRL->STATUS.bit.READY == 0);
    NVMCTRL->ADDR.reg = (uint32_t)NVMCTRL_USER;
    NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMDEX_KEY | cmd;
    while (NVMCTRL->STATUS.bit.READY == 0);
}

void nvmctrl_clear_errors(void)
{
    // Reading ECCERR clears ECCSE and ECCDE. The remaining error flags are
    // write-one-to-clear.
    (void)hri_nvmctrl_read_ECCERR_reg(NVMCTRL);
    hri_nvmctrl_clear_INTFLAG_reg(NVMCTRL, NVMCTRL_NON_ECC_ERROR_FLAGS);
}

bool nvmctrl_wait_ready_and_check_errors(void)
{
    while (!hri_nvmctrl_get_STATUS_READY_bit(NVMCTRL)) {
    }

    const uint16_t flags_before = hri_nvmctrl_read_INTFLAG_reg(NVMCTRL) & NVMCTRL_ERROR_FLAGS;
    // Reading ECCERR both captures the error type and clears ECCSE/ECCDE.
    const uint32_t eccerr = hri_nvmctrl_read_ECCERR_reg(NVMCTRL);
    const uint16_t flags_after = hri_nvmctrl_read_INTFLAG_reg(NVMCTRL) & NVMCTRL_ERROR_FLAGS;
    hri_nvmctrl_clear_INTFLAG_reg(NVMCTRL, NVMCTRL_NON_ECC_ERROR_FLAGS);

    // Single-bit errors are corrected by hardware. Accept only NONE/SINGLE in both halves;
    // a dual-bit or unexpected type remains fatal even if ECCDE was not set.
    const uint32_t type_low = eccerr & NVMCTRL_ECCERR_TYPEL_Msk;
    const uint32_t type_high = eccerr & NVMCTRL_ECCERR_TYPEH_Msk;
    return (flags_before | flags_after) == 0 &&
           (type_low == NVMCTRL_ECCERR_TYPEL_NONE || type_low == NVMCTRL_ECCERR_TYPEL_SINGLE) &&
           (type_high == NVMCTRL_ECCERR_TYPEH_NONE || type_high == NVMCTRL_ECCERR_TYPEH_SINGLE);
}

bool nvmctrl_read(const volatile uint8_t* flash, uint8_t* out, size_t len)
{
    nvmctrl_clear_errors();
    for (size_t i = 0; i < len; i++) {
        out[i] = flash[i];
    }
    return nvmctrl_wait_ready_and_check_errors();
}

bool nvmctrl_read_block_checked(uintptr_t address, uint8_t* out)
{
    const volatile uint8_t* flash = (const volatile uint8_t*)address;
    uint8_t confirm[ERASE_BLOCK_SIZE];
    return nvmctrl_read(flash, out, ERASE_BLOCK_SIZE) &&
           nvmctrl_read(flash, confirm, ERASE_BLOCK_SIZE) &&
           memcmp(out, confirm, ERASE_BLOCK_SIZE) == 0;
}

bool nvmctrl_write_block(uint32_t address, const uint8_t* data)
{
    if (data == NULL || address % ERASE_BLOCK_SIZE != 0 ||
        address > FLASH_SIZE - ERASE_BLOCK_SIZE) {
        return false;
    }
    const uint32_t lock_size = FLASH_REGION_PAGE_NUM * FLASH_PAGE_SIZE;
    const uint32_t lock_addr = address & ~(lock_size - 1U);

    nvmctrl_clear_errors();
    const int32_t unlock_result = flash_unlock(&FLASH_0, lock_addr, FLASH_REGION_PAGE_NUM);
    if (!nvmctrl_wait_ready_and_check_errors() || unlock_result != FLASH_REGION_PAGE_NUM) {
        return false;
    }

    nvmctrl_clear_errors();
    const int32_t erase_result = flash_erase(&FLASH_0, address, FLASH_ERASE_PAGE_NUM);
    if (!nvmctrl_wait_ready_and_check_errors() || erase_result != ERR_NONE) {
        return false;
    }

    uint32_t page_words[FLASH_PAGE_SIZE / sizeof(uint32_t)];
    for (size_t offset = 0; offset < ERASE_BLOCK_SIZE; offset += FLASH_PAGE_SIZE) {
        memcpy(page_words, data + offset, FLASH_PAGE_SIZE);
        nvmctrl_clear_errors();
        const int32_t write_result =
            flash_append(&FLASH_0, address + offset, (uint8_t*)page_words, FLASH_PAGE_SIZE);
        if (!nvmctrl_wait_ready_and_check_errors() || write_result != ERR_NONE) {
            return false;
        }
    }

    nvmctrl_clear_errors();
    const int32_t lock_result = flash_lock(&FLASH_0, lock_addr, FLASH_REGION_PAGE_NUM);
    if (!nvmctrl_wait_ready_and_check_errors() || lock_result != FLASH_REGION_PAGE_NUM) {
        return false;
    }
    return true;
}
