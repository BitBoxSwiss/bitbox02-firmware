// Copyright 2025 Shift Crypto AG
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

#include "spi_mem.h"
#include "bitbox02_pins.h"
#include "random.h"
#include "screen.h"
#include "util.h"
#include <hal_delay.h>
#include <spi_lite.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SECTOR_MASK 0xFFFFF000
#define MEMORY_LIMIT (SPI_MEM_MEMORY_SIZE - 1)
#define SR_WIP 0x01
#define SR_PROTECT_BITS_MASK 0x3C
#define SR_PROTECT_BITS_SHIFT 2
#define SR_PROTECT_BITS (SPI_MEM_PROTECTED_BLOCKS << SR_PROTECT_BITS_SHIFT)
#define CR1_TB_BIT_BOTTOM 0x8
#define CR1_TB_BIT_MASK 0x8
#define CMD_READ 0x03
#define CMD_WREN 0x06
#define CMD_WRSR 0x01
#define CMD_SE 0x20
#define CMD_PP 0x02
#define CMD_RDSR 0x05
#define CMD_RDCR 0x15
#define CMD_CE 0x60

// Drives the chip select pin low
static void _spi_mem_cs_low(void)
{
    gpio_set_pin_level(PIN_MEM_CS, 0);
}

// Drives the chip select pin high
static void _spi_mem_cs_high(void)
{
    gpio_set_pin_level(PIN_MEM_CS, 1);
}

// Reads the status register
static uint8_t _spi_mem_read_sr(void)
{
    uint8_t buffer[2] = {0};
    buffer[0] = CMD_RDSR;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 2);
    _spi_mem_cs_high();
    return buffer[1];
}

// Reads the configuration register
static void _spi_mem_read_cr(uint8_t* data_out)
{
    uint8_t buffer[3] = {0};
    buffer[0] = CMD_RDCR;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 3);
    _spi_mem_cs_high();

    memcpy(data_out, &buffer[1], 2);
}

// Waits until the WIP bits goes low
static void _spi_mem_wait(void)
{
    uint8_t status;
    do {
        status = _spi_mem_read_sr();
    } while (status & SR_WIP);
}

// Set write enable bit
static void _spi_mem_write_enable(void)
{
    uint8_t cmd = CMD_WREN;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(&cmd, 1);
    _spi_mem_cs_high();
}

// Verify if the given address is protected
static bool _spi_mem_verify_address_protected(uint32_t address)
{
    uint8_t protected_blocks = (_spi_mem_read_sr() & SR_PROTECT_BITS_MASK) >> SR_PROTECT_BITS_SHIFT;
    if (address < (protected_blocks * SPI_MEM_BLOCK_SIZE)) {
        return true;
    }
    return false;
}

// Write the status and configuration registers
static void _spi_mem_write_sr(uint8_t* data_in)
{
    _spi_mem_write_enable();
    uint8_t buffer[4] = {0};
    buffer[0] = CMD_WRSR;
    memcpy(&buffer[1], data_in, 3);
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 4);
    _spi_mem_cs_high();
    _spi_mem_wait();
}

// Reads `size` bytes starting from `address` and writes the data into `buffer`
static void _spi_mem_read(uint32_t address, size_t size, uint8_t* buffer)
{
    buffer[0] = CMD_READ;
    buffer[1] = (address >> 16) & 0xFF;
    buffer[2] = (address >> 8) & 0xFF;
    buffer[3] = address & 0xFF;
    memset(&buffer[4], 0x00, size);

    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, size + 4);
    _spi_mem_cs_high();
}

bool spi_mem_full_erase(void)
{
    if (_spi_mem_read_sr() & SR_PROTECT_BITS_MASK) {
        util_log("Cannot erase with protected area locked.");
        return false;
    }
    _spi_mem_write_enable();

    uint8_t cmd = CMD_CE;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(&cmd, 1);
    _spi_mem_cs_high();

    _spi_mem_wait();
    return true;
}

bool spi_mem_sector_erase(uint32_t sector_addr)
{
    if (sector_addr & ~SECTOR_MASK || (sector_addr + SPI_MEM_SECTOR_SIZE - 1) > MEMORY_LIMIT) {
        util_log("Invalid sector address %p", (void*)(uintptr_t)sector_addr);
        return false;
    }
    if (_spi_mem_verify_address_protected(sector_addr)) {
        util_log("Sector address %p protected", (void*)(uintptr_t)sector_addr);
        return false;
    }

    _spi_mem_write_enable();

    // --- Sector Erase (write 4 bytes) ---
    uint8_t buffer[SPI_MEM_PAGE_SIZE + 4];
    buffer[0] = CMD_SE;
    buffer[1] = (sector_addr >> 16) & 0xFF;
    buffer[2] = (sector_addr >> 8) & 0xFF;
    buffer[3] = sector_addr & 0xFF;

    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 4);
    _spi_mem_cs_high();

    // --- Wait for write to end ---
    _spi_mem_wait();

    // --- Check that sector has been actually erased ---
    uint8_t page_data[SPI_MEM_PAGE_SIZE];
    for (size_t i = 0; i < SPI_MEM_SECTOR_SIZE / SPI_MEM_PAGE_SIZE; i++) {
        uint32_t page_addr = sector_addr + i * SPI_MEM_PAGE_SIZE;
        if (!spi_mem_page_read(page_addr, page_data)) {
            util_log("Read after sector erase at %p failed", (void*)(uintptr_t)page_addr);
            return false;
        }
        for (size_t j = 0; j < SPI_MEM_PAGE_SIZE; j++) {
            if (page_data[j] != 0xFF) {
                util_log("Sector erase at %p failed", (void*)(uintptr_t)(page_addr + j));
                return false;
            }
        }
    }
    return true;
}

bool spi_mem_page_read(uint32_t page_addr, uint8_t* data_out)
{
    if (page_addr % SPI_MEM_PAGE_SIZE != 0) {
        util_log("Invalid page read address %p", (void*)(uintptr_t)page_addr);
        return false;
    }

    uint8_t tmp_buf[SPI_MEM_PAGE_SIZE + 4];
    _spi_mem_read(page_addr, SPI_MEM_PAGE_SIZE, tmp_buf);
    memcpy(data_out, &tmp_buf[4], SPI_MEM_PAGE_SIZE);
    return true;
}

uint8_t* spi_mem_read(uint32_t address, size_t size)
{
    if (address + size - 1 > MEMORY_LIMIT || size < 1) {
        util_log("Invalid read address %p or size %i", (void*)(uintptr_t)address, (int)size);
        return NULL;
    }

    uint8_t* buffer = malloc(size + 4);
    if (!buffer) {
        util_log("Memory allocation failed");
        return NULL;
    }

    _spi_mem_read(address, size, buffer);

    // shift the read data at the beginning of the buffer, overriding the command and the address.
    for (size_t i = 0; i < size; i++) {
        buffer[i] = buffer[i + 4];
    }
    return buffer;
}

// Writes SPI_MEM_PAGE_SIZE bytes from `input` at `page_addr`
static bool _spi_mem_page_write(uint32_t page_addr, const uint8_t* input)
{
    if (page_addr % SPI_MEM_PAGE_SIZE != 0) {
        util_log("Invalid page write address %p", (void*)(uintptr_t)page_addr);
        return false;
    }

    if (_spi_mem_verify_address_protected(page_addr)) {
        util_log("Page address %p protected", (void*)(uintptr_t)page_addr);
        return false;
    }

    _spi_mem_write_enable();

    // --- Page Program (write 4 bytes) ---
    uint8_t buffer[SPI_MEM_PAGE_SIZE + 4];
    buffer[0] = CMD_PP;
    buffer[1] = (page_addr >> 16) & 0xFF;
    buffer[2] = (page_addr >> 8) & 0xFF;
    buffer[3] = page_addr & 0xFF;
    memcpy(&buffer[4], input, SPI_MEM_PAGE_SIZE);

    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 4 + SPI_MEM_PAGE_SIZE);
    _spi_mem_cs_high();

    // --- Wait for write to end ---
    _spi_mem_wait();

    // --- Check that input data has been properly written ---
    uint8_t read_data[SPI_MEM_PAGE_SIZE];
    if (!spi_mem_page_read(page_addr, read_data)) {
        util_log("Read after page write at %p failed", (void*)(uintptr_t)page_addr);
        return false;
    }
    if (memcmp(read_data, input, SPI_MEM_PAGE_SIZE) != 0) {
        util_log("Write page at %p failed", (void*)(uintptr_t)page_addr);
        return false;
    }
    return true;
}

bool spi_mem_write(uint32_t address, const uint8_t* input, size_t size)
{
    if (address + size - 1 > MEMORY_LIMIT || size < 1) {
        util_log("Invalid write address %p or size %i", (void*)(uintptr_t)address, (int)size);
        return false;
    }

    if (_spi_mem_verify_address_protected(address)) {
        util_log("Address %p protected", (void*)(uintptr_t)address);
        return false;
    }

    uint32_t initial_sector_addr = address & SECTOR_MASK;
    uint32_t final_sector_addr = ((address + size - 1) & SECTOR_MASK) + SPI_MEM_SECTOR_SIZE;
    uint16_t sectors = (final_sector_addr - initial_sector_addr) / SPI_MEM_SECTOR_SIZE;

    // read all the affected sectors data
    uint8_t* buffer = spi_mem_read(initial_sector_addr, (size_t)(sectors * SPI_MEM_SECTOR_SIZE));
    if (!buffer) {
        return false;
    }

    // update data in the buffer
    memcpy(&buffer[address - initial_sector_addr], input, size);

    // erase sectors and write data
    for (uint32_t i = 0; i < sectors; i++) {
        uint32_t sector_addr = initial_sector_addr + (i * SPI_MEM_SECTOR_SIZE);
        if (!spi_mem_sector_erase(sector_addr)) {
            free(buffer);
            return false;
        }
        for (uint32_t p = 0; p < (SPI_MEM_SECTOR_SIZE / SPI_MEM_PAGE_SIZE); p++) {
            uint32_t page_addr = sector_addr + p * SPI_MEM_PAGE_SIZE;
            if (!_spi_mem_page_write(
                    page_addr, &buffer[(i * SPI_MEM_SECTOR_SIZE) + (p * SPI_MEM_PAGE_SIZE)])) {
                free(buffer);
                return false;
            }
        }
    }
    free(buffer);
    return true;
}

int32_t spi_mem_smart_erase(void)
{
    uint32_t erased_sectors = 0;
    uint8_t buffer[SPI_MEM_SECTOR_SIZE + 4];
    for (uint32_t i = 0; i < (SPI_MEM_MEMORY_SIZE / SPI_MEM_SECTOR_SIZE); i++) {
        _spi_mem_read(i * SPI_MEM_SECTOR_SIZE, SPI_MEM_SECTOR_SIZE, buffer);
        for (size_t j = 0; j < SPI_MEM_SECTOR_SIZE; j++) {
            if (buffer[j + 4] != 0xFF) {
                util_log(
                    "Sector at address 0x%06X not erased. Erasing...",
                    (unsigned int)(i * SPI_MEM_SECTOR_SIZE));
                if (!spi_mem_sector_erase(i * SPI_MEM_SECTOR_SIZE)) {
                    util_log("Error erasing sector.");
                    return -1;
                }
                erased_sectors++;
                break;
            }
        }
    }

    return erased_sectors;
}

// Writes the `protection` bits into the status register and sets the
// Top/Bottom bit to bottom in the configuration register.
static void _spi_mem_set_protection(uint8_t protection)
{
    uint8_t reg[3];
    reg[0] = _spi_mem_read_sr();
    _spi_mem_read_cr(&reg[1]);

    // clean and update status register with protection bits
    reg[0] &= ~SR_PROTECT_BITS_MASK;
    reg[0] |= protection & SR_PROTECT_BITS_MASK;

    // set the top/bottom protection bit.
    // This is an OTP bit,so the write will have an effect
    // only the first time.
    reg[1] = reg[1] | CR1_TB_BIT_BOTTOM;

    _spi_mem_write_sr(reg);
}

void spi_mem_protected_area_lock(void)
{
    _spi_mem_set_protection(SR_PROTECT_BITS);
}

void spi_mem_protected_area_unlock(void)
{
    _spi_mem_set_protection(0x0);
}

bool spi_mem_protected_area_write(uint32_t address, const uint8_t* input, size_t size)
{
    // Additional assert to simplify debug.
    ASSERT(_spi_mem_verify_address_protected(address + size));
    if (!_spi_mem_verify_address_protected(address + size)) {
        util_log(
            "Write address %p and size %i outside protected area",
            (void*)(uintptr_t)address,
            (int)size);
        return false;
    }
    uint8_t protection = _spi_mem_read_sr() & SR_PROTECT_BITS_MASK;
    _spi_mem_set_protection(0x0);
    bool result = spi_mem_write(address, input, size);
    _spi_mem_set_protection(protection);

    return result;
}
