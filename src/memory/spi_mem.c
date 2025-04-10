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
#include "screen.h"
#include "util.h"
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifndef TESTING
#include "bitbox02_pins.h"
#include <hal_delay.h>
#include <spi_lite.h>
#endif

#define SECTOR_MASK 0xFFFFF000
#define MEMORY_LIMIT (SPI_MEM_MEMORY_SIZE - 1)
#define SR_WIP 0x01
#define CMD_READ 0x03
#define CMD_WREN 0x06
#define CMD_SE 0x20
#define CMD_PP 0x02
#define CMD_RDSR 0x05
#define CMD_CE 0x60

static void _spi_mem_cs_low(void)
{
#ifndef TESTING
    gpio_set_pin_level(PIN_MEM_CS, 0);
#endif
}

static void _spi_mem_cs_high(void)
{
#ifndef TESTING
    gpio_set_pin_level(PIN_MEM_CS, 1);
#endif
}

static uint8_t _spi_mem_read_sr(void)
{
    uint8_t buffer[2] = {0};
    buffer[0] = CMD_RDSR;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 2);
    _spi_mem_cs_high();
    return buffer[1];
}

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

static void _spi_mem_wait(void)
{
    uint8_t status;
    do {
        status = _spi_mem_read_sr();
    } while (status & SR_WIP);
}

void spi_mem_chip_erase(void)
{
    uint8_t buffer[2];

    // --- Enable Write ---
    buffer[0] = CMD_WREN;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 1);
    _spi_mem_cs_high();

    // --- Chip Erase ---
    buffer[0] = CMD_CE;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 1);
    _spi_mem_cs_high();

    _spi_mem_wait();
}

bool spi_mem_sector_erase(uint32_t sector_addr)
{
    if (sector_addr & ~SECTOR_MASK || (sector_addr + SPI_MEM_SECTOR_SIZE - 1) > MEMORY_LIMIT) {
        util_log("Invalid sector address %p", (void*)(uintptr_t)sector_addr);
        return false;
    }

    uint8_t buffer[SPI_MEM_PAGE_SIZE + 4];
    // --- Enable Write ---
    buffer[0] = CMD_WREN;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 1);
    _spi_mem_cs_high();

    // --- Sector Erase (write 4 bytes) ---
    buffer[0] = CMD_SE;
    buffer[1] = (sector_addr >> 16) & 0xFF;
    buffer[2] = (sector_addr >> 8) & 0xFF;
    buffer[3] = sector_addr & 0xFF;

    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 4);
    _spi_mem_cs_high();

    // --- Wait for write to end ---
    _spi_mem_wait();

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
    for (int i = 0; i < (int)size; i++) {
        buffer[i] = buffer[i + 4];
    }
    return buffer;
}

static bool _spi_mem_page_write(uint32_t page_addr, const uint8_t* input)
{
    if (page_addr % SPI_MEM_PAGE_SIZE != 0) {
        util_log("Invalid page write address %p", (void*)(uintptr_t)page_addr);
        return false;
    }

    uint8_t buffer[SPI_MEM_PAGE_SIZE + 4];
    // --- Enable Write ---
    buffer[0] = CMD_WREN;
    _spi_mem_cs_low();
    SPI_MEM_exchange_block(buffer, 1);
    _spi_mem_cs_high();

    // --- Page Program (write 4 bytes) ---
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

    return true;
}

bool spi_mem_write(uint32_t address, const uint8_t* input, size_t size)
{
    if (address + size - 1 > MEMORY_LIMIT || size < 1) {
        util_log("Invalid write address %p or size %i", (void*)(uintptr_t)address, (int)size);
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
    for (int i = 0; i < sectors; i++) {
        uint32_t sector_addr = initial_sector_addr + (i * SPI_MEM_SECTOR_SIZE);
        if (!spi_mem_sector_erase(sector_addr)) {
            free(buffer);
            return false;
        }
        for (int p = 0; p < (SPI_MEM_SECTOR_SIZE / SPI_MEM_PAGE_SIZE); p++) {
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

bool spi_mem_verify_erased(void)
{
    uint8_t buffer[SPI_MEM_BLOCK_SIZE + 4];
    for (int i = 0; i < (SPI_MEM_MEMORY_SIZE / SPI_MEM_BLOCK_SIZE); i++) {
        _spi_mem_read((uint32_t)i * SPI_MEM_BLOCK_SIZE, SPI_MEM_BLOCK_SIZE, buffer);
        for (int j = 0; j < SPI_MEM_BLOCK_SIZE; j++) {
            if (buffer[j + 4] != 0xFF) {
                util_log("Error in block %i at byte %i", i, j);
                return false;
            }
        }
    }

    return true;
}

// This is an utility function, useful to test the code, but not to be merged.
void spi_mem_test(void)
{
    util_log("==== Starting spi_mem_test ====");

    bool success = true;

    // --- Setup test buffers ---
    uint8_t write_data[SPI_MEM_PAGE_SIZE];
    uint8_t read_data[SPI_MEM_PAGE_SIZE];

    for (int i = 0; i < SPI_MEM_PAGE_SIZE; i++) {
        write_data[i] = (uint8_t)i;
    }

    // === Test 1: Valid page write/read at address 0 ===
    uint32_t addr_start = 0x00000000;
    if (!spi_mem_sector_erase(addr_start)) {
        util_log("Test 1: Sector erase failed");
        success = false;
    }

    if (!_spi_mem_page_write(addr_start, write_data)) {
        util_log("Test 1: Page write at 0 failed");
        success = false;
    }

    memset(read_data, 0x00, SPI_MEM_PAGE_SIZE);
    if (!spi_mem_page_read(addr_start, read_data)) {
        util_log("Test 1: Page read at 0 failed");
        success = false;
    }

    if (memcmp(write_data, read_data, SPI_MEM_PAGE_SIZE) != 0) {
        util_log("Test 1: Data mismatch at address 0");
        success = false;
    } else {
        util_log("Test 1: Valid page write/read at address 0 OK");
    }

    // === Test 2: Write and read near memory boundary ===
    uint32_t addr_end = MEMORY_LIMIT - SPI_MEM_PAGE_SIZE + 1;
    if (!spi_mem_sector_erase(addr_end & SECTOR_MASK)) {
        util_log("Test 2: Sector erase failed");
        success = false;
    }

    if (!_spi_mem_page_write(addr_end, write_data)) {
        util_log("Test 2: Page write at end boundary failed");
        success = false;
    }

    memset(read_data, 0x00, SPI_MEM_PAGE_SIZE);
    if (!spi_mem_page_read(addr_end, read_data)) {
        util_log("Test 2: Page read at end boundary failed");
        success = false;
    }

    if (memcmp(write_data, read_data, SPI_MEM_PAGE_SIZE) != 0) {
        util_log("Test 2: Boundary data mismatch");
        success = false;
    } else {
        util_log("Test 2: Write and read near memory boundary OK");
    }

    // === Test 3: Full sector write and read ===
    uint32_t sector_addr = 0x00020000;
    uint8_t sector_write_data[SPI_MEM_SECTOR_SIZE];
    uint8_t* sector_read_data;

    for (int i = 0; i < SPI_MEM_SECTOR_SIZE; i++) {
        sector_write_data[i] = (uint8_t)(i ^ 0xA5);
    }

    if (!spi_mem_sector_erase(sector_addr)) {
        util_log("Test 3: Sector erase failed");
        success = false;
    }
    if (!spi_mem_write(sector_addr, sector_write_data, SPI_MEM_SECTOR_SIZE)) {
        util_log("Test 3: Full sector write failed");
        success = false;
    }

    sector_read_data = spi_mem_read(sector_addr, SPI_MEM_SECTOR_SIZE);
    if (!sector_read_data) {
        util_log("Test 3: Full sector read failed");
        success = false;
    }

    if (memcmp(sector_write_data, sector_read_data, SPI_MEM_SECTOR_SIZE) != 0) {
        util_log("Test 3: Full sector data mismatch");
        success = false;
    } else {
        util_log("Test 3: Full sector write and read OK");
    }

    free(sector_read_data);

    // === Test 4: Erase verification ===
    if (!spi_mem_sector_erase(sector_addr)) {
        util_log("Test 4: Erase failed");
        success = false;
    }
    sector_read_data = spi_mem_read(sector_addr, SPI_MEM_SECTOR_SIZE);
    if (!sector_read_data) {
        util_log("Test 4: Erase read failed");
        success = false;
    }

    bool erased = true;
    for (int i = 0; i < SPI_MEM_SECTOR_SIZE; i++) {
        if (sector_read_data[i] != 0xFF) {
            erased = false;
            break;
        }
    }

    if (!erased) {
        util_log("Test 4: Sector not properly erased");
        success = false;
    } else {
        util_log("Test 4: Sector erase verified OK");
        free(sector_read_data);
    }

    // === Test 5: Invalid read/write addresses ===
    uint32_t invalid_addr = MEMORY_LIMIT + 1;

    uint8_t* tmp = spi_mem_read(invalid_addr, 10);
    if (tmp != NULL) {
        util_log("Test 5: Invalid read not rejected");
        success = false;
        free(tmp);
    }

    if (spi_mem_write(invalid_addr, write_data, 10)) {
        util_log("Test 5: Invalid write not rejected");
        success = false;
    }

    if (spi_mem_sector_erase(invalid_addr)) {
        util_log("Test 5: Invalid sector erase not rejected");
        success = false;
    }
    if (success) {
        util_log("Test 5: Invalid read/write addresses OK");
    }

    // === Test 6: Unaligned page and sector accesses ===
    if (_spi_mem_page_write(sector_addr + 1, write_data)) {
        util_log("Test 6: Unaligned page write not rejected");
        success = false;
    }

    if (spi_mem_page_read(sector_addr + 1, read_data)) {
        util_log("Test 6: Unaligned page read not rejected");
        success = false;
    }

    if (spi_mem_sector_erase(sector_addr + 1)) {
        util_log("Test 6: Unaligned sector erase not rejected");
        success = false;
    }

    if (success) {
        util_log("Test 6: Unaligned page and sector accesses OK");
    }

    // === Test 7: Partial write within a sector ===
    uint32_t partial_addr = sector_addr + 100;
    uint8_t partial_input[32];
    uint8_t* partial_output;

    for (int i = 0; i < 32; i++) {
        partial_input[i] = 0x42 + i;
    }

    if (!spi_mem_sector_erase(sector_addr)) {
        util_log("Test 7: Sector erase failed");
        success = false;
    }
    if (!spi_mem_write(partial_addr, partial_input, 32)) {
        util_log("Test 7: Partial write failed");
        success = false;
    }

    partial_output = spi_mem_read(partial_addr, 32);
    if (!partial_output) {
        util_log("Test 7: Partial read failed");
        success = false;
    } else {
        free(partial_output);
    }

    if (memcmp(partial_input, partial_output, 32) != 0) {
        util_log("Test 7: Partial data mismatch");
        success = false;
    } else {
        util_log("Test 7: Partial write/read OK");
    }

    // === Test 8: Cross-sector write and read ===
    const uint32_t cross_start = 0x00030000 + 129; // Start 128 bytes into a sector
    const size_t cross_size = SPI_MEM_SECTOR_SIZE + 267; // Cross into next sector
    uint8_t cross_input[cross_size];
    uint8_t* cross_output;

    for (int i = 0; i < (int)cross_size; i++) {
        cross_input[i] = (uint8_t)(0x77 + (i % 39));
    }

    // Erase both sectors before writing
    if (!spi_mem_sector_erase(cross_start & SECTOR_MASK)) {
        util_log("Test 8: first sector erase failed");
        success = false;
    }
    if (!spi_mem_sector_erase((cross_start + cross_size - 1) & SECTOR_MASK)) {
        util_log("Test 8: second sector erase failed");
        success = false;
    }

    if (!spi_mem_write(cross_start, cross_input, cross_size)) {
        util_log("Test 8: Cross-sector write failed");
        success = false;
    }

    cross_output = spi_mem_read(cross_start, cross_size);
    if (!cross_output) {
        util_log("Test 8: Cross-sector read failed");
        success = false;
    } else {
        free(cross_output);
    }

    if (memcmp(cross_input, cross_output, cross_size) != 0) {
        util_log("Test 8: Cross-sector data mismatch");
        success = false;
    } else {
        util_log("Test 8: Cross-sector write/read OK");
    }

    // === Test 9: Chip erase test ===
    util_log("Test 9: Chip erase in progress...");
    spi_mem_chip_erase();
    util_log("Test 9: Chip erased. Verification in progress..");
    if (!spi_mem_verify_erased()) {
        util_log("Test 9: Chip erase verification failed");
        success = false;
    } else {
        util_log("Test 9: Chip erase verification completed");
    }

    if (!spi_mem_write(partial_addr, partial_input, 32)) {
        util_log("Test 9: Write failed");
        success = false;
    }

    if (spi_mem_verify_erased()) {
        util_log("Test 9: Chip erase verification should have failed");
        success = false;
    }

    if (success) {
        util_log("Test 9: Chip erase verification OK");
    }

    // === Test 10: Off-by-one address/size edge cases with data verification ===
    util_log("Test 10: Off-by-one address/size checks");

    uint8_t tmp_write = 0xAA;
    uint8_t* tmp_read;
    uint8_t* buffer;

    // 10.1: Last valid byte read/write (should pass)
    if (!spi_mem_write(MEMORY_LIMIT, &tmp_write, 1)) {
        util_log("Test 10.1: Write to last valid byte FAILED");
        success = false;
    } else {
        tmp_read = spi_mem_read(MEMORY_LIMIT, 1);
        if (!tmp_read) {
            util_log("Test 10.1: Read from last valid byte FAILED");
            success = false;
        } else if (*tmp_read != tmp_write) {
            util_log("Test 10.1: Data mismatch at last valid byte");
            success = false;
        } else {
            util_log("Test 10.1: Write/read last valid byte OK");
            free(tmp_read);
        }
    }

    // 10.2: Read 2 bytes from last byte (should fail)

    buffer = spi_mem_read(MEMORY_LIMIT, 2);
    if (buffer) {
        util_log("Test 10.2: Read past memory limit NOT rejected");
        success = false;
        free(buffer);
    } else {
        util_log("Test 10.2: Read past memory limit correctly rejected");
    }

    // 10.3: Read from second-to-last with size 2 (should pass and match data)
    tmp_write = 0x55;
    uint32_t addr10_3 = MEMORY_LIMIT - 1;
    if (!spi_mem_write(addr10_3, &tmp_write, 1)) {
        util_log("Test 10.3: Write before last byte FAILED");
        success = false;
    } else {
        buffer = spi_mem_read(addr10_3, 2);
        if (!buffer) {
            util_log("Test 10.3: Read into last byte FAILED");
            success = false;
        } else if (buffer[0] != tmp_write || buffer[1] != 0xAA) {
            util_log("Test 10.3: Data mismatch at edge read");
            success = false;
        } else {
            util_log("Test 10.3: Read into last byte OK");
        }
        if (buffer) {
            free(buffer);
        }
    }

    // 10.4: Write to last byte, confirm change again
    tmp_write = 0x66;
    if (!spi_mem_write(MEMORY_LIMIT, &tmp_write, 1)) {
        util_log("Test 10.4: Write last byte again FAILED");
        success = false;
    } else {
        tmp_read = spi_mem_read(MEMORY_LIMIT, 1);
        if (!tmp_read) {
            util_log("Test 10.4: Read last byte again FAILED");
            success = false;
        } else if (*tmp_read != tmp_write) {
            util_log("Test 10.4: Last byte not updated correctly");
            success = false;
        } else {
            util_log("Test 10.4: Last byte write/read confirmed OK");
        }
        if (tmp_read) {
            free(tmp_read);
        }
    }

    // 10.5: Zero-size write (should fail)
    if (spi_mem_write(0x00000000, &tmp_write, 0)) {
        util_log("Test 10.5: Zero-size write NOT rejected");
        success = false;
    } else {
        util_log("Test 10.5: Zero-size write correctly rejected");
    }

    // 10.6: Zero-size read (should fail)
    tmp_read = spi_mem_read(0x00000000, 0);
    if (tmp_read) {
        util_log("Test 10.6: Zero-size read NOT rejected");
        success = false;
        free(tmp_read);
    } else {
        util_log("Test 10.6: Zero-size read correctly rejected");
    }

    util_log("==== spi_mem_test %s ====", success ? "PASSED ✅" : "FAILED ❌");
    screen_sprintf_debug(10000, "Test result: %s", success ? "OK" : "FAILED");
}
