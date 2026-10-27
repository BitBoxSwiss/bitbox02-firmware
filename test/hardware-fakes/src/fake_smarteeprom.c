// SPDX-License-Identifier: Apache-2.0

/**
 * Mock for the SmartEEPROM functions.
 */
#include <memory/smarteeprom.h>

#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>

static char* _mem_image = NULL;
static size_t _allocated_space = 0;
static size_t _enabled = 0;

static FILE* _open_image(const char* mode)
{
    const char* base_path = getenv("FAKE_MEMORY_FILEPATH");
    if (base_path == NULL) {
        return NULL;
    }
    // Virtual pages are separate from the raw flash backing in fake_memory.c.
    char path[512];
    int written = snprintf(path, sizeof(path), "%s_smarteeprom", base_path);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        fprintf(stderr, "SmartEEPROM image path too long\n");
        abort();
    }
    FILE* file = fopen(path, mode);
    if (file == NULL && !(mode[0] == 'r' && errno == ENOENT)) {
        perror(path);
        abort();
    }
    return file;
}

static void _persist(void)
{
    FILE* file = _open_image("wb");
    if (file != NULL) {
        bool success = fwrite(_mem_image, 1, _allocated_space, file) == _allocated_space;
        if (fclose(file) != 0 || !success) {
            fprintf(stderr, "Could not write SmartEEPROM image\n");
            abort();
        }
    }
}

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
    FILE* file = _open_image("rb");
    if (file != NULL) {
        bool success = fread(_mem_image, 1, _allocated_space, file) == _allocated_space;
        if (fclose(file) != 0 || !success) {
            fprintf(stderr, "Could not read SmartEEPROM image\n");
            abort();
        }
    }
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
    assert(address + bytes < _allocated_space);
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
    _persist();
}

bool smarteeprom_is_enabled(void)
{
    return _enabled;
}

void smarteeprom_disable(void)
{
    if (!_enabled) {
        return;
    }
    memset(_mem_image, 0xFF, _allocated_space);
    _persist();
    _enabled = 0;
    _allocated_space = 0;
    free(_mem_image);
    _mem_image = NULL;
}

void smarteeprom_bb02_config(void)
{
    if (!_enabled) {
        smarteeprom_setup();
    }
}
