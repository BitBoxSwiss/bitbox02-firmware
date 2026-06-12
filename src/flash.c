// SPDX-License-Identifier: Apache-2.0

#include "flash.h"

#include "flags.h"

#include <hpl_nvmctrl_config.h>
#include <hri_nvmctrl_d51.h>
#include <sam.h>
#include <string.h>

#define FLASH_ERROR_FLAGS \
    (NVMCTRL_INTFLAG_ADDRE | NVMCTRL_INTFLAG_PROGE | NVMCTRL_INTFLAG_LOCKE | NVMCTRL_INTFLAG_NVME)

#define FLASH_BLOCK_SIZE (FLASH_ERASE_PAGE_NUM * FLASH_PAGE_SIZE)
#define FLASH_PAGE_WORDS (FLASH_PAGE_SIZE / sizeof(uint32_t))
#define FLASH_BLOCK_WORDS (FLASH_BLOCK_SIZE / sizeof(uint32_t))
#define NVM_MEMORY ((volatile uint32_t*)FLASH_ADDR)

_Static_assert(FLASH_PAGE_SIZE == NVMCTRL_PAGE_SIZE, "flash page size mismatch");
_Static_assert(FLASH_BLOCK_SIZE == NVMCTRL_BLOCK_SIZE, "flash block size mismatch");

static bool _ready(void)
{
    return hri_nvmctrl_get_STATUS_READY_bit(NVMCTRL);
}

static void _clear_errors(void)
{
    hri_nvmctrl_clear_INTFLAG_reg(NVMCTRL, FLASH_ERROR_FLAGS);
}

static bool _status_ok(void)
{
    return (hri_nvmctrl_read_INTFLAG_reg(NVMCTRL) & FLASH_ERROR_FLAGS) == 0;
}

static bool _ready_wait(void)
{
    while (!_ready()) {
    }
    return _status_ok();
}

static bool _start_command(uint32_t addr, uint16_t command)
{
    if (!_ready_wait()) {
        return false;
    }
    _clear_errors();
    hri_nvmctrl_write_ADDR_reg(NVMCTRL, addr);
    hri_nvmctrl_write_CTRLB_reg(NVMCTRL, command | NVMCTRL_CTRLB_CMDEX_KEY);
    return true;
}

static bool _start_command_no_addr(uint16_t command)
{
    if (!_ready_wait()) {
        return false;
    }
    _clear_errors();
    hri_nvmctrl_write_CTRLB_reg(NVMCTRL, command | NVMCTRL_CTRLB_CMDEX_KEY);
    return true;
}

static bool _flash_range_ok(uint32_t addr, size_t len)
{
    if (len == 0 || len > FLASH_SIZE) {
        return false;
    }
#if FLASH_ADDR != 0
    if (addr < FLASH_ADDR) {
        return false;
    }
#endif
    return addr - FLASH_ADDR <= FLASH_SIZE - len;
}

static bool _flash_page_range_ok(uint32_t addr, uint32_t num_pages)
{
    if (num_pages == 0 || (addr % FLASH_PAGE_SIZE) != 0 ||
        num_pages > FLASH_SIZE / FLASH_PAGE_SIZE) {
        return false;
    }
    return _flash_range_ok(addr, (size_t)num_pages * FLASH_PAGE_SIZE);
}

static bool _flash_erase_block(uint32_t block_addr);
static bool _flash_program_page(uint32_t page_addr, const uint32_t* page_words);

static bool _program_block(uint32_t block_addr, const uint32_t* block_words)
{
    for (uint32_t offset = 0; offset < FLASH_BLOCK_SIZE; offset += FLASH_PAGE_SIZE) {
        if (!_flash_program_page(block_addr + offset, &block_words[offset / sizeof(uint32_t)]) ||
            !_ready_wait()) {
            return false;
        }
    }
    return true;
}

void flash_init(void)
{
    (void)_ready_wait();

    uint32_t ctrla = hri_nvmctrl_read_CTRLA_reg(NVMCTRL);
    ctrla &= ~(NVMCTRL_CTRLA_CACHEDIS0 | NVMCTRL_CTRLA_CACHEDIS1 | NVMCTRL_CTRLA_PRM_Msk);
    ctrla |= (CONF_NVM_CACHE0 << NVMCTRL_CTRLA_CACHEDIS0_Pos) |
             (CONF_NVM_CACHE1 << NVMCTRL_CTRLA_CACHEDIS1_Pos) |
             NVMCTRL_CTRLA_PRM(CONF_NVM_SLEEPPRM);
    hri_nvmctrl_write_CTRLA_reg(NVMCTRL, ctrla);
}

void flash_deinit(void)
{
    (void)_ready_wait();
}

bool flash_disable_bootprot(void)
{
    return _start_command_no_addr(NVMCTRL_CTRLB_CMD_SBPDIS) && _ready_wait();
}

bool flash_lock_region(uint32_t addr_in_region)
{
    return _start_command(addr_in_region, NVMCTRL_CTRLB_CMD_LR) && _ready_wait();
}

bool flash_unlock_region(uint32_t addr_in_region)
{
    return _start_command(addr_in_region, NVMCTRL_CTRLB_CMD_UR) && _ready_wait();
}

bool flash_erase_pages(uint32_t page_addr, uint32_t num_pages)
{
    if (!_ready_wait()) {
        return false;
    }
    if (!_flash_page_range_ok(page_addr, num_pages)) {
        return false;
    }

    uint32_t block[FLASH_BLOCK_WORDS];
    uint32_t addr = page_addr;
    size_t remaining = (size_t)num_pages * FLASH_PAGE_SIZE;
    while (remaining > 0) {
        const uint32_t block_addr = addr & ~(FLASH_BLOCK_SIZE - 1U);
        const size_t block_offset = addr - block_addr;
        size_t len = FLASH_BLOCK_SIZE - block_offset;
        if (len > remaining) {
            len = remaining;
        }

        if (block_offset == 0 && len == FLASH_BLOCK_SIZE) {
            if (!_flash_erase_block(block_addr) || !_ready_wait()) {
                return false;
            }
        } else {
            memcpy(block, (const void*)(uintptr_t)block_addr, sizeof(block));
            memset((uint8_t*)block + block_offset, 0xff, len);
            if (!_flash_erase_block(block_addr) || !_ready_wait() ||
                !_program_block(block_addr, block)) {
                return false;
            }
        }
        addr += len;
        remaining -= len;
    }
    return true;
}

bool flash_write(uint32_t addr, const uint8_t* data, size_t len)
{
    if (!_ready_wait()) {
        return false;
    }
    if (data == NULL || !_flash_range_ok(addr, len)) {
        return false;
    }

    uint32_t block[FLASH_BLOCK_WORDS];
    const uint8_t* src = data;
    size_t remaining = len;
    while (remaining > 0) {
        const uint32_t block_addr = addr & ~(FLASH_BLOCK_SIZE - 1U);
        const size_t block_offset = addr - block_addr;
        size_t len_in_block = FLASH_BLOCK_SIZE - block_offset;
        if (len_in_block > remaining) {
            len_in_block = remaining;
        }

        memcpy(block, (const void*)(uintptr_t)block_addr, sizeof(block));
        memcpy((uint8_t*)block + block_offset, src, len_in_block);
        if (!_flash_erase_block(block_addr) || !_ready_wait() ||
            !_program_block(block_addr, block)) {
            return false;
        }

        addr += len_in_block;
        src += len_in_block;
        remaining -= len_in_block;
    }
    return true;
}

static bool _flash_erase_block(uint32_t block_addr)
{
    return _start_command(block_addr, NVMCTRL_CTRLB_CMD_EB);
}

static bool _flash_program_page(uint32_t page_addr, const uint32_t* page_words)
{
    if (!_start_command_no_addr(NVMCTRL_CTRLB_CMD_PBC) || !_ready_wait()) {
        return false;
    }

    volatile uint32_t* dst = &NVM_MEMORY[page_addr / sizeof(uint32_t)];
    for (uint32_t i = 0; i < FLASH_PAGE_WORDS; i++) {
        dst[i] = page_words[i];
    }

    return _start_command(page_addr, NVMCTRL_CTRLB_CMD_WP);
}
