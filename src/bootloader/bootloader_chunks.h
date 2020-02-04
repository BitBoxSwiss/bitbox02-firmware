#ifndef __BOOTLOADER_CHUNKS_H
#define __BOOTLOADER_CHUNKS_H

#include <stdint.h>

#include <flags.h>

#define FIRMWARE_CHUNK_LEN (8U * FLASH_PAGE_SIZE) // 4kB
#define FIRMWARE_MAX_NUM_CHUNKS \
    (FLASH_APP_LEN / FIRMWARE_CHUNK_LEN) // app len must be a multiple of chunk len
#if (FIRMWARE_MAX_NUM_CHUNKS > UINT8_MAX)
#error "incompatible variable type"
#endif

uint8_t bootloader_chunks_write_chunk(uint32_t address, const uint8_t* data);

#endif // __BOOTLOADER_CHUNKS_H
