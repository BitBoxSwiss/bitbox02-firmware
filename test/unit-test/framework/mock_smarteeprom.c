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

/**
 * Mock for the SmartEEPROM functions.
 */
#include <memory/smarteeprom.h>

#include <assert.h>
#include <string.h>

static char* _mem_image = NULL;
static size_t _allocated_space = 0;
static size_t _enabled = 0;

void smarteeprom_setup(void)
{
    /* Allocate an equivalent amount of raw memory */
    size_t n_virtual_pages = SMARTEEPROM_ALLOCATED_BLOCKS * 8192 / SMARTEEPROM_PAGE_SIZE;
    /*
     * The maximum number of virtual pages is limited to 128.
     * Ref. SAMD5x/E5x Family Data Sheet sec. 25.6.8.3
     */
    if (n_virtual_pages > 128) {
        n_virtual_pages = 128;
    }
    _allocated_space = n_virtual_pages * SMARTEEPROM_PAGE_SIZE;
    _mem_image = malloc(_allocated_space);
    if (!_mem_image) {
        abort();
    }
    memset(_mem_image, 0xFF, _allocated_space);
    _enabled = 1;
}

/**
 * Reads N contiguous bytes from the SmartEEPROM, starting at
 * the specified address.
 *
 * @param[in] address Start address.
 * @param[in] size Number of bytes to read.
 * @param[out] out_buffer Buffer in which to read.
 *                        Must be at least size bytes wide.
 */
void smarteeprom_read(size_t address, size_t bytes, uint8_t* out_buffer)
{
    assert(address < _allocated_space);
    assert(_enabled);
    memcpy(out_buffer, _mem_image + address, bytes);
}

/**
 * Writes N contiguous bytes to the SmartEEPROM, starting at
 * the specified address.
 *
 * Note that this will not affect the write endurance of the
 * underlying memory, unless the content currently stored at
 * the specified address has some bits that must be flipped from
 * "0" to "1". Use this wisely!
 *
 * @param[in] address Start address.
 * @param[in] size Number of bytes to write.
 * @param[in] buffer Buffer from which to read the data.
 *                   Must be at least size bytes wide.
 */
void smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer)
{
    assert(_allocated_space > bytes && address < _allocated_space - bytes);
    assert(_enabled);
    memcpy(_mem_image + address, buffer, bytes);
}

bool smarteeprom_is_enabled(void)
{
    return _enabled;
}

void smarteeprom_disable(void)
{
    _enabled = 0;
    _allocated_space = 0;
    free(_mem_image);
}

void smarteeprom_bb02_config(void)
{
    if (!_enabled) {
        smarteeprom_setup();
    }
}
